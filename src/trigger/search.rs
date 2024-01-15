use serde::Deserialize;

use crate::trigger::ZabbixTrigger;

#[derive(Deserialize)]
pub struct TriggerSearchResponse {
    pub result: Vec<ZabbixTrigger>
}