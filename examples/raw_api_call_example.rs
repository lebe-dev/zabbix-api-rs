use reqwest::blocking::Client;
use serde::Serialize;
use std::env;
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
use zabbix_api::host::model::ZabbixHost; // Assuming you want to get host data

// Define a structure for your API call's parameters
// This example uses parameters for "host.get" to fetch a specific host.
#[derive(Serialize)]
struct GetHostParams {
    output: String,
    filter: HostFilter,
}

#[derive(Serialize)]
struct HostFilter {
    host: Vec<String>,
}

fn main() -> Result<(), ZabbixApiError> {
    let zabbix_api_url =
        env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL environment variable not set");
    let zabbix_api_user =
        env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER environment variable not set");
    let zabbix_api_password =
        env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD environment variable not set");

    let http_client = Client::new();
    let client = ZabbixApiClientImpl::new(http_client, &zabbix_api_url);

    let session = client.get_auth_session(&zabbix_api_user, &zabbix_api_password)?;
    println!("Authenticated successfully.");

    // Prepare parameters for the "host.get" method
    let params = GetHostParams {
        output: "extend".to_string(),
        filter: HostFilter {
            host: vec!["Zabbix server".to_string()], // Example host name
        },
    };

    println!("Calling raw_api_call for method 'host.get'...");

    // Make the raw API call
    // The second type parameter `Vec<ZabbixHost>` specifies the expected structure of the 'result' field.
    match client.raw_api_call::<GetHostParams, Vec<ZabbixHost>>(&session, "host.get", &params) {
        Ok(response) => {
            if let Some(hosts) = response.result {
                if hosts.is_empty() {
                    println!("No hosts found matching the criteria.");
                } else {
                    println!("Successfully fetched {} host(s):", hosts.len());
                    for host in hosts {
                        println!("  Host ID: {}, Host Name: {}", host.host_id, host.host);
                    }
                }
            } else if let Some(error) = response.error {
                eprintln!("Zabbix API Error: {:?}", error);
                return Err(ZabbixApiError::ApiCallError { zabbix: error });
            } else {
                eprintln!("Received an empty or unexpected response.");
            }
        }
        Err(e) => {
            eprintln!("Error during raw_api_call: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
