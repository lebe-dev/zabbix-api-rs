use serde::{Deserialize, Serialize};
use crate::host::model::HostStatus;

/// Represents a host update request in Zabbix API
#[derive(Debug, Serialize)]
pub struct UpdateHostRequest {
    /// The ID of the host to update
    pub hostid: String,
    pub status: HostStatus,
}

impl UpdateHostRequest {
    /// Creates a new `UpdateHost` with the given host ID and disabled status (status = 1)
    ///
    /// # Arguments
    /// * `hostid` - The ID of the host to disable
    ///
    /// # Returns
    /// A new `UpdateHost` instance with status set to 1 (disabled)
    pub fn disable_host(hostid: String) -> Self {
        Self {
            hostid,
            status: HostStatus::Disabled,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdateHostResponse {
    #[serde(rename = "hostids")]
    pub host_ids: Vec<String>,
}