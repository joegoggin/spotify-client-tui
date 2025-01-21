use log::error;
use reqwest::{Client, Url};

use crate::AppResult;

use super::config::Config;

#[derive(Debug, Clone)]
pub struct SpotifyClient {
    pub access_token: Option<String>,
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
            code: None,
            auth_url: url.to_string(),
            http_client: Client::new(),
        })
    }
}
