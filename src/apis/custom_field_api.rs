use reqwest::{Method, Response, StatusCode};
use serde::{Deserialize, de::DeserializeOwned};
use crate::{
    apis::configuration::Configuration,
    models,
};
use thiserror::Error;

pub struct CustomFieldApi {
    config: Configuration,
}

impl CustomFieldApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn get_custom_field_audit_logs_with_history(
        &self,
        request: GetCustomFieldAuditLogsWithHistoryRequest<'_>,
    ) -> Result<Vec<models::AuditLog>, CustomFieldApiError> {
        let url = format!("{}/1.0/kb/customFields/{}/auditLogsWithHistory", self.config.base_path, request.custom_field_id);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_custom_fields(
        &self,
        request: GetCustomFieldsRequest<'_>,
    ) -> Result<Vec<models::CustomField>, CustomFieldApiError> {
        let url = format!("{}/1.0/kb/customFields/pagination", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[
                ("offset", request.offset.map(|o| o.to_string())),
                ("limit", request.limit.map(|l| l.to_string())),
                ("audit", request.audit.map(|a| a.to_string())),
            ]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn search_custom_fields(
        &self,
        request: SearchCustomFieldsRequest<'_>,
    ) -> Result<Vec<models::CustomField>, CustomFieldApiError> {
        let url = format!("{}/1.0/kb/customFields/search/{}", self.config.base_path, request.search_key);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[
                ("offset", request.offset.map(|o| o.to_string())),
                ("limit", request.limit.map(|l| l.to_string())),
                ("audit", request.audit.map(|a| a.to_string())),
            ]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn search_custom_fields_by_type_name(
        &self,
        request: SearchCustomFieldsByTypeNameRequest<'_>,
    ) -> Result<Vec<models::CustomField>, CustomFieldApiError> {
        let url = format!("{}/1.0/kb/customFields/search", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[
                ("objectType", request.object_type),
                ("fieldName", request.field_name),
                ("fieldValue", request.field_value),
                ("offset", request.offset.map(|o| o.to_string()).as_deref()),
                ("limit", request.limit.map(|l| l.to_string()).as_deref()),
                ("audit", request.audit),
            ]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(
        response: Response
    ) -> Result<T, CustomFieldApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(CustomFieldApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(CustomFieldApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetCustomFieldAuditLogsWithHistoryRequest<'a> {
    pub(crate) custom_field_id: &'a str,
}

impl<'a> GetCustomFieldAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetCustomFieldAuditLogsWithHistoryRequestBuilder<'a> {
        GetCustomFieldAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetCustomFieldAuditLogsWithHistoryRequestBuilder<'a> {
    custom_field_id: Option<&'a str>,
}

impl<'a> GetCustomFieldAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn custom_field_id(mut self, custom_field_id: &'a str) -> Self {
        self.custom_field_id = Some(custom_field_id);
        self
    }

    pub fn build(self) -> Result<GetCustomFieldAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetCustomFieldAuditLogsWithHistoryRequest {
            custom_field_id: self.custom_field_id.ok_or("custom_field_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetCustomFieldsRequest<'a> {
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetCustomFieldsRequest<'a> {
    pub fn builder() -> GetCustomFieldsRequestBuilder<'a> {
        GetCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetCustomFieldsRequestBuilder<'a> {
    offset: Option<i64>,
    limit: Option<i64>,
    audit: Option<&'a str>,
}

impl<'a> GetCustomFieldsRequestBuilder<'a> {
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetCustomFieldsRequest<'a>, &'static str> {
        Ok(GetCustomFieldsRequest {
            offset: self.offset,
            limit: self.limit,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SearchCustomFieldsRequest<'a> {
    pub(crate) search_key: &'a str,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> SearchCustomFieldsRequest<'a> {
    pub fn builder() -> SearchCustomFieldsRequestBuilder<'a> {
        SearchCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SearchCustomFieldsRequestBuilder<'a> {
    search_key: Option<&'a str>,
    offset: Option<i64>,
    limit: Option<i64>,
    audit: Option<&'a str>,
}

impl<'a> SearchCustomFieldsRequestBuilder<'a> {
    pub fn search_key(mut self, search_key: &'a str) -> Self {
        self.search_key = Some(search_key);
        self
    }

    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<SearchCustomFieldsRequest<'a>, &'static str> {
        Ok(SearchCustomFieldsRequest {
            search_key: self.search_key.ok_or("search_key is required")?,
            offset: self.offset,
            limit: self.limit,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SearchCustomFieldsByTypeNameRequest<'a> {
    pub(crate) object_type: Option<&'a str>,
    pub(crate) field_name: Option<&'a str>,
    pub(crate) field_value: Option<&'a str>,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> SearchCustomFieldsByTypeNameRequest<'a> {
    pub fn builder() -> SearchCustomFieldsByTypeNameRequestBuilder<'a> {
        SearchCustomFieldsByTypeNameRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SearchCustomFieldsByTypeNameRequestBuilder<'a> {
    object_type: Option<&'a str>,
    field_name: Option<&'a str>,
    field_value: Option<&'a str>,
    offset: Option<i64>,
    limit: Option<i64>,
    audit: Option<&'a str>,
}

impl<'a> SearchCustomFieldsByTypeNameRequestBuilder<'a> {
    pub fn object_type(mut self, object_type: &'a str) -> Self {
        self.object_type = Some(object_type);
        self
    }

    pub fn field_name(mut self, field_name: &'a str) -> Self {
        self.field_name = Some(field_name);
        self
    }

    pub fn field_value(mut self, field_value: &'a str) -> Self {
        self.field_value = Some(field_value);
        self
    }

    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<SearchCustomFieldsByTypeNameRequest<'a>, &'static str> {
        Ok(SearchCustomFieldsByTypeNameRequest {
            object_type: self.object_type,
            field_name: self.field_name,
            field_value: self.field_value,
            offset: self.offset,
            limit: self.limit,
            audit: self.audit,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CustomFieldApiError {
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    ApiError {
        status: StatusCode,
        message: String,
    },

    #[error("Configuration error: {0}")]
    ConfigError(#[from] crate::apis::configuration::ConfigError),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl CustomFieldApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{AuditLog, CustomField};

    #[tokio::test]
    async fn test_get_custom_field_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/customFields/test-custom-field-id/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"change_type": "test-change-type"}]"#)
            .create_async()
            .await;

        let config = Configuration::builder()
            .base_path(server.url())
            .build()
            .unwrap();

        let api = CustomFieldApi::new(config);
        let request = GetCustomFieldAuditLogsWithHistoryRequest::builder()
            .custom_field_id("test-custom-field-id")
            .build()
            .unwrap();

        let result = api.get_custom_field_audit_logs_with_history(request).await;
        assert!(result.is_ok());
        
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/customFields/pagination")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-custom-field"}]"#)
            .create_async()
            .await;

        let config = Configuration::builder()
            .base_path(server.url())
            .build()
            .unwrap();

        let api = CustomFieldApi::new(config);
        let request = GetCustomFieldsRequest::builder()
            .build()
            .unwrap();

        let result = api.get_custom_fields(request).await;
        assert!(result.is_ok());
        
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_search_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/customFields/search/test-search-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-custom-field"}]"#)
            .create_async()
            .await;

        let config = Configuration::builder()
            .base_path(server.url())
            .build()
            .unwrap();

        let api = CustomFieldApi::new(config);
        let request = SearchCustomFieldsRequest::builder()
            .search_key("test-search-key")
            .build()
            .unwrap();

        let result = api.search_custom_fields(request).await;
        assert!(result.is_ok());
        
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_search_custom_fields_by_type_name() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/customFields/search")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-custom-field"}]"#)
            .create_async()
            .await;

        let config = Configuration::builder()
            .base_path(server.url())
            .build()
            .unwrap();

        let api = CustomFieldApi::new(config);
        let request = SearchCustomFieldsByTypeNameRequest::builder()
            .build()
            .unwrap();

        let result = api.search_custom_fields_by_type_name(request).await;
        assert!(result.is_ok());
        
        mock.assert_async().await;
    }
}