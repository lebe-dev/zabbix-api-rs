use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{r#macro::model::ZabbixHostMacro, template::model::ZabbixTemplate};

use super::model::{ZabbixHostGroup, ZabbixHostInterface, ZabbixHostTag};

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

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/create
#[derive(Serialize, Debug)]
pub struct CreateHostRequest {
    pub host: String,
    pub groups: Vec<ZabbixHostGroup>,
    pub interfaces: Vec<ZabbixHostInterface>,
    pub tags: Vec<ZabbixHostTag>,
    pub templates: Vec<ZabbixTemplate>,
    pub macros: Vec<ZabbixHostMacro>,
    pub inventory_mode: u8,
    pub inventory: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct CreateHostResponse {
    #[serde(rename = "hostids")]
    pub host_ids: Vec<String>,
}
