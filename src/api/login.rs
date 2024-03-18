use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc2822, OffsetDateTime, PrimitiveDateTime, UtcOffset};

use super::Api;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginData {
    #[serde(rename = "access_token")]
    pub access_token: String,
    pub class_name: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i32,
    pub matrikel_nr: String,
    pub role: String,
    pub user_name: String,
    #[serde(rename = ".issued")]
    pub issued: String,
    #[serde(rename = ".expires")]
    pub expires: String,
}

impl Api {
    pub fn get_login_data(&self) -> anyhow::Result<&LoginData> {
        Ok(self
            .login_cache
            .as_ref()
            .ok_or(anyhow::Error::msg("No login res."))?)
    }

    pub fn login<'b, F: FnOnce() -> Cow<'b, str>>(
        &mut self,
        login_cache: &Option<LoginData>,
        force_renew: bool,
        username: &str,
        password_cb: F,
    ) -> anyhow::Result<&LoginData> {
        if let Some(res) = login_cache.as_ref() {
            self.login_cache = Some(res.clone());
        }
        let invalid_session = force_renew
            || self
                .login_cache
                .as_ref()
                .map_or(Ok::<bool, anyhow::Error>(true), |res| {
                    let expires_date = PrimitiveDateTime::parse(&res.expires, &Rfc2822)?;
                    let expires_date = expires_date.assume_offset(UtcOffset::UTC);
                    let now = OffsetDateTime::now_utc();
                    let is_expired = now.gt(&expires_date);
                    Ok(is_expired || res.user_name.eq(username))
                })?;
        if invalid_session {
            let password = password_cb();
            let mut params = HashMap::new();
            params.insert("grant_type", "password");
            params.insert("username", &username);
            params.insert("password", &password);
            let res = self.post("Token")?.form(&params).send()?;
            let data: LoginData = Api::parse(res)?;
            self.login_cache.replace(data);
        }
        Ok(&self.login_cache.as_ref().unwrap())
    }
}
