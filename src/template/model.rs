use serde::{Deserialize, Serialize};

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/template/object
#[derive(Serialize, Deserialize, Debug)]
pub struct ZabbixTemplate {
    #[serde(rename = "templateid")]
    pub template_id: String,
    pub host: String,
    pub description: String,
    pub name: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ZabbixTemplateId {
    #[serde(rename = "templateid")]
    pub template_id: String,
}

impl From<ZabbixTemplate> for ZabbixTemplateId {
    fn from(value: ZabbixTemplate) -> Self {
        ZabbixTemplateId {
            template_id: value.template_id,
        }
    }
}
