use std::collections::HashMap;

use log::{debug, error, info};
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::client::jsonrpc::{ZabbixApiRequest, ZabbixApiResponse};
use crate::client::post::send_post_request;
use crate::client::ZabbixApiClient;
use crate::error::ZabbixApiError;
use crate::host::create::{CreateHostRequest, CreateHostResponse};

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
                let response = serde_json::from_str::<ZabbixApiResponse<String>>(&response_body)?;

                match response.result {
                    Some(session) => {
                        info!("auth ok");
                        Ok(session)
                    }
                    None => {
                        match response.error {
                            Some(error) => {
                                error!("{:?}", error);

                                Err(ZabbixApiError::ApiCallError {
                                    zabbix: error,
                                })
                            }
                            None => Err(ZabbixApiError::BadRequestError)
                        }
                    }
                }
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
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response = serde_json::from_str::<ZabbixApiResponse<R>>(&response_body)?;

                match response.result {
                    Some(_) => {
                        info!("api method '{method}' has been successfully called");
                        Ok(response)
                    }
                    None => {
                        match response.error {
                            Some(error) => {
                                error!("{:?}", error);

                                Err(ZabbixApiError::ApiCallError {
                                    zabbix: error,
                                })
                            }
                            None => Err(ZabbixApiError::BadRequestError)
                        }
                    }
                }
            }
            Err(e) => {
                error!("auth error: {}", e);
                Err(e)
            }
        }
    }

    fn create_host(&self, session: &str, request: &CreateHostRequest) -> Result<u32, ZabbixApiError> {
        info!("creating host '{}'..", request.host);

        let api_request = ZabbixApiRequest {
            jsonrpc: "2.0".to_string(),
            method: "host.create".to_string(),
            params: request,
            id: 1,
            auth: Some(session.to_string()),
        };

        match send_post_request(&self.client, &self.api_endpoint_url, api_request) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response = serde_json::from_str::<ZabbixApiResponse<CreateHostResponse>>(&response_body)?;

                match response.result {
                    Some(result) => {

                        info!("host '{}' has been created", request.host);

                        match result.host_ids.first() {
                            Some(host_id) => {
                                host_id.parse::<u32>().map_err(|_| ZabbixApiError::Error)
                            }
                            None => {
                                error!("unexpected error, server returned empty id list");
                                Err(ZabbixApiError::Error)
                            }
                        }
                    }
                    None => {
                        match response.error {
                            Some(error) => {
                                error!("{:?}", error);

                                Err(ZabbixApiError::ApiCallError {
                                    zabbix: error,
                                })
                            }
                            None => Err(ZabbixApiError::BadRequestError)
                        }
                    }
                }
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
    use std::collections::HashMap;
    use std::error::Error;

    use log::{error, info};
    use reqwest::blocking::Client;
    use serde::Serialize;

    use crate::client::v6::ZabbixApiV6Client;
    use crate::client::ZabbixApiClient;
    use crate::host::create::CreateHostRequest;
    use crate::host::ZabbixHost;
    use crate::tests::{get_random_string, init_logging};
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
                            let results = response.result.unwrap();
                            info!("{:?}", results.first().unwrap());
                            assert_eq!(1, results.len())
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

    #[test]
    fn create_host() {
        init_logging();

        if are_integration_tests_enabled() {
            let http_client = Client::new();

            let tests_config = get_integration_tests_config();

            let client = ZabbixApiV6Client::new(http_client, &tests_config.zabbix_api_url);

            match client.get_auth_session(&tests_config.zabbix_api_user, &tests_config.zabbix_api_password) {
                Ok(session) => {

                    let host = get_random_string();

                    let params = CreateHostRequest {
                        host: host.to_string(),
                        groups: vec![],
                        interfaces: vec![],
                        tags: vec![],
                        templates: vec![],
                        macros: vec![],
                        inventory_mode: 0,
                        inventory: HashMap::new(),
                    };

                    match client.create_host(&session, &params) {
                        Ok(host_id) => {
                            assert!(host_id > 0)
                        }
                        Err(e) => {
                            if let Some(inner_source) = e.source() {
                                println!("Caused by: {}", inner_source);
                            }
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