use reqwest::blocking::Client;
use serde::Serialize;
use std::env;
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
// Note: We define a custom struct for parameters to include 'output', 'filter', etc.
// as required by the Zabbix API 'host.get' method.

#[derive(Serialize)]
struct HostFilterForGet {
    host: Vec<String>,
}

#[derive(Serialize)]
struct GetHostsParams {
    output: String,
    filter: HostFilterForGet,
    // You can add other parameters like 'selectInterfaces: "extend".to_string()' here
}

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

    // Prepare request to get hosts.
    // This example filters for hosts named "Zabbix server". Adjust as needed.
    let request_params = GetHostsParams {
        output: "extend".to_string(), // Required to get detailed host information
        filter: HostFilterForGet {
            host: vec!["Zabbix server".to_string()],
        },
    };

    match client.get_hosts(&session, &request_params) {
        Ok(hosts) => {
            if hosts.is_empty() {
                println!("No hosts found matching the criteria.");
            } else {
                println!("Successfully fetched {} host(s):", hosts.len());
                for host in hosts {
                    println!("  Host ID: {}, Host Name: {}", host.host_id, host.host);
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching hosts: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
