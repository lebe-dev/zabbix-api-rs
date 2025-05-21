pub mod client;

#[cfg(feature = "host")]
pub mod host;

#[cfg(feature = "host")]
pub mod hostgroup;

#[cfg(feature = "item")]
pub mod item;

#[cfg(feature = "trigger")]
pub mod trigger;

pub mod webscenario;

pub mod template;

pub mod r#macro;

#[cfg(feature = "user")]
pub mod usergroup;

pub mod error;

pub const ZABBIX_EXTEND_PROPERTY_VALUE: &str = "extend";

#[cfg(test)]
mod tests;
