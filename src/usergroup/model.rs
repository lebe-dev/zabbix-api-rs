use serde::{Deserialize, Serialize};

/// Represents the permissions for a host group or template group within a user group.
/// Corresponds to the "Permission" object in Zabbix API documentation.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserGroupPermission {
    /// ID of the host group or template group.
    pub id: String,
    /// Access level to the host group or template group.
    /// Possible values:
    /// 0 - access denied;
    /// 2 - read-only access;
    /// 3 - read-write access.
    pub permission: i32,
}

/// Represents a tag-based permission for a user group.
/// Corresponds to the "Tag-based permission" object in Zabbix API documentation.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserGroupTagFilter {
    /// ID of the host group to add permission to.
    pub groupid: String,
    /// Tag name.
    pub tag: String,
    /// Tag value.
    pub value: String,
}

/// Represents a user to be added to a user group.
/// Only the `userid` property is required.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserGroupUser {
    #[serde(rename = "userid")]
    pub user_id: String,
}

/// Parameters for the `usergroup.create` API method.
/// See: https://www.zabbix.com/documentation/current/en/manual/api/reference/usergroup/create
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateUserGroupRequest {
    /// Name of the user group.
    pub name: String,

    /// (optional) Whether debug mode is enabled or disabled.
    /// 0 - (default) disabled;
    /// 1 - enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_mode: Option<i32>,

    /// (optional) Frontend authentication method of the users in the group.
    /// 0 - (default) use the system default authentication method;
    /// 1 - use internal authentication;
    /// 2 - use LDAP authentication;
    /// 3 - disable access to the frontend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gui_access: Option<i32>,

    /// (optional) Whether the user group is enabled or disabled.
    /// 0 - (default) enabled;
    /// 1 - disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users_status: Option<i32>,

    /// (optional) Host group permissions to assign to the user group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostgroup_rights: Option<Vec<UserGroupPermission>>,

    /// (optional) Template group permissions to assign to the user group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templategroup_rights: Option<Vec<UserGroupPermission>>,

    /// (optional) Tag-based permissions to assign to the user group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<UserGroupTagFilter>>,

    /// (optional) Users to add to the user group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserGroupUser>>,
}

/// Response structure for the `usergroup.create` API method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserGroupResponse {
    #[serde(rename = "usrgrpids")]
    pub user_group_ids: Vec<String>,
}
