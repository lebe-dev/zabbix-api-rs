use reqwest::blocking::Client;
use serde::Serialize;
use std::env;
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
use zabbix_api::hostgroup::get::GetHostGroupsRequest; // Use the actual request struct

// Define a filter structure for host group queries
#[derive(Serialize)]
struct HostGroupFilter {
    name: Vec<String>, // Example: filter by a list of names
                       // Add other filter fields as needed, e.g., hostids, groupids
}

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

    // Prepare request to get host groups.
    // This example filters for host groups named "Linux servers".
    // To get all host groups, you might pass an empty Vec or a filter that matches all.
    let group_names_to_filter = vec!["Linux servers".to_string()]; // Example group name to filter by

    let request_params = GetHostGroupsRequest {
        output: "extend".to_string(), // "extend" is common to get more details
        filter: HostGroupFilter {
            name: group_names_to_filter.clone(),
        },
        // You can add other parameters from the Zabbix API documentation for "hostgroup.get", like:
        // selectHosts: "extend", // To get hosts in the group
        // selectApplications: "extend", // To get applications in the group
    };

    println!(
        "Attempting to fetch host groups with names: {:?}",
        group_names_to_filter
    );

    match client.get_host_groups(&session, &request_params) {
        Ok(host_groups) => {
            if host_groups.is_empty() {
                println!(
                    "No host groups found matching the criteria: {:?}.",
                    group_names_to_filter
                );
            } else {
                println!("Successfully fetched {} host group(s):", host_groups.len());
                for group in host_groups {
                    println!(
                        "  Group ID: {}, Group Name: '{}'",
                        group.group_id, group.name
                    );
                    // If selectHosts was used, you could iterate group.hosts and print host details
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching host groups: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
