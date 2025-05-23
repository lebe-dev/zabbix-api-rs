use serde::{Deserialize, Serialize};

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/hostgroup/create
#[derive(Serialize, Debug)]
pub struct CreateHostGroupRequest {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateHostGroupResponse {
    #[serde(rename = "groupids")]
    pub group_ids: Vec<String>,
}
