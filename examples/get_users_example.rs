use reqwest::blocking::Client;
use serde::Serialize;
use std::env;
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
// ZabbixUser type will be inferred from its usage in client.get_users
// but it's good practice to import it if you know the type.
// use zabbix_api::user::model::ZabbixUser;

// Define a structure for your API call's parameters for "user.get"
// API Reference: https://www.zabbix.com/documentation/current/en/manual/api/reference/user/get
#[derive(Serialize)]
struct GetUsersParams {
    output: String,
    // Example: Filter by user alias. To get all users, you might omit the filter
    // or use a filter that matches all, depending on API behavior.
    // For this example, we'll fetch users with a common alias like "Admin" if it exists,
    // or adjust the filter as needed.
    // filter: Option<UserFilter>,
    // selectMediatypes: Option<String>, // Example: "extend" to get media types
}

// #[derive(Serialize)]
// struct UserFilter {
//     alias: Vec<String>,
// }

fn main() -> Result<(), ZabbixApiError> {
    let zabbix_api_url =
        env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL environment variable not set (e.g., http://localhost:3080/api_jsonrpc.php)");
    let zabbix_api_user =
        env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER environment variable not set (e.g., Admin)");
    let zabbix_api_password =
        env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD environment variable not set (e.g., zabbix)");

    let http_client = Client::new();
    let client = ZabbixApiClientImpl::new(http_client, &zabbix_api_url);

    let session = client.get_auth_session(&zabbix_api_user, &zabbix_api_password)?;
    println!("Authenticated successfully.");

    // Prepare parameters for the "user.get" method
    // This example fetches users with extended output.
    // To filter for specific users, you would add a `filter` field to GetUsersParams.
    // For example, to find a user by alias:
    // filter: Some(UserFilter { alias: vec!["Admin".to_string()] }),
    let request_params = GetUsersParams {
        output: "extend".to_string(),
        // filter: None, // Uncomment and modify UserFilter to add specific filters
    };

    println!("\nCalling client.get_users()...");

    match client.get_users(&session, &request_params) {
        Ok(users) => {
            if users.is_empty() {
                println!("No users found matching the criteria.");
            } else {
                println!("Successfully fetched {} user(s):", users.len());
                for user in users {
                    println!(
                        "  User ID: {}, Alias: '{}', Name: '{}', Surname: '{}', Role ID: {:?}",
                        user.user_id,
                        user.alias,
                        user.name.as_deref().unwrap_or("N/A"),
                        user.surname.as_deref().unwrap_or("N/A"),
                        user.role_id.as_deref().unwrap_or("N/A")
                    );
                    // If selectMediatypes was "extend", you could iterate user.mediatypes
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching users: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
