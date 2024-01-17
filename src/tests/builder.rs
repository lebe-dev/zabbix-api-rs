use log::error;
use reqwest::blocking::Client;

use crate::client::v6::ZabbixApiV6Client;
use crate::client::ZabbixApiClient;
use crate::tests::init_logging;
use crate::tests::integration::{get_integration_tests_config, IntegrationTestsConfig};

pub struct TestEnvBuilder {
    pub client: ZabbixApiV6Client,
    pub integration_tests_config: IntegrationTestsConfig,
    pub session: String
}

impl TestEnvBuilder {
    pub fn build() -> TestEnvBuilder {
        init_logging();

        let http_client = Client::new();

        let tests_config = get_integration_tests_config();

        TestEnvBuilder {
            client: ZabbixApiV6Client::new(http_client, &tests_config.zabbix_api_url),
            integration_tests_config: tests_config,
            session: "".to_string(),
        }
    }

    pub fn get_session(&mut self) {
        match self.client.get_auth_session(
            &self.integration_tests_config.zabbix_api_user,
            &self.integration_tests_config.zabbix_api_password
        ) {
            Ok(session) => {
                self.session = session;
            },
            Err(e) => {
                error!("auth error: {}", e);
                panic!("{}", e)
            }
        }
    }
}