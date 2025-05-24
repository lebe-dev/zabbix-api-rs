use reqwest::blocking::Client;
use std::env;
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;

fn main() -> Result<(), ZabbixApiError> {
    let zabbix_api_url =
        env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL environment variable not set (e.g., http://localhost:3080/api_jsonrpc.php)");
    let zabbix_api_user =
        env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER environment variable not set (e.g., Admin)");
    let zabbix_api_password =
        env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD environment variable not set (e.g., zabbix)");

    let http_client = Client::new();
    let client = ZabbixApiClientImpl::new(http_client, &zabbix_api_url);

    // Get API Info
    match client.get_api_info() {
        Ok(api_version) => {
            println!("Successfully connected to Zabbix API version: {}", api_version);
        }
        Err(e) => {
            eprintln!("Error getting API info: {}", e);
            return Err(e);
        }
    }

    // Get Auth Session
    match client.get_auth_session(&zabbix_api_user, &zabbix_api_password) {
        Ok(session_token) => {
            println!("Successfully obtained session token (first 10 chars): {}...", &session_token[..10.min(session_token.len())]);
        }
        Err(e) => {
            eprintln!("Error getting auth session: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
