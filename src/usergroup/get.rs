use serde::Serialize;

#[derive(Serialize, Debug, Default)]
pub struct GetUserGroupsRequest<F: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<F>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usrgrpids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "selectUsers")]
    pub select_users: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "selectRights")]
    pub select_rights: Option<String>,
}

#[derive(Serialize, Debug, Default)]
pub struct UserGroupFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<String>>,
}
