use reqwest::blocking::Client;
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
use zabbix_api::user::create::{CreateUserRequest, UserGroupId};
use zabbix_api::usergroup::model::CreateUserGroupRequest; // For creating a user group

// Helper to generate a unique name
fn generate_unique_name(prefix: &str) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}_{}", prefix, timestamp)
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

    // 1. Create a user group for the new user
    let user_group_name = generate_unique_name("example_ug_for_user");
    let create_group_request = CreateUserGroupRequest {
        name: user_group_name.clone(),
        ..Default::default()
    };

    let group_id = match client.create_user_group(&session, &create_group_request) {
        Ok(id) => {
            println!(
                "Successfully created user group '{}' with ID: {}",
                user_group_name, id
            );
            id.to_string()
        }
        Err(e) => {
            eprintln!("Error creating user group '{}': {}", user_group_name, e);
            return Err(e);
        }
    };

    // 2. Prepare request to create a user
    let user_alias = generate_unique_name("example_user");
    let user_password = "Password123!"; // Example password
                                        // Role ID "3" is often "Admin role" or "User role" depending on Zabbix version/customization.
                                        // In this project's tests, "3" is referred to as "User role".
                                        // Standard Zabbix: Guest=1, User=2, Admin=3, Super Admin=4.
                                        // Using "3" to align with existing test conventions if they imply a specific setup.
                                        // For a generic "User role", "2" might be more standard.
                                        // Let's use "2" for "User role" as it's more standard for a general example.
    let role_id = "2"; // User role

    let create_user_request = CreateUserRequest {
        username: user_alias.clone(),
        passwd: user_password.to_string(),
        roleid: role_id.to_string(),
        usrgrps: vec![UserGroupId {
            usrgrpid: group_id.clone(),
        }],
        name: Some("Example".to_string()),
        surname: Some("User".to_string()),
        autologin: Some(1), // Optional: enable auto-login
        ..Default::default()
    };

    println!(
        "Attempting to create user '{}' in group ID '{}' with role ID '{}'...",
        user_alias, group_id, role_id
    );

    match client.create_user(&session, &create_user_request) {
        Ok(user_id) => {
            println!(
                "Successfully created user '{}' with ID: {}",
                user_alias, user_id
            );
        }
        Err(e) => {
            eprintln!("Error creating user '{}': {}", user_alias, e);
            // Consider cleaning up the created user group if user creation fails
            return Err(e);
        }
    }

    Ok(())
}
