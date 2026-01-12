use reqwest::blocking::Client;
use std::env;
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
use zabbix_api::hostgroup::create::CreateHostGroupRequest;

fn main() -> Result<(), ZabbixApiError> {
    let zabbix_api_url = env::var("ZABBIX_API_URL").expect(
        "ZABBIX_API_URL environment variable not set (e.g., http://localhost:3080/api_jsonrpc.php)",
    );
    let zabbix_api_user = env::var("ZABBIX_API_USER")
        .expect("ZABBIX_API_USER environment variable not set (e.g., Admin)");
    let zabbix_api_password = env::var("ZABBIX_API_PASSWORD")
        .expect("ZABBIX_API_PASSWORD environment variable not set (e.g., zabbix)");

    let http_client = Client::new();
    let client = ZabbixApiClientImpl::new(http_client, &zabbix_api_url);

    let session = client.get_auth_session(&zabbix_api_user, &zabbix_api_password)?;
    println!("Authenticated successfully.");

    // Generate a somewhat unique name for the host group for this example run.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| ZabbixApiError::Error)? // Basic error conversion
        .as_secs();
    let host_group_name = format!("example_group_{}", timestamp);

    let create_request = CreateHostGroupRequest {
        name: host_group_name.clone(),
    };

    match client.create_host_group(&session, &create_request) {
        Ok(group_id) => {
            println!(
                "Successfully created host group '{}' with ID: {}",
                host_group_name, group_id
            );
        }
        Err(e) => {
            eprintln!("Error creating host group '{}': {}", host_group_name, e);
            return Err(e);
        }
    }

    Ok(())
}
