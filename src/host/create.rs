use super::model::{ZabbixHostInterface, ZabbixHostInventory, ZabbixHostTag};
use crate::r#macro::create::CreateZabbixHostMacro;
use crate::{hostgroup::model::ZabbixHostGroupId, template::model::ZabbixTemplateId};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;

const PSK: u8 = 2;
const CERT: u8 = 4;

#[derive(Serialize, Debug)]
#[skip_serializing_none]
pub struct TlsConfig {
    tls_connect: u8,
    tls_accept: u8,
    tls_psk_identity: Option<String>,
    tls_psk: Option<String>,
    tls_issuer: Option<String>,
    tls_subject: Option<String>,
}

impl TlsConfig {
    pub fn new_psk(psk_identity: String, psk: String) -> TlsConfig {
        TlsConfig {
            tls_connect: PSK,
            tls_accept: PSK,
            tls_psk_identity: Some(psk_identity),
            tls_psk: Some(psk),
            tls_issuer: None,
            tls_subject: None,
        }
    }

    pub fn new_cert(issuer: String, subject: String) -> TlsConfig {
        TlsConfig {
            tls_connect: CERT,
            tls_accept: CERT,
            tls_psk_identity: None,
            tls_psk: None,
            tls_issuer: Some(issuer),
            tls_subject: Some(subject),
        }
    }
}

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/create
#[derive(Serialize, Debug, Default)]
#[skip_serializing_none]
pub struct CreateHostRequest {
    pub host: String,
    pub name: Option<String>,
    pub groups: Vec<ZabbixHostGroupId>,
    pub interfaces: Vec<ZabbixHostInterface>,
    pub tags: Vec<ZabbixHostTag>,
    pub templates: Vec<ZabbixTemplateId>,
    pub macros: Vec<CreateZabbixHostMacro>,
    pub inventory_mode: InventoryMode,
    pub inventory: ZabbixHostInventory,
    #[serde(flatten)]
    pub tls_config: Option<TlsConfig>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize_repr, Serialize_repr)]
#[repr(i8)]
pub enum InventoryMode {
    #[default]
    Disabled = -1,
    Manual = 0,
    Automatic = 1,
}

impl CreateHostRequest {
    pub fn builder(host: impl ToString) -> CreateHostRequestBuilder {
        CreateHostRequestBuilder::new(host)
    }
}

pub struct CreateHostRequestBuilder {
    inner: CreateHostRequest,
}

impl CreateHostRequestBuilder {
    pub fn new(host: impl ToString) -> Self {
        Self {
            inner: CreateHostRequest {
                host: host.to_string(),
                ..Default::default()
            },
        }
    }

    pub fn name(mut self, name: impl ToString) -> Self {
        self.inner.name = Some(name.to_string());
        self
    }

    pub fn group(mut self, group_id: impl ToString) -> Self {
        self.inner.groups.push(ZabbixHostGroupId {
            group_id: group_id.to_string(),
        });
        self
    }

    pub fn groups(mut self, groups: Vec<ZabbixHostGroupId>) -> Self {
        self.inner.groups = groups;
        self
    }

    pub fn interface(mut self, interface: ZabbixHostInterface) -> Self {
        self.inner.interfaces.push(interface);
        self
    }

    pub fn interfaces(mut self, interfaces: Vec<ZabbixHostInterface>) -> Self {
        self.inner.interfaces = interfaces;
        self
    }

    pub fn tag(mut self, tag: impl ToString, value: impl ToString) -> Self {
        self.inner.tags.push(ZabbixHostTag {
            tag: tag.to_string(),
            value: value.to_string(),
        });
        self
    }

    pub fn tags(mut self, tags: Vec<ZabbixHostTag>) -> Self {
        self.inner.tags = tags;
        self
    }

    pub fn template(mut self, template_id: impl ToString) -> Self {
        self.inner.templates.push(ZabbixTemplateId {
            template_id: template_id.to_string(),
        });
        self
    }

    pub fn templates(mut self, templates: Vec<ZabbixTemplateId>) -> Self {
        self.inner.templates = templates;
        self
    }

    pub fn macro_entry(mut self, macro_entry: CreateZabbixHostMacro) -> Self {
        self.inner.macros.push(macro_entry);
        self
    }

    pub fn macros(mut self, macros: Vec<CreateZabbixHostMacro>) -> Self {
        self.inner.macros = macros;
        self
    }

    pub fn inventory_mode(mut self, mode: InventoryMode) -> Self {
        self.inner.inventory_mode = mode;
        self
    }

    pub fn inventory_disabled(mut self) -> Self {
        self.inner.inventory_mode = InventoryMode::Disabled;
        self
    }

    pub fn inventory_manual(mut self) -> Self {
        self.inner.inventory_mode = InventoryMode::Manual;
        self
    }

    pub fn inventory_automatic(mut self) -> Self {
        self.inner.inventory_mode = InventoryMode::Automatic;
        self
    }

    pub fn inventory(mut self, inventory: ZabbixHostInventory) -> Self {
        self.inner.inventory = inventory;
        self
    }

    pub fn tls_psk(mut self, psk_identity: impl ToString, psk: impl ToString) -> Self {
        self.inner.tls_config = Some(TlsConfig::new_psk(
            psk_identity.to_string(),
            psk.to_string(),
        ));
        self
    }

    pub fn tls_cert(mut self, issuer: impl ToString, subject: impl ToString) -> Self {
        self.inner.tls_config = Some(TlsConfig::new_cert(issuer.to_string(), subject.to_string()));
        self
    }

    pub fn tls_config(mut self, tls_config: TlsConfig) -> Self {
        self.inner.tls_config = Some(tls_config);
        self
    }

    pub fn build(self) -> CreateHostRequest {
        self.inner
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateHostResponse {
    #[serde(rename = "hostids")]
    pub host_ids: Vec<String>,
}
