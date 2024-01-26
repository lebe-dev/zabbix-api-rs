pub mod client;

pub mod host;

pub mod item;

pub mod trigger;

pub mod webscenario;

pub mod template;

pub mod r#macro;

pub mod error;

pub const ZABBIX_EXTEND_PROPERTY_VALUE: &str = "extend";

#[cfg(test)]
mod tests;
