use serde::{Deserialize, Serialize};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EearlyWarning {
    #[serde(alias = "Fach_ID")]
    pub id: i32,
    #[serde(alias = "Fach")]
    pub name: String,
    #[serde(alias = "Fachbezeichnung")]
    pub description: String,
}

pub type EarlyWarnings = Vec<EearlyWarning>;

impl Api {
    pub fn get_early_warning(&self, id: i32) -> anyhow::Result<EearlyWarning> {
        Ok(Api::parse(
            self.get(&format!("api/Fruehwarnungen/{id}"))?.send()?,
        )?)
    }

    pub fn get_early_warnings(&self, subject: &Option<String>) -> anyhow::Result<EarlyWarnings> {
        let mat_no = &self.get_login_data()?.mat_no;
        let path = if let Some(ref subject) = subject {
            format!("api/Schueler/{mat_no}/Faecher/{subject}/Fruehwarnungen")
        } else {
            format!("api/Schueler/{mat_no}/Fruehwarnungen")
        };
        Ok(Api::parse(self.get(&path)?.send()?)?)
    }
}
