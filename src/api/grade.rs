use serde::{Deserialize, Serialize};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Grade {
    #[serde(alias = "Note")]
    pub grade: Option<i32>,
    #[serde(alias = "Punkte")]
    pub points: Option<f64>,
    #[serde(alias = "Kommentar")]
    pub comment: String,
}

impl Api {
    pub fn get_grade(&self, assessment_id: &str) -> anyhow::Result<Grade> {
        let path = format!(
            "api/LFs/{assessment_id}/Schueler/{}/Noten",
            self.get_login_data()?.mat_no
        );
        Ok(Api::parse(self.get(&path)?.send()?)?)
    }
}
