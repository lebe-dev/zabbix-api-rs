use serde::Deserialize;

use crate::error::ZabbixError;

#[derive(Deserialize, Debug)]
pub struct ZabbixApiResponse<R> {
    pub jsonrpc: String,
    pub result: Option<R>,
    pub id: i8,
    pub error: Option<ZabbixError>,
}
