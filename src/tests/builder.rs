use std::collections::HashMap;
use std::error::Error;

use crate::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
use crate::host::create::TlsConfig;
use crate::host::update::UpdateHostRequest;
use crate::hostgroup::create::CreateHostGroupRequest;
use crate::hostgroup::model::ZabbixHostGroupId;
use crate::webscenario::model::ZabbixWebScenarioStep;
use log::{debug, error};
use reqwest::blocking::Client;

use crate::host::create::CreateHostRequest;
use crate::host::model::ZabbixHostInterface;
use crate::item::create::CreateItemRequest;
use crate::tests::integration::{get_integration_tests_config, IntegrationTestsConfig};
use crate::trigger::create::CreateTriggerRequest;
use crate::webscenario::create::CreateWebScenarioRequest;

use super::logging::init_logging;

pub struct TestEnvBuilder {
    pub client: ZabbixApiClientImpl,
    pub integration_tests_config: IntegrationTestsConfig,
    pub session: String,

    pub latest_host_group_id: u32,
    pub latest_host_group_name: String,
    pub latest_host_id: u32,
    pub latest_item_id: u32,
    pub latest_trigger_id: u32,
    pub latest_webscenario_id: u32,
}

impl TestEnvBuilder {
    pub fn build() -> TestEnvBuilder {
        init_logging();

        let http_client = Client::new();

        let tests_config = get_integration_tests_config();

        TestEnvBuilder {
            client: ZabbixApiClientImpl::new(http_client, &tests_config.zabbix_api_url),
            integration_tests_config: tests_config,
            session: "".to_string(),
            latest_host_group_id: 0,
            latest_host_group_name: "".to_string(),
            latest_host_id: 0,
            latest_item_id: 0,
            latest_trigger_id: 0,
            latest_webscenario_id: 0,
        }
    }

    pub fn get_session(&mut self) -> &mut Self {
        match self.client.get_auth_session(
            &self.integration_tests_config.zabbix_api_user,
            &self.integration_tests_config.zabbix_api_password,
        ) {
            Ok(session) => {
                self.session = session;
                self
            }
            Err(e) => {
                error!("auth error: {}", e);
                panic!("{}", e)
            }
        }
    }

    pub fn create_host_group(&mut self, name: &str) -> &mut Self {
        let request = CreateHostGroupRequest {
            name: name.to_string(),
        };

        match &self.client.create_host_group(&self.session, &request) {
            Ok(host_group_id) => {
                self.latest_host_group_name = name.to_string();
                self.latest_host_group_id = host_group_id.to_owned();
                self
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                error!("host group create error: {}", e);
                panic!("{}", e)
            }
        }
    }

    pub fn create_host(&mut self, name: &str, tls_config: Option<TlsConfig>) -> &mut Self {
        let params = CreateHostRequest {
            host: name.to_string(),
            groups: vec![ZabbixHostGroupId {
                group_id: self.latest_host_group_id.to_string(),
            }],
            interfaces: vec![ZabbixHostInterface { r#type: 1, main: 1, use_ip: 1, ip: "127.0.0.1".to_string(), dns: "".to_string(), port: "10050".to_string() }],
            tags: vec![],
            templates: vec![],
            macros: vec![],
            inventory_mode: 0,
            inventory: HashMap::new(),
            tls_config: tls_config,
            ..Default::default()
        };

        match &self.client.create_host(&self.session, &params) {
            Ok(host_id) => {
                self.latest_host_id = host_id.to_owned();
                self
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                error!("host create error: {}", e);
                panic!("{}", e)
            }
        }
    }

    pub fn update_host(&mut self, update_host: UpdateHostRequest) -> &mut Self {
        use crate::host::get::GetHostsByIdsRequest;

        match &self.client.update_host(&self.session, &update_host) {
            Ok(_) => {
                match &self.client.get_hosts(&self.session, &GetHostsByIdsRequest {
                    hostids: vec![update_host.hostid.clone()],
                }) {
                    Ok(hosts) => {
                        if hosts.is_empty() {
                            error!("host update error: {}", "host not found");
                            panic!("host not found");
                        }

                        let host = hosts.first().unwrap();

                        debug!("host: {:?}, update_host: {:?}", host.status, update_host.status);
                        if host.status != update_host.status {
                            error!("host update error: {}", "host status not updated");
                            panic!("host status not updated");
                        }
                    }
                    Err(e) => {
                        if let Some(inner_source) = e.source() {
                            println!("Caused by: {}", inner_source);
                        }

                        error!("host get error: {}", e);
                        panic!("{}", e)
                    }
                }

                self
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                error!("host update error: {}", e);
                panic!("{}", e)
            }
        }
    }

    pub fn delete_hosts(&mut self, host_ids: &Vec<String>) -> &mut Self {
        match self.client.delete_hosts(&self.session, host_ids) {
            Ok(ids) => {
                println!("Successfully deleted hosts with IDs: {:?}", ids);
                self
            }
            Err(e) => {
                error!("Error deleting hosts: {}", e);
                panic!("{}", e)
            }
        }
    }

    pub fn create_item(&mut self, name: &str, key_: &str) -> &mut Self {
        let params = CreateItemRequest {
            name: name.to_string(),
            key_: key_.to_string(),
            host_id: self.latest_host_id.to_string(),
            r#type: 7,
            value_type: 0,
            interface_id: "0".to_string(),
            tags: vec![],
            delay: "60s".to_string(),
        };

        match &self.client.create_item(&self.session, &params) {
            Ok(item_id) => {
                self.latest_item_id = item_id.to_owned();
                self
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                error!("item create error: {}", e);
                panic!("{}", e)
            }
        }
    }

    pub fn create_trigger(&mut self, description: &str, expression: &str) -> &mut Self {
        let params = CreateTriggerRequest {
            description: description.to_string(),
            expression: expression.to_string(),
            priority: 4,
            recovery_mode: None,
            recovery_expression: None,
            url: None,
            event_name: None,
            dependencies: vec![],
            tags: vec![],
        };

        match &self.client.create_trigger(&self.session, &params) {
            Ok(trigger_id) => {
                self.latest_trigger_id = trigger_id.to_owned();
                self
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                error!("trigger create error: {}", e);
                panic!("{}", e)
            }
        }
    }

    pub fn create_web_scenario(&mut self, name: &str) -> &mut Self {
        let step = ZabbixWebScenarioStep {
            name: "Check github.com page".to_string(),
            url: "https://github.com".to_string(),
            status_codes: "200".to_string(),
            no: "0".to_string(),
        };

        let request = CreateWebScenarioRequest {
            name: name.to_string(),
            host_id: self.latest_host_id.to_string(),
            steps: vec![step],
        };

        match &self.client.create_webscenario(&self.session, &request) {
            Ok(webscenario_id) => {
                self.latest_webscenario_id = webscenario_id.to_owned();
                self
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                error!("web-scenario create error: {}", e);
                panic!("{}", e)
            }
        }
    }
}
