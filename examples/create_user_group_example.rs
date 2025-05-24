use reqwest::blocking::Client;
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
use zabbix_api::usergroup::model::CreateUserGroupRequest; // Ensure this path is correct

// Helper to generate a unique name for the user group
fn generate_unique_user_group_name() -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("example_user_group_{}", timestamp)
}

fn main() -> Result<(), ZabbixApiError> {
    // Credentials are hardcoded as per run-example.sh
    let zabbix_api_url = "http://localhost:3080/api_jsonrpc.php".to_string();
    let zabbix_api_user = "Admin".to_string();
    let zabbix_api_password = "zabbix".to_string();

    let http_client = Client::new();
    let client = ZabbixApiClientImpl::new(http_client, &zabbix_api_url);

    let session = client.get_auth_session(&zabbix_api_user, &zabbix_api_password)?;
    println!("Authenticated successfully.");

    let user_group_name = generate_unique_user_group_name();

    // Prepare request to create a user group.
    // This example creates a basic user group.
    // You can customize it by adding user IDs, host group permissions, etc.
    let create_request = CreateUserGroupRequest {
        name: user_group_name.clone(),
        // gui_access: Some(0), // Optional: System default GUI access
        // users_status: Some(0), // Optional: Enabled users
        // users: None, // Optional: Vec<UserGroupUser>
        // hostgroup_rights: None, // Optional: Vec<UserGroupPermission>
        ..Default::default() // If your struct derives Default and has more fields
    };

    println!(
        "Attempting to create user group '{}'...",
        user_group_name
    );

    match client.create_user_group(&session, &create_request) {
        Ok(user_group_id) => {
            println!(
                "Successfully created user group '{}' with ID: {}",
                user_group_name, user_group_id
            );
        }
        Err(e) => {
            eprintln!(
                "Error creating user group '{}': {}",
                user_group_name, e
            );
            return Err(e);
        }
    }

    Ok(())
}
