use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct ZabbixHost {
    #[serde(rename = "hostid")]
    pub host_id: String,
    pub host: String
}