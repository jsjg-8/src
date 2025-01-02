use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use uuid::Uuid;
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct InvoicePaymentApi {
    config: Configuration,
}

impl InvoicePaymentApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn complete_invoice_payment_transaction(
        &self,
        request: CompleteInvoicePaymentTransactionRequest<'_>
    ) -> Result<(), InvoicePaymentApiError> {
        let url = format!(
            "{}/1.0/kb/invoicePayments/{}",
            self.config.base_path,
            request.payment_id
        );

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |names| {
                    names
                        .iter()
                        .map(|p| ("controlPluginName", p.to_string()))
                        .collect()
                })
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
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn create_chargeback(
        &self,
        request: CreateChargebackRequest<'_>
    ) -> Result<models::InvoicePayment, InvoicePaymentApiError> {
        let url = format!(
            "{}/1.0/kb/invoicePayments/{}/chargebacks",
            self.config.base_path,
            request.payment_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
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

    pub async fn create_chargeback_reversal(
        &self,
        request: CreateChargebackReversalRequest<'_>
    ) -> Result<models::InvoicePayment, InvoicePaymentApiError> {
        let url = format!(
            "{}/1.0/kb/invoicePayments/{}/chargebackReversals",
            self.config.base_path,
            request.payment_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
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

    pub async fn create_invoice_payment_custom_fields(
        &self,
        request: CreateInvoicePaymentCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, InvoicePaymentApiError> {
        let url = format!(
            "{}/1.0/kb/invoicePayments/{}/customFields",
            self.config.base_path,
            request.payment_id
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

    pub async fn create_invoice_payment_tags(
        &self,
        request: CreateInvoicePaymentTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, InvoicePaymentApiError> {
        let url = format!(
            "{}/1.0/kb/invoicePayments/{}/tags",
            self.config.base_path,
            request.payment_id
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

    pub async fn create_refund_with_adjustments(
        &self,
        request: CreateRefundWithAdjustmentsRequest<'_>
    ) -> Result<models::InvoicePayment, InvoicePaymentApiError> {
        let url = format!(
            "{}/1.0/kb/invoicePayments/{}/refunds",
            self.config.base_path,
            request.payment_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("externalPayment", request.external_payment.map(|b| b.to_string())),
                    ("paymentMethodId", request.payment_method_id.map(|s| s.to_string())),
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
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn delete_invoice_payment_custom_fields(
        &self,
        request: DeleteInvoicePaymentCustomFieldsRequest<'_>
    ) -> Result<(), InvoicePaymentApiError> {
        let url = format!(
            "{}/1.0/kb/invoicePayments/{}/customFields",
            self.config.base_path,
            request.payment_id
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

    pub async fn delete_invoice_payment_tags(
        &self,
        request: DeleteInvoicePaymentTagsRequest<'_>
    ) -> Result<(), InvoicePaymentApiError> {
        let url = format!(
            "{}/1.0/kb/invoicePayments/{}/tags",
            self.config.base_path,
            request.payment_id
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

    pub async fn get_invoice_payment(
        &self,
        request: GetInvoicePaymentRequest<'_>
    ) -> Result<models::InvoicePayment, InvoicePaymentApiError> {
        let url = format!(
            "{}/1.0/kb/invoicePayments/{}",
            self.config.base_path,
            request.payment_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("withPluginInfo", request.with_plugin_info.map(|b| b.to_string())),
                    ("withAttempts", request.with_attempts.map(|b| b.to_string())),
                    ("audit", request.audit.map(|b| b.to_string())),
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

    pub async fn get_invoice_payment_audit_logs_with_history(
        &self,
        request: GetInvoicePaymentAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, InvoicePaymentApiError> {
        let url = format!(
            "{}/1.0/kb/invoicePayments/{}/auditLogsWithHistory",
            self.config.base_path,
            request.invoice_payment_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice_payment_custom_fields(
        &self,
        request: GetInvoicePaymentCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, InvoicePaymentApiError> {
        let url = format!(
            "{}/1.0/kb/invoicePayments/{}/customFields",
            self.config.base_path,
            request.payment_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("audit", request.audit)]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice_payment_tags(
        &self,
        request: GetInvoicePaymentTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, InvoicePaymentApiError> {
        let url = format!(
            "{}/1.0/kb/invoicePayments/{}/tags",
            self.config.base_path,
            request.payment_id
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

    pub async fn modify_invoice_payment_custom_fields(
        &self,
        request: ModifyInvoicePaymentCustomFieldsRequest<'_>
    ) -> Result<(), InvoicePaymentApiError> {
        let url = format!(
            "{}/1.0/kb/invoicePayments/{}/customFields",
            self.config.base_path,
            request.payment_id
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
    ) -> Result<T, InvoicePaymentApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(InvoicePaymentApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(InvoicePaymentApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), InvoicePaymentApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(InvoicePaymentApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompleteInvoicePaymentTransactionRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CompleteInvoicePaymentTransactionRequest<'a> {
    pub fn builder() -> CompleteInvoicePaymentTransactionRequestBuilder<'a> {
        CompleteInvoicePaymentTransactionRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CompleteInvoicePaymentTransactionRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CompleteInvoicePaymentTransactionRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
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

    pub fn build(self) -> Result<CompleteInvoicePaymentTransactionRequest<'a>, &'static str> {
        Ok(CompleteInvoicePaymentTransactionRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            control_plugin_name: self.control_plugin_name,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateChargebackRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::InvoicePaymentTransaction,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateChargebackRequest<'a> {
    pub fn builder() -> CreateChargebackRequestBuilder<'a> {
        CreateChargebackRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateChargebackRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::InvoicePaymentTransaction>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateChargebackRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::InvoicePaymentTransaction) -> Self {
        self.body = Some(body);
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

    pub fn build(self) -> Result<CreateChargebackRequest<'a>, &'static str> {
        Ok(CreateChargebackRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateChargebackReversalRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::InvoicePaymentTransaction,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateChargebackReversalRequest<'a> {
    pub fn builder() -> CreateChargebackReversalRequestBuilder<'a> {
        CreateChargebackReversalRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateChargebackReversalRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::InvoicePaymentTransaction>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateChargebackReversalRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::InvoicePaymentTransaction) -> Self {
        self.body = Some(body);
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

    pub fn build(self) -> Result<CreateChargebackReversalRequest<'a>, &'static str> {
        Ok(CreateChargebackReversalRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateInvoicePaymentCustomFieldsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInvoicePaymentCustomFieldsRequest<'a> {
    pub fn builder() -> CreateInvoicePaymentCustomFieldsRequestBuilder<'a> {
        CreateInvoicePaymentCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateInvoicePaymentCustomFieldsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInvoicePaymentCustomFieldsRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
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

    pub fn build(self) -> Result<CreateInvoicePaymentCustomFieldsRequest<'a>, &'static str> {
        Ok(CreateInvoicePaymentCustomFieldsRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
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
pub struct CreateInvoicePaymentTagsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<Uuid>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInvoicePaymentTagsRequest<'a> {
    pub fn builder() -> CreateInvoicePaymentTagsRequestBuilder<'a> {
        CreateInvoicePaymentTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateInvoicePaymentTagsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInvoicePaymentTagsRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
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

    pub fn build(self) -> Result<CreateInvoicePaymentTagsRequest<'a>, &'static str> {
        Ok(CreateInvoicePaymentTagsRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
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
pub struct CreateRefundWithAdjustmentsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::InvoicePaymentTransaction,
    pub(crate) external_payment: Option<bool>,
    pub(crate) payment_method_id: Option<&'a str>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateRefundWithAdjustmentsRequest<'a> {
    pub fn builder() -> CreateRefundWithAdjustmentsRequestBuilder<'a> {
        CreateRefundWithAdjustmentsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateRefundWithAdjustmentsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::InvoicePaymentTransaction>,
    external_payment: Option<bool>,
    payment_method_id: Option<&'a str>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateRefundWithAdjustmentsRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::InvoicePaymentTransaction) -> Self {
        self.body = Some(body);
        self
    }

    pub fn external_payment(mut self, external_payment: bool) -> Self {
        self.external_payment = Some(external_payment);
        self
    }

    pub fn payment_method_id(mut self, payment_method_id: &'a str) -> Self {
        self.payment_method_id = Some(payment_method_id);
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

    pub fn build(self) -> Result<CreateRefundWithAdjustmentsRequest<'a>, &'static str> {
        Ok(CreateRefundWithAdjustmentsRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            external_payment: self.external_payment,
            payment_method_id: self.payment_method_id,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteInvoicePaymentCustomFieldsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) custom_field: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteInvoicePaymentCustomFieldsRequest<'a> {
    pub fn builder() -> DeleteInvoicePaymentCustomFieldsRequestBuilder<'a> {
        DeleteInvoicePaymentCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteInvoicePaymentCustomFieldsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    custom_field: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteInvoicePaymentCustomFieldsRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
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

    pub fn build(self) -> Result<DeleteInvoicePaymentCustomFieldsRequest<'a>, &'static str> {
        Ok(DeleteInvoicePaymentCustomFieldsRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
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
pub struct DeleteInvoicePaymentTagsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) tag_def: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteInvoicePaymentTagsRequest<'a> {
    pub fn builder() -> DeleteInvoicePaymentTagsRequestBuilder<'a> {
        DeleteInvoicePaymentTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteInvoicePaymentTagsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    tag_def: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteInvoicePaymentTagsRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
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

    pub fn build(self) -> Result<DeleteInvoicePaymentTagsRequest<'a>, &'static str> {
        Ok(DeleteInvoicePaymentTagsRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
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
pub struct GetInvoicePaymentRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) with_attempts: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoicePaymentRequest<'a> {
    pub fn builder() -> GetInvoicePaymentRequestBuilder<'a> {
        GetInvoicePaymentRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoicePaymentRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    with_plugin_info: Option<bool>,
    with_attempts: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoicePaymentRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
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

    pub fn build(self) -> Result<GetInvoicePaymentRequest<'a>, &'static str> {
        Ok(GetInvoicePaymentRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
            with_plugin_info: self.with_plugin_info,
            with_attempts: self.with_attempts,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoicePaymentAuditLogsWithHistoryRequest<'a> {
    pub(crate) invoice_payment_id: &'a str,
}

impl<'a> GetInvoicePaymentAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetInvoicePaymentAuditLogsWithHistoryRequestBuilder<'a> {
        GetInvoicePaymentAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoicePaymentAuditLogsWithHistoryRequestBuilder<'a> {
    invoice_payment_id: Option<&'a str>,
}

impl<'a> GetInvoicePaymentAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn invoice_payment_id(mut self, invoice_payment_id: &'a str) -> Self {
        self.invoice_payment_id = Some(invoice_payment_id);
        self
    }

    pub fn build(self) -> Result<GetInvoicePaymentAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetInvoicePaymentAuditLogsWithHistoryRequest {
            invoice_payment_id: self.invoice_payment_id.ok_or("invoice_payment_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoicePaymentCustomFieldsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoicePaymentCustomFieldsRequest<'a> {
    pub fn builder() -> GetInvoicePaymentCustomFieldsRequestBuilder<'a> {
        GetInvoicePaymentCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoicePaymentCustomFieldsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoicePaymentCustomFieldsRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetInvoicePaymentCustomFieldsRequest<'a>, &'static str> {
        Ok(GetInvoicePaymentCustomFieldsRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoicePaymentTagsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) included_deleted: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoicePaymentTagsRequest<'a> {
    pub fn builder() -> GetInvoicePaymentTagsRequestBuilder<'a> {
        GetInvoicePaymentTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoicePaymentTagsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    included_deleted: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoicePaymentTagsRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
        self
    }

    pub fn included_deleted(mut self, included_deleted: bool) -> Self {
        self.included_deleted = Some(included_deleted);
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

    pub fn build(self) -> Result<GetInvoicePaymentTagsRequest<'a>, &'static str> {
        Ok(GetInvoicePaymentTagsRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
            included_deleted: self.included_deleted,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModifyInvoicePaymentCustomFieldsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyInvoicePaymentCustomFieldsRequest<'a> {
    pub fn builder() -> ModifyInvoicePaymentCustomFieldsRequestBuilder<'a> {
        ModifyInvoicePaymentCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ModifyInvoicePaymentCustomFieldsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyInvoicePaymentCustomFieldsRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
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

    pub fn build(self) -> Result<ModifyInvoicePaymentCustomFieldsRequest<'a>, &'static str> {
        Ok(ModifyInvoicePaymentCustomFieldsRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
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
pub enum InvoicePaymentApiError {
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

impl InvoicePaymentApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use models::InvoicePaymentTransaction;
    use crate::models::{ PaymentTransaction, InvoicePayment, CustomField, Tag, AuditLog };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_complete_invoice_payment_transaction() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/invoicePayments/test-payment-id")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoicePaymentApi::new(config);
        let request = CompleteInvoicePaymentTransactionRequest::builder()
            .payment_id("test-payment-id")
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.complete_invoice_payment_transaction(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_chargeback() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoicePayments/test-payment-id/chargebacks")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"payment_id": "test-payment-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoicePaymentApi::new(config);
        let request = CreateChargebackRequest::builder()
            .payment_id("test-payment-id")
            .x_killbill_created_by("test")
            .body(InvoicePaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.create_chargeback(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_chargeback_reversal() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoicePayments/test-payment-id/chargebackReversals")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"payment_id": "test-payment-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoicePaymentApi::new(config);
        let request = CreateChargebackReversalRequest::builder()
            .payment_id("test-payment-id")
            .x_killbill_created_by("test")
            .body(InvoicePaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.create_chargeback_reversal(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_invoice_payment_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoicePayments/test-payment-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoicePaymentApi::new(config);
        let request = CreateInvoicePaymentCustomFieldsRequest::builder()
            .payment_id("test-payment-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_invoice_payment_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_invoice_payment_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoicePayments/test-payment-id/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tag_definition_id": "test-tag-def-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoicePaymentApi::new(config);
        let request = CreateInvoicePaymentTagsRequest::builder()
            .payment_id("test-payment-id")
            .x_killbill_created_by("test")
            .body(vec![Uuid::new_v4()])
            .build()
            .unwrap();

        let result = api.create_invoice_payment_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_refund_with_adjustments() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoicePayments/test-payment-id/refunds")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"payment_id": "test-payment-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoicePaymentApi::new(config);
        let request = CreateRefundWithAdjustmentsRequest::builder()
            .payment_id("test-payment-id")
            .x_killbill_created_by("test")
            .body(InvoicePaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.create_refund_with_adjustments(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_invoice_payment_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/invoicePayments/test-payment-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoicePaymentApi::new(config);
        let request = DeleteInvoicePaymentCustomFieldsRequest::builder()
            .payment_id("test-payment-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_invoice_payment_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_invoice_payment_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/invoicePayments/test-payment-id/tags")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoicePaymentApi::new(config);
        let request = DeleteInvoicePaymentTagsRequest::builder()
            .payment_id("test-payment-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_invoice_payment_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_payment() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoicePayments/test-payment-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"payment_id": "test-payment-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoicePaymentApi::new(config);
        let request = GetInvoicePaymentRequest::builder()
            .payment_id("test-payment-id")
            .build()
            .unwrap();

        let result = api.get_invoice_payment(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_payment_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoicePayments/test-payment-id/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"change_type": "test-change-type"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoicePaymentApi::new(config);
        let request = GetInvoicePaymentAuditLogsWithHistoryRequest::builder()
            .invoice_payment_id("test-payment-id")
            .build()
            .unwrap();

        let result = api.get_invoice_payment_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_payment_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoicePayments/test-payment-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoicePaymentApi::new(config);
        let request = GetInvoicePaymentCustomFieldsRequest::builder()
            .payment_id("test-payment-id")
            .build()
            .unwrap();

        let result = api.get_invoice_payment_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_payment_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoicePayments/test-payment-id/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tag_definition_id": "test-tag-def-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoicePaymentApi::new(config);
        let request = GetInvoicePaymentTagsRequest::builder()
            .payment_id("test-payment-id")
            .build()
            .unwrap();

        let result = api.get_invoice_payment_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_modify_invoice_payment_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/invoicePayments/test-payment-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoicePaymentApi::new(config);
        let request = ModifyInvoicePaymentCustomFieldsRequest::builder()
            .payment_id("test-payment-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.modify_invoice_payment_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
