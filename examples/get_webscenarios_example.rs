use reqwest::blocking::Client;
use serde::Serialize;
use std::env;
use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use zabbix_api::error::ZabbixApiError;
// The ZabbixWebScenario type will be inferred from its usage in client.get_webscenarios
// but it's good practice to import it if you know the type.
// use zabbix_api::webscenario::model::ZabbixWebScenario;

// Define a structure for your API call's parameters for "httptest.get"
// API Reference: https://www.zabbix.com/documentation/current/en/manual/api/reference/httptest/get
#[derive(Serialize)]
struct GetWebScenariosParams {
    output: String,
    #[serde(rename = "selectSteps")] // Zabbix API expects "selectSteps"
    select_steps: String,
    // You can add more parameters like 'limit', 'filter', 'search', 'hostids', etc.
    // For example, to limit results:
    // limit: Option<u32>,
}

fn main() -> Result<(), ZabbixApiError> {
    // Initialize logger (optional, but helpful for debugging)
    // Ensure env_logger is in your [dev-dependencies] in Cargo.toml
    // and you call init_logging or env_logger::init()
    // For this example, we'll assume logging is initialized if needed elsewhere or skip for simplicity.
    // env_logger::init();

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
    println!("Authenticated successfully. Session ID: {}", session);

    // Prepare parameters for the "httptest.get" method
    // This example fetches all web scenarios with their steps.
    let request_params = GetWebScenariosParams {
        output: "extend".to_string(),
        select_steps: "extend".to_string(),
        // limit: Some(5), // Example: limit to 5 scenarios
    };

    println!("\nCalling client.get_webscenarios()...");

    match client.get_webscenarios(&session, &request_params) {
        Ok(webscenarios) => {
            if webscenarios.is_empty() {
                println!("No web scenarios found matching the criteria.");
            } else {
                println!(
                    "Successfully fetched {} web scenario(s):",
                    webscenarios.len()
                );
                for scenario in webscenarios {
                    println!(
                        "\n  Scenario Name: '{}', Host ID: {}",
                        scenario.name, scenario.host_id
                    );
                    // The ZabbixWebScenario model includes 'steps' Vec<ZabbixWebScenarioStep>
                    if scenario.steps.is_empty() {
                        println!("    No steps defined for this scenario.");
                    } else {
                        println!("    Steps ({}):", scenario.steps.len());
                        for step in scenario.steps {
                            println!(
                                "      - Step No: {}, Name: '{}', URL: '{}', Expected Status Codes: '{}'",
                                step.no, step.name, step.url, step.status_codes
                            );
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching web scenarios: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
