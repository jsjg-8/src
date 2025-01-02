use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use uuid::Uuid;
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct InvoiceItemApi {
    config: Configuration,
}

impl InvoiceItemApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn create_invoice_item_custom_fields(
        &self,
        request: CreateInvoiceItemCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, InvoiceItemApiError> {
        let url = format!(
            "{}/1.0/kb/invoiceItems/{}/customFields",
            self.config.base_path,
            request.invoice_item_id
        );

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

    pub async fn create_invoice_item_tags(
        &self,
        request: CreateInvoiceItemTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, InvoiceItemApiError> {
        let url = format!(
            "{}/1.0/kb/invoiceItems/{}/tags",
            self.config.base_path,
            request.invoice_item_id
        );

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

    pub async fn delete_invoice_item_custom_fields(
        &self,
        request: DeleteInvoiceItemCustomFieldsRequest<'_>
    ) -> Result<(), InvoiceItemApiError> {
        let url = format!(
            "{}/1.0/kb/invoiceItems/{}/customFields",
            self.config.base_path,
            request.invoice_item_id
        );

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.custom_field.as_ref().map_or(vec![], |ids| {
                    ids.iter()
                        .map(|id| ("customField", id.to_string()))
                        .collect()
                })
            )
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn delete_invoice_item_tags(
        &self,
        request: DeleteInvoiceItemTagsRequest<'_>
    ) -> Result<(), InvoiceItemApiError> {
        let url = format!(
            "{}/1.0/kb/invoiceItems/{}/tags",
            self.config.base_path,
            request.invoice_item_id
        );

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.tag_def.as_ref().map_or(vec![], |ids| {
                    ids.iter()
                        .map(|id| ("tagDef", id.to_string()))
                        .collect()
                })
            )
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn get_invoice_item_audit_logs_with_history(
        &self,
        request: GetInvoiceItemAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, InvoiceItemApiError> {
        let url = format!(
            "{}/1.0/kb/invoiceItems/{}/auditLogsWithHistory",
            self.config.base_path,
            request.invoice_item_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice_item_custom_fields(
        &self,
        request: GetInvoiceItemCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, InvoiceItemApiError> {
        let url = format!(
            "{}/1.0/kb/invoiceItems/{}/customFields",
            self.config.base_path,
            request.invoice_item_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("audit", request.audit)]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice_item_tags(
        &self,
        request: GetInvoiceItemTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, InvoiceItemApiError> {
        let url = format!(
            "{}/1.0/kb/invoiceItems/{}/tags",
            self.config.base_path,
            request.invoice_item_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("accountId", request.account_id),
                    (
                        "includedDeleted",
                        request.included_deleted
                            .map(|b| if b { "true" } else { "false" })
                            .unwrap_or("false"),
                    ),
                    ("audit", request.audit.unwrap_or("")),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn modify_invoice_item_custom_fields(
        &self,
        request: ModifyInvoiceItemCustomFieldsRequest<'_>
    ) -> Result<(), InvoiceItemApiError> {
        let url = format!(
            "{}/1.0/kb/invoiceItems/{}/customFields",
            self.config.base_path,
            request.invoice_item_id
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
    ) -> Result<T, InvoiceItemApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(InvoiceItemApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(InvoiceItemApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), InvoiceItemApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(InvoiceItemApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateInvoiceItemCustomFieldsRequest<'a> {
    pub(crate) invoice_item_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInvoiceItemCustomFieldsRequest<'a> {
    pub fn builder() -> CreateInvoiceItemCustomFieldsRequestBuilder<'a> {
        CreateInvoiceItemCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateInvoiceItemCustomFieldsRequestBuilder<'a> {
    invoice_item_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInvoiceItemCustomFieldsRequestBuilder<'a> {
    pub fn invoice_item_id(mut self, invoice_item_id: &'a str) -> Self {
        self.invoice_item_id = Some(invoice_item_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: Vec<models::CustomField>) -> Self {
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

    pub fn build(self) -> Result<CreateInvoiceItemCustomFieldsRequest<'a>, &'static str> {
        Ok(CreateInvoiceItemCustomFieldsRequest {
            invoice_item_id: self.invoice_item_id.ok_or("invoice_item_id is required")?,
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
pub struct CreateInvoiceItemTagsRequest<'a> {
    pub(crate) invoice_item_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<Uuid>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInvoiceItemTagsRequest<'a> {
    pub fn builder() -> CreateInvoiceItemTagsRequestBuilder<'a> {
        CreateInvoiceItemTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateInvoiceItemTagsRequestBuilder<'a> {
    invoice_item_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInvoiceItemTagsRequestBuilder<'a> {
    pub fn invoice_item_id(mut self, invoice_item_id: &'a str) -> Self {
        self.invoice_item_id = Some(invoice_item_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: Vec<Uuid>) -> Self {
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

    pub fn build(self) -> Result<CreateInvoiceItemTagsRequest<'a>, &'static str> {
        Ok(CreateInvoiceItemTagsRequest {
            invoice_item_id: self.invoice_item_id.ok_or("invoice_item_id is required")?,
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
pub struct DeleteInvoiceItemCustomFieldsRequest<'a> {
    pub(crate) invoice_item_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) custom_field: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteInvoiceItemCustomFieldsRequest<'a> {
    pub fn builder() -> DeleteInvoiceItemCustomFieldsRequestBuilder<'a> {
        DeleteInvoiceItemCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteInvoiceItemCustomFieldsRequestBuilder<'a> {
    invoice_item_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    custom_field: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteInvoiceItemCustomFieldsRequestBuilder<'a> {
    pub fn invoice_item_id(mut self, invoice_item_id: &'a str) -> Self {
        self.invoice_item_id = Some(invoice_item_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn custom_field(mut self, custom_field: Vec<Uuid>) -> Self {
        self.custom_field = Some(custom_field);
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

    pub fn build(self) -> Result<DeleteInvoiceItemCustomFieldsRequest<'a>, &'static str> {
        Ok(DeleteInvoiceItemCustomFieldsRequest {
            invoice_item_id: self.invoice_item_id.ok_or("invoice_item_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            custom_field: self.custom_field,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteInvoiceItemTagsRequest<'a> {
    pub(crate) invoice_item_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) tag_def: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteInvoiceItemTagsRequest<'a> {
    pub fn builder() -> DeleteInvoiceItemTagsRequestBuilder<'a> {
        DeleteInvoiceItemTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteInvoiceItemTagsRequestBuilder<'a> {
    invoice_item_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    tag_def: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteInvoiceItemTagsRequestBuilder<'a> {
    pub fn invoice_item_id(mut self, invoice_item_id: &'a str) -> Self {
        self.invoice_item_id = Some(invoice_item_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn tag_def(mut self, tag_def: Vec<Uuid>) -> Self {
        self.tag_def = Some(tag_def);
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

    pub fn build(self) -> Result<DeleteInvoiceItemTagsRequest<'a>, &'static str> {
        Ok(DeleteInvoiceItemTagsRequest {
            invoice_item_id: self.invoice_item_id.ok_or("invoice_item_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            tag_def: self.tag_def,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoiceItemAuditLogsWithHistoryRequest<'a> {
    pub(crate) invoice_item_id: &'a str,
}

impl<'a> GetInvoiceItemAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetInvoiceItemAuditLogsWithHistoryRequestBuilder<'a> {
        GetInvoiceItemAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoiceItemAuditLogsWithHistoryRequestBuilder<'a> {
    invoice_item_id: Option<&'a str>,
}

impl<'a> GetInvoiceItemAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn invoice_item_id(mut self, invoice_item_id: &'a str) -> Self {
        self.invoice_item_id = Some(invoice_item_id);
        self
    }

    pub fn build(self) -> Result<GetInvoiceItemAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetInvoiceItemAuditLogsWithHistoryRequest {
            invoice_item_id: self.invoice_item_id.ok_or("invoice_item_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoiceItemCustomFieldsRequest<'a> {
    pub(crate) invoice_item_id: &'a str,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoiceItemCustomFieldsRequest<'a> {
    pub fn builder() -> GetInvoiceItemCustomFieldsRequestBuilder<'a> {
        GetInvoiceItemCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoiceItemCustomFieldsRequestBuilder<'a> {
    invoice_item_id: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoiceItemCustomFieldsRequestBuilder<'a> {
    pub fn invoice_item_id(mut self, invoice_item_id: &'a str) -> Self {
        self.invoice_item_id = Some(invoice_item_id);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetInvoiceItemCustomFieldsRequest<'a>, &'static str> {
        Ok(GetInvoiceItemCustomFieldsRequest {
            invoice_item_id: self.invoice_item_id.ok_or("invoice_item_id is required")?,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoiceItemTagsRequest<'a> {
    pub(crate) invoice_item_id: &'a str,
    pub(crate) account_id: &'a str,
    pub(crate) included_deleted: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoiceItemTagsRequest<'a> {
    pub fn builder() -> GetInvoiceItemTagsRequestBuilder<'a> {
        GetInvoiceItemTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoiceItemTagsRequestBuilder<'a> {
    invoice_item_id: Option<&'a str>,
    account_id: Option<&'a str>,
    included_deleted: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoiceItemTagsRequestBuilder<'a> {
    pub fn invoice_item_id(mut self, invoice_item_id: &'a str) -> Self {
        self.invoice_item_id = Some(invoice_item_id);
        self
    }

    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn included_deleted(mut self, included_deleted: bool) -> Self {
        self.included_deleted = Some(included_deleted);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetInvoiceItemTagsRequest<'a>, &'static str> {
        Ok(GetInvoiceItemTagsRequest {
            invoice_item_id: self.invoice_item_id.ok_or("invoice_item_id is required")?,
            account_id: self.account_id.ok_or("account_id is required")?,
            included_deleted: self.included_deleted,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModifyInvoiceItemCustomFieldsRequest<'a> {
    pub(crate) invoice_item_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyInvoiceItemCustomFieldsRequest<'a> {
    pub fn builder() -> ModifyInvoiceItemCustomFieldsRequestBuilder<'a> {
        ModifyInvoiceItemCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ModifyInvoiceItemCustomFieldsRequestBuilder<'a> {
    invoice_item_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyInvoiceItemCustomFieldsRequestBuilder<'a> {
    pub fn invoice_item_id(mut self, invoice_item_id: &'a str) -> Self {
        self.invoice_item_id = Some(invoice_item_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: Vec<models::CustomField>) -> Self {
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

    pub fn build(self) -> Result<ModifyInvoiceItemCustomFieldsRequest<'a>, &'static str> {
        Ok(ModifyInvoiceItemCustomFieldsRequest {
            invoice_item_id: self.invoice_item_id.ok_or("invoice_item_id is required")?,
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
pub enum InvoiceItemApiError {
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

impl InvoiceItemApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{ CustomField, AuditLog, Tag };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_invoice_item_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoiceItems/test-invoice-item-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceItemApi::new(config);
        let request = CreateInvoiceItemCustomFieldsRequest::builder()
            .invoice_item_id("test-invoice-item-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_invoice_item_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_invoice_item_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoiceItems/test-invoice-item-id/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tag_definition_id": "test-tag-def-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceItemApi::new(config);
        let request = CreateInvoiceItemTagsRequest::builder()
            .invoice_item_id("test-invoice-item-id")
            .x_killbill_created_by("test")
            .body(vec![Uuid::new_v4()])
            .build()
            .unwrap();

        let result = api.create_invoice_item_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_invoice_item_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/invoiceItems/test-invoice-item-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceItemApi::new(config);
        let request = DeleteInvoiceItemCustomFieldsRequest::builder()
            .invoice_item_id("test-invoice-item-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_invoice_item_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_invoice_item_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/invoiceItems/test-invoice-item-id/tags")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceItemApi::new(config);
        let request = DeleteInvoiceItemTagsRequest::builder()
            .invoice_item_id("test-invoice-item-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_invoice_item_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_item_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoiceItems/test-invoice-item-id/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"change_type": "test-change-type"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceItemApi::new(config);
        let request = GetInvoiceItemAuditLogsWithHistoryRequest::builder()
            .invoice_item_id("test-invoice-item-id")
            .build()
            .unwrap();

        let result = api.get_invoice_item_audit_logs_with_history(request).await;
        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_item_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoiceItems/test-invoice-item-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceItemApi::new(config);
        let request = GetInvoiceItemCustomFieldsRequest::builder()
            .invoice_item_id("test-invoice-item-id")
            .build()
            .unwrap();

        let result = api.get_invoice_item_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_item_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoiceItems/test-invoice-item-id/tags?accountId=test-account-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tag_definition_id": "test-tag-def-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceItemApi::new(config);
        let request = GetInvoiceItemTagsRequest::builder()
            .invoice_item_id("test-invoice-item-id")
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_invoice_item_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_modify_invoice_item_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/invoiceItems/test-invoice-item-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceItemApi::new(config);
        let request = ModifyInvoiceItemCustomFieldsRequest::builder()
            .invoice_item_id("test-invoice-item-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.modify_invoice_item_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
