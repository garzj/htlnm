use serde::{Deserialize, Serialize};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    #[serde(alias = "Name")]
    pub name: String,
}

type Classes = Vec<Class>;

impl Api {
    pub fn get_classes(&self) -> anyhow::Result<Classes> {
        Ok(Api::parse(self.get("api/Klassen")?.send()?)?)
    }
}
