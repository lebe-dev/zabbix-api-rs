use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Debug, Clone)]
pub struct UserGroupId {
    pub usrgrpid: String,
}

#[derive(Serialize, Debug, Clone, Default)]
#[skip_serializing_none]
pub struct UserMedia {
    pub mediatypeid: String,
    pub sendto: String,
    pub active: i32,
    pub severity: i32,
    pub period: Option<String>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[skip_serializing_none]
pub struct CreateUserRequest {
    pub username: String,
    pub passwd: String,
    pub roleid: String,
    pub usrgrps: Vec<UserGroupId>,

    pub name: Option<String>,
    pub surname: Option<String>,
    pub url: Option<String>,
    pub autologin: Option<i32>,
    pub autologout: Option<String>,
    pub lang: Option<String>,
    pub refresh: Option<String>,
    pub theme: Option<String>,
    #[serde(rename = "type")]
    pub user_type: Option<i32>,
    pub user_medias: Option<Vec<UserMedia>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateUserResponse {
    #[serde(rename = "userids")]
    pub user_ids: Vec<String>,
}
