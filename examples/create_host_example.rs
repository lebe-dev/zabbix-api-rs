use reqwest::blocking::Client;
use std::collections::HashMap;
// std::env is no longer needed as credentials will be hardcoded.
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
use zabbix_api::host::create::CreateHostRequest;
// ZabbixHostInterface is no longer used in this example since interfaces are empty.
use zabbix_api::hostgroup::create::CreateHostGroupRequest;
use zabbix_api::hostgroup::model::ZabbixHostGroupId;

// Helper to generate a unique name
fn generate_unique_name(prefix: &str) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}_{}", prefix, timestamp)
}

fn main() -> Result<(), ZabbixApiError> {
    // Credentials are now hardcoded from run-example.sh
    // This is generally not recommended for shared examples but fulfills the request.
    let zabbix_api_url = "http://localhost:3080/api_jsonrpc.php".to_string();
    let zabbix_api_user = "Admin".to_string();
    let zabbix_api_password = "zabbix".to_string();

    let http_client = Client::new();
    let client = ZabbixApiClientImpl::new(http_client, &zabbix_api_url);

    let session = client.get_auth_session(&zabbix_api_user, &zabbix_api_password)?;
    println!("Authenticated successfully.");

    // 1. Create a host group for the new host
    let host_group_name = generate_unique_name("example_host_group_for_host");
    let create_group_request = CreateHostGroupRequest {
        name: host_group_name.clone(),
    };

    let group_id = match client.create_host_group(&session, &create_group_request) {
        Ok(id) => {
            println!(
                "Successfully created host group '{}' with ID: {}",
                host_group_name, id
            );
            id.to_string()
        }
        Err(e) => {
            eprintln!("Error creating host group '{}': {}", host_group_name, e);
            return Err(e);
        }
    };

    // 2. Prepare request to create a host
    let host_name = generate_unique_name("example_host");

    let create_host_request = CreateHostRequest {
        host: host_name.clone(),
        groups: vec![ZabbixHostGroupId {
            group_id: group_id.clone(),
        }],
        interfaces: vec![], // Create host without interfaces initially
        // To add a functional agent interface, the ZabbixHostInterface model
        // would need to support specifying a 'port' (e.g., "10050").
        // If src/host/model.rs is updated, an interface could be added like:
        // interfaces: vec![ZabbixHostInterface {
        //     r#type: 1, main: 1, ip: "127.0.0.1".to_string(), dns: "".to_string(), useip: 1, port: "10050".to_string(),
        // }],
        tags: vec![],      // Optional: Add host tags if needed
        templates: vec![], // Optional: Link templates if needed
        macros: vec![],    // Optional: Add host macros if needed
        inventory_mode: 0, // Optional: Inventory mode (0 for disabled)
        inventory: HashMap::new(), // Optional: Host inventory
    };

    println!(
        "Attempting to create host '{}' in group ID '{}'...",
        host_name, group_id
    );

    match client.create_host(&session, &create_host_request) {
        Ok(host_id) => {
            println!(
                "Successfully created host '{}' with ID: {}",
                host_name, host_id
            );
        }
        Err(e) => {
            eprintln!("Error creating host '{}': {}", host_name, e);
            // Consider cleaning up the created host group if host creation fails
            return Err(e);
        }
    }

    Ok(())
}
