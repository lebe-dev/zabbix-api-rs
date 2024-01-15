use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

#[derive(Serialize)]
pub struct ZabbixApiRequest<T: Serialize> {
    pub jsonrpc: String,
    pub method: String,
    pub params: T,
    pub id: i8,
    pub auth: Option<String>
}

#[derive(Deserialize)]
pub struct ZabbixApiResponse<R> {
    pub jsonrpc: String,
    pub result: R,
    pub id: i8,
}