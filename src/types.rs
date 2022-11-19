use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct FingerPlan {
    pub finger: Finger,
    pub plan: String,
}
