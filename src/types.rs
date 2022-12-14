use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone)]
pub struct Finger {
    /// Username
    pub username: String,
    /// Real name
    pub realname: String,
    /// Free form location
    pub location: Option<String>,
    /// Text description
    pub description: Option<String>,
    /// URL for profile picture
    pub image: Option<String>,
    /// Unordered list of site names and URLs
    pub links: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct FingerPlan {
    pub finger: Finger,
    pub plan: String,
    #[serde(default)]
    pub friends: Vec<Friend>,
    pub self_link: Option<String>,
    pub plan_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct FingerPlanHistory {
    pub finger: Finger,
    pub plan_history: PlanHistory,
}

#[derive(Deserialize, Serialize)]
pub struct Feed {
    pub finger: Finger,
    pub friends_plans: Vec<FingerPlan>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Friend {
    pub username: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PlanHistory {
    pub entries: Vec<PlanHistoryEntry>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PlanHistoryEntry {
    pub date: DateTime<Utc>,
    pub plan: String,
}

impl Friend {
    pub fn load_finger(&self) -> anyhow::Result<FingerPlan> {
        let uri = self.url.to_owned() + "uc";

        let res =
            spin_sdk::http::send(http::Request::builder().method("GET").uri(uri).body(None)?)?;
        match res.body() {
            Some(bytes) => {
                let mut fp: FingerPlan = serde_json::from_slice(bytes)?;
                let rendered = super::md_to_html(&fp.plan);
                fp.plan = rendered;
                fp.self_link = Some(self.url.to_owned());
                Ok(fp)
            }
            None => Ok(FingerPlan {
                finger: Finger {
                    username: self.username.clone(),
                    realname: "Unknown".to_owned(),
                    location: None,
                    description: None,
                    image: None,
                    links: None,
                },
                plan: "".to_owned(),
                plan_date: None,
                friends: vec![],
                self_link: None,
            }),
        }
    }
}

/// This models the form data sent from the /plan/edit page.
#[derive(Deserialize, Serialize)]
pub struct EditPlanQueryParams {
    pub plan: String,
}
