use serde::{Deserialize, Serialize};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    #[serde(alias = "Fach_ID")]
    pub id: i32,
    #[serde(alias = "Fach")]
    pub name: String,
    #[serde(alias = "Fachbezeichnung")]
    pub description: String,
}

pub type Subjects = Vec<Subject>;

impl Api {
    pub fn get_subjects(&self, filter_early_warnings: bool) -> anyhow::Result<Subjects> {
        let mat_no = &self.get_login_data()?.mat_no;
        let path = if filter_early_warnings {
            format!("api/Schueler/{mat_no}/Fruehwarnungen/Faecher")
        } else {
            format!("api/Schueler/{mat_no}/Faecher")
        };
        Ok(Api::parse(self.get(&path)?.send()?)?)
    }
}
