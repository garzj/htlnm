use serde::{Deserialize, Serialize};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Student {
    #[serde(rename = "Matrikelnummer")]
    pub matrikel_no: i32,
    pub nachname: String,
    pub vorname: String,
    pub klasse: String,
    pub e_mail_addresse1: Option<String>,
    pub e_mail_addresse2: Option<String>,
}

impl Api {
    pub fn get_student(&self) -> anyhow::Result<Student> {
        let path = format!("api/Schueler/{}", self.get_login_data()?.matrikel_nr);
        Ok(Api::parse(self.get(&path)?.send()?)?)
    }
}
