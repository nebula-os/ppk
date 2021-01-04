use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Builder, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Var {
    value: Option<String>,
    /// String, Number, Boolean
    r#type: Option<String>,
    /// If the value has to be one of the given ones
    oneof: Option<Vec<String>>,
}
