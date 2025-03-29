use std::collections::HashMap;

use async_recursion::async_recursion;
use serde_json::{json, Value};

use crate::{utils::value::GetOrDefault, AppResult};

use super::client::SpotifyClient;

#[derive(Debug, Clone)]
pub struct Device {
    pub available_devices: HashMap<String, String>,
    pub current_device_name: Option<String>,
}

impl Default for Device {
    fn default() -> Self {
        Self {
            available_devices: HashMap::new().into(),
            current_device_name: None,
        }
    }
}

impl Device {
    #[async_recursion]
    pub async fn refresh(&mut self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let auth_header = spotify_client.get_auth_header()?;

        let response = spotify_client
            .http_client
            .get("https://api.spotify.com/v1/me/player/devices")
            .header("Authorization", auth_header)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            spotify_client.refresh_auth_token().await?;

            return self.refresh(spotify_client).await;
        }

        if status == 200 {
            let mut available_devices = HashMap::new();

            let json = response.json::<Value>().await?;
            let devices_array = json.get_array_or_default("devices");

            for device in devices_array {
                let name = device.get_string_or_default("name");
                let id = device.get_string_or_default("id");
                let is_active = device.get_bool_or_default("is_active");

                if is_active {
                    self.current_device_name = Some(name.clone());
                }

                available_devices.insert(name, id);
            }

            self.available_devices = available_devices;
        }

        Ok(())
    }

    pub fn get_available_devices_names(&self) -> Vec<String> {
        let mut devices_names = Vec::new();

        for device in self.available_devices.clone() {
            if let Some(current_device_name) = self.current_device_name.clone() {
                if device.0 == current_device_name {
                    devices_names.push(format!("* {} *", device.0));
                    continue;
                }

                devices_names.push(device.0);
            }
        }

        devices_names
    }

    #[async_recursion]
    pub async fn set_current_device(
        &mut self,
        spotify_client: &mut SpotifyClient,
        device_id: String,
    ) -> AppResult<()> {
        let auth_header = spotify_client.get_auth_header()?;

        let body = json!({
            "device_ids": [&device_id],
            "play": true,
        });

        let response = spotify_client
            .http_client
            .put("https://api.spotify.com/v1/me/player")
            .header("Authorization", auth_header)
            .json(&body)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            spotify_client.refresh_auth_token().await?;

            return self.set_current_device(spotify_client, device_id).await;
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn print_devices(&self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let auth_header = spotify_client.get_auth_header()?;

        let response = spotify_client
            .http_client
            .get("https://api.spotify.com/v1/me/player/devices")
            .header("Authorization", auth_header)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            spotify_client.refresh_auth_token().await?;

            return self.print_devices(spotify_client).await;
        }

        if status == 200 {
            let response_json = response.json::<Value>().await?;

            if let Some(devices) = response_json.get("devices") {
                if let Value::Array(devices) = devices {
                    for device in devices {
                        if let Some(id) = device.get("id") {
                            if let Value::String(id) = id {
                                println!("id: {}", id);
                            }
                        }

                        if let Some(name) = device.get("name") {
                            if let Value::String(name) = name {
                                println!("name: {}", name);
                            }
                        }

                        println!();
                    }
                }
            }
        }

        Ok(())
    }
}
