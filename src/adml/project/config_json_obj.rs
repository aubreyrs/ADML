use serde::{Deserialize, Serialize};

// project configuration file aka adml.json
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConfigJsonObj {
    pub id: String, // used for pack.mcmeta and namespace
    pub display_name: String, // used for custom display in pack.mcmeta
}