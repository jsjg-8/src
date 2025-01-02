use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use uuid::Uuid;
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct PaymentMethodApi {
    config: Configuration,
}

impl PaymentMethodApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn create_payment_method_custom_fields(
        &self,
        request: CreatePaymentMethodCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, PaymentMethodApiError> {
        let url = format!(
            "{}/1.0/kb/paymentMethods/{}/customFields",
            self.config.base_path,
            request.payment_method_id
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

    pub async fn delete_payment_method(
        &self,
        request: DeletePaymentMethodRequest<'_>
    ) -> Result<(), PaymentMethodApiError> {
        let url = format!(
            "{}/1.0/kb/paymentMethods/{}",
            self.config.base_path,
            request.payment_method_id
        );

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    (
                        "deleteDefaultPmWithAutoPayOff",
                        request.delete_default_pm_with_auto_pay_off.map(|b| b.to_string()),
                    ),
                    (
                        "forceDefaultPmDeletion",
                        request.force_default_pm_deletion.map(|b| b.to_string()),
                    ),
                ]
            )
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
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn delete_payment_method_custom_fields(
        &self,
        request: DeletePaymentMethodCustomFieldsRequest<'_>
    ) -> Result<(), PaymentMethodApiError> {
        let url = format!(
            "{}/1.0/kb/paymentMethods/{}/customFields",
            self.config.base_path,
            request.payment_method_id
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

    pub async fn get_payment_method(
        &self,
        request: GetPaymentMethodRequest<'_>
    ) -> Result<models::PaymentMethod, PaymentMethodApiError> {
        let url = format!(
            "{}/1.0/kb/paymentMethods/{}",
            self.config.base_path,
            request.payment_method_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("includedDeleted", request.included_deleted.map(|b| b.to_string())),
                    ("withPluginInfo", request.with_plugin_info.map(|b| b.to_string())),
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

    pub async fn get_payment_method_audit_logs_with_history(
        &self,
        request: GetPaymentMethodAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, PaymentMethodApiError> {
        let url = format!(
            "{}/1.0/kb/paymentMethods/{}/auditLogsWithHistory",
            self.config.base_path,
            request.payment_method_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_payment_method_by_key(
        &self,
        request: GetPaymentMethodByKeyRequest<'_>
    ) -> Result<models::PaymentMethod, PaymentMethodApiError> {
        let url = format!("{}/1.0/kb/paymentMethods", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("externalKey", request.external_key),
                    ("includedDeleted", request.included_deleted.map(|b| if b { "true" } else { "false" }).unwrap_or("false")),
                    ("withPluginInfo", request.with_plugin_info.map(|b| if b { "true" } else { "false" }).unwrap_or("false")),
                    ("audit", request.audit.unwrap_or_default()),
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

    pub async fn get_payment_method_custom_fields(
        &self,
        request: GetPaymentMethodCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, PaymentMethodApiError> {
        let url = format!(
            "{}/1.0/kb/paymentMethods/{}/customFields",
            self.config.base_path,
            request.payment_method_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("audit", request.audit)]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_payment_methods(
        &self,
        request: GetPaymentMethodsRequest<'_>
    ) -> Result<Vec<models::PaymentMethod>, PaymentMethodApiError> {
        let url = format!("{}/1.0/kb/paymentMethods/pagination", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("offset", request.offset.map(|o| o.to_string())),
                    ("limit", request.limit.map(|l| l.to_string())),
                    ("pluginName", request.plugin_name.map(|s| s.to_string())),
                    ("withPluginInfo", request.with_plugin_info.map(|b| b.to_string())),
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

    pub async fn modify_payment_method_custom_fields(
        &self,
        request: ModifyPaymentMethodCustomFieldsRequest<'_>
    ) -> Result<(), PaymentMethodApiError> {
        let url = format!(
            "{}/1.0/kb/paymentMethods/{}/customFields",
            self.config.base_path,
            request.payment_method_id
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

    pub async fn search_payment_methods(
        &self,
        request: SearchPaymentMethodsRequest<'_>
    ) -> Result<Vec<models::PaymentMethod>, PaymentMethodApiError> {
        let url = format!(
            "{}/1.0/kb/paymentMethods/search/{}",
            self.config.base_path,
            request.search_key
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("offset", request.offset.map(|o| o.to_string())),
                    ("limit", request.limit.map(|l| l.to_string())),
                    ("pluginName", request.plugin_name.map(|s| s.to_string())),
                    ("withPluginInfo", request.with_plugin_info.map(|b| b.to_string())),
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

    async fn handle_response<T: DeserializeOwned>(
        response: Response
    ) -> Result<T, PaymentMethodApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(PaymentMethodApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(PaymentMethodApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), PaymentMethodApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(PaymentMethodApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreatePaymentMethodCustomFieldsRequest<'a> {
    pub(crate) payment_method_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreatePaymentMethodCustomFieldsRequest<'a> {
    pub fn builder() -> CreatePaymentMethodCustomFieldsRequestBuilder<'a> {
        CreatePaymentMethodCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreatePaymentMethodCustomFieldsRequestBuilder<'a> {
    payment_method_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreatePaymentMethodCustomFieldsRequestBuilder<'a> {
    pub fn payment_method_id(mut self, payment_method_id: &'a str) -> Self {
        self.payment_method_id = Some(payment_method_id);
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

    pub fn build(self) -> Result<CreatePaymentMethodCustomFieldsRequest<'a>, &'static str> {
        Ok(CreatePaymentMethodCustomFieldsRequest {
            payment_method_id: self.payment_method_id.ok_or("payment_method_id is required")?,
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
pub struct DeletePaymentMethodRequest<'a> {
    pub(crate) payment_method_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) delete_default_pm_with_auto_pay_off: Option<bool>,
    pub(crate) force_default_pm_deletion: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeletePaymentMethodRequest<'a> {
    pub fn builder() -> DeletePaymentMethodRequestBuilder<'a> {
        DeletePaymentMethodRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeletePaymentMethodRequestBuilder<'a> {
    payment_method_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    delete_default_pm_with_auto_pay_off: Option<bool>,
    force_default_pm_deletion: Option<bool>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeletePaymentMethodRequestBuilder<'a> {
    pub fn payment_method_id(mut self, payment_method_id: &'a str) -> Self {
        self.payment_method_id = Some(payment_method_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn delete_default_pm_with_auto_pay_off(
        mut self,
        delete_default_pm_with_auto_pay_off: bool
    ) -> Self {
        self.delete_default_pm_with_auto_pay_off = Some(delete_default_pm_with_auto_pay_off);
        self
    }

    pub fn force_default_pm_deletion(mut self, force_default_pm_deletion: bool) -> Self {
        self.force_default_pm_deletion = Some(force_default_pm_deletion);
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

    pub fn build(self) -> Result<DeletePaymentMethodRequest<'a>, &'static str> {
        Ok(DeletePaymentMethodRequest {
            payment_method_id: self.payment_method_id.ok_or("payment_method_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            delete_default_pm_with_auto_pay_off: self.delete_default_pm_with_auto_pay_off,
            force_default_pm_deletion: self.force_default_pm_deletion,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeletePaymentMethodCustomFieldsRequest<'a> {
    pub(crate) payment_method_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) custom_field: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeletePaymentMethodCustomFieldsRequest<'a> {
    pub fn builder() -> DeletePaymentMethodCustomFieldsRequestBuilder<'a> {
        DeletePaymentMethodCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeletePaymentMethodCustomFieldsRequestBuilder<'a> {
    payment_method_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    custom_field: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeletePaymentMethodCustomFieldsRequestBuilder<'a> {
    pub fn payment_method_id(mut self, payment_method_id: &'a str) -> Self {
        self.payment_method_id = Some(payment_method_id);
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

    pub fn build(self) -> Result<DeletePaymentMethodCustomFieldsRequest<'a>, &'static str> {
        Ok(DeletePaymentMethodCustomFieldsRequest {
            payment_method_id: self.payment_method_id.ok_or("payment_method_id is required")?,
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
pub struct GetPaymentMethodRequest<'a> {
    pub(crate) payment_method_id: &'a str,
    pub(crate) included_deleted: Option<bool>,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentMethodRequest<'a> {
    pub fn builder() -> GetPaymentMethodRequestBuilder<'a> {
        GetPaymentMethodRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentMethodRequestBuilder<'a> {
    payment_method_id: Option<&'a str>,
    included_deleted: Option<bool>,
    with_plugin_info: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentMethodRequestBuilder<'a> {
    pub fn payment_method_id(mut self, payment_method_id: &'a str) -> Self {
        self.payment_method_id = Some(payment_method_id);
        self
    }

    pub fn included_deleted(mut self, included_deleted: bool) -> Self {
        self.included_deleted = Some(included_deleted);
        self
    }

    pub fn with_plugin_info(mut self, with_plugin_info: bool) -> Self {
        self.with_plugin_info = Some(with_plugin_info);
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

    pub fn build(self) -> Result<GetPaymentMethodRequest<'a>, &'static str> {
        Ok(GetPaymentMethodRequest {
            payment_method_id: self.payment_method_id.ok_or("payment_method_id is required")?,
            included_deleted: self.included_deleted,
            with_plugin_info: self.with_plugin_info,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentMethodAuditLogsWithHistoryRequest<'a> {
    pub(crate) payment_method_id: &'a str,
}

impl<'a> GetPaymentMethodAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetPaymentMethodAuditLogsWithHistoryRequestBuilder<'a> {
        GetPaymentMethodAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentMethodAuditLogsWithHistoryRequestBuilder<'a> {
    payment_method_id: Option<&'a str>,
}

impl<'a> GetPaymentMethodAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn payment_method_id(mut self, payment_method_id: &'a str) -> Self {
        self.payment_method_id = Some(payment_method_id);
        self
    }

    pub fn build(self) -> Result<GetPaymentMethodAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetPaymentMethodAuditLogsWithHistoryRequest {
            payment_method_id: self.payment_method_id.ok_or("payment_method_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentMethodByKeyRequest<'a> {
    pub(crate) external_key: &'a str,
    pub(crate) included_deleted: Option<bool>,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentMethodByKeyRequest<'a> {
    pub fn builder() -> GetPaymentMethodByKeyRequestBuilder<'a> {
        GetPaymentMethodByKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentMethodByKeyRequestBuilder<'a> {
    external_key: Option<&'a str>,
    included_deleted: Option<bool>,
    with_plugin_info: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentMethodByKeyRequestBuilder<'a> {
    pub fn external_key(mut self, external_key: &'a str) -> Self {
        self.external_key = Some(external_key);
        self
    }

    pub fn included_deleted(mut self, included_deleted: bool) -> Self {
        self.included_deleted = Some(included_deleted);
        self
    }

    pub fn with_plugin_info(mut self, with_plugin_info: bool) -> Self {
        self.with_plugin_info = Some(with_plugin_info);
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

    pub fn build(self) -> Result<GetPaymentMethodByKeyRequest<'a>, &'static str> {
        Ok(GetPaymentMethodByKeyRequest {
            external_key: self.external_key.ok_or("external_key is required")?,
            included_deleted: self.included_deleted,
            with_plugin_info: self.with_plugin_info,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentMethodCustomFieldsRequest<'a> {
    pub(crate) payment_method_id: &'a str,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentMethodCustomFieldsRequest<'a> {
    pub fn builder() -> GetPaymentMethodCustomFieldsRequestBuilder<'a> {
        GetPaymentMethodCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentMethodCustomFieldsRequestBuilder<'a> {
    payment_method_id: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentMethodCustomFieldsRequestBuilder<'a> {
    pub fn payment_method_id(mut self, payment_method_id: &'a str) -> Self {
        self.payment_method_id = Some(payment_method_id);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetPaymentMethodCustomFieldsRequest<'a>, &'static str> {
        Ok(GetPaymentMethodCustomFieldsRequest {
            payment_method_id: self.payment_method_id.ok_or("payment_method_id is required")?,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentMethodsRequest<'a> {
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) plugin_name: Option<&'a str>,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentMethodsRequest<'a> {
    pub fn builder() -> GetPaymentMethodsRequestBuilder<'a> {
        GetPaymentMethodsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentMethodsRequestBuilder<'a> {
    offset: Option<i64>,
    limit: Option<i64>,
    plugin_name: Option<&'a str>,
    with_plugin_info: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentMethodsRequestBuilder<'a> {
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn plugin_name(mut self, plugin_name: &'a str) -> Self {
        self.plugin_name = Some(plugin_name);
        self
    }

    pub fn with_plugin_info(mut self, with_plugin_info: bool) -> Self {
        self.with_plugin_info = Some(with_plugin_info);
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

    pub fn build(self) -> Result<GetPaymentMethodsRequest<'a>, &'static str> {
        Ok(GetPaymentMethodsRequest {
            offset: self.offset,
            limit: self.limit,
            plugin_name: self.plugin_name,
            with_plugin_info: self.with_plugin_info,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModifyPaymentMethodCustomFieldsRequest<'a> {
    pub(crate) payment_method_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyPaymentMethodCustomFieldsRequest<'a> {
    pub fn builder() -> ModifyPaymentMethodCustomFieldsRequestBuilder<'a> {
        ModifyPaymentMethodCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ModifyPaymentMethodCustomFieldsRequestBuilder<'a> {
    payment_method_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyPaymentMethodCustomFieldsRequestBuilder<'a> {
    pub fn payment_method_id(mut self, payment_method_id: &'a str) -> Self {
        self.payment_method_id = Some(payment_method_id);
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

    pub fn build(self) -> Result<ModifyPaymentMethodCustomFieldsRequest<'a>, &'static str> {
        Ok(ModifyPaymentMethodCustomFieldsRequest {
            payment_method_id: self.payment_method_id.ok_or("payment_method_id is required")?,
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
pub struct SearchPaymentMethodsRequest<'a> {
    pub(crate) search_key: &'a str,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) plugin_name: Option<&'a str>,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> SearchPaymentMethodsRequest<'a> {
    pub fn builder() -> SearchPaymentMethodsRequestBuilder<'a> {
        SearchPaymentMethodsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SearchPaymentMethodsRequestBuilder<'a> {
    search_key: Option<&'a str>,
    offset: Option<i64>,
    limit: Option<i64>,
    plugin_name: Option<&'a str>,
    with_plugin_info: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> SearchPaymentMethodsRequestBuilder<'a> {
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

    pub fn plugin_name(mut self, plugin_name: &'a str) -> Self {
        self.plugin_name = Some(plugin_name);
        self
    }

    pub fn with_plugin_info(mut self, with_plugin_info: bool) -> Self {
        self.with_plugin_info = Some(with_plugin_info);
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

    pub fn build(self) -> Result<SearchPaymentMethodsRequest<'a>, &'static str> {
        Ok(SearchPaymentMethodsRequest {
            search_key: self.search_key.ok_or("search_key is required")?,
            offset: self.offset,
            limit: self.limit,
            plugin_name: self.plugin_name,
            with_plugin_info: self.with_plugin_info,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PaymentMethodApiError {
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

impl PaymentMethodApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{ CustomField, PaymentMethod, AuditLog };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_payment_method_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/paymentMethods/test-payment-method-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentMethodApi::new(config);
        let request = CreatePaymentMethodCustomFieldsRequest::builder()
            .payment_method_id("test-payment-method-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_payment_method_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_payment_method() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/paymentMethods/test-payment-method-id")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentMethodApi::new(config);
        let request = DeletePaymentMethodRequest::builder()
            .payment_method_id("test-payment-method-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_payment_method(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_payment_method_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/paymentMethods/test-payment-method-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentMethodApi::new(config);
        let request = DeletePaymentMethodCustomFieldsRequest::builder()
            .payment_method_id("test-payment-method-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_payment_method_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment_method() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/paymentMethods/test-payment-method-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"payment_method_id": "test-payment-method-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentMethodApi::new(config);
        let request = GetPaymentMethodRequest::builder()
            .payment_method_id("test-payment-method-id")
            .build()
            .unwrap();

        let result = api.get_payment_method(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment_method_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/paymentMethods/test-payment-method-id/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"change_type": "test-change-type"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentMethodApi::new(config);
        let request = GetPaymentMethodAuditLogsWithHistoryRequest::builder()
            .payment_method_id("test-payment-method-id")
            .build()
            .unwrap();

        let result = api.get_payment_method_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment_method_by_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/paymentMethods?externalKey=test-external-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"payment_method_id": "test-payment-method-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentMethodApi::new(config);
        let request = GetPaymentMethodByKeyRequest::builder()
            .external_key("test-external-key")
            .build()
            .unwrap();

        let result = api.get_payment_method_by_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment_method_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/paymentMethods/test-payment-method-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentMethodApi::new(config);
        let request = GetPaymentMethodCustomFieldsRequest::builder()
            .payment_method_id("test-payment-method-id")
            .build()
            .unwrap();

        let result = api.get_payment_method_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment_methods() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/paymentMethods/pagination")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"payment_method_id": "test-payment-method-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentMethodApi::new(config);
        let request = GetPaymentMethodsRequest::builder().build().unwrap();

        let result = api.get_payment_methods(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_modify_payment_method_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/paymentMethods/test-payment-method-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentMethodApi::new(config);
        let request = ModifyPaymentMethodCustomFieldsRequest::builder()
            .payment_method_id("test-payment-method-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.modify_payment_method_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_search_payment_methods() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/paymentMethods/search/test-search-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"payment_method_id": "test-payment-method-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentMethodApi::new(config);
        let request = SearchPaymentMethodsRequest::builder()
            .search_key("test-search-key")
            .build()
            .unwrap();

        let result = api.search_payment_methods(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
