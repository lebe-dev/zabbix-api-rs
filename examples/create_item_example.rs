use reqwest::blocking::Client;
use std::env;
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
use zabbix_api::item::create::CreateItemRequest; // Ensure this path is correct based on your module structure

// A simple helper to generate a unique key for the example item
fn generate_unique_item_key() -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("example.item.key_{}", timestamp)
}

fn main() -> Result<(), ZabbixApiError> {
    let zabbix_api_url =
        env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL environment variable not set (e.g., http://localhost:3080/api_jsonrpc.php)");
    let zabbix_api_user =
        env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER environment variable not set (e.g., Admin)");
    let zabbix_api_password =
        env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD environment variable not set (e.g., zabbix)");

    // IMPORTANT: Replace with a valid host ID from your Zabbix instance
    let host_id_for_item = env::var("ZABBIX_HOST_ID_FOR_ITEM_EXAMPLE")
        .expect("ZABBIX_HOST_ID_FOR_ITEM_EXAMPLE environment variable not set (e.g., 10010). This host must exist.");

    let http_client = Client::new();
    let client = ZabbixApiClientImpl::new(http_client, &zabbix_api_url);

    let session = client.get_auth_session(&zabbix_api_user, &zabbix_api_password)?;
    println!("Authenticated successfully.");

    let item_name = "Example Item".to_string();
    let item_key = generate_unique_item_key();

    // Prepare request to create an item.
    // This example creates a Zabbix agent item that collects text data.
    // Adjust type, value_type, interface_id, delay, etc., as needed for your specific item.
    let create_request = CreateItemRequest {
        name: item_name.clone(),
        key_: item_key.clone(),
        host_id: host_id_for_item.clone(),
        r#type: 0, // Type 0: Zabbix agent. Adjust if using a different item type.
        value_type: 3, // Type 3: Numeric (unsigned). Adjust for other data types (e.g., 4 for Text).
        interface_id: "0".to_string(), // Use "0" for the first available agent interface, or provide a specific interface ID.
        delay: "1m".to_string(), // Collect data every 1 minute.
        tags: vec![], // Optional: Add item tags if needed.
        // Add other optional fields as necessary
    };

    println!(
        "Attempting to create item '{}' with key '{}' on host ID '{}'...",
        item_name, item_key, host_id_for_item
    );

    match client.create_item(&session, &create_request) {
        Ok(item_id) => {
            println!(
                "Successfully created item '{}' with ID: {}",
                item_name, item_id
            );
        }
        Err(e) => {
            eprintln!(
                "Error creating item '{}': {}",
                item_name, e
            );
            return Err(e);
        }
    }

    Ok(())
}
