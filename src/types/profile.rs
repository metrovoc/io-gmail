use alloc::string::String;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub email_address: String,
    #[serde(default)]
    pub messages_total: Option<u64>,
    #[serde(default)]
    pub threads_total: Option<u64>,
    #[serde(default)]
    pub history_id: Option<String>,
}
