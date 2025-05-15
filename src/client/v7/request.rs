use serde::Serialize;

#[derive(Serialize)]
pub struct ZabbixApiRequest<T: Serialize> {
    pub jsonrpc: String,
    pub method: String,
    pub params: T,
    pub id: i8,
}
