use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::PartialEq;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum HostStatus {
    Enabled,
    Disabled,
}

impl Serialize for HostStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            HostStatus::Enabled => "0",
            HostStatus::Disabled => "1",
        };

        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for HostStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        HostStatus::from_str(&value).map_err(|_| serde::de::Error::custom(format!("Invalid HostStatus value: {}", value)))
    }
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

    #[serde(rename = "useip")]
    pub use_ip: u8,
}
