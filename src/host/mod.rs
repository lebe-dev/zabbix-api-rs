use serde::Deserialize;

#[derive(Deserialize)]
pub struct ZabbixHost {
    #[serde(rename = "hostid")]
    pub host_id: String,
    pub host: String
}