use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ZabbixUser {
    #[serde(rename = "userid")]
    pub user_id: String,
    #[serde(alias = "username")]
    pub alias: String,
    pub name: Option<String>,
    pub surname: Option<String>,
    #[serde(rename = "roleid")]
    pub role_id: Option<String>,
    #[serde(rename = "type")]
    pub user_type: Option<i32>,
    pub url: Option<String>,
}
