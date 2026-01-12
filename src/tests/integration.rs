use std::env;

const ENV_ZABBIX_API_URL: &str = "ZABBIX_API_URL";
const ENV_ZABBIX_API_USER: &str = "ZABBIX_API_USER";
const ENV_ZABBIX_API_PASSWORD: &str = "ZABBIX_API_PASSWORD";

pub fn are_integration_tests_enabled() -> bool {
    let result = env::var(ENV_ZABBIX_API_URL).is_ok()
        && env::var(ENV_ZABBIX_API_USER).is_ok()
        && env::var(ENV_ZABBIX_API_PASSWORD).is_ok();

    if !result {
        println!("/!\\ integration tests are disabled /!\\")
    }

    result
}

pub struct IntegrationTestsConfig {
    pub zabbix_api_url: String,
    pub zabbix_api_user: String,
    pub zabbix_api_password: String,
}

pub fn get_integration_tests_config() -> IntegrationTestsConfig {
    IntegrationTestsConfig {
        zabbix_api_url: env::var(ENV_ZABBIX_API_URL).unwrap(),
        zabbix_api_user: env::var(ENV_ZABBIX_API_USER).unwrap(),
        zabbix_api_password: env::var(ENV_ZABBIX_API_PASSWORD).unwrap(),
    }
}
