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
    pub class: Option<String>, // todo: depends on path
    #[serde(alias = "Fach")]
    pub subject: Option<String>, // todo: depends on path
    #[serde(alias = "Typ")]
    pub r#type: String,
    #[serde(alias = "MaxPunkte")]
    pub max_points: Option<f64>,
    #[serde(alias = "Kommentar")]
    pub comment: String,
    #[serde(alias = "Notenspiegel")]
    pub grade_distribution: Option<[i32; 6]>,
}

pub type Assessments = Vec<Assessment>;

impl Api {
    pub fn get_assessment(&self, id: &str) -> anyhow::Result<Assessment> {
        let path = format!("api/LFs/{id}");
        Ok(Api::parse(self.get(&path)?.send()?)?)
    }

    pub fn get_assessments(
        &self,
        class: &Option<String>,
        subject: &Option<String>,
    ) -> anyhow::Result<Assessments> {
        let mat_no = &self.get_login_data()?.mat_no;
        let path = if let Some(ref class) = class {
            if let Some(subject) = subject {
                format!("api/Klassen/{class}/Faecher/{subject}/LFs")
            } else {
                format!("api/Klassen/{class}/LFs")
            }
        } else if let Some(ref subject) = subject {
            format!("api/Schueler/{mat_no}/Faecher/{subject}/Noten")
        } else {
            format!("api/Schueler/{mat_no}/Noten")
        };
        Ok(Api::parse(self.get(&path)?.send()?)?)
    }
}
