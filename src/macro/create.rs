use crate::r#macro::macrotype::MacroType;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Debug, Default)]
pub struct CreateZabbixHostMacro {
    #[serde(rename = "macro")]
    pub macro_name: String,
    #[serde(rename = "value")]
    pub macro_value: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub macro_type: Option<MacroType>,
}

impl CreateZabbixHostMacro {
    pub fn builder() -> CreateZabbixHostMacroBuilder {
        CreateZabbixHostMacroBuilder {
            inner: CreateZabbixHostMacro::default(),
        }
    }
}

pub struct CreateZabbixHostMacroBuilder {
    inner: CreateZabbixHostMacro,
}

impl CreateZabbixHostMacroBuilder {
    pub fn macro_name(mut self, value: impl ToString) -> Self {
        self.inner.macro_name = value.to_string();
        self
    }

    pub fn value(mut self, value: impl ToString) -> Self {
        self.inner.macro_value = value.to_string();
        self
    }

    pub fn description(mut self, value: impl ToString) -> Self {
        self.inner.description = Some(value.to_string());
        self
    }

    pub fn text(mut self) -> Self {
        self.inner.macro_type = Some(MacroType::Text);
        self
    }

    pub fn secret(mut self) -> Self {
        self.inner.macro_type = Some(MacroType::Secret);
        self
    }

    pub fn vault(mut self) -> Self {
        self.inner.macro_type = Some(MacroType::Vault);
        self
    }

    pub fn build(self) -> CreateZabbixHostMacro {
        self.inner
    }
}
