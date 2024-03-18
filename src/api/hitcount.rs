use serde::{Deserialize, Serialize};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct HitCount {
    pub hit_count: i32,
    pub hit_count_start_date: String,
}

impl Api {
    pub fn get_hitcount(&self) -> anyhow::Result<HitCount> {
        Ok(Api::parse(self.get("api/HitCount")?.send()?)?)
    }
}
