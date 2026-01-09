use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Default, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum MacroType {
    #[default]
    Text = 0,
    Secret = 1,
    Vault = 2,
}