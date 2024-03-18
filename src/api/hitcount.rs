use serde::{Deserialize, Serialize};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HitCount {
    #[serde(alias = "HitCount")]
    pub hit_count: i32,
    #[serde(alias = "HitCountStartDate")]
    pub hit_count_start_date: String,
}

impl Api {
    pub fn get_hitcount(&self) -> anyhow::Result<HitCount> {
        Ok(Api::parse(self.get("api/HitCount")?.send()?)?)
    }
}
