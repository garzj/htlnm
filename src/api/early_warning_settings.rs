use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EarlyWarningSettings {
    #[serde(alias = "EMailAdresse")]
    pub email: String,
    #[serde(alias = "SemesterDatum")]
    pub semester_date: NaiveDateTime,
    #[serde(alias = "LetzterExport")]
    pub last_export: NaiveDateTime,
    #[serde(alias = "LetzterDownload")]
    pub last_download: NaiveDateTime,
}

impl Api {
    pub fn get_early_warning_settings(&self) -> anyhow::Result<EarlyWarningSettings> {
        Ok(Api::parse(
            self.get("api/Fruehwarnungen/Einstellungen")?.send()?,
        )?)
    }
}
