use serde::Serialize;

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/item/get
#[derive(Serialize,Debug)]
pub struct GetHostGroupsRequest<R> {
    pub output: String,
    pub filter: R
}