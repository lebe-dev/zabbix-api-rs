use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub enum MacroType {
    #[default]
    #[serde(rename = "0")]
    Text,
    #[serde(rename = "1")]
    Secret,
    #[serde(rename = "2")]
    Vault,
}