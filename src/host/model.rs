use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::cmp::PartialEq;
use std::str::FromStr;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum HostStatus {
    Enabled = 0,
    Disabled = 1,
}

impl FromStr for HostStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(HostStatus::Enabled),
            "1" => Ok(HostStatus::Disabled),
            _ => Err(()),
        }
    }
}

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/object
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct ZabbixHost {
    #[serde(rename = "hostid")]
    pub host_id: String,
    pub host: String,
    pub status: HostStatus,
}

// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/object#host-tag
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ZabbixHostTag {
    pub tag: String,
    pub value: String,
}

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/hostinterface/object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ZabbixHostInterface {
    pub r#type: u8,
    pub main: u8,
    pub ip: String,
    pub dns: String,
    pub port: String,
    #[serde(rename = "useip")]
    pub use_ip: u8,
}

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/object#host-inventory
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
#[skip_serializing_none]
pub struct ZabbixHostInventory {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub type_full: Option<String>,
    pub name: Option<String>,
    pub alias: Option<String>,
    pub os: Option<String>,
    pub os_full: Option<String>,
    pub os_short: Option<String>,
    pub serialno_a: Option<String>,
    pub serialno_b: Option<String>,
    pub tag: Option<String>,
    pub asset_tag: Option<String>,
    pub macaddress_a: Option<String>,
    pub macaddress_b: Option<String>,
    pub hardware: Option<String>,
    pub hardware_full: Option<String>,
    pub software: Option<String>,
    pub software_full: Option<String>,
    pub software_app_a: Option<String>,
    pub software_app_b: Option<String>,
    pub software_app_c: Option<String>,
    pub software_app_d: Option<String>,
    pub software_app_e: Option<String>,
    pub contact: Option<String>,
    pub location: Option<String>,
    pub location_lat: Option<String>,
    pub location_lon: Option<String>,
    pub notes: Option<String>,
    pub chassis: Option<String>,
    pub model: Option<String>,
    pub hw_arch: Option<String>,
    pub vendor: Option<String>,
    pub contract_number: Option<String>,
    pub installer_name: Option<String>,
    pub deployment_status: Option<String>,
    pub url_a: Option<String>,
    pub url_b: Option<String>,
    pub url_c: Option<String>,
    pub host_networks: Option<String>,
    pub host_netmask: Option<String>,
    pub host_router: Option<String>,
    pub oob_ip: Option<String>,
    pub oob_netmask: Option<String>,
    pub oob_router: Option<String>,
    pub date_hw_purchase: Option<String>,
    pub date_hw_install: Option<String>,
    pub date_hw_expiry: Option<String>,
    pub date_hw_decomm: Option<String>,
    pub site_address_a: Option<String>,
    pub site_address_b: Option<String>,
    pub site_address_c: Option<String>,
    pub site_city: Option<String>,
    pub site_state: Option<String>,
    pub site_country: Option<String>,
    pub site_zip: Option<String>,
    pub site_rack: Option<String>,
    pub site_notes: Option<String>,
    pub poc_1_name: Option<String>,
    pub poc_1_email: Option<String>,
    pub poc_1_phone_a: Option<String>,
    pub poc_1_phone_b: Option<String>,
    pub poc_1_cell: Option<String>,
    pub poc_1_screen: Option<String>,
    pub poc_1_notes: Option<String>,
    pub poc_2_name: Option<String>,
    pub poc_2_email: Option<String>,
    pub poc_2_phone_a: Option<String>,
    pub poc_2_phone_b: Option<String>,
    pub poc_2_cell: Option<String>,
    pub poc_2_screen: Option<String>,
    pub poc_2_notes: Option<String>,
}

impl ZabbixHostInventory {
    pub fn builder() -> ZabbixHostInventoryBuilder {
        ZabbixHostInventoryBuilder::default()
    }
}

#[derive(Default)]
pub struct ZabbixHostInventoryBuilder {
    inner: ZabbixHostInventory,
}

impl ZabbixHostInventoryBuilder {
    pub fn r#type(mut self, value: impl ToString) -> Self {
        self.inner.r#type = Some(value.to_string());
        self
    }

    pub fn type_full(mut self, value: impl ToString) -> Self {
        self.inner.type_full = Some(value.to_string());
        self
    }

    pub fn name(mut self, value: impl ToString) -> Self {
        self.inner.name = Some(value.to_string());
        self
    }

    pub fn alias(mut self, value: impl ToString) -> Self {
        self.inner.alias = Some(value.to_string());
        self
    }

    pub fn os(mut self, value: impl ToString) -> Self {
        self.inner.os = Some(value.to_string());
        self
    }

    pub fn os_full(mut self, value: impl ToString) -> Self {
        self.inner.os_full = Some(value.to_string());
        self
    }

    pub fn os_short(mut self, value: impl ToString) -> Self {
        self.inner.os_short = Some(value.to_string());
        self
    }

    pub fn serialno_a(mut self, value: impl ToString) -> Self {
        self.inner.serialno_a = Some(value.to_string());
        self
    }

    pub fn serialno_b(mut self, value: impl ToString) -> Self {
        self.inner.serialno_b = Some(value.to_string());
        self
    }

    pub fn tag(mut self, value: impl ToString) -> Self {
        self.inner.tag = Some(value.to_string());
        self
    }

    pub fn asset_tag(mut self, value: impl ToString) -> Self {
        self.inner.asset_tag = Some(value.to_string());
        self
    }

    pub fn macaddress_a(mut self, value: impl ToString) -> Self {
        self.inner.macaddress_a = Some(value.to_string());
        self
    }

    pub fn macaddress_b(mut self, value: impl ToString) -> Self {
        self.inner.macaddress_b = Some(value.to_string());
        self
    }

    pub fn hardware(mut self, value: impl ToString) -> Self {
        self.inner.hardware = Some(value.to_string());
        self
    }

    pub fn hardware_full(mut self, value: impl ToString) -> Self {
        self.inner.hardware_full = Some(value.to_string());
        self
    }

    pub fn software(mut self, value: impl ToString) -> Self {
        self.inner.software = Some(value.to_string());
        self
    }

    pub fn software_full(mut self, value: impl ToString) -> Self {
        self.inner.software_full = Some(value.to_string());
        self
    }

    pub fn software_app_a(mut self, value: impl ToString) -> Self {
        self.inner.software_app_a = Some(value.to_string());
        self
    }

    pub fn software_app_b(mut self, value: impl ToString) -> Self {
        self.inner.software_app_b = Some(value.to_string());
        self
    }

    pub fn software_app_c(mut self, value: impl ToString) -> Self {
        self.inner.software_app_c = Some(value.to_string());
        self
    }

    pub fn software_app_d(mut self, value: impl ToString) -> Self {
        self.inner.software_app_d = Some(value.to_string());
        self
    }

    pub fn software_app_e(mut self, value: impl ToString) -> Self {
        self.inner.software_app_e = Some(value.to_string());
        self
    }

    pub fn contact(mut self, value: impl ToString) -> Self {
        self.inner.contact = Some(value.to_string());
        self
    }

    pub fn location(mut self, value: impl ToString) -> Self {
        self.inner.location = Some(value.to_string());
        self
    }

    pub fn location_lat(mut self, value: impl ToString) -> Self {
        self.inner.location_lat = Some(value.to_string());
        self
    }

    pub fn location_lon(mut self, value: impl ToString) -> Self {
        self.inner.location_lon = Some(value.to_string());
        self
    }

    pub fn notes(mut self, value: impl ToString) -> Self {
        self.inner.notes = Some(value.to_string());
        self
    }

    pub fn chassis(mut self, value: impl ToString) -> Self {
        self.inner.chassis = Some(value.to_string());
        self
    }

    pub fn model(mut self, value: impl ToString) -> Self {
        self.inner.model = Some(value.to_string());
        self
    }

    pub fn hw_arch(mut self, value: impl ToString) -> Self {
        self.inner.hw_arch = Some(value.to_string());
        self
    }

    pub fn vendor(mut self, value: impl ToString) -> Self {
        self.inner.vendor = Some(value.to_string());
        self
    }

    pub fn contract_number(mut self, value: impl ToString) -> Self {
        self.inner.contract_number = Some(value.to_string());
        self
    }

    pub fn installer_name(mut self, value: impl ToString) -> Self {
        self.inner.installer_name = Some(value.to_string());
        self
    }

    pub fn deployment_status(mut self, value: impl ToString) -> Self {
        self.inner.deployment_status = Some(value.to_string());
        self
    }

    pub fn url_a(mut self, value: impl ToString) -> Self {
        self.inner.url_a = Some(value.to_string());
        self
    }

    pub fn url_b(mut self, value: impl ToString) -> Self {
        self.inner.url_b = Some(value.to_string());
        self
    }

    pub fn url_c(mut self, value: impl ToString) -> Self {
        self.inner.url_c = Some(value.to_string());
        self
    }

    pub fn host_networks(mut self, value: impl ToString) -> Self {
        self.inner.host_networks = Some(value.to_string());
        self
    }

    pub fn host_netmask(mut self, value: impl ToString) -> Self {
        self.inner.host_netmask = Some(value.to_string());
        self
    }

    pub fn host_router(mut self, value: impl ToString) -> Self {
        self.inner.host_router = Some(value.to_string());
        self
    }

    pub fn oob_ip(mut self, value: impl ToString) -> Self {
        self.inner.oob_ip = Some(value.to_string());
        self
    }

    pub fn oob_netmask(mut self, value: impl ToString) -> Self {
        self.inner.oob_netmask = Some(value.to_string());
        self
    }

    pub fn oob_router(mut self, value: impl ToString) -> Self {
        self.inner.oob_router = Some(value.to_string());
        self
    }

    pub fn date_hw_purchase(mut self, value: impl ToString) -> Self {
        self.inner.date_hw_purchase = Some(value.to_string());
        self
    }

    pub fn date_hw_install(mut self, value: impl ToString) -> Self {
        self.inner.date_hw_install = Some(value.to_string());
        self
    }

    pub fn date_hw_expiry(mut self, value: impl ToString) -> Self {
        self.inner.date_hw_expiry = Some(value.to_string());
        self
    }

    pub fn date_hw_decomm(mut self, value: impl ToString) -> Self {
        self.inner.date_hw_decomm = Some(value.to_string());
        self
    }

    pub fn site_address_a(mut self, value: impl ToString) -> Self {
        self.inner.site_address_a = Some(value.to_string());
        self
    }

    pub fn site_address_b(mut self, value: impl ToString) -> Self {
        self.inner.site_address_b = Some(value.to_string());
        self
    }

    pub fn site_address_c(mut self, value: impl ToString) -> Self {
        self.inner.site_address_c = Some(value.to_string());
        self
    }

    pub fn site_city(mut self, value: impl ToString) -> Self {
        self.inner.site_city = Some(value.to_string());
        self
    }

    pub fn site_state(mut self, value: impl ToString) -> Self {
        self.inner.site_state = Some(value.to_string());
        self
    }

    pub fn site_country(mut self, value: impl ToString) -> Self {
        self.inner.site_country = Some(value.to_string());
        self
    }

    pub fn site_zip(mut self, value: impl ToString) -> Self {
        self.inner.site_zip = Some(value.to_string());
        self
    }

    pub fn site_rack(mut self, value: impl ToString) -> Self {
        self.inner.site_rack = Some(value.to_string());
        self
    }

    pub fn site_notes(mut self, value: impl ToString) -> Self {
        self.inner.site_notes = Some(value.to_string());
        self
    }

    pub fn poc_1_name(mut self, value: impl ToString) -> Self {
        self.inner.poc_1_name = Some(value.to_string());
        self
    }

    pub fn poc_1_email(mut self, value: impl ToString) -> Self {
        self.inner.poc_1_email = Some(value.to_string());
        self
    }

    pub fn poc_1_phone_a(mut self, value: impl ToString) -> Self {
        self.inner.poc_1_phone_a = Some(value.to_string());
        self
    }

    pub fn poc_1_phone_b(mut self, value: impl ToString) -> Self {
        self.inner.poc_1_phone_b = Some(value.to_string());
        self
    }

    pub fn poc_1_cell(mut self, value: impl ToString) -> Self {
        self.inner.poc_1_cell = Some(value.to_string());
        self
    }

    pub fn poc_1_screen(mut self, value: impl ToString) -> Self {
        self.inner.poc_1_screen = Some(value.to_string());
        self
    }

    pub fn poc_1_notes(mut self, value: impl ToString) -> Self {
        self.inner.poc_1_notes = Some(value.to_string());
        self
    }

    pub fn poc_2_name(mut self, value: impl ToString) -> Self {
        self.inner.poc_2_name = Some(value.to_string());
        self
    }

    pub fn poc_2_email(mut self, value: impl ToString) -> Self {
        self.inner.poc_2_email = Some(value.to_string());
        self
    }

    pub fn poc_2_phone_a(mut self, value: impl ToString) -> Self {
        self.inner.poc_2_phone_a = Some(value.to_string());
        self
    }

    pub fn poc_2_phone_b(mut self, value: impl ToString) -> Self {
        self.inner.poc_2_phone_b = Some(value.to_string());
        self
    }

    pub fn poc_2_cell(mut self, value: impl ToString) -> Self {
        self.inner.poc_2_cell = Some(value.to_string());
        self
    }

    pub fn poc_2_screen(mut self, value: impl ToString) -> Self {
        self.inner.poc_2_screen = Some(value.to_string());
        self
    }

    pub fn poc_2_notes(mut self, value: impl ToString) -> Self {
        self.inner.poc_2_notes = Some(value.to_string());
        self
    }

    pub fn build(self) -> ZabbixHostInventory {
        self.inner
    }
}
