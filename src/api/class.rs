use serde::{Deserialize, Serialize};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Class {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
}

type Classes = Vec<Class>;

impl Api {
    pub fn get_classes(&self) -> anyhow::Result<Classes> {
        Ok(Api::parse(self.get("api/Klassen")?.send()?)?)
    }
}
