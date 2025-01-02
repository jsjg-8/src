use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct PluginsApi {
    config: Configuration,
}

impl PluginsApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn get_plugins_info(&self) -> Result<Vec<models::PluginInfo>, PluginsApiError> {
        let url = format!("{}/1.0/kb/pluginsInfo", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(
        response: Response
    ) -> Result<T, PluginsApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(PluginsApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(PluginsApiError::from_response(status, text))
            }
        }
    }
}


#[derive(Debug, thiserror::Error)]
pub enum PluginsApiError {
    #[error("Request failed: {0}")] RequestFailed(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")] ApiError {
        status: StatusCode,
        message: String,
    },

    #[error("Configuration error: {0}")] ConfigError(
        #[from] crate::apis::configuration::ConfigError,
    ),

    #[error("Validation error: {0}")] ValidationError(String),
}

impl PluginsApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_get_plugins_info() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/pluginsInfo")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"pluginName": "test-plugin"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PluginsApi::new(config);

        let result = api.get_plugins_info().await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
