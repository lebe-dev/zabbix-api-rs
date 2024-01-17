use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::host::{ZabbixHostGroup, ZabbixHostInterface, ZabbixHostTag};
use crate::r#macro::ZabbixHostMacro;
use crate::template::ZabbixTemplate;

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/create
#[derive(Serialize,Debug)]
pub struct CreateHostRequest {
    pub host: String,
    pub groups: Vec<ZabbixHostGroup>,
    pub interfaces: Vec<ZabbixHostInterface>,
    pub tags: Vec<ZabbixHostTag>,
    pub templates: Vec<ZabbixTemplate>,
    pub macros: Vec<ZabbixHostMacro>,
    pub inventory_mode: u8,
    pub inventory: HashMap<String, String>
}

#[derive(Deserialize,Debug)]
pub struct CreateHostResponse {
    #[serde(rename = "hostids")]
    pub host_ids: Vec<String>
}