use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ZabbixItem {
    pub name: String,

    pub key_: String,

    #[serde(rename = "hostid")]
    pub host_id: String
}