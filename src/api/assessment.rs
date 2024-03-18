use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Assessment {
    #[serde(rename(deserialize = "LF_ID"))]
    pub id: i32,
    #[serde(rename(deserialize = "Datum"))]
    pub date: NaiveDateTime,
    #[serde(rename(deserialize = "Lehrer_ID", serialize = "teacherId"))]
    pub teacher_id: i32,
    #[serde(rename(deserialize = "Klasse"))]
    pub class: String,
    #[serde(rename(deserialize = "Fach"))]
    pub subject: String,
    #[serde(rename(deserialize = "Typ"))]
    pub r#type: String,
    #[serde(rename(deserialize = "MaxPunkte", serialize = "maxPoints"))]
    pub max_points: Option<f64>,
    #[serde(rename(deserialize = "Kommentar"))]
    pub comment: String,
    #[serde(rename(deserialize = "Notenspiegel", serialize = "gradeDistribution"))]
    pub grade_distribution: [i32; 6],
}

impl Api {
    pub fn get_assessment(&self, id: &str) -> anyhow::Result<Assessment> {
        let path = format!("api/LFs/{id}");
        Ok(Api::parse(self.get(&path)?.send()?)?)
    }
}
