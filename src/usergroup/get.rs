use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Serialize, Debug, Default)]
#[skip_serializing_none]
pub struct GetUserGroupsRequest<F: Serialize> {
    pub output: Option<String>,
    pub filter: Option<F>,
    pub usrgrpids: Option<Vec<String>>,
    pub userids: Option<Vec<String>>,
    pub status: Option<i32>,
    #[serde(rename = "selectUsers")]
    pub select_users: Option<String>,
    #[serde(rename = "selectRights")]
    pub select_rights: Option<String>,
}

#[derive(Serialize, Debug, Default)]
#[skip_serializing_none]
pub struct UserGroupFilter {
    pub name: Option<Vec<String>>,
}
