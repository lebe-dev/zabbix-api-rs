use serde::Serialize;

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/get
#[derive(Serialize, Debug)]
pub struct GetHostsRequest<R> {
    pub filter: R,
}

#[derive(Serialize, Debug)]
pub struct GetHostsByIdsRequest {
    pub hostids: Vec<String>,
}
