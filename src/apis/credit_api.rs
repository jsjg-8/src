use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct CreditApi {
    config: Configuration,
}

impl CreditApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn create_credits(
        &self,
        request: CreateCreditsRequest<'_>
    ) -> Result<Vec<models::InvoiceItem>, CreditApiError> {
        let url = format!("{}/1.0/kb/credits", self.config.base_path);

        let headers = self.config.get_auth_headers();

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(headers)
            .query(&[("autoCommit", request.auto_commit.map(|b| b.to_string()))])
            .query(
                &request.plugin_property.as_ref().map_or(vec![], |props| {
                    props
                        .iter()
                        .map(|p| ("pluginProperty", p.to_string()))
                        .collect()
                })
            )
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_credit(
        &self,
        request: GetCreditRequest<'_>
    ) -> Result<models::InvoiceItem, CreditApiError> {
        let url = format!("{}/1.0/kb/credits/{}", self.config.base_path, request.credit_id);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T, CreditApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(CreditApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(CreditApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateCreditsRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::InvoiceItem>,
    pub(crate) auto_commit: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateCreditsRequest<'a> {
    pub fn builder() -> CreateCreditsRequestBuilder<'a> {
        CreateCreditsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateCreditsRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::InvoiceItem>>,
    auto_commit: Option<bool>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateCreditsRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: Vec<models::InvoiceItem>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn auto_commit(mut self, auto_commit: bool) -> Self {
        self.auto_commit = Some(auto_commit);
        self
    }

    pub fn plugin_property(mut self, plugin_property: Vec<String>) -> Self {
        self.plugin_property = Some(plugin_property);
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

    pub fn build(self) -> Result<CreateCreditsRequest<'a>, &'static str> {
        Ok(CreateCreditsRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            auto_commit: self.auto_commit,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetCreditRequest<'a> {
    pub(crate) credit_id: &'a str,
}

impl<'a> GetCreditRequest<'a> {
    pub fn builder() -> GetCreditRequestBuilder<'a> {
        GetCreditRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetCreditRequestBuilder<'a> {
    credit_id: Option<&'a str>,
}

impl<'a> GetCreditRequestBuilder<'a> {
    pub fn credit_id(mut self, credit_id: &'a str) -> Self {
        self.credit_id = Some(credit_id);
        self
    }

    pub fn build(self) -> Result<GetCreditRequest<'a>, &'static str> {
        Ok(GetCreditRequest {
            credit_id: self.credit_id.ok_or("credit_id is required")?,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreditApiError {
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

impl CreditApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::InvoiceItem;

    #[tokio::test]
    async fn test_create_credits() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/credits")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"id": "test-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = CreditApi::new(config);
        let request = CreateCreditsRequest::builder()
            .x_killbill_created_by("test")
            .body(
                vec![InvoiceItem {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_credits(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_credit() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/credits/test-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": "test-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = CreditApi::new(config);
        let request = GetCreditRequest::builder().credit_id("test-id").build().unwrap();

        let result = api.get_credit(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
