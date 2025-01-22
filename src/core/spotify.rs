use std::collections::HashMap;

use base64::{
    engine::{general_purpose, GeneralPurpose},
    Engine,
};
use log::{debug, error};
use reqwest::{Client, Url};
use serde_json::Value;

use crate::AppResult;

use super::config::Config;

#[derive(Debug, Clone)]
pub struct SpotifyClient {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub code: Option<String>,
    pub auth_url: String,
    http_client: Client,
}

impl SpotifyClient {
    pub fn new(config: &Config) -> AppResult<Self> {
        let mut url = Url::parse("https://accounts.spotify.com/authorize")?;

        url.query_pairs_mut().append_pair("response_type", "code");

        let mut error_message = "Failed to create Spotify Client: \n".to_string();

        match config.client_id.clone() {
            Some(client_id) => {
                url.query_pairs_mut().append_pair("client_id", &client_id);
            }
            None => {
                error_message = error_message + "No Client ID provided.";

                error!("{}", error_message);
                panic!("{}", error_message);
            }
        }

        match config.redirect_uri.clone() {
            Some(redirect_uri) => {
                url.query_pairs_mut()
                    .append_pair("redirect_uri", &redirect_uri);
            }
            None => {
                error_message = error_message + "No Redirect URI provided.";

                error!("{}", error_message);
                panic!("{}", error_message);
            }
        }

        match config.scope.clone() {
            Some(scope) => {
                url.query_pairs_mut().append_pair("scope", &scope);
            }
            None => {
                error_message = error_message + "No Scope Provied.";

                error!("{}", error_message);
                panic!("{}", error_message);
            }
        }

        Ok(Self {
            access_token: None,
            refresh_token: None,
            code: None,
            auth_url: url.to_string(),
            http_client: Client::new(),
        })
    }

    pub async fn set_code_and_access_token(
        &mut self,
        code: String,
        config: &Config,
    ) -> AppResult<()> {
        self.code = Some(code.clone());

        if let Some(client_id) = config.client_id.clone() {
            if let Some(client_secret) = config.client_secret.clone() {
                if let Some(redirect_uri) = config.redirect_uri.clone() {
                    let auth_header = format!(
                        "Basic {}",
                        general_purpose::STANDARD.encode(format!(
                            "{}:{}",
                            client_id.clone(),
                            client_secret.clone()
                        ))
                    );

                    let mut body = HashMap::<&str, &str>::new();

                    body.insert("code", &code);
                    body.insert("grant_type", "authorization_code");
                    body.insert("redirect_uri", &redirect_uri);

                    let response = self
                        .http_client
                        .post("https://accounts.spotify.com/api/token")
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .header("Authorization", auth_header)
                        .form(&body)
                        .send()
                        .await?
                        .json::<Value>()
                        .await?;

                    if let Some(access_token) = response.get("access_token") {
                        match access_token.to_owned() {
                            Value::String(access_token) => self.access_token = Some(access_token),
                            _ => {}
                        }
                    }

                    if let Some(refresh_token) = response.get("refresh_token") {
                        match refresh_token.to_owned() {
                            Value::String(refresh_token) => {
                                self.refresh_token = Some(refresh_token)
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
