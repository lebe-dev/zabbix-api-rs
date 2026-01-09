use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::cmp::PartialEq;
use std::str::FromStr;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum HostStatus {
    Enabled = 0,
    Disabled = 1,
}

impl FromStr for HostStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(HostStatus::Enabled),
            "1" => Ok(HostStatus::Disabled),
            _ => Err(()),
        }
    }
}

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/object
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct ZabbixHost {
    #[serde(rename = "hostid")]
    pub host_id: String,
    pub host: String,
    pub status: HostStatus,
}

// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/object#host-tag
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ZabbixHostTag {
    pub tag: String,
    pub value: String,
}

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/hostinterface/object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ZabbixHostInterface {
    pub r#type: u8,

    pub main: u8,

    pub ip: String,

    pub dns: String,

    pub port: String,

    #[serde(rename = "useip")]
    pub use_ip: u8,
}
