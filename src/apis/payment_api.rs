use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;
use uuid::Uuid;

pub struct PaymentApi {
    config: Configuration,
}

impl PaymentApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn cancel_scheduled_payment_transaction_by_external_key(
        &self,
        request: CancelScheduledPaymentTransactionByExternalKeyRequest<'_>
    ) -> Result<(), PaymentApiError> {
        let url = format!(
            "{}/1.0/kb/payments/cancelScheduledPaymentTransaction",
            self.config.base_path
        );

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("transactionExternalKey", request.transaction_external_key)])
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn cancel_scheduled_payment_transaction_by_id(
        &self,
        request: CancelScheduledPaymentTransactionByIdRequest<'_>
    ) -> Result<(), PaymentApiError> {
        let url = format!(
            "{}/1.0/kb/payments/{}/cancelScheduledPaymentTransaction",
            self.config.base_path,
            request.payment_transaction_id
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

    pub async fn capture_authorization(
        &self,
        request: CaptureAuthorizationRequest<'_>
    ) -> Result<models::Payment, PaymentApiError> {
        let url = format!("{}/1.0/kb/payments/{}", self.config.base_path, request.payment_id);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |props| {
                    props
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
        Self::handle_response(response).await
    }

    pub async fn capture_authorization_by_external_key(
        &self,
        request: CaptureAuthorizationByExternalKeyRequest<'_>
    ) -> Result<models::Payment, PaymentApiError> {
        let url = format!("{}/1.0/kb/payments", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |props| {
                    props
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
        Self::handle_response(response).await
    }

    pub async fn chargeback_payment(
        &self,
        request: ChargebackPaymentRequest<'_>
    ) -> Result<models::Payment, PaymentApiError> {
        let url = format!(
            "{}/1.0/kb/payments/{}/chargebacks",
            self.config.base_path,
            request.payment_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |props| {
                    props
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
        Self::handle_response(response).await
    }

    pub async fn chargeback_payment_by_external_key(
        &self,
        request: ChargebackPaymentByExternalKeyRequest<'_>
    ) -> Result<models::Payment, PaymentApiError> {
        let url = format!("{}/1.0/kb/payments/chargebacks", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |props| {
                    props
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
        Self::handle_response(response).await
    }

    pub async fn chargeback_reversal_payment(
        &self,
        request: ChargebackReversalPaymentRequest<'_>
    ) -> Result<models::Payment, PaymentApiError> {
        let url = format!(
            "{}/1.0/kb/payments/{}/chargebackReversals",
            self.config.base_path,
            request.payment_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |props| {
                    props
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
        Self::handle_response(response).await
    }

    pub async fn chargeback_reversal_payment_by_external_key(
        &self,
        request: ChargebackReversalPaymentByExternalKeyRequest<'_>
    ) -> Result<models::Payment, PaymentApiError> {
        let url = format!("{}/1.0/kb/payments/chargebackReversals", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |props| {
                    props
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
        Self::handle_response(response).await
    }

    pub async fn complete_transaction(
        &self,
        request: CompleteTransactionRequest<'_>
    ) -> Result<(), PaymentApiError> {
        let url = format!("{}/1.0/kb/payments/{}", self.config.base_path, request.payment_id);

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |props| {
                    props
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

    pub async fn complete_transaction_by_external_key(
        &self,
        request: CompleteTransactionByExternalKeyRequest<'_>
    ) -> Result<(), PaymentApiError> {
        let url = format!("{}/1.0/kb/payments", self.config.base_path);

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |props| {
                    props
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

    pub async fn create_combo_payment(
        &self,
        request: CreateComboPaymentRequest<'_>
    ) -> Result<models::Payment, PaymentApiError> {
        let url = format!("{}/1.0/kb/payments/combo", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |props| {
                    props
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

    pub async fn create_payment_custom_fields(
        &self,
        request: CreatePaymentCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, PaymentApiError> {
        let url = format!(
            "{}/1.0/kb/payments/{}/customFields",
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

    pub async fn create_payment_tags(
        &self,
        request: CreatePaymentTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, PaymentApiError> {
        let url = format!("{}/1.0/kb/payments/{}/tags", self.config.base_path, request.payment_id);

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

    pub async fn delete_payment_custom_fields(
        &self,
        request: DeletePaymentCustomFieldsRequest<'_>
    ) -> Result<(), PaymentApiError> {
        let url = format!(
            "{}/1.0/kb/payments/{}/customFields",
            self.config.base_path,
            request.payment_id
        );

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.custom_field.as_ref().map_or(vec![], |props| {
                    props
                        .iter()
                        .map(|p| ("customField", p.to_string()))
                        .collect()
                })
            )
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn delete_payment_tags(
        &self,
        request: DeletePaymentTagsRequest<'_>
    ) -> Result<(), PaymentApiError> {
        let url = format!("{}/1.0/kb/payments/{}/tags", self.config.base_path, request.payment_id);

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.tag_def.as_ref().map_or(vec![], |props| {
                    props
                        .iter()
                        .map(|p| ("tagDef", p.to_string()))
                        .collect()
                })
            )
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn get_payment(
        &self,
        request: GetPaymentRequest<'_>
    ) -> Result<models::Payment, PaymentApiError> {
        let url = format!("{}/1.0/kb/payments/{}", self.config.base_path, request.payment_id);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("withPluginInfo", request.with_plugin_info.map(|b| b.to_string())),
                    ("withAttempts", request.with_attempts.map(|b| b.to_string())),
                    ("audit", request.audit.map(|id| id.to_string())),
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

    pub async fn get_payment_attempt_audit_logs_with_history(
        &self,
        request: GetPaymentAttemptAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, PaymentApiError> {
        let url = format!(
            "{}/1.0/kb/payments/attempts/{}/auditLogsWithHistory",
            self.config.base_path,
            request.payment_attempt_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_payment_audit_logs_with_history(
        &self,
        request: GetPaymentAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, PaymentApiError> {
        let url = format!(
            "{}/1.0/kb/payments/{}/auditLogsWithHistory",
            self.config.base_path,
            request.payment_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_payment_by_external_key(
        &self,
        request: GetPaymentByExternalKeyRequest<'_>
    ) -> Result<models::Payment, PaymentApiError> {
        let url = format!("{}/1.0/kb/payments", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("externalKey", request.external_key),
                    ("withPluginInfo", request.with_plugin_info.map(|b| if b { "true" } else { "false" }).unwrap_or("false")),
                    ("withAttempts", request.with_attempts.map(|b| b.to_string()).as_deref().unwrap_or("")),
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

    pub async fn get_payment_custom_fields(
        &self,
        request: GetPaymentCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, PaymentApiError> {
        let url = format!(
            "{}/1.0/kb/payments/{}/customFields",
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

    pub async fn get_payment_tags(
        &self,
        request: GetPaymentTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, PaymentApiError> {
        let url = format!("{}/1.0/kb/payments/{}/tags", self.config.base_path, request.payment_id);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("includedDeleted", request.included_deleted.map(|b| b.to_string())),
                    ("audit", request.audit.map(|id| id.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_payments(
        &self,
        request: GetPaymentsRequest<'_>
    ) -> Result<Vec<models::Payment>, PaymentApiError> {
        let url = format!("{}/1.0/kb/payments/pagination", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("offset", request.offset.map(|o| o.to_string())),
                    ("limit", request.limit.map(|l| l.to_string())),
                    ("pluginName", request.plugin_name.map(|id| id.to_string())),
                    ("withPluginInfo", request.with_plugin_info.map(|b| b.to_string())),
                    ("withAttempts", request.with_attempts.map(|b| b.to_string())),
                    ("audit", request.audit.map(|id| id.to_string())),
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

    pub async fn modify_payment_custom_fields(
        &self,
        request: ModifyPaymentCustomFieldsRequest<'_>
    ) -> Result<(), PaymentApiError> {
        let url = format!(
            "{}/1.0/kb/payments/{}/customFields",
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

    pub async fn refund_payment(
        &self,
        request: RefundPaymentRequest<'_>
    ) -> Result<models::Payment, PaymentApiError> {
        let url = format!(
            "{}/1.0/kb/payments/{}/refunds",
            self.config.base_path,
            request.payment_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |props| {
                    props
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
        Self::handle_response(response).await
    }

    pub async fn refund_payment_by_external_key(
        &self,
        request: RefundPaymentByExternalKeyRequest<'_>
    ) -> Result<models::Payment, PaymentApiError> {
        let url = format!("{}/1.0/kb/payments/refunds", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |props| {
                    props
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
        Self::handle_response(response).await
    }

    pub async fn search_payments(
        &self,
        request: SearchPaymentsRequest<'_>
    ) -> Result<Vec<models::Payment>, PaymentApiError> {
        let url = format!(
            "{}/1.0/kb/payments/search/{}",
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
                    ("pluginName", request.plugin_name.map(|id| id.to_string())),
                    ("withPluginInfo", request.with_plugin_info.map(|b| b.to_string())),
                    ("withAttempts", request.with_attempts.map(|b| b.to_string())),
                    ("audit", request.audit.map(|id| id.to_string())),
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

    pub async fn void_payment(
        &self,
        request: VoidPaymentRequest<'_>
    ) -> Result<(), PaymentApiError> {
        let url = format!("{}/1.0/kb/payments/{}", self.config.base_path, request.payment_id);

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |props| {
                    props
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

    pub async fn void_payment_by_external_key(
        &self,
        request: VoidPaymentByExternalKeyRequest<'_>
    ) -> Result<(), PaymentApiError> {
        let url = format!("{}/1.0/kb/payments", self.config.base_path);

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.control_plugin_name.as_ref().map_or(vec![], |props| {
                    props
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

    async fn handle_response<T: DeserializeOwned>(
        response: Response
    ) -> Result<T, PaymentApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(PaymentApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(PaymentApiError::from_response(status, text))
            }
        }
    }
    async fn handle_empty_response(response: Response) -> Result<(), PaymentApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(PaymentApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CancelScheduledPaymentTransactionByExternalKeyRequest<'a> {
    pub(crate) transaction_external_key: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CancelScheduledPaymentTransactionByExternalKeyRequest<'a> {
    pub fn builder() -> CancelScheduledPaymentTransactionByExternalKeyRequestBuilder<'a> {
        CancelScheduledPaymentTransactionByExternalKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CancelScheduledPaymentTransactionByExternalKeyRequestBuilder<'a> {
    transaction_external_key: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CancelScheduledPaymentTransactionByExternalKeyRequestBuilder<'a> {
    pub fn transaction_external_key(mut self, transaction_external_key: &'a str) -> Self {
        self.transaction_external_key = Some(transaction_external_key);
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

    pub fn build(
        self
    ) -> Result<CancelScheduledPaymentTransactionByExternalKeyRequest<'a>, &'static str> {
        Ok(CancelScheduledPaymentTransactionByExternalKeyRequest {
            transaction_external_key: self.transaction_external_key.ok_or(
                "transaction_external_key is required"
            )?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CancelScheduledPaymentTransactionByIdRequest<'a> {
    pub(crate) payment_transaction_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CancelScheduledPaymentTransactionByIdRequest<'a> {
    pub fn builder() -> CancelScheduledPaymentTransactionByIdRequestBuilder<'a> {
        CancelScheduledPaymentTransactionByIdRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CancelScheduledPaymentTransactionByIdRequestBuilder<'a> {
    payment_transaction_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CancelScheduledPaymentTransactionByIdRequestBuilder<'a> {
    pub fn payment_transaction_id(mut self, payment_transaction_id: &'a str) -> Self {
        self.payment_transaction_id = Some(payment_transaction_id);
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

    pub fn build(self) -> Result<CancelScheduledPaymentTransactionByIdRequest<'a>, &'static str> {
        Ok(CancelScheduledPaymentTransactionByIdRequest {
            payment_transaction_id: self.payment_transaction_id.ok_or(
                "payment_transaction_id is required"
            )?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CaptureAuthorizationRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CaptureAuthorizationRequest<'a> {
    pub fn builder() -> CaptureAuthorizationRequestBuilder<'a> {
        CaptureAuthorizationRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CaptureAuthorizationRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CaptureAuthorizationRequestBuilder<'a> {
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

    pub fn build(self) -> Result<CaptureAuthorizationRequest<'a>, &'static str> {
        Ok(CaptureAuthorizationRequest {
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
pub struct CaptureAuthorizationByExternalKeyRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CaptureAuthorizationByExternalKeyRequest<'a> {
    pub fn builder() -> CaptureAuthorizationByExternalKeyRequestBuilder<'a> {
        CaptureAuthorizationByExternalKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CaptureAuthorizationByExternalKeyRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CaptureAuthorizationByExternalKeyRequestBuilder<'a> {
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

    pub fn build(self) -> Result<CaptureAuthorizationByExternalKeyRequest<'a>, &'static str> {
        Ok(CaptureAuthorizationByExternalKeyRequest {
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
pub struct ChargebackPaymentRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ChargebackPaymentRequest<'a> {
    pub fn builder() -> ChargebackPaymentRequestBuilder<'a> {
        ChargebackPaymentRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ChargebackPaymentRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ChargebackPaymentRequestBuilder<'a> {
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

    pub fn build(self) -> Result<ChargebackPaymentRequest<'a>, &'static str> {
        Ok(ChargebackPaymentRequest {
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
pub struct ChargebackPaymentByExternalKeyRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ChargebackPaymentByExternalKeyRequest<'a> {
    pub fn builder() -> ChargebackPaymentByExternalKeyRequestBuilder<'a> {
        ChargebackPaymentByExternalKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ChargebackPaymentByExternalKeyRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ChargebackPaymentByExternalKeyRequestBuilder<'a> {
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

    pub fn build(self) -> Result<ChargebackPaymentByExternalKeyRequest<'a>, &'static str> {
        Ok(ChargebackPaymentByExternalKeyRequest {
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
pub struct ChargebackReversalPaymentRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ChargebackReversalPaymentRequest<'a> {
    pub fn builder() -> ChargebackReversalPaymentRequestBuilder<'a> {
        ChargebackReversalPaymentRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ChargebackReversalPaymentRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ChargebackReversalPaymentRequestBuilder<'a> {
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

    pub fn build(self) -> Result<ChargebackReversalPaymentRequest<'a>, &'static str> {
        Ok(ChargebackReversalPaymentRequest {
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
pub struct ChargebackReversalPaymentByExternalKeyRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ChargebackReversalPaymentByExternalKeyRequest<'a> {
    pub fn builder() -> ChargebackReversalPaymentByExternalKeyRequestBuilder<'a> {
        ChargebackReversalPaymentByExternalKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ChargebackReversalPaymentByExternalKeyRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ChargebackReversalPaymentByExternalKeyRequestBuilder<'a> {
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

    pub fn build(self) -> Result<ChargebackReversalPaymentByExternalKeyRequest<'a>, &'static str> {
        Ok(ChargebackReversalPaymentByExternalKeyRequest {
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
pub struct CompleteTransactionRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CompleteTransactionRequest<'a> {
    pub fn builder() -> CompleteTransactionRequestBuilder<'a> {
        CompleteTransactionRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CompleteTransactionRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CompleteTransactionRequestBuilder<'a> {
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

    pub fn build(self) -> Result<CompleteTransactionRequest<'a>, &'static str> {
        Ok(CompleteTransactionRequest {
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
pub struct CompleteTransactionByExternalKeyRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CompleteTransactionByExternalKeyRequest<'a> {
    pub fn builder() -> CompleteTransactionByExternalKeyRequestBuilder<'a> {
        CompleteTransactionByExternalKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CompleteTransactionByExternalKeyRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CompleteTransactionByExternalKeyRequestBuilder<'a> {
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

    pub fn build(self) -> Result<CompleteTransactionByExternalKeyRequest<'a>, &'static str> {
        Ok(CompleteTransactionByExternalKeyRequest {
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
pub struct CreateComboPaymentRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::ComboPaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateComboPaymentRequest<'a> {
    pub fn builder() -> CreateComboPaymentRequestBuilder<'a> {
        CreateComboPaymentRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateComboPaymentRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::ComboPaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateComboPaymentRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::ComboPaymentTransaction) -> Self {
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

    pub fn build(self) -> Result<CreateComboPaymentRequest<'a>, &'static str> {
        Ok(CreateComboPaymentRequest {
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

#[derive(Debug, Clone)]
pub struct CreatePaymentCustomFieldsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreatePaymentCustomFieldsRequest<'a> {
    pub fn builder() -> CreatePaymentCustomFieldsRequestBuilder<'a> {
        CreatePaymentCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreatePaymentCustomFieldsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreatePaymentCustomFieldsRequestBuilder<'a> {
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

    pub fn build(self) -> Result<CreatePaymentCustomFieldsRequest<'a>, &'static str> {
        Ok(CreatePaymentCustomFieldsRequest {
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
pub struct CreatePaymentTagsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<Uuid>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreatePaymentTagsRequest<'a> {
    pub fn builder() -> CreatePaymentTagsRequestBuilder<'a> {
        CreatePaymentTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreatePaymentTagsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreatePaymentTagsRequestBuilder<'a> {
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

    pub fn build(self) -> Result<CreatePaymentTagsRequest<'a>, &'static str> {
        Ok(CreatePaymentTagsRequest {
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
pub struct DeletePaymentCustomFieldsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) custom_field: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeletePaymentCustomFieldsRequest<'a> {
    pub fn builder() -> DeletePaymentCustomFieldsRequestBuilder<'a> {
        DeletePaymentCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeletePaymentCustomFieldsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    custom_field: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeletePaymentCustomFieldsRequestBuilder<'a> {
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

    pub fn build(self) -> Result<DeletePaymentCustomFieldsRequest<'a>, &'static str> {
        Ok(DeletePaymentCustomFieldsRequest {
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
pub struct DeletePaymentTagsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) tag_def: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeletePaymentTagsRequest<'a> {
    pub fn builder() -> DeletePaymentTagsRequestBuilder<'a> {
        DeletePaymentTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeletePaymentTagsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    tag_def: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeletePaymentTagsRequestBuilder<'a> {
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

    pub fn build(self) -> Result<DeletePaymentTagsRequest<'a>, &'static str> {
        Ok(DeletePaymentTagsRequest {
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
pub struct GetPaymentRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) with_attempts: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentRequest<'a> {
    pub fn builder() -> GetPaymentRequestBuilder<'a> {
        GetPaymentRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    with_plugin_info: Option<bool>,
    with_attempts: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentRequestBuilder<'a> {
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

    pub fn build(self) -> Result<GetPaymentRequest<'a>, &'static str> {
        Ok(GetPaymentRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
            with_plugin_info: self.with_plugin_info,
            with_attempts: self.with_attempts,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentAttemptAuditLogsWithHistoryRequest<'a> {
    pub(crate) payment_attempt_id: &'a str,
}

impl<'a> GetPaymentAttemptAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetPaymentAttemptAuditLogsWithHistoryRequestBuilder<'a> {
        GetPaymentAttemptAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentAttemptAuditLogsWithHistoryRequestBuilder<'a> {
    payment_attempt_id: Option<&'a str>,
}

impl<'a> GetPaymentAttemptAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn payment_attempt_id(mut self, payment_attempt_id: &'a str) -> Self {
        self.payment_attempt_id = Some(payment_attempt_id);
        self
    }

    pub fn build(self) -> Result<GetPaymentAttemptAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetPaymentAttemptAuditLogsWithHistoryRequest {
            payment_attempt_id: self.payment_attempt_id.ok_or("payment_attempt_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentAuditLogsWithHistoryRequest<'a> {
    pub(crate) payment_id: &'a str,
}

impl<'a> GetPaymentAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetPaymentAuditLogsWithHistoryRequestBuilder<'a> {
        GetPaymentAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentAuditLogsWithHistoryRequestBuilder<'a> {
    payment_id: Option<&'a str>,
}

impl<'a> GetPaymentAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
        self
    }

    pub fn build(self) -> Result<GetPaymentAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetPaymentAuditLogsWithHistoryRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentByExternalKeyRequest<'a> {
    pub(crate) external_key: &'a str,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) with_attempts: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentByExternalKeyRequest<'a> {
    pub fn builder() -> GetPaymentByExternalKeyRequestBuilder<'a> {
        GetPaymentByExternalKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentByExternalKeyRequestBuilder<'a> {
    external_key: Option<&'a str>,
    with_plugin_info: Option<bool>,
    with_attempts: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentByExternalKeyRequestBuilder<'a> {
    pub fn external_key(mut self, external_key: &'a str) -> Self {
        self.external_key = Some(external_key);
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

    pub fn build(self) -> Result<GetPaymentByExternalKeyRequest<'a>, &'static str> {
        Ok(GetPaymentByExternalKeyRequest {
            external_key: self.external_key.ok_or("external_key is required")?,
            with_plugin_info: self.with_plugin_info,
            with_attempts: self.with_attempts,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentCustomFieldsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentCustomFieldsRequest<'a> {
    pub fn builder() -> GetPaymentCustomFieldsRequestBuilder<'a> {
        GetPaymentCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentCustomFieldsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentCustomFieldsRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetPaymentCustomFieldsRequest<'a>, &'static str> {
        Ok(GetPaymentCustomFieldsRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentTagsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) included_deleted: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentTagsRequest<'a> {
    pub fn builder() -> GetPaymentTagsRequestBuilder<'a> {
        GetPaymentTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentTagsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    included_deleted: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentTagsRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
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

    pub fn build(self) -> Result<GetPaymentTagsRequest<'a>, &'static str> {
        Ok(GetPaymentTagsRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
            included_deleted: self.included_deleted,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentsRequest<'a> {
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) plugin_name: Option<&'a str>,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) with_attempts: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentsRequest<'a> {
    pub fn builder() -> GetPaymentsRequestBuilder<'a> {
        GetPaymentsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentsRequestBuilder<'a> {
    offset: Option<i64>,
    limit: Option<i64>,
    plugin_name: Option<&'a str>,
    with_plugin_info: Option<bool>,
    with_attempts: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentsRequestBuilder<'a> {
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

    pub fn build(self) -> Result<GetPaymentsRequest<'a>, &'static str> {
        Ok(GetPaymentsRequest {
            offset: self.offset,
            limit: self.limit,
            plugin_name: self.plugin_name,
            with_plugin_info: self.with_plugin_info,
            with_attempts: self.with_attempts,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModifyPaymentCustomFieldsRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyPaymentCustomFieldsRequest<'a> {
    pub fn builder() -> ModifyPaymentCustomFieldsRequestBuilder<'a> {
        ModifyPaymentCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ModifyPaymentCustomFieldsRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyPaymentCustomFieldsRequestBuilder<'a> {
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

    pub fn build(self) -> Result<ModifyPaymentCustomFieldsRequest<'a>, &'static str> {
        Ok(ModifyPaymentCustomFieldsRequest {
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
pub struct RefundPaymentRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> RefundPaymentRequest<'a> {
    pub fn builder() -> RefundPaymentRequestBuilder<'a> {
        RefundPaymentRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct RefundPaymentRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> RefundPaymentRequestBuilder<'a> {
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

    pub fn build(self) -> Result<RefundPaymentRequest<'a>, &'static str> {
        Ok(RefundPaymentRequest {
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
pub struct RefundPaymentByExternalKeyRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> RefundPaymentByExternalKeyRequest<'a> {
    pub fn builder() -> RefundPaymentByExternalKeyRequestBuilder<'a> {
        RefundPaymentByExternalKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct RefundPaymentByExternalKeyRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> RefundPaymentByExternalKeyRequestBuilder<'a> {
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

    pub fn build(self) -> Result<RefundPaymentByExternalKeyRequest<'a>, &'static str> {
        Ok(RefundPaymentByExternalKeyRequest {
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
pub struct SearchPaymentsRequest<'a> {
    pub(crate) search_key: &'a str,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) with_attempts: Option<bool>,
    pub(crate) plugin_name: Option<&'a str>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> SearchPaymentsRequest<'a> {
    pub fn builder() -> SearchPaymentsRequestBuilder<'a> {
        SearchPaymentsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SearchPaymentsRequestBuilder<'a> {
    search_key: Option<&'a str>,
    offset: Option<i64>,
    limit: Option<i64>,
    with_plugin_info: Option<bool>,
    with_attempts: Option<bool>,
    plugin_name: Option<&'a str>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> SearchPaymentsRequestBuilder<'a> {
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

    pub fn with_plugin_info(mut self, with_plugin_info: bool) -> Self {
        self.with_plugin_info = Some(with_plugin_info);
        self
    }

    pub fn with_attempts(mut self, with_attempts: bool) -> Self {
        self.with_attempts = Some(with_attempts);
        self
    }

    pub fn plugin_name(mut self, plugin_name: &'a str) -> Self {
        self.plugin_name = Some(plugin_name);
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

    pub fn build(self) -> Result<SearchPaymentsRequest<'a>, &'static str> {
        Ok(SearchPaymentsRequest {
            search_key: self.search_key.ok_or("search_key is required")?,
            offset: self.offset,
            limit: self.limit,
            with_plugin_info: self.with_plugin_info,
            with_attempts: self.with_attempts,
            plugin_name: self.plugin_name,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct VoidPaymentRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> VoidPaymentRequest<'a> {
    pub fn builder() -> VoidPaymentRequestBuilder<'a> {
        VoidPaymentRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct VoidPaymentRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> VoidPaymentRequestBuilder<'a> {
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

    pub fn build(self) -> Result<VoidPaymentRequest<'a>, &'static str> {
        Ok(VoidPaymentRequest {
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
pub struct VoidPaymentByExternalKeyRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> VoidPaymentByExternalKeyRequest<'a> {
    pub fn builder() -> VoidPaymentByExternalKeyRequestBuilder<'a> {
        VoidPaymentByExternalKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct VoidPaymentByExternalKeyRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> VoidPaymentByExternalKeyRequestBuilder<'a> {
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

    pub fn build(self) -> Result<VoidPaymentByExternalKeyRequest<'a>, &'static str> {
        Ok(VoidPaymentByExternalKeyRequest {
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

#[derive(Debug, thiserror::Error)]
pub enum PaymentApiError {
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

impl PaymentApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{ PaymentTransaction, CustomField, Tag, ComboPaymentTransaction };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_cancel_scheduled_payment_transaction_by_external_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/payments/cancelScheduledPaymentTransaction")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = CancelScheduledPaymentTransactionByExternalKeyRequest::builder()
            .transaction_external_key("test-transaction")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.cancel_scheduled_payment_transaction_by_external_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_cancel_scheduled_payment_transaction_by_id() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/payments/test-transaction/cancelScheduledPaymentTransaction")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = CancelScheduledPaymentTransactionByIdRequest::builder()
            .payment_transaction_id("test-transaction")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.cancel_scheduled_payment_transaction_by_id(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_capture_authorization() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/payments/test-payment")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"paymentId": "test-payment"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = CaptureAuthorizationRequest::builder()
            .payment_id("test-payment")
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.capture_authorization(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_capture_authorization_by_external_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/payments")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"paymentId": "test-payment"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = CaptureAuthorizationByExternalKeyRequest::builder()
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.capture_authorization_by_external_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_chargeback_payment() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/payments/test-payment/chargebacks")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"paymentId": "test-payment"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = ChargebackPaymentRequest::builder()
            .payment_id("test-payment")
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.chargeback_payment(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_chargeback_payment_by_external_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/payments/chargebacks")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"paymentId": "test-payment"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = ChargebackPaymentByExternalKeyRequest::builder()
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.chargeback_payment_by_external_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_chargeback_reversal_payment() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/payments/test-payment/chargebackReversals")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"paymentId": "test-payment"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = ChargebackReversalPaymentRequest::builder()
            .payment_id("test-payment")
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.chargeback_reversal_payment(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_chargeback_reversal_payment_by_external_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/payments/chargebackReversals")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"paymentId": "test-payment"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = ChargebackReversalPaymentByExternalKeyRequest::builder()
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.chargeback_reversal_payment_by_external_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_complete_transaction() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/payments/test-payment")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = CompleteTransactionRequest::builder()
            .payment_id("test-payment")
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.complete_transaction(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_complete_transaction_by_external_key() {
        let mut server = Server::new_async().await;
        let mock = server.mock("PUT", "/1.0/kb/payments").with_status(204).create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = CompleteTransactionByExternalKeyRequest::builder()
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.complete_transaction_by_external_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_combo_payment() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/payments/combo")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"paymentId": "test-payment"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = CreateComboPaymentRequest::builder()
            .x_killbill_created_by("test")
            .body(ComboPaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.create_combo_payment(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_payment_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/payments/test-payment/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = CreatePaymentCustomFieldsRequest::builder()
            .payment_id("test-payment")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_payment_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_payment_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/payments/test-payment/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tagId": "test-tag"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = CreatePaymentTagsRequest::builder()
            .payment_id("test-payment")
            .x_killbill_created_by("test")
            .body(vec![Uuid::new_v4()])
            .build()
            .unwrap();

        let result = api.create_payment_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_payment_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/payments/test-payment/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = DeletePaymentCustomFieldsRequest::builder()
            .payment_id("test-payment")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_payment_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_payment_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/payments/test-payment/tags")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = DeletePaymentTagsRequest::builder()
            .payment_id("test-payment")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_payment_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/payments/test-payment")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"paymentId": "test-payment"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = GetPaymentRequest::builder().payment_id("test-payment").build().unwrap();

        let result = api.get_payment(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment_attempt_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/payments/attempts/test-attempt/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"changeType": "INSERT"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = GetPaymentAttemptAuditLogsWithHistoryRequest::builder()
            .payment_attempt_id("test-attempt")
            .build()
            .unwrap();

        let result = api.get_payment_attempt_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/payments/test-payment/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"changeType": "INSERT"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = GetPaymentAuditLogsWithHistoryRequest::builder()
            .payment_id("test-payment")
            .build()
            .unwrap();

        let result = api.get_payment_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment_by_external_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/payments")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"paymentId": "test-payment"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = GetPaymentByExternalKeyRequest::builder()
            .external_key("test-payment")
            .build()
            .unwrap();

        let result = api.get_payment_by_external_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/payments/test-payment/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = GetPaymentCustomFieldsRequest::builder()
            .payment_id("test-payment")
            .build()
            .unwrap();

        let result = api.get_payment_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/payments/test-payment/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tagId": "test-tag"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = GetPaymentTagsRequest::builder().payment_id("test-payment").build().unwrap();

        let result = api.get_payment_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payments() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/payments/pagination")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"paymentId": "test-payment"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = GetPaymentsRequest::builder().build().unwrap();

        let result = api.get_payments(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_modify_payment_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/payments/test-payment/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = ModifyPaymentCustomFieldsRequest::builder()
            .payment_id("test-payment")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.modify_payment_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_refund_payment() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/payments/test-payment/refunds")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"paymentId": "test-payment"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = RefundPaymentRequest::builder()
            .payment_id("test-payment")
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.refund_payment(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_refund_payment_by_external_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/payments/refunds")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"paymentId": "test-payment"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = RefundPaymentByExternalKeyRequest::builder()
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.refund_payment_by_external_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_search_payments() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/payments/search/test-search")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"paymentId": "test-payment"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = SearchPaymentsRequest::builder().search_key("test-search").build().unwrap();

        let result = api.search_payments(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_void_payment() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/payments/test-payment")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = VoidPaymentRequest::builder()
            .payment_id("test-payment")
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.void_payment(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_void_payment_by_external_key() {
        let mut server = Server::new_async().await;
        let mock = server.mock("DELETE", "/1.0/kb/payments").with_status(204).create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = PaymentApi::new(config);
        let request = VoidPaymentByExternalKeyRequest::builder()
            .x_killbill_created_by("test")
            .body(PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.void_payment_by_external_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
