use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct UserGroupId {
    pub usrgrpid: String,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct UserMedia {
    pub mediatypeid: String,
    pub sendto: String,
    pub active: i32,
    pub severity: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct CreateUserRequest {
    pub username: String,
    pub passwd: String,
    pub roleid: String,
    pub usrgrps: Vec<UserGroupId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autologin: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autologout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_medias: Option<Vec<UserMedia>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateUserResponse {
    #[serde(rename = "userids")]
    pub user_ids: Vec<String>,
}
