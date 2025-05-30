use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    hostgroup::model::ZabbixHostGroupId, r#macro::model::ZabbixHostMacro,
    template::model::ZabbixTemplate,
};

use super::model::{ZabbixHostInterface, ZabbixHostTag};

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/create
#[derive(Serialize, Debug)]
pub struct CreateHostRequest {
    pub host: String,
    pub groups: Vec<ZabbixHostGroupId>,
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
