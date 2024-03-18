use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Assessment {
    #[serde(alias = "LF_ID")]
    pub id: i32,
    #[serde(alias = "Datum")]
    pub date: NaiveDateTime,
    #[serde(alias = "Lehrer_ID")]
    pub teacher_id: i32,
    #[serde(alias = "Klasse")]
    pub class: String,
    #[serde(alias = "Fach")]
    pub subject: String,
    #[serde(alias = "Typ")]
    pub r#type: String,
    #[serde(alias = "MaxPunkte")]
    pub max_points: Option<f64>,
    #[serde(alias = "Kommentar")]
    pub comment: String,
    #[serde(alias = "Notenspiegel")]
    pub grade_distribution: [i32; 6],
}

impl Api {
    pub fn get_assessment(&self, id: &str) -> anyhow::Result<Assessment> {
        let path = format!("api/LFs/{id}");
        Ok(Api::parse(self.get(&path)?.send()?)?)
    }
}
