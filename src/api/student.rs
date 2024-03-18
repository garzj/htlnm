use serde::{Deserialize, Serialize};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    #[serde(alias = "Matrikelnummer")]
    pub mat_no: i32,
    #[serde(alias = "Nachname")]
    pub last_name: String,
    #[serde(alias = "Vorname")]
    pub first_name: String,
    #[serde(alias = "Klasse")]
    pub class: String,
    #[serde(alias = "EMailAdresse1")]
    pub email1: String,
    #[serde(alias = "EMailAdresse2")]
    pub email2: Option<String>,
}

impl Api {
    pub fn get_student(&self) -> anyhow::Result<Student> {
        let path = format!("api/Schueler/{}", self.get_login_data()?.mat_no);
        Ok(Api::parse(self.get(&path)?.send()?)?)
    }
}
