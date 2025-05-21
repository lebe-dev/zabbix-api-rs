use serde::Serialize;

/// API: https://www.zabbix.com/documentation/7.0/en/manual/api/reference/hostgroup/get
#[derive(Serialize, Debug)]
pub struct GetHostGroupsRequest<R> {
    pub output: String,
    pub filter: R,
}
