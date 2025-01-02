use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use uuid::Uuid;
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct PaymentTransactionApi {
    config: Configuration,
}

impl PaymentTransactionApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn create_transaction_custom_fields(
        &self,
        request: CreateTransactionCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, PaymentTransactionApiError> {
        let url = format!(
            "{}/1.0/kb/paymentTransactions/{}/customFields",
            self.config.base_path,
            request.transaction_id
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

    pub async fn create_transaction_tags(
        &self,
        request: CreateTransactionTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, PaymentTransactionApiError> {
        let url = format!(
            "{}/1.0/kb/paymentTransactions/{}/tags",
            self.config.base_path,
            request.transaction_id
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

    pub async fn delete_transaction_custom_fields(
        &self,
        request: DeleteTransactionCustomFieldsRequest<'_>
    ) -> Result<(), PaymentTransactionApiError> {
        let url = format!(
            "{}/1.0/kb/paymentTransactions/{}/customFields",
            self.config.base_path,
            request.transaction_id
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

    pub async fn delete_transaction_tags(
        &self,
        request: DeleteTransactionTagsRequest<'_>
    ) -> Result<(), PaymentTransactionApiError> {
        let url = format!(
            "{}/1.0/kb/paymentTransactions/{}/tags",
            self.config.base_path,
            request.transaction_id
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

    pub async fn get_payment_by_transaction_external_key(
        &self,
        request: GetPaymentByTransactionExternalKeyRequest<'_>
    ) -> Result<models::Payment, PaymentTransactionApiError> {
        let url = format!("{}/1.0/kb/paymentTransactions", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("transactionExternalKey", request.transaction_external_key),
                    ("withPluginInfo", request.with_plugin_info.map(|b| if b { "true" } else { "false" }).unwrap_or("false")),
                    ("withAttempts", request.with_attempts.map(|b| if b { "true" } else { "false" }).unwrap_or("false")),
                    ("audit", request.audit.unwrap_or("")),
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

    pub async fn get_payment_by_transaction_id(
        &self,
        request: GetPaymentByTransactionIdRequest<'_>
    ) -> Result<models::Payment, PaymentTransactionApiError> {
        let url = format!(
            "{}/1.0/kb/paymentTransactions/{}",
            self.config.base_path,
            request.transaction_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("withPluginInfo", request.with_plugin_info.map(|b| b.to_string())),
                    ("withAttempts", request.with_attempts.map(|b| b.to_string())),
                    ("audit", request.audit.map(|s| s.to_string())),
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

    pub async fn get_transaction_audit_logs_with_history(
        &self,
        request: GetTransactionAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, PaymentTransactionApiError> {
        let url = format!(
            "{}/1.0/kb/paymentTransactions/{}/auditLogsWithHistory",
            self.config.base_path,
            request.transaction_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_transaction_custom_fields(
        &self,
        request: GetTransactionCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, PaymentTransactionApiError> {
        let url = format!(
            "{}/1.0/kb/paymentTransactions/{}/customFields",
            self.config.base_path,
            request.transaction_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("audit", request.audit)]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_transaction_tags(
        &self,
        request: GetTransactionTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, PaymentTransactionApiError> {
        let url = format!(
            "{}/1.0/kb/paymentTransactions/{}/tags",
            self.config.base_path,
            request.transaction_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
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

    pub async fn modify_transaction_custom_fields(
        &self,
        request: ModifyTransactionCustomFieldsRequest<'_>
    ) -> Result<(), PaymentTransactionApiError> {
        let url = format!(
            "{}/1.0/kb/paymentTransactions/{}/customFields",
            self.config.base_path,
            request.transaction_id
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

    pub async fn notify_state_changed(
        &self,
        request: NotifyStateChangedRequest<'_>
    ) -> Result<models::Payment, PaymentTransactionApiError> {
        let url = format!(
            "{}/1.0/kb/paymentTransactions/{}",
            self.config.base_path,
            request.transaction_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |names| {
                    names
                        .iter()
                        .map(|p| ("controlPluginName", p.to_string()))
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

    async fn handle_response<T: DeserializeOwned>(
        response: Response
    ) -> Result<T, PaymentTransactionApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(PaymentTransactionApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(PaymentTransactionApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), PaymentTransactionApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(PaymentTransactionApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateTransactionCustomFieldsRequest<'a> {
    pub(crate) transaction_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateTransactionCustomFieldsRequest<'a> {
    pub fn builder() -> CreateTransactionCustomFieldsRequestBuilder<'a> {
        CreateTransactionCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateTransactionCustomFieldsRequestBuilder<'a> {
    transaction_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateTransactionCustomFieldsRequestBuilder<'a> {
    pub fn transaction_id(mut self, transaction_id: &'a str) -> Self {
        self.transaction_id = Some(transaction_id);
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

    pub fn build(self) -> Result<CreateTransactionCustomFieldsRequest<'a>, &'static str> {
        Ok(CreateTransactionCustomFieldsRequest {
            transaction_id: self.transaction_id.ok_or("transaction_id is required")?,
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
pub struct CreateTransactionTagsRequest<'a> {
    pub(crate) transaction_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<Uuid>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateTransactionTagsRequest<'a> {
    pub fn builder() -> CreateTransactionTagsRequestBuilder<'a> {
        CreateTransactionTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateTransactionTagsRequestBuilder<'a> {
    transaction_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateTransactionTagsRequestBuilder<'a> {
    pub fn transaction_id(mut self, transaction_id: &'a str) -> Self {
        self.transaction_id = Some(transaction_id);
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

    pub fn build(self) -> Result<CreateTransactionTagsRequest<'a>, &'static str> {
        Ok(CreateTransactionTagsRequest {
            transaction_id: self.transaction_id.ok_or("transaction_id is required")?,
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
pub struct DeleteTransactionCustomFieldsRequest<'a> {
    pub(crate) transaction_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) custom_field: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteTransactionCustomFieldsRequest<'a> {
    pub fn builder() -> DeleteTransactionCustomFieldsRequestBuilder<'a> {
        DeleteTransactionCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteTransactionCustomFieldsRequestBuilder<'a> {
    transaction_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    custom_field: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteTransactionCustomFieldsRequestBuilder<'a> {
    pub fn transaction_id(mut self, transaction_id: &'a str) -> Self {
        self.transaction_id = Some(transaction_id);
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

    pub fn build(self) -> Result<DeleteTransactionCustomFieldsRequest<'a>, &'static str> {
        Ok(DeleteTransactionCustomFieldsRequest {
            transaction_id: self.transaction_id.ok_or("transaction_id is required")?,
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
pub struct DeleteTransactionTagsRequest<'a> {
    pub(crate) transaction_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) tag_def: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteTransactionTagsRequest<'a> {
    pub fn builder() -> DeleteTransactionTagsRequestBuilder<'a> {
        DeleteTransactionTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteTransactionTagsRequestBuilder<'a> {
    transaction_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    tag_def: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteTransactionTagsRequestBuilder<'a> {
    pub fn transaction_id(mut self, transaction_id: &'a str) -> Self {
        self.transaction_id = Some(transaction_id);
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

    pub fn build(self) -> Result<DeleteTransactionTagsRequest<'a>, &'static str> {
        Ok(DeleteTransactionTagsRequest {
            transaction_id: self.transaction_id.ok_or("transaction_id is required")?,
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
pub struct GetPaymentByTransactionExternalKeyRequest<'a> {
    pub(crate) transaction_external_key: &'a str,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) with_attempts: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentByTransactionExternalKeyRequest<'a> {
    pub fn builder() -> GetPaymentByTransactionExternalKeyRequestBuilder<'a> {
        GetPaymentByTransactionExternalKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentByTransactionExternalKeyRequestBuilder<'a> {
    transaction_external_key: Option<&'a str>,
    with_plugin_info: Option<bool>,
    with_attempts: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentByTransactionExternalKeyRequestBuilder<'a> {
    pub fn transaction_external_key(mut self, transaction_external_key: &'a str) -> Self {
        self.transaction_external_key = Some(transaction_external_key);
        self
    }

    pub fn with_plugin_info(mut self, with_plugin_info: bool) -> Self {
        self.with_plugin_info = Some(with_plugin_info);
        self
    }

    pub fn with_attempts(mut self, with_attempts: bool) -> Self {
        self.with_attempts = Some(with_attempts);
        self
    }

    pub fn plugin_property(mut self, plugin_property: Vec<String>) -> Self {
        self.plugin_property = Some(plugin_property);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetPaymentByTransactionExternalKeyRequest<'a>, &'static str> {
        Ok(GetPaymentByTransactionExternalKeyRequest {
            transaction_external_key: self.transaction_external_key.ok_or(
                "transaction_external_key is required"
            )?,
            with_plugin_info: self.with_plugin_info,
            with_attempts: self.with_attempts,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentByTransactionIdRequest<'a> {
    pub(crate) transaction_id: &'a str,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) with_attempts: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentByTransactionIdRequest<'a> {
    pub fn builder() -> GetPaymentByTransactionIdRequestBuilder<'a> {
        GetPaymentByTransactionIdRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentByTransactionIdRequestBuilder<'a> {
    transaction_id: Option<&'a str>,
    with_plugin_info: Option<bool>,
    with_attempts: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentByTransactionIdRequestBuilder<'a> {
    pub fn transaction_id(mut self, transaction_id: &'a str) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }

    pub fn with_plugin_info(mut self, with_plugin_info: bool) -> Self {
        self.with_plugin_info = Some(with_plugin_info);
        self
    }

    pub fn with_attempts(mut self, with_attempts: bool) -> Self {
        self.with_attempts = Some(with_attempts);
        self
    }

    pub fn plugin_property(mut self, plugin_property: Vec<String>) -> Self {
        self.plugin_property = Some(plugin_property);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetPaymentByTransactionIdRequest<'a>, &'static str> {
        Ok(GetPaymentByTransactionIdRequest {
            transaction_id: self.transaction_id.ok_or("transaction_id is required")?,
            with_plugin_info: self.with_plugin_info,
            with_attempts: self.with_attempts,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetTransactionAuditLogsWithHistoryRequest<'a> {
    pub(crate) transaction_id: &'a str,
}

impl<'a> GetTransactionAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetTransactionAuditLogsWithHistoryRequestBuilder<'a> {
        GetTransactionAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetTransactionAuditLogsWithHistoryRequestBuilder<'a> {
    transaction_id: Option<&'a str>,
}

impl<'a> GetTransactionAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn transaction_id(mut self, transaction_id: &'a str) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }

    pub fn build(self) -> Result<GetTransactionAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetTransactionAuditLogsWithHistoryRequest {
            transaction_id: self.transaction_id.ok_or("transaction_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetTransactionCustomFieldsRequest<'a> {
    pub(crate) transaction_id: &'a str,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetTransactionCustomFieldsRequest<'a> {
    pub fn builder() -> GetTransactionCustomFieldsRequestBuilder<'a> {
        GetTransactionCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetTransactionCustomFieldsRequestBuilder<'a> {
    transaction_id: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetTransactionCustomFieldsRequestBuilder<'a> {
    pub fn transaction_id(mut self, transaction_id: &'a str) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetTransactionCustomFieldsRequest<'a>, &'static str> {
        Ok(GetTransactionCustomFieldsRequest {
            transaction_id: self.transaction_id.ok_or("transaction_id is required")?,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetTransactionTagsRequest<'a> {
    pub(crate) transaction_id: &'a str,
    pub(crate) included_deleted: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetTransactionTagsRequest<'a> {
    pub fn builder() -> GetTransactionTagsRequestBuilder<'a> {
        GetTransactionTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetTransactionTagsRequestBuilder<'a> {
    transaction_id: Option<&'a str>,
    included_deleted: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetTransactionTagsRequestBuilder<'a> {
    pub fn transaction_id(mut self, transaction_id: &'a str) -> Self {
        self.transaction_id = Some(transaction_id);
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

    pub fn build(self) -> Result<GetTransactionTagsRequest<'a>, &'static str> {
        Ok(GetTransactionTagsRequest {
            transaction_id: self.transaction_id.ok_or("transaction_id is required")?,
            included_deleted: self.included_deleted,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModifyTransactionCustomFieldsRequest<'a> {
    pub(crate) transaction_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyTransactionCustomFieldsRequest<'a> {
    pub fn builder() -> ModifyTransactionCustomFieldsRequestBuilder<'a> {
        ModifyTransactionCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ModifyTransactionCustomFieldsRequestBuilder<'a> {
    transaction_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyTransactionCustomFieldsRequestBuilder<'a> {
    pub fn transaction_id(mut self, transaction_id: &'a str) -> Self {
        self.transaction_id = Some(transaction_id);
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

    pub fn build(self) -> Result<ModifyTransactionCustomFieldsRequest<'a>, &'static str> {
        Ok(ModifyTransactionCustomFieldsRequest {
            transaction_id: self.transaction_id.ok_or("transaction_id is required")?,
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
pub struct NotifyStateChangedRequest<'a> {
    pub(crate) transaction_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> NotifyStateChangedRequest<'a> {
    pub fn builder() -> NotifyStateChangedRequestBuilder<'a> {
        NotifyStateChangedRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct NotifyStateChangedRequestBuilder<'a> {
    transaction_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> NotifyStateChangedRequestBuilder<'a> {
    pub fn transaction_id(mut self, transaction_id: &'a str) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::PaymentTransaction) -> Self {
        self.body = Some(body);
        self
    }

    pub fn control_plugin_name(mut self, control_plugin_name: Vec<String>) -> Self {
        self.control_plugin_name = Some(control_plugin_name);
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

    pub fn build(self) -> Result<NotifyStateChangedRequest<'a>, &'static str> {
        Ok(NotifyStateChangedRequest {
            transaction_id: self.transaction_id.ok_or("transaction_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            control_plugin_name: self.control_plugin_name,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PaymentTransactionApiError {
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

impl PaymentTransactionApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{ CustomField, Tag, AuditLog, Payment, PaymentTransaction };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_transaction_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/paymentTransactions/test-transaction-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentTransactionApi::new(config);
        let request = CreateTransactionCustomFieldsRequest::builder()
            .transaction_id("test-transaction-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_transaction_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_transaction_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/paymentTransactions/test-transaction-id/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tag_definition_id": "test-tag-def-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentTransactionApi::new(config);
        let request = CreateTransactionTagsRequest::builder()
            .transaction_id("test-transaction-id")
            .x_killbill_created_by("test")
            .body(vec![Uuid::new_v4()])
            .build()
            .unwrap();

        let result = api.create_transaction_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_transaction_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/paymentTransactions/test-transaction-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentTransactionApi::new(config);
        let request = DeleteTransactionCustomFieldsRequest::builder()
            .transaction_id("test-transaction-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_transaction_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_transaction_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/paymentTransactions/test-transaction-id/tags")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentTransactionApi::new(config);
        let request = DeleteTransactionTagsRequest::builder()
            .transaction_id("test-transaction-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_transaction_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment_by_transaction_external_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock(
                "GET",
                "/1.0/kb/paymentTransactions?transactionExternalKey=test-transaction-external-key"
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"payment_id": "test-payment-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentTransactionApi::new(config);
        let request = GetPaymentByTransactionExternalKeyRequest::builder()
            .transaction_external_key("test-transaction-external-key")
            .build()
            .unwrap();

        let result = api.get_payment_by_transaction_external_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment_by_transaction_id() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/paymentTransactions/test-transaction-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"payment_id": "test-payment-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentTransactionApi::new(config);
        let request = GetPaymentByTransactionIdRequest::builder()
            .transaction_id("test-transaction-id")
            .build()
            .unwrap();

        let result = api.get_payment_by_transaction_id(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_transaction_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/paymentTransactions/test-transaction-id/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"change_type": "test-change-type"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentTransactionApi::new(config);
        let request = GetTransactionAuditLogsWithHistoryRequest::builder()
            .transaction_id("test-transaction-id")
            .build()
            .unwrap();

        let result = api.get_transaction_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_transaction_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/paymentTransactions/test-transaction-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentTransactionApi::new(config);
        let request = GetTransactionCustomFieldsRequest::builder()
            .transaction_id("test-transaction-id")
            .build()
            .unwrap();

        let result = api.get_transaction_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_transaction_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/paymentTransactions/test-transaction-id/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tag_definition_id": "test-tag-def-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentTransactionApi::new(config);
        let request = GetTransactionTagsRequest::builder()
            .transaction_id("test-transaction-id")
            .build()
            .unwrap();

        let result = api.get_transaction_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_modify_transaction_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/paymentTransactions/test-transaction-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentTransactionApi::new(config);
        let request = ModifyTransactionCustomFieldsRequest::builder()
            .transaction_id("test-transaction-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.modify_transaction_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_notify_state_changed() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/paymentTransactions/test-transaction-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"payment_id": "test-payment-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentTransactionApi::new(config);
        let request = NotifyStateChangedRequest::builder()
            .transaction_id("test-transaction-id")
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.notify_state_changed(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
