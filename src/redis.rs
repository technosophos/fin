use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};

use crate::types::{PlanHistory, PlanHistoryEntry};

const REDIS_HOST: &str = "REDIS_HOST";

pub fn get_string(key: String) -> Result<String> {
    let address = std::env::var(REDIS_HOST)?;

    let value =
        spin_sdk::redis::get(&address, &key).map_err(|_| anyhow!("Error getting from Redis"))?;

    Ok(String::from_utf8(value)?)
}

pub fn set_string(key: String, value: String) -> Result<()> {
    let address = std::env::var(REDIS_HOST)?;
    spin_sdk::redis::set(&address, &key, value.as_bytes())
        .map_err(|_| anyhow!("Error setting to Redis"))
}

/// Read the plan of the given user
///
/// As of v8, this reads the latest plan from the history
pub fn read_plan(username: &str) -> Result<PlanHistoryEntry> {
    let history = read_plan_history(username)?;
    if history.entries.is_empty() {
        return Ok(PlanHistoryEntry {
            plan: String::new(),
            date: Utc::now(),
        });
    }
    Ok(history.entries[0].clone())
}

/// Write the plan for a given user
///
/// As of v8, this writes the plan to the history.
pub fn write_plan(username: &str, plan: String) -> Result<()> {
    let now = Utc::now();
    write_plan_history(username, plan, now)
}

/// Read the plan of the given user
pub fn read_plan_history(username: &str) -> Result<PlanHistory> {
    let key = format!("{}-plan-history", username);
    let raw = get_string(key)?;
    if raw.is_empty() {
        return Ok(PlanHistory { entries: vec![] });
    }
    let history = serde_json::from_str(&raw)?;
    Ok(history)
}

pub fn write_plan_history(username: &str, plan: String, date: DateTime<Utc>) -> Result<()> {
    // Get the plan history
    let mut history = read_plan_history(username)?;
    // Update the plan history, shifting this value onto the top.
    history.entries.insert(0, PlanHistoryEntry { date, plan });

    let value = serde_json::to_string(&history)?;

    // TODO: This is currently not atomic.
    let key = format!("{}-plan-history", username);
    set_string(key, value)
}
