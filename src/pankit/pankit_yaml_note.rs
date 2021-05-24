use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PankitYamlNote {
    #[serde(flatten)]
    pub fields: HashMap<String, String>,
    pub deck: Option<String>,
    pub model: Option<String>,
}
