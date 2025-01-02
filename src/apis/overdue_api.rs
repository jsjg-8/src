use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct OverdueApi {
    config: Configuration,
}

impl OverdueApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn get_overdue_config_json(&self) -> Result<models::Overdue, OverdueApiError> {
        let url = format!("{}/1.0/kb/overdue", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_overdue_config_xml(&self) -> Result<String, OverdueApiError> {
        let url = format!("{}/1.0/kb/overdue/xml", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn upload_overdue_config_json(
        &self,
        request: UploadOverdueConfigJsonRequest<'_>
    ) -> Result<models::Overdue, OverdueApiError> {
        let url = format!("{}/1.0/kb/overdue", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn upload_overdue_config_xml(
        &self,
        request: UploadOverdueConfigXmlRequest<'_>
    ) -> Result<String, OverdueApiError> {
        let url = format!("{}/1.0/kb/overdue/xml", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(
        response: Response
    ) -> Result<T, OverdueApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(OverdueApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(OverdueApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct UploadOverdueConfigJsonRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::Overdue,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadOverdueConfigJsonRequest<'a> {
    pub fn builder() -> UploadOverdueConfigJsonRequestBuilder<'a> {
        UploadOverdueConfigJsonRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UploadOverdueConfigJsonRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::Overdue>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadOverdueConfigJsonRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::Overdue) -> Self {
        self.body = Some(body);
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

    pub fn build(self) -> Result<UploadOverdueConfigJsonRequest<'a>, &'static str> {
        Ok(UploadOverdueConfigJsonRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UploadOverdueConfigXmlRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadOverdueConfigXmlRequest<'a> {
    pub fn builder() -> UploadOverdueConfigXmlRequestBuilder<'a> {
        UploadOverdueConfigXmlRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UploadOverdueConfigXmlRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadOverdueConfigXmlRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: &'a str) -> Self {
        self.body = Some(body);
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

    pub fn build(self) -> Result<UploadOverdueConfigXmlRequest<'a>, &'static str> {
        Ok(UploadOverdueConfigXmlRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OverdueApiError {
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

impl OverdueApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::Overdue;

    #[tokio::test]
    async fn test_get_overdue_config_json() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/overdue")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"name": "test-overdue"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = OverdueApi::new(config);
        let result = api.get_overdue_config_json().await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_overdue_config_xml() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/overdue/xml")
            .with_status(200)
            .with_header("content-type", "application/xml")
            .with_body("<overdue></overdue>")
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = OverdueApi::new(config);
        let result = api.get_overdue_config_xml().await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_upload_overdue_config_json() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/overdue")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"name": "test-overdue"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = OverdueApi::new(config);
        let request = UploadOverdueConfigJsonRequest::builder()
            .x_killbill_created_by("test")
            .body(Overdue {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.upload_overdue_config_json(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_upload_overdue_config_xml() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/overdue/xml")
            .with_status(200)
            .with_header("content-type", "application/xml")
            .with_body("<overdue></overdue>")
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = OverdueApi::new(config);
        let request = UploadOverdueConfigXmlRequest::builder()
            .x_killbill_created_by("test")
            .body("<overdue></overdue>")
            .build()
            .unwrap();

        let result = api.upload_overdue_config_xml(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
