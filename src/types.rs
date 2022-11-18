use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Finger {
    pub username: String,
    pub realname: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub links: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize)]
pub struct FingerPlan {
    pub finger: Finger,
    pub plan: String,
}
