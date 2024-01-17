use std::collections::HashMap;

use log::{error, info};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

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

    fn raw_api_call<P: Serialize, R: DeserializeOwned>(&self, session: &str,
                                           method: &str, params: &P) -> Result<ZabbixApiResponse<R>, ZabbixApiError> {
        info!("call api method '{method}'..");

        let request = ZabbixApiRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: 1,
            auth: Some(session.to_string()),
        };

        match send_post_request(&self.client, &self.api_endpoint_url, request) {
            Ok(response_body) => {
                let response = serde_json::from_str::<ZabbixApiResponse<R>>(&response_body)?;
                Ok(response)
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
    use log::{error, info};
    use reqwest::blocking::Client;
    use serde::Serialize;

    use crate::client::v6::ZabbixApiV6Client;
    use crate::client::ZabbixApiClient;
    use crate::host::ZabbixHost;
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

    #[test]
    fn raw_api_call_test() {
        init_logging();

        if are_integration_tests_enabled() {
            let http_client = Client::new();

            let tests_config = get_integration_tests_config();

            let client = ZabbixApiV6Client::new(http_client, &tests_config.zabbix_api_url);

            match client.get_auth_session(&tests_config.zabbix_api_user, &tests_config.zabbix_api_password) {
                Ok(session) => {

                    #[derive(Serialize)]
                    struct Params {
                        pub filter: Filter
                    }

                    #[derive(Serialize)]
                    struct Filter {
                        pub host: Vec<String>
                    }

                    let params = Params {
                        filter: Filter {
                            host: vec!["Zabbix server".to_string()],
                        },
                    };

                    match client.raw_api_call::<Params, Vec<ZabbixHost>>(&session, "host.get", &params) {
                        Ok(response) => {
                            info!("{:?}", response.result.first().unwrap());
                            assert_eq!(1, response.result.len())
                        }
                        Err(e) => {
                            error!("api call error: {}", e);
                            panic!("unexpected api call error")
                        }
                    }

                },
                Err(e) => {
                    error!("auth error: {}", e);
                    panic!("unexpected auth error")
                }
            }
        }
    }
}