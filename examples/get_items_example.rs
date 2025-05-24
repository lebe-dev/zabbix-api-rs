use reqwest::blocking::Client;
use serde::Serialize;
use std::env;
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
use zabbix_api::item::get::{GetItemsRequestByKey, SearchByKey}; // Ensure this path is correct

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

    // Prepare request to get items.
    // This example searches for items with the key "agent.ping". Adjust as needed.
    // The GetItemsRequestByKey::new constructor sets output to "extend" by default.
    let item_key_to_search = "agent.ping";
    let request_params = GetItemsRequestByKey::new(item_key_to_search);

    // Alternatively, to search by host ID, you might use GetItemsRequestById or a more complex filter:
    // #[derive(Serialize)]
    // struct ItemFilterForGet {
    //     hostid: String, // ID of the host to get items from
    //     // key_: Option<String>, // Optionally filter by key
    // }
    // #[derive(Serialize)]
    // struct GetItemsParams<F: Serialize> {
    //     output: String,
    //     filter: F,
    //     // hostids: Option<String>, // Alternative way to specify host
    //     // search: Option<HashMap<String, String>>, // For more generic search
    // }
    // let request_params = GetItemsParams {
    //     output: "extend".to_string(),
    //     filter: ItemFilterForGet { hostid: "your_host_id".to_string() },
    // };


    println!("Searching for items with key '{}'...", item_key_to_search);

    match client.get_items(&session, &request_params) {
        Ok(items) => {
            if items.is_empty() {
                println!("No items found matching the key '{}'.", item_key_to_search);
            } else {
                println!("Successfully fetched {} item(s) with key '{}':", items.len(), item_key_to_search);
                for item in items {
                    println!(
                        "  Item ID: {}, Name: '{}', Key: '{}', Host ID: {}",
                        item.item_id, item.name, item.key_, item.host_id
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching items: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
