use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct TagApi {
    config: Configuration,
}

impl TagApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn get_tag_audit_logs_with_history(
        &self,
        request: GetTagAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, TagApiError> {
        let url = format!(
            "{}/1.0/kb/tags/{}/auditLogsWithHistory",
            self.config.base_path,
            request.tag_id
        );
        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());
        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_tags(
        &self,
        request: GetTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, TagApiError> {
        let url = format!("{}/1.0/kb/tags/pagination", self.config.base_path);
        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&Self::build_pagination_query(&request.offset, &request.limit, &request.audit));
        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn search_tags(
        &self,
        request: SearchTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, TagApiError> {
        let url = format!("{}/1.0/kb/tags/search/{}", self.config.base_path, request.search_key);
        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&Self::build_pagination_query(&request.offset, &request.limit, &request.audit));
        let response = req.send().await?;
        Self::handle_response(response).await
    }

    fn build_pagination_query(
        offset: &Option<i64>,
        limit: &Option<i64>,
        audit: &Option<&str>
    ) -> Vec<(&'static str, String)> {
        let mut query = vec![];
        if let Some(offset) = offset {
            query.push(("offset", offset.to_string()));
        }
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }
        if let Some(audit) = audit {
            query.push(("audit", audit.to_string()));
        }
        query
    }

    async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T, TagApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(TagApiError::from)
            }
            StatusCode::NO_CONTENT => Ok(serde_json::from_str("null").unwrap()),
            status => {
                let text = response.text().await?;
                Err(TagApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetTagAuditLogsWithHistoryRequest<'a> {
    pub(crate) tag_id: &'a str,
}

impl<'a> GetTagAuditLogsWithHistoryRequest<'a> {
    pub fn new(tag_id: &'a str) -> Self {
        Self { tag_id }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GetTagsRequest<'a> {
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetTagsRequest<'a> {
    pub fn builder() -> GetTagsRequestBuilder<'a> {
        GetTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetTagsRequestBuilder<'a> {
    offset: Option<i64>,
    limit: Option<i64>,
    audit: Option<&'a str>,
}

impl<'a> GetTagsRequestBuilder<'a> {
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

    pub fn build(self) -> GetTagsRequest<'a> {
        GetTagsRequest {
            offset: self.offset,
            limit: self.limit,
            audit: self.audit,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SearchTagsRequest<'a> {
    pub(crate) search_key: &'a str,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> SearchTagsRequest<'a> {
    pub fn builder() -> SearchTagsRequestBuilder<'a> {
        SearchTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SearchTagsRequestBuilder<'a> {
    search_key: Option<&'a str>,
    offset: Option<i64>,
    limit: Option<i64>,
    audit: Option<&'a str>,
}

impl<'a> SearchTagsRequestBuilder<'a> {
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

    pub fn build(self) -> SearchTagsRequest<'a> {
        SearchTagsRequest {
            search_key: self.search_key.unwrap_or_default(),
            offset: self.offset,
            limit: self.limit,
            audit: self.audit,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TagApiError {
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

impl TagApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{ AuditLog, Tag };

    #[tokio::test]
    async fn test_get_tag_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/tags/test-tag/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"changeType": "INSERT"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = TagApi::new(config);
        let request = GetTagAuditLogsWithHistoryRequest::new("test-tag");

        let result = api.get_tag_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/tags/pagination")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tagId": "test-tag"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = TagApi::new(config);
        let request = GetTagsRequest::builder().build();

        let result = api.get_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_search_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/tags/search/test-search")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tagId": "test-tag"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = TagApi::new(config);
        let request = SearchTagsRequest::builder().search_key("test-search").build();

        let result = api.search_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
