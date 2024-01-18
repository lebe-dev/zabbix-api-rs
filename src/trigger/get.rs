use serde::Serialize;

#[derive(Serialize,Debug)]
pub struct GetTriggerRequest {
    #[serde(rename = "triggerids")]
    pub trigger_ids: String,
    pub output: String,
    #[serde(rename = "selectFunctions")]
    pub select_functions: String
}