use serde::{Deserialize, Serialize};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Absences {
    #[serde(alias = "Matrikelnummer")]
    pub mat_no: i32,
    #[serde(alias = "Nachname")]
    pub lastname: String,
    #[serde(alias = "Vorname")]
    pub firstname: String,
    #[serde(alias = "Klasse")]
    pub class_name: String,
    #[serde(alias = "Fehlstunden_Entschuldigt")]
    pub excused: i32,
    #[serde(alias = "Fehlstunden_NichtEntschuldigt")]
    pub unexcused: i32,
    #[serde(alias = "Fehlstunden_Offen")]
    pub open: i32,
}

impl Api {
    pub fn get_absences(&self) -> anyhow::Result<Absences> {
        let path = format!("api/Schueler/{}/Fehlstunden", self.get_login_data()?.mat_no);
        Ok(Api::parse(self.get(&path)?.send()?)?)
    }
}
