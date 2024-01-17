use log::error;
use reqwest::blocking::Client;

use crate::client::v6::ZabbixApiV6Client;
use crate::client::ZabbixApiClient;
use crate::host::create::CreateHostGroupRequest;
use crate::tests::init_logging;
use crate::tests::integration::{get_integration_tests_config, IntegrationTestsConfig};

pub struct TestEnvBuilder {
    pub client: ZabbixApiV6Client,
    pub integration_tests_config: IntegrationTestsConfig,
    pub session: String,

    pub latest_host_group_id: u32,
    pub latest_host_id: u32
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
            latest_host_group_id: 0,
            latest_host_id: 0
        }
    }

    pub fn get_session(&mut self) -> &mut Self {
        match self.client.get_auth_session(
            &self.integration_tests_config.zabbix_api_user,
            &self.integration_tests_config.zabbix_api_password
        ) {
            Ok(session) => {
                self.session = session;
                self
            },
            Err(e) => {
                error!("auth error: {}", e);
                panic!("{}", e)
            }
        }
    }

    pub fn create_host_group(&mut self, name: &str) -> &Self {
        let request = CreateHostGroupRequest {
            name: name.to_string(),
        };

        match &self.client.create_host_group(&self.session, &request) {
            Ok(host_group_id) => {
                self.latest_host_group_id = host_group_id.to_owned();
                self
            }
            Err(e) => {
                error!("host group create error: {}", e);
                panic!("{}", e)
            }
        }
    }
}