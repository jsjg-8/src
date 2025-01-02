use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct NodesApi {
    config: Configuration,
}

impl NodesApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn get_nodes_info(
        &self,
    ) -> Result<Vec<models::NodeInfo>, NodesApiError> {
        let url = format!("{}/1.0/kb/nodesInfo", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn trigger_node_command(
        &self,
        request: TriggerNodeCommandRequest<'_>
    ) -> Result<(), NodesApiError> {
        let url = format!("{}/1.0/kb/nodesInfo", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("localNodeOnly", request.local_node_only.map(|b| b.to_string()))])
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T, NodesApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(NodesApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(NodesApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), NodesApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(NodesApiError::from_response(status, text))
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct TriggerNodeCommandRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::NodeCommand,
    pub(crate) local_node_only: Option<bool>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> TriggerNodeCommandRequest<'a> {
    pub fn builder() -> TriggerNodeCommandRequestBuilder<'a> {
        TriggerNodeCommandRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct TriggerNodeCommandRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::NodeCommand>,
    local_node_only: Option<bool>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> TriggerNodeCommandRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::NodeCommand) -> Self {
        self.body = Some(body);
        self
    }

    pub fn local_node_only(mut self, local_node_only: bool) -> Self {
        self.local_node_only = Some(local_node_only);
        self
    }

    pub fn x_killbill_reason(mut self, x_killbill_reason: &'a str) -> Self {
        self.x_killbill_reason = Some(x_killbill_reason);
        self
    }

    pub fn x_killbill_comment(mut self, x_killbill_comment: &'a str) -> Self {
        self.x_killbill_comment = Some(x_killbill_comment);
        self
    }

    pub fn build(self) -> Result<TriggerNodeCommandRequest<'a>, &'static str> {
        Ok(TriggerNodeCommandRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            local_node_only: self.local_node_only,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NodesApiError {
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

impl NodesApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::NodeCommand;

    #[tokio::test]
    async fn test_get_nodes_info() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/nodesInfo")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"nodeName": "test-node"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = NodesApi::new(config);

        let result = api.get_nodes_info().await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_trigger_node_command() {
        let mut server = Server::new_async().await;
        let mock = server.mock("POST", "/1.0/kb/nodesInfo").with_status(204).create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = NodesApi::new(config);
        let request = TriggerNodeCommandRequest::builder()
            .x_killbill_created_by("test")
            .body(NodeCommand {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.trigger_node_command(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
