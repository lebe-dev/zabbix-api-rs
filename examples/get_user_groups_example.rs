use reqwest::blocking::Client;
// std::env is not used as credentials are hardcoded
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
use zabbix_api::usergroup::get::{GetUserGroupsRequest, UserGroupFilter};
// ZabbixUserGroup type will be inferred from its usage in client.get_user_groups

// Define a structure for your API call's parameters for "usergroup.get"
// API Reference: https://www.zabbix.com/documentation/current/en/manual/api/reference/usergroup/get
// We use GetUserGroupsRequest from the library directly.

fn main() -> Result<(), ZabbixApiError> {
    let zabbix_api_url = "http://localhost:3080/api_jsonrpc.php".to_string();
    let zabbix_api_user = "Admin".to_string();
    let zabbix_api_password = "zabbix".to_string();

    let http_client = Client::new();
    let client = ZabbixApiClientImpl::new(http_client, &zabbix_api_url);

    let session = client.get_auth_session(&zabbix_api_user, &zabbix_api_password)?;
    println!("Authenticated successfully.");

    // Prepare parameters for the "usergroup.get" method
    // This example fetches all user groups with extended output and their users.
    // To filter for specific user groups, you would modify the `filter` field.
    // For example, to find a user group by name:
    // filter: Some(UserGroupFilter { name: Some(vec!["Zabbix administrators".to_string()]) }),
    let request_params = GetUserGroupsRequest {
        output: Some("extend".to_string()),
        select_users: Some("extend".to_string()), // To get users in the group
        filter: Option::<UserGroupFilter>::None, // No filter, get all groups
        // You can add other parameters like:
        // status: Some(0), // To get enabled groups
        // select_rights: Some("extend"), // To get group rights
        ..Default::default()
    };

    println!("\nCalling client.get_user_groups()...");

    match client.get_user_groups(&session, &request_params) {
        Ok(user_groups) => {
            if user_groups.is_empty() {
                println!("No user groups found matching the criteria.");
            } else {
                println!("Successfully fetched {} user group(s):", user_groups.len());
                for group in user_groups {
                    println!(
                        "  Group ID: {}, Group Name: '{}', GUI Access: {:?}, Users Status: {:?}",
                        group.usrgrp_id,
                        group.name,
                        group.gui_access.as_deref().unwrap_or("N/A"),
                        group.users_status.as_deref().unwrap_or("N/A")
                    );
                    if let Some(users) = &group.users {
                        if users.is_empty() {
                            println!("    No users in this group.");
                        } else {
                            println!("    Users ({}):", users.len());
                            for user in users {
                                println!("      - User ID: {}, Alias: '{}'", user.user_id, user.alias);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching user groups: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
