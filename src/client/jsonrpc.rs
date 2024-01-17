use serde::{Deserialize, Serialize};

use crate::error::ZabbixError;

#[derive(Serialize)]
pub struct ZabbixApiRequest<T: Serialize> {
    pub jsonrpc: String,
    pub method: String,
    pub params: T,
    pub id: i8,
    pub auth: Option<String>
}

#[derive(Deserialize,Debug)]
pub struct ZabbixApiResponse<R> {
    pub jsonrpc: String,
    pub result: Option<R>,
    pub id: i8,
    pub error: Option<ZabbixError>
}