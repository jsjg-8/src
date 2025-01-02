use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct ExportApi {
    config: Configuration,
}

impl ExportApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn export_data_for_account(
        &self,
        request: ExportDataForAccountRequest<'_>
    ) -> Result<(), ExportApiError> {
        let url = format!("{}/1.0/kb/export/{}", self.config.base_path, request.account_id);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    async fn handle_empty_response(response: Response) -> Result<(), ExportApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(ExportApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExportDataForAccountRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ExportDataForAccountRequest<'a> {
    pub fn builder() -> ExportDataForAccountRequestBuilder<'a> {
        ExportDataForAccountRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ExportDataForAccountRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ExportDataForAccountRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
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

    pub fn build(self) -> Result<ExportDataForAccountRequest<'a>, &'static str> {
        Ok(ExportDataForAccountRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExportApiError {
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

impl ExportApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_export_data_for_account() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/export/test-account")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = ExportApi::new(config);
        let request = ExportDataForAccountRequest::builder()
            .account_id("test-account")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.export_data_for_account(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
