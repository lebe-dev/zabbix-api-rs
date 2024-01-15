use std::collections::HashMap;

use log::error;
use reqwest::blocking::Client;

use crate::client::jsonrpc::{ZabbixApiRequest, ZabbixApiResponse};
use crate::client::post::send_post_request;
use crate::client::ZabbixApiClient;
use crate::error::ZabbixApiError;

pub struct ZabbixApiV6Client {
    client: Client,
    api_endpoint_url: String
}

impl ZabbixApiV6Client {
    pub fn new(client: Client, api_endpoint_url: &str) -> ZabbixApiV6Client {
        ZabbixApiV6Client {
            client,
            api_endpoint_url: api_endpoint_url.to_string()
        }
    }
}

impl ZabbixApiClient for ZabbixApiV6Client {
    fn get_auth_session(&self,  login: &str, token: &str) -> Result<String, ZabbixApiError> {
        let params = HashMap::from([
            ("username".to_string(), login.to_string()),
            ("password".to_string(), token.to_string()),
        ]);

        let request = ZabbixApiRequest {
            jsonrpc: "2.0".to_string(),
            method: "user.login".to_string(),
            params,
            id: 1,
            auth: None,
        };

        match send_post_request(&self.client, &self.api_endpoint_url, request) {
            Ok(response_body) => {
                let session = serde_json::from_str::<ZabbixApiResponse<String>>(&response_body)?;
                Ok(session.result)
            }
            Err(e) => {
                error!("auth error: {}", e);
                Err(e)
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use log::error;
    use reqwest::blocking::Client;
    use crate::client::v6::ZabbixApiV6Client;
    use crate::client::ZabbixApiClient;
    use crate::tests::init_logging;
    use crate::tests::integration::{are_integration_tests_enabled, get_integration_tests_config};

    #[test]
    fn session_should_be_returned() {
        init_logging();

        if are_integration_tests_enabled() {
            let http_client = Client::new();

            let tests_config = get_integration_tests_config();

            let client = ZabbixApiV6Client::new(http_client, &tests_config.zabbix_api_url);

            match client.get_auth_session(&tests_config.zabbix_api_user, &tests_config.zabbix_api_password) {
                Ok(session) => assert!(session.len() > 0),
                Err(e) => {
                    error!("error: {}", e);
                    panic!("unexpected error")
                }
            }
        }
    }
}