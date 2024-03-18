use std::str::FromStr;

use reqwest::{blocking::Client, Method, StatusCode, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use self::login::LoginData;

pub mod assessment;
pub mod class;
pub mod hitcount;
pub mod login;
pub mod student;

pub struct Api {
    client: Client,
    login_cache: Option<LoginData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiError {
    error: String,
    error_description: String,
}

impl Api {
    pub fn new() -> Self {
        Api {
            client: reqwest::blocking::Client::new(),
            login_cache: None,
        }
    }

    fn request(
        &self,
        method: Method,
        path: &str,
    ) -> anyhow::Result<reqwest::blocking::RequestBuilder> {
        let api_base_url = Url::from_str("https://notenmanagement.htl-braunau.at/rest/")
            .expect("Failed to parse api base url.");
        let url_str = api_base_url.join(&path)?.to_string();
        let mut builder = self.client.request(method, url_str);
        if let Some(login_cache) = self.login_cache.as_ref() {
            builder = builder.bearer_auth(&login_cache.access_token)
        }
        Ok(builder)
    }

    fn get(&self, path: &str) -> anyhow::Result<reqwest::blocking::RequestBuilder> {
        self.request(Method::GET, path)
    }

    fn post(&self, path: &str) -> anyhow::Result<reqwest::blocking::RequestBuilder> {
        self.request(Method::POST, path)
    }

    fn parse<T>(res: reqwest::blocking::Response) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let status = res.status();
        Ok(match status {
            StatusCode::OK => res.json::<T>()?,
            status => res.json::<ApiError>().map(|err| {
                Err(anyhow::Error::msg(format!(
                    "Status code {status}: {} ({})",
                    err.error, err.error_description
                )))
            })??,
        })
    }
}
