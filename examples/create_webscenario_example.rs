use reqwest::blocking::Client;
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
use zabbix_api::host::create::CreateHostRequest;
use zabbix_api::hostgroup::create::CreateHostGroupRequest;
use zabbix_api::hostgroup::model::ZabbixHostGroupId;
use zabbix_api::webscenario::create::CreateWebScenarioRequest;
use zabbix_api::webscenario::model::ZabbixWebScenarioStep; // Ensure this path is correct

// Helper to generate a unique name
fn generate_unique_name(prefix: &str) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}_{}", prefix, timestamp)
}

fn main() -> Result<(), ZabbixApiError> {
    // Credentials are hardcoded as per run-example.sh
    let zabbix_api_url = "http://localhost:3080/api_jsonrpc.php".to_string();
    let zabbix_api_user = "Admin".to_string();
    let zabbix_api_password = "zabbix".to_string();

    let http_client = Client::new();
    let client = ZabbixApiClientImpl::new(http_client, &zabbix_api_url);

    let session = client.get_auth_session(&zabbix_api_user, &zabbix_api_password)?;
    println!("Authenticated successfully.");

    // 1. Create a host group for the new host
    let host_group_name = generate_unique_name("example_group_for_webscenario");
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

    // 2. Create a host for the web scenario
    let host_name = generate_unique_name("example_host_for_webscenario");
    let create_host_request = CreateHostRequest {
        host: host_name.clone(),
        groups: vec![ZabbixHostGroupId {
            group_id: group_id.clone(),
        }],
        // No interfaces needed for a simple web scenario example
        ..Default::default()
    };

    let host_id = match client.create_host(&session, &create_host_request) {
        Ok(id) => {
            println!("Successfully created host '{}' with ID: {}", host_name, id);
            id.to_string()
        }
        Err(e) => {
            eprintln!("Error creating host '{}': {}", host_name, e);
            // Consider cleaning up the created host group if host creation fails
            return Err(e);
        }
    };

    // 3. Prepare request to create a web scenario
    let web_scenario_name = generate_unique_name("example_web_scenario");
    let web_scenario_step = ZabbixWebScenarioStep {
        name: "Check example.com".to_string(),
        url: "http://example.com".to_string(),
        status_codes: "200".to_string(),
        no: "1".to_string(), // Step number, usually starts from 1
    };

    let create_web_scenario_request = CreateWebScenarioRequest {
        name: web_scenario_name.clone(),
        host_id: host_id.clone(),
        steps: vec![web_scenario_step],
        // Add other optional fields like agent, delay, retries, etc., if needed
    };

    println!(
        "Attempting to create web scenario '{}' on host ID '{}'...",
        web_scenario_name, host_id
    );

    match client.create_webscenario(&session, &create_web_scenario_request) {
        Ok(web_scenario_id) => {
            println!(
                "Successfully created web scenario '{}' with ID: {}",
                web_scenario_name, web_scenario_id
            );
        }
        Err(e) => {
            eprintln!("Error creating web scenario '{}': {}", web_scenario_name, e);
            // Consider cleaning up created host and host group if web scenario creation fails
            return Err(e);
        }
    }

    Ok(())
}
