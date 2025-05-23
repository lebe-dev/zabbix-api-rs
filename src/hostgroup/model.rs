use serde::{Deserialize, Serialize};

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/hostgroup/object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ZabbixHostGroup {
    pub name: String,
    #[serde(rename = "groupid")]
    pub group_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ZabbixHostGroupId {
    #[serde(rename = "groupid")]
    pub group_id: String,
}

impl From<ZabbixHostGroup> for ZabbixHostGroupId {
    fn from(value: ZabbixHostGroup) -> Self {
        ZabbixHostGroupId {
            group_id: value.group_id,
        }
    }
}
