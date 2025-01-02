use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct UsageApi {
    config: Configuration,
}

impl UsageApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn get_all_usage(
        &self,
        request: GetAllUsageRequest<'_>
    ) -> Result<models::RolledUpUsage, UsageApiError> {
        let url = format!("{}/1.0/kb/usages/{}", self.config.base_path, request.subscription_id);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("startDate", request.start_date),
                    ("endDate", request.end_date),
                ]
            )
            .query(
                &request.plugin_property.as_ref().map_or(vec![], |props| {
                    props
                        .iter()
                        .map(|p| ("pluginProperty", p.to_string()))
                        .collect()
                })
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_usage(
        &self,
        request: GetUsageRequest<'_>
    ) -> Result<models::RolledUpUsage, UsageApiError> {
        let url = format!(
            "{}/1.0/kb/usages/{}/{}",
            self.config.base_path,
            request.subscription_id,
            request.unit_type
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("startDate", request.start_date),
                    ("endDate", request.end_date),
                ]
            )
            .query(
                &request.plugin_property.as_ref().map_or(vec![], |props| {
                    props
                        .iter()
                        .map(|p| ("pluginProperty", p.to_string()))
                        .collect()
                })
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn record_usage(&self, request: RecordUsageRequest<'_>) -> Result<(), UsageApiError> {
        let url = format!("{}/1.0/kb/usages", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T, UsageApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(UsageApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(UsageApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), UsageApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(UsageApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetAllUsageRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) start_date: Option<String>,
    pub(crate) end_date: Option<String>,
    pub(crate) plugin_property: Option<Vec<String>>,
}

impl<'a> GetAllUsageRequest<'a> {
    pub fn builder() -> GetAllUsageRequestBuilder<'a> {
        GetAllUsageRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAllUsageRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    start_date: Option<String>,
    end_date: Option<String>,
    plugin_property: Option<Vec<String>>,
}

impl<'a> GetAllUsageRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }

    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }

    pub fn plugin_property(mut self, plugin_property: Vec<String>) -> Self {
        self.plugin_property = Some(plugin_property);
        self
    }

    pub fn build(self) -> Result<GetAllUsageRequest<'a>, &'static str> {
        Ok(GetAllUsageRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
            start_date: self.start_date,
            end_date: self.end_date,
            plugin_property: self.plugin_property,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetUsageRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) unit_type: &'a str,
    pub(crate) start_date: Option<String>,
    pub(crate) end_date: Option<String>,
    pub(crate) plugin_property: Option<Vec<String>>,
}

impl<'a> GetUsageRequest<'a> {
    pub fn builder() -> GetUsageRequestBuilder<'a> {
        GetUsageRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetUsageRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    unit_type: Option<&'a str>,
    start_date: Option<String>,
    end_date: Option<String>,
    plugin_property: Option<Vec<String>>,
}

impl<'a> GetUsageRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn unit_type(mut self, unit_type: &'a str) -> Self {
        self.unit_type = Some(unit_type);
        self
    }

    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }

    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }

    pub fn plugin_property(mut self, plugin_property: Vec<String>) -> Self {
        self.plugin_property = Some(plugin_property);
        self
    }

    pub fn build(self) -> Result<GetUsageRequest<'a>, &'static str> {
        Ok(GetUsageRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
            unit_type: self.unit_type.ok_or("unit_type is required")?,
            start_date: self.start_date,
            end_date: self.end_date,
            plugin_property: self.plugin_property,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RecordUsageRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::SubscriptionUsageRecord,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> RecordUsageRequest<'a> {
    pub fn builder() -> RecordUsageRequestBuilder<'a> {
        RecordUsageRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct RecordUsageRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::SubscriptionUsageRecord>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> RecordUsageRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::SubscriptionUsageRecord) -> Self {
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

    pub fn build(self) -> Result<RecordUsageRequest<'a>, &'static str> {
        Ok(RecordUsageRequest {
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
pub enum UsageApiError {
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

impl UsageApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use models::UnitUsageRecord;
    use uuid::Uuid;
    use crate::models::{ SubscriptionUsageRecord, UsageRecord };

    #[tokio::test]
    async fn test_get_all_usage() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/usages/test-subscription")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"subscriptionId": "test-subscription"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = UsageApi::new(config);
        let request = GetAllUsageRequest::builder()
            .subscription_id("test-subscription")
            .build()
            .unwrap();

        let result = api.get_all_usage(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_usage() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/usages/test-subscription/test-unit")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"subscriptionId": "550e8400-e29b-41d4-a716-446655440000", "unitType": "test-unit"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = UsageApi::new(config);
        let request = GetUsageRequest::builder()
            .subscription_id("test-subscription")
            .unit_type("test-unit")
            .build()
            .unwrap();

        let result = api.get_usage(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_record_usage() {
        let mut server = Server::new_async().await;
        let mock = server.mock("POST", "/1.0/kb/usages").with_status(204).create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = UsageApi::new(config);
        let request = RecordUsageRequest::builder()
            .x_killbill_created_by("test")
            .body(SubscriptionUsageRecord {
                subscription_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
                unit_usage_records: vec![UnitUsageRecord {
                    ..Default::default()
                }],
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.record_usage(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
