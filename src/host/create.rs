use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    hostgroup::model::ZabbixHostGroupId, r#macro::model::ZabbixHostMacro,
    template::model::ZabbixTemplate,
};

use super::model::{ZabbixHostInterface, ZabbixHostTag};

const PSK: u8 = 2;
const CERT: u8 = 4;

#[derive(Serialize, Debug)]
pub struct TlsConfig {
    tls_connect: u8,
    tls_accept: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_psk_identity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_psk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_subject: Option<String>,
}

impl TlsConfig {
    pub fn new_psk(psk_identity: String, psk: String) -> TlsConfig {
        TlsConfig {
            tls_connect: PSK,
            tls_accept: PSK,
            tls_psk_identity: Some(psk_identity),
            tls_psk: Some(psk),
            tls_issuer: None,
            tls_subject: None,
        }
    }

    pub fn new_cert(issuer: String, subject: String) -> TlsConfig {
        TlsConfig {
            tls_connect: CERT,
            tls_accept: CERT,
            tls_psk_identity: None,
            tls_psk: None,
            tls_issuer: Some(issuer),
            tls_subject: Some(subject),
        }
    }
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub tls_config: Option<TlsConfig>,
}

impl Default for CreateHostRequest {
    fn default() -> CreateHostRequest {
        CreateHostRequest {
            host: "".to_string(),
            groups: Vec::new(),
            interfaces: Vec::new(),
            tags: Vec::new(),
            templates: Vec::new(),
            macros: Vec::new(),
            inventory_mode: 0,
            inventory: HashMap::new(),
            tls_config: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateHostResponse {
    #[serde(rename = "hostids")]
    pub host_ids: Vec<String>,
}
