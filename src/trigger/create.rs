use serde::{Deserialize, Serialize};

use crate::error::ZabbixError;

#[derive(Serialize)]
pub struct TriggerCreateRequestParams {
    pub description: String,
    pub expression: String,
    pub priority: String,
    pub url: String
}

#[derive(Deserialize)]
pub struct CreateTriggerResponse {
    pub error: Option<ZabbixError>
}