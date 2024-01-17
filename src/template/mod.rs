use serde::{Deserialize, Serialize};

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/template/object
#[derive(Serialize,Deserialize,Debug)]
pub struct ZabbixTemplate {
    #[serde(rename = "templateid")]
    pub template_id: String,
    pub host: String,
    pub description: String,
    pub name: String,
    pub uuid: String
}