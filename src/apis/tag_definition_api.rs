use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct TagDefinitionApi {
    config: Configuration,
}

impl TagDefinitionApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn create_tag_definition(
        &self,
        request: CreateTagDefinitionRequest<'_>
    ) -> Result<models::TagDefinition, TagDefinitionApiError> {
        let url = format!("{}/1.0/kb/tagDefinitions", self.config.base_path);

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

    pub async fn delete_tag_definition(
        &self,
        request: DeleteTagDefinitionRequest<'_>
    ) -> Result<(), TagDefinitionApiError> {
        let url = format!(
            "{}/1.0/kb/tagDefinitions/{}",
            self.config.base_path,
            request.tag_definition_id
        );

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn get_tag_definition(
        &self,
        request: GetTagDefinitionRequest<'_>
    ) -> Result<models::TagDefinition, TagDefinitionApiError> {
        let url = format!(
            "{}/1.0/kb/tagDefinitions/{}",
            self.config.base_path,
            request.tag_definition_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("audit", request.audit)]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_tag_definition_audit_logs_with_history(
        &self,
        request: GetTagDefinitionAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, TagDefinitionApiError> {
        let url = format!(
            "{}/1.0/kb/tagDefinitions/{}/auditLogsWithHistory",
            self.config.base_path,
            request.tag_definition_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_tag_definitions(
        &self,
        request: GetTagDefinitionsRequest<'_>
    ) -> Result<Vec<models::TagDefinition>, TagDefinitionApiError> {
        let url = format!("{}/1.0/kb/tagDefinitions", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("audit", request.audit)]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(
        response: Response
    ) -> Result<T, TagDefinitionApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(TagDefinitionApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(TagDefinitionApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), TagDefinitionApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(TagDefinitionApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateTagDefinitionRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::TagDefinition,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateTagDefinitionRequest<'a> {
    pub fn builder() -> CreateTagDefinitionRequestBuilder<'a> {
        CreateTagDefinitionRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateTagDefinitionRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::TagDefinition>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateTagDefinitionRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::TagDefinition) -> Self {
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

    pub fn build(self) -> Result<CreateTagDefinitionRequest<'a>, &'static str> {
        Ok(CreateTagDefinitionRequest {
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
pub struct DeleteTagDefinitionRequest<'a> {
    pub(crate) tag_definition_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteTagDefinitionRequest<'a> {
    pub fn builder() -> DeleteTagDefinitionRequestBuilder<'a> {
        DeleteTagDefinitionRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteTagDefinitionRequestBuilder<'a> {
    tag_definition_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteTagDefinitionRequestBuilder<'a> {
    pub fn tag_definition_id(mut self, tag_definition_id: &'a str) -> Self {
        self.tag_definition_id = Some(tag_definition_id);
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

    pub fn build(self) -> Result<DeleteTagDefinitionRequest<'a>, &'static str> {
        Ok(DeleteTagDefinitionRequest {
            tag_definition_id: self.tag_definition_id.ok_or("tag_definition_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetTagDefinitionRequest<'a> {
    pub(crate) tag_definition_id: &'a str,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetTagDefinitionRequest<'a> {
    pub fn builder() -> GetTagDefinitionRequestBuilder<'a> {
        GetTagDefinitionRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetTagDefinitionRequestBuilder<'a> {
    tag_definition_id: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetTagDefinitionRequestBuilder<'a> {
    pub fn tag_definition_id(mut self, tag_definition_id: &'a str) -> Self {
        self.tag_definition_id = Some(tag_definition_id);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetTagDefinitionRequest<'a>, &'static str> {
        Ok(GetTagDefinitionRequest {
            tag_definition_id: self.tag_definition_id.ok_or("tag_definition_id is required")?,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetTagDefinitionAuditLogsWithHistoryRequest<'a> {
    pub(crate) tag_definition_id: &'a str,
}

impl<'a> GetTagDefinitionAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetTagDefinitionAuditLogsWithHistoryRequestBuilder<'a> {
        GetTagDefinitionAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetTagDefinitionAuditLogsWithHistoryRequestBuilder<'a> {
    tag_definition_id: Option<&'a str>,
}

impl<'a> GetTagDefinitionAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn tag_definition_id(mut self, tag_definition_id: &'a str) -> Self {
        self.tag_definition_id = Some(tag_definition_id);
        self
    }

    pub fn build(self) -> Result<GetTagDefinitionAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetTagDefinitionAuditLogsWithHistoryRequest {
            tag_definition_id: self.tag_definition_id.ok_or("tag_definition_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetTagDefinitionsRequest<'a> {
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetTagDefinitionsRequest<'a> {
    pub fn builder() -> GetTagDefinitionsRequestBuilder<'a> {
        GetTagDefinitionsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetTagDefinitionsRequestBuilder<'a> {
    audit: Option<&'a str>,
}

impl<'a> GetTagDefinitionsRequestBuilder<'a> {
    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetTagDefinitionsRequest<'a>, &'static str> {
        Ok(GetTagDefinitionsRequest {
            audit: self.audit,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TagDefinitionApiError {
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

impl TagDefinitionApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{ TagDefinition, AuditLog };

    #[tokio::test]
    async fn test_create_tag_definition() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/tagDefinitions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"name": "test-tag"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = TagDefinitionApi::new(config);
        let request = CreateTagDefinitionRequest::builder()
            .x_killbill_created_by("test")
            .body(TagDefinition {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.create_tag_definition(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_tag_definition() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/tagDefinitions/test-tag")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = TagDefinitionApi::new(config);
        let request = DeleteTagDefinitionRequest::builder()
            .tag_definition_id("test-tag")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_tag_definition(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_tag_definition() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/tagDefinitions/test-tag")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"name": "test-tag"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = TagDefinitionApi::new(config);
        let request = GetTagDefinitionRequest::builder()
            .tag_definition_id("test-tag")
            .build()
            .unwrap();

        let result = api.get_tag_definition(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_tag_definition_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/tagDefinitions/test-tag/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"changeType": "INSERT"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = TagDefinitionApi::new(config);
        let request = GetTagDefinitionAuditLogsWithHistoryRequest::builder()
            .tag_definition_id("test-tag")
            .build()
            .unwrap();

        let result = api.get_tag_definition_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_tag_definitions() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/tagDefinitions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-tag"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = TagDefinitionApi::new(config);
        let request = GetTagDefinitionsRequest::builder().build().unwrap();

        let result = api.get_tag_definitions(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
