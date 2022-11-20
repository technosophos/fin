use anyhow::{anyhow, Result};

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
pub fn read_plan(username: &str) -> Result<String> {
    let key = format!("{}-plan", username);
    get_string(key)
}

/// Write the plan for a given user
pub fn write_plan(username: &str, plan: String) -> Result<()> {
    let key = format!("{}-plan", username);
    set_string(key, plan)
}
