use serde::Serialize;

pub const JSON_RPC_VERSION: &str = "2.0";

#[cfg(feature = "v7")]
use super::v7::request::ZabbixApiRequest;

#[cfg(feature = "v7")]
pub fn get_api_request<T: Serialize>(
    method: &str,
    params: T,
    _session: Option<String>,
) -> ZabbixApiRequest<T> {
    ZabbixApiRequest {
        jsonrpc: JSON_RPC_VERSION.to_string(),
        method: method.to_string(),
        params,
        id: 1,
    }
}

#[cfg(feature = "v6")]
use super::v6::request::ZabbixApiRequest;

#[cfg(feature = "v6")]
pub fn get_api_request<T: Serialize>(
    method: &str,
    params: T,
    session: Option<String>,
) -> ZabbixApiRequest<T> {
    ZabbixApiRequest {
        jsonrpc: JSON_RPC_VERSION.to_string(),
        method: method.to_string(),
        params,
        id: 1,
        auth: session,
    }
}
