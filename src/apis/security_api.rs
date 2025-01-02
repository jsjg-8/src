use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct SecurityApi {
    config: Configuration,
}

impl SecurityApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn add_role_definition(
        &self,
        request: AddRoleDefinitionRequest<'_>
    ) -> Result<models::RoleDefinition, SecurityApiError> {
        let url = format!("{}/1.0/kb/security/roles", self.config.base_path);

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

    pub async fn add_user_roles(
        &self,
        request: AddUserRolesRequest<'_>
    ) -> Result<models::UserRoles, SecurityApiError> {
        let url = format!("{}/1.0/kb/security/users", self.config.base_path);

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

    pub async fn get_current_user_permissions(&self) -> Result<Vec<String>, SecurityApiError> {
        let url = format!("{}/1.0/kb/security/permissions", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_current_user_subject(&self) -> Result<models::Subject, SecurityApiError> {
        let url = format!("{}/1.0/kb/security/subject", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_role_definition(
        &self,
        request: GetRoleDefinitionRequest<'_>
    ) -> Result<models::RoleDefinition, SecurityApiError> {
        let url = format!("{}/1.0/kb/security/roles/{}", self.config.base_path, request.role);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_user_roles(
        &self,
        request: GetUserRolesRequest<'_>
    ) -> Result<models::UserRoles, SecurityApiError> {
        let url = format!(
            "{}/1.0/kb/security/users/{}/roles",
            self.config.base_path,
            request.username
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn invalidate_user(
        &self,
        request: InvalidateUserRequest<'_>
    ) -> Result<(), SecurityApiError> {
        let url = format!("{}/1.0/kb/security/users/{}", self.config.base_path, request.username);

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn update_role_definition(
        &self,
        request: UpdateRoleDefinitionRequest<'_>
    ) -> Result<(), SecurityApiError> {
        let url = format!("{}/1.0/kb/security/roles", self.config.base_path);

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn update_user_password(
        &self,
        request: UpdateUserPasswordRequest<'_>
    ) -> Result<(), SecurityApiError> {
        let url = format!(
            "{}/1.0/kb/security/users/{}/password",
            self.config.base_path,
            request.username
        );

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn update_user_roles(
        &self,
        request: UpdateUserRolesRequest<'_>
    ) -> Result<(), SecurityApiError> {
        let url = format!(
            "{}/1.0/kb/security/users/{}/roles",
            self.config.base_path,
            request.username
        );

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(
        response: Response
    ) -> Result<T, SecurityApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(SecurityApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(SecurityApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), SecurityApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(SecurityApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddRoleDefinitionRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::RoleDefinition,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> AddRoleDefinitionRequest<'a> {
    pub fn builder() -> AddRoleDefinitionRequestBuilder<'a> {
        AddRoleDefinitionRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct AddRoleDefinitionRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::RoleDefinition>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> AddRoleDefinitionRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::RoleDefinition) -> Self {
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

    pub fn build(self) -> Result<AddRoleDefinitionRequest<'a>, &'static str> {
        Ok(AddRoleDefinitionRequest {
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
pub struct AddUserRolesRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::UserRoles,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> AddUserRolesRequest<'a> {
    pub fn builder() -> AddUserRolesRequestBuilder<'a> {
        AddUserRolesRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct AddUserRolesRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::UserRoles>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> AddUserRolesRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::UserRoles) -> Self {
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

    pub fn build(self) -> Result<AddUserRolesRequest<'a>, &'static str> {
        Ok(AddUserRolesRequest {
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
pub struct GetRoleDefinitionRequest<'a> {
    pub(crate) role: &'a str,
}

impl<'a> GetRoleDefinitionRequest<'a> {
    pub fn builder() -> GetRoleDefinitionRequestBuilder<'a> {
        GetRoleDefinitionRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetRoleDefinitionRequestBuilder<'a> {
    role: Option<&'a str>,
}

impl<'a> GetRoleDefinitionRequestBuilder<'a> {
    pub fn role(mut self, role: &'a str) -> Self {
        self.role = Some(role);
        self
    }

    pub fn build(self) -> Result<GetRoleDefinitionRequest<'a>, &'static str> {
        Ok(GetRoleDefinitionRequest {
            role: self.role.ok_or("role is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetUserRolesRequest<'a> {
    pub(crate) username: &'a str,
}

impl<'a> GetUserRolesRequest<'a> {
    pub fn builder() -> GetUserRolesRequestBuilder<'a> {
        GetUserRolesRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetUserRolesRequestBuilder<'a> {
    username: Option<&'a str>,
}

impl<'a> GetUserRolesRequestBuilder<'a> {
    pub fn username(mut self, username: &'a str) -> Self {
        self.username = Some(username);
        self
    }

    pub fn build(self) -> Result<GetUserRolesRequest<'a>, &'static str> {
        Ok(GetUserRolesRequest {
            username: self.username.ok_or("username is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InvalidateUserRequest<'a> {
    pub(crate) username: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> InvalidateUserRequest<'a> {
    pub fn builder() -> InvalidateUserRequestBuilder<'a> {
        InvalidateUserRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct InvalidateUserRequestBuilder<'a> {
    username: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> InvalidateUserRequestBuilder<'a> {
    pub fn username(mut self, username: &'a str) -> Self {
        self.username = Some(username);
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

    pub fn build(self) -> Result<InvalidateUserRequest<'a>, &'static str> {
        Ok(InvalidateUserRequest {
            username: self.username.ok_or("username is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UpdateRoleDefinitionRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::RoleDefinition,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdateRoleDefinitionRequest<'a> {
    pub fn builder() -> UpdateRoleDefinitionRequestBuilder<'a> {
        UpdateRoleDefinitionRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UpdateRoleDefinitionRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::RoleDefinition>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdateRoleDefinitionRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::RoleDefinition) -> Self {
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

    pub fn build(self) -> Result<UpdateRoleDefinitionRequest<'a>, &'static str> {
        Ok(UpdateRoleDefinitionRequest {
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
pub struct UpdateUserPasswordRequest<'a> {
    pub(crate) username: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::UserRoles,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdateUserPasswordRequest<'a> {
    pub fn builder() -> UpdateUserPasswordRequestBuilder<'a> {
        UpdateUserPasswordRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UpdateUserPasswordRequestBuilder<'a> {
    username: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::UserRoles>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdateUserPasswordRequestBuilder<'a> {
    pub fn username(mut self, username: &'a str) -> Self {
        self.username = Some(username);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::UserRoles) -> Self {
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

    pub fn build(self) -> Result<UpdateUserPasswordRequest<'a>, &'static str> {
        Ok(UpdateUserPasswordRequest {
            username: self.username.ok_or("username is required")?,
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
pub struct UpdateUserRolesRequest<'a> {
    pub(crate) username: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::UserRoles,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdateUserRolesRequest<'a> {
    pub fn builder() -> UpdateUserRolesRequestBuilder<'a> {
        UpdateUserRolesRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UpdateUserRolesRequestBuilder<'a> {
    username: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::UserRoles>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdateUserRolesRequestBuilder<'a> {
    pub fn username(mut self, username: &'a str) -> Self {
        self.username = Some(username);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::UserRoles) -> Self {
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

    pub fn build(self) -> Result<UpdateUserRolesRequest<'a>, &'static str> {
        Ok(UpdateUserRolesRequest {
            username: self.username.ok_or("username is required")?,
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
pub enum SecurityApiError {
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

impl SecurityApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{ RoleDefinition, UserRoles, Subject };

    #[tokio::test]
    async fn test_add_role_definition() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/security/roles")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"name": "test-role"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SecurityApi::new(config);
        let request = AddRoleDefinitionRequest::builder()
            .x_killbill_created_by("test")
            .body(RoleDefinition {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.add_role_definition(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_add_user_roles() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/security/users")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"username": "test-user"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SecurityApi::new(config);
        let request = AddUserRolesRequest::builder()
            .x_killbill_created_by("test")
            .body(UserRoles {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.add_user_roles(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_current_user_permissions() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/security/permissions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"["test-permission"]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SecurityApi::new(config);
        let result = api.get_current_user_permissions().await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_current_user_subject() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/security/subject")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"principal": "test-user"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SecurityApi::new(config);
        let result = api.get_current_user_subject().await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_role_definition() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/security/roles/test-role")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"name": "test-role"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SecurityApi::new(config);
        let request = GetRoleDefinitionRequest::builder().role("test-role").build().unwrap();

        let result = api.get_role_definition(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_user_roles() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/security/users/test-user/roles")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"username": "test-user"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SecurityApi::new(config);
        let request = GetUserRolesRequest::builder().username("test-user").build().unwrap();

        let result = api.get_user_roles(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_invalidate_user() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/security/users/test-user")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SecurityApi::new(config);
        let request = InvalidateUserRequest::builder()
            .username("test-user")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.invalidate_user(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_update_role_definition() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/security/roles")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SecurityApi::new(config);
        let request = UpdateRoleDefinitionRequest::builder()
            .x_killbill_created_by("test")
            .body(RoleDefinition {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.update_role_definition(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_update_user_password() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/security/users/test-user/password")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SecurityApi::new(config);
        let request = UpdateUserPasswordRequest::builder()
            .username("test-user")
            .x_killbill_created_by("test")
            .body(UserRoles {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.update_user_password(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_update_user_roles() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/security/users/test-user/roles")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SecurityApi::new(config);
        let request = UpdateUserRolesRequest::builder()
            .username("test-user")
            .x_killbill_created_by("test")
            .body(UserRoles {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.update_user_roles(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
