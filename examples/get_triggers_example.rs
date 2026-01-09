use reqwest::blocking::Client;
use serde::Serialize;
use std::env;
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
// ZabbixTrigger type will be inferred from its usage in client.get_triggers

// Define a structure for your API call's parameters for "trigger.get"
#[derive(Serialize)]
struct GetTriggersParams {
    output: String,
    #[serde(rename = "selectTags")] // Zabbix API expects "selectTags"
    select_tags: String,
    limit: u32,
    // Optionally, you could add filters, e.g., by host, severity, etc.
    // filter: TriggerFilter,
    // sortfield: String, // e.g. "description"
    // sortorder: String, // e.g. "ASC"
}

// Example filter structure (not used in this basic example)
// #[derive(Serialize)]
// struct TriggerFilter {
//     value: Option<u32>, // e.g., 1 for PROBLEM state triggers
//     // hostid: Option<String>,
// }

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

    // Prepare parameters for the "trigger.get" method
    // This example fetches up to 5 triggers with their tags.
    let request_params = GetTriggersParams {
        output: "extend".to_string(),
        select_tags: "extend".to_string(),
        limit: 5,
        // sortfield: "triggerid".to_string(), // Sort to get consistent results if needed
        // sortorder: "ASC".to_string(),
    };

    println!("Calling client.get_triggers()...");

    match client.get_triggers::<GetTriggersParams>(&session, &request_params) {
        Ok(triggers) => {
            if triggers.is_empty() {
                println!("No triggers found matching the criteria.");
            } else {
                println!("Successfully fetched {} trigger(s):", triggers.len());
                for trigger in triggers {
                    println!(
                        "  Trigger ID: {}, Description: '{}', Expression: '{}'",
                        trigger.trigger_id, trigger.description, trigger.expression
                    );
                    // The following lines are commented out because the ZabbixTrigger struct
                    // currently does not have a 'tags' field. To enable this,
                    // src/trigger/model.rs would need to be updated.
                    // if !trigger.tags.is_empty() {
                    //     println!("    Tags:");
                    //     for tag in trigger.tags {
                    //         println!("      - Tag: '{}', Value: '{}'", tag.tag, tag.value);
                    //     }
                    // }
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching triggers: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
