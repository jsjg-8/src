use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use uuid::Uuid;
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct AccountApi {
    config: Configuration,
}

impl AccountApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn add_account_blocking_state(
        &self,
        request: AddAccountBlockingStateRequest<'_>
    ) -> Result<Vec<models::BlockingState>, AccountApiError> {
        let url = format!("{}/1.0/kb/accounts/{}/block", self.config.base_path, request.account_id);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("requestedDate", request.requested_date)])
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

    pub async fn add_email(
        &self,
        request: AddEmailRequest<'_>
    ) -> Result<Vec<models::AccountEmail>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/emails",
            self.config.base_path,
            request.account_id
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

    pub async fn close_account(
        &self,
        request: CloseAccountRequest<'_>
    ) -> Result<(), AccountApiError> {
        let url = format!("{}/1.0/kb/accounts/{}", self.config.base_path, request.account_id);

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    (
                        "cancelAllSubscriptions",
                        request.cancel_all_subscriptions.map(|b| b.to_string()),
                    ),
                    (
                        "writeOffUnpaidInvoices",
                        request.write_off_unpaid_invoices.map(|b| b.to_string()),
                    ),
                    (
                        "itemAdjustUnpaidInvoices",
                        request.item_adjust_unpaid_invoices.map(|b| b.to_string()),
                    ),
                    (
                        "removeFutureNotifications",
                        request.remove_future_notifications.map(|b| b.to_string()),
                    ),
                ]
            )
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn create_account(
        &self,
        request: CreateAccountRequest<'_>
    ) -> Result<models::Account, AccountApiError> {
        let url = format!("{}/1.0/kb/accounts", self.config.base_path);

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

    pub async fn create_account_custom_fields(
        &self,
        request: CreateAccountCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/customFields",
            self.config.base_path,
            request.account_id
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

    pub async fn create_account_tags(
        &self,
        request: CreateAccountTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, AccountApiError> {
        let url = format!("{}/1.0/kb/accounts/{}/tags", self.config.base_path, request.account_id);

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

    pub async fn create_payment_method(
        &self,
        request: CreatePaymentMethodRequest<'_>
    ) -> Result<models::PaymentMethod, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/paymentMethods",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("isDefault", request.is_default.map(|b| b.to_string())),
                    (
                        "payAllUnpaidInvoices",
                        request.pay_all_unpaid_invoices.map(|b| b.to_string()),
                    ),
                ]
            )
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
        Self::handle_response(response).await
    }

    pub async fn delete_account_custom_fields(
        &self,
        request: DeleteAccountCustomFieldsRequest<'_>
    ) -> Result<(), AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/customFields",
            self.config.base_path,
            request.account_id
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

    pub async fn delete_account_tags(
        &self,
        request: DeleteAccountTagsRequest<'_>
    ) -> Result<(), AccountApiError> {
        let url = format!("{}/1.0/kb/accounts/{}/tags", self.config.base_path, request.account_id);

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

    pub async fn get_account(
        &self,
        request: GetAccountRequest<'_>
    ) -> Result<models::Account, AccountApiError> {
        let url = format!("{}/1.0/kb/accounts/{}", self.config.base_path, request.account_id);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("accountWithBalance", request.account_with_balance.map(|b| b.to_string())),
                    (
                        "accountWithBalanceAndCBA",
                        request.account_with_balance_and_cba.map(|b| b.to_string()),
                    ),
                    ("audit", request.audit.map(|a| a.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_account_audit_logs(
        &self,
        request: GetAccountAuditLogsRequest<'_>
    ) -> Result<Vec<models::AuditLog>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/auditLogs",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_account_audit_logs_with_history(
        &self,
        request: GetAccountAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/auditLogsWithHistory",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_account_bundles(
        &self,
        request: GetAccountBundlesRequest<'_>
    ) -> Result<Vec<models::Bundle>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/bundles",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("externalKey", request.external_key),
                    ("bundlesFilter", request.bundles_filter),
                    ("audit", request.audit),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_account_bundles_paginated(
        &self,
        request: GetAccountBundlesPaginatedRequest<'_>
    ) -> Result<Vec<models::Bundle>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/bundles/pagination",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("offset", request.offset.map(|o| o.to_string())),
                    ("limit", request.limit.map(|l| l.to_string())),
                    ("audit", request.audit.map(|a| a.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_account_by_key(
        &self,
        request: GetAccountByKeyRequest<'_>
    ) -> Result<models::Account, AccountApiError> {
        let url = format!("{}/1.0/kb/accounts", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("externalKey", request.external_key),
                    ("accountWithBalance", request.account_with_balance.map(|b| if b { "true" } else { "false" }).unwrap_or("false")),
                    (
                        "accountWithBalanceAndCBA",
                        request.account_with_balance_and_cba.map(|b| if b { "true" } else { "false" }).unwrap_or("false"),
                    ),
                    ("audit", request.audit.unwrap_or("")),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_account_custom_fields(
        &self,
        request: GetAccountCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/customFields",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("audit", request.audit)]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_account_email_audit_logs_with_history(
        &self,
        request: GetAccountEmailAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/emails/{}/auditLogsWithHistory",
            self.config.base_path,
            request.account_id,
            request.account_email_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_account_tags(
        &self,
        request: GetAccountTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, AccountApiError> {
        let url = format!("{}/1.0/kb/accounts/{}/tags", self.config.base_path, request.account_id);

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

    pub async fn get_account_timeline(
        &self,
        request: GetAccountTimelineRequest<'_>
    ) -> Result<models::AccountTimeline, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/timeline",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("parallel", request.parallel.map(|b| b.to_string())),
                    ("audit", request.audit.map(|b| b.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_accounts(
        &self,
        request: GetAccountsRequest<'_>
    ) -> Result<Vec<models::Account>, AccountApiError> {
        let url = format!("{}/1.0/kb/accounts/pagination", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("offset", request.offset.map(|o| o.to_string())),
                    ("limit", request.limit.map(|l| l.to_string())),
                    ("accountWithBalance", request.account_with_balance.map(|b| b.to_string())),
                    (
                        "accountWithBalanceAndCBA",
                        request.account_with_balance_and_cba.map(|b| b.to_string()),
                    ),
                    ("audit", request.audit.map(|l| l.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_all_custom_fields(
        &self,
        request: GetAllCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/allCustomFields",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("objectType", request.object_type),
                    ("audit", request.audit),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_all_tags(
        &self,
        request: GetAllTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/allTags",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("objectType", request.object_type),
                    (
                        "includedDeleted",
                        request.included_deleted
                            .map(|b| if b { "true" } else { "false" })
                            ,
                    ),
                    ("audit", request.audit),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_blocking_state_audit_logs_with_history(
        &self,
        request: GetBlockingStateAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/block/{}/auditLogsWithHistory",
            self.config.base_path,
            request.blocking_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_blocking_states(
        &self,
        request: GetBlockingStatesRequest<'_>
    ) -> Result<Vec<models::BlockingState>, AccountApiError> {
        let url = format!("{}/1.0/kb/accounts/{}/block", self.config.base_path, request.account_id);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &request.blocking_state_types.as_ref().map_or(vec![], |types| {
                    types
                        .iter()
                        .map(|p| ("blockingStateTypes", p.to_string()))
                        .collect()
                })
            )
            .query(
                &request.blocking_state_svcs.as_ref().map_or(vec![], |svcs| {
                    svcs.iter()
                        .map(|p| ("blockingStateSvcs", p.to_string()))
                        .collect()
                })
            )
            .query(&[("audit", request.audit)]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_children_accounts(
        &self,
        request: GetChildrenAccountsRequest<'_>
    ) -> Result<Vec<models::Account>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/children",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("accountWithBalance", request.account_with_balance.map(|b| b.to_string())),
                    (
                        "accountWithBalanceAndCBA",
                        request.account_with_balance_and_cba.map(|b| b.to_string()),
                    ),
                    ("audit", request.audit.map(|b| b.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_emails(
        &self,
        request: GetEmailsRequest<'_>
    ) -> Result<Vec<models::AccountEmail>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/emails",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice_payments(
        &self,
        request: GetInvoicePaymentsRequest<'_>
    ) -> Result<Vec<models::InvoicePayment>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/invoicePayments",
            self.config.base_path,
            request.account_id
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
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoices_for_account(
        &self,
        request: GetInvoicesForAccountRequest<'_>
    ) -> Result<Vec<models::Invoice>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/invoices",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("startDate", request.start_date),
                    ("endDate", request.end_date),
                    (
                        "withMigrationInvoices",
                        request.with_migration_invoices.map(|b| b.to_string()),
                    ),
                    ("unpaidInvoicesOnly", request.unpaid_invoices_only.map(|b| b.to_string())),
                    (
                        "includeVoidedInvoices",
                        request.include_voided_invoices.map(|b| b.to_string()),
                    ),
                    (
                        "includeInvoiceComponents",
                        request.include_invoice_components.map(|b| b.to_string()),
                    ),
                    ("invoicesFilter", request.invoices_filter.map(|b| b.to_string())),
                    ("audit", request.audit.map(|b| b.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoices_for_account_paginated(
        &self,
        request: GetInvoicesForAccountPaginatedRequest<'_>
    ) -> Result<Vec<models::Invoice>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/invoices/pagination",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("offset", request.offset.map(|o| o.to_string())),
                    ("limit", request.limit.map(|l| l.to_string())),
                    ("audit", request.audit.map(|b| b.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_overdue_account(
        &self,
        request: GetOverdueAccountRequest<'_>
    ) -> Result<models::OverdueState, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/overdue",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_payment_methods_for_account(
        &self,
        request: GetPaymentMethodsForAccountRequest<'_>
    ) -> Result<Vec<models::PaymentMethod>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/paymentMethods",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("withPluginInfo", request.with_plugin_info.map(|b| b.to_string())),
                    ("includedDeleted", request.included_deleted.map(|b| b.to_string())),
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

    pub async fn get_payments_for_account(
        &self,
        request: GetPaymentsForAccountRequest<'_>
    ) -> Result<Vec<models::Payment>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/payments",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("withAttempts", request.with_attempts.map(|b| b.to_string())),
                    ("withPluginInfo", request.with_plugin_info.map(|b| b.to_string())),
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

    pub async fn modify_account_custom_fields(
        &self,
        request: ModifyAccountCustomFieldsRequest<'_>
    ) -> Result<(), AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/customFields",
            self.config.base_path,
            request.account_id
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

    pub async fn pay_all_invoices(
        &self,
        request: PayAllInvoicesRequest<'_>
    ) -> Result<Vec<models::Invoice>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/invoicePayments",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("paymentMethodId", request.payment_method_id),
                    ("externalPayment", request.external_payment.map(|b| b.to_string()).as_deref()),
                    ("paymentAmount", request.payment_amount.map(|a| a.to_string()).as_deref()),
                    ("targetDate", request.target_date.as_deref()),
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
        Self::handle_response(response).await
    }

    pub async fn process_payment(
        &self,
        request: ProcessPaymentRequest<'_>
    ) -> Result<models::Payment, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/payments",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("paymentMethodId", request.payment_method_id)])
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
        Self::handle_response(response).await
    }

    pub async fn process_payment_by_external_key(
        &self,
        request: ProcessPaymentByExternalKeyRequest<'_>
    ) -> Result<models::Payment, AccountApiError> {
        let url = format!("{}/1.0/kb/accounts/payments", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("externalKey", request.external_key),
                    ("paymentMethodId", request.payment_method_id.unwrap()),
                ]
            )
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
        Self::handle_response(response).await
    }

    pub async fn rebalance_existing_cbaon_account(
        &self,
        request: RebalanceExistingCbaonAccountRequest<'_>
    ) -> Result<(), AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/cbaRebalancing",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn refresh_payment_methods(
        &self,
        request: RefreshPaymentMethodsRequest<'_>
    ) -> Result<(), AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/paymentMethods/refresh",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("pluginName", request.plugin_name)])
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

    pub async fn remove_email(
        &self,
        request: RemoveEmailRequest<'_>
    ) -> Result<(), AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/emails/{}",
            self.config.base_path,
            request.account_id,
            request.email
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

    pub async fn search_accounts(
        &self,
        request: SearchAccountsRequest<'_>
    ) -> Result<Vec<models::Account>, AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/search/{}",
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
                    ("accountWithBalance", request.account_with_balance.map(|b| b.to_string())),
                    (
                        "accountWithBalanceAndCBA",
                        request.account_with_balance_and_cba.map(|b| b.to_string()),
                    ),
                    ("audit", request.audit.map(|b| b.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn set_default_payment_method(
        &self,
        request: SetDefaultPaymentMethodRequest<'_>
    ) -> Result<(), AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/paymentMethods/{}/setDefault",
            self.config.base_path,
            request.account_id,
            request.payment_method_id
        );

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[("payAllUnpaidInvoices", request.pay_all_unpaid_invoices.map(|b| b.to_string()))]
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

    pub async fn transfer_child_credit_to_parent(
        &self,
        request: TransferChildCreditToParentRequest<'_>
    ) -> Result<(), AccountApiError> {
        let url = format!(
            "{}/1.0/kb/accounts/{}/transferCredit",
            self.config.base_path,
            request.child_account_id
        );

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn update_account(
        &self,
        request: UpdateAccountRequest<'_>
    ) -> Result<(), AccountApiError> {
        let url = format!("{}/1.0/kb/accounts/{}", self.config.base_path, request.account_id);

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("treatNullAsReset", request.treat_null_as_reset.map(|b| b.to_string()))])
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(
        response: Response
    ) -> Result<T, AccountApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(AccountApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(AccountApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), AccountApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(AccountApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddAccountBlockingStateRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::BlockingState,
    pub(crate) requested_date: Option<String>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> AddAccountBlockingStateRequest<'a> {
    pub fn builder() -> AddAccountBlockingStateRequestBuilder<'a> {
        AddAccountBlockingStateRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct AddAccountBlockingStateRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::BlockingState>,
    requested_date: Option<String>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> AddAccountBlockingStateRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::BlockingState) -> Self {
        self.body = Some(body);
        self
    }

    pub fn requested_date(mut self, requested_date: impl Into<String>) -> Self {
        self.requested_date = Some(requested_date.into());
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

    pub fn build(self) -> Result<AddAccountBlockingStateRequest<'a>, &'static str> {
        Ok(AddAccountBlockingStateRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            requested_date: self.requested_date,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AddEmailRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::AccountEmail,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> AddEmailRequest<'a> {
    pub fn builder() -> AddEmailRequestBuilder<'a> {
        AddEmailRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct AddEmailRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::AccountEmail>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> AddEmailRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::AccountEmail) -> Self {
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

    pub fn build(self) -> Result<AddEmailRequest<'a>, &'static str> {
        Ok(AddEmailRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
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
pub struct CloseAccountRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) cancel_all_subscriptions: Option<bool>,
    pub(crate) write_off_unpaid_invoices: Option<bool>,
    pub(crate) item_adjust_unpaid_invoices: Option<bool>,
    pub(crate) remove_future_notifications: Option<bool>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CloseAccountRequest<'a> {
    pub fn builder() -> CloseAccountRequestBuilder<'a> {
        CloseAccountRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CloseAccountRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    cancel_all_subscriptions: Option<bool>,
    write_off_unpaid_invoices: Option<bool>,
    item_adjust_unpaid_invoices: Option<bool>,
    remove_future_notifications: Option<bool>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CloseAccountRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn cancel_all_subscriptions(mut self, cancel_all_subscriptions: bool) -> Self {
        self.cancel_all_subscriptions = Some(cancel_all_subscriptions);
        self
    }

    pub fn write_off_unpaid_invoices(mut self, write_off_unpaid_invoices: bool) -> Self {
        self.write_off_unpaid_invoices = Some(write_off_unpaid_invoices);
        self
    }

    pub fn item_adjust_unpaid_invoices(mut self, item_adjust_unpaid_invoices: bool) -> Self {
        self.item_adjust_unpaid_invoices = Some(item_adjust_unpaid_invoices);
        self
    }

    pub fn remove_future_notifications(mut self, remove_future_notifications: bool) -> Self {
        self.remove_future_notifications = Some(remove_future_notifications);
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

    pub fn build(self) -> Result<CloseAccountRequest<'a>, &'static str> {
        Ok(CloseAccountRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            cancel_all_subscriptions: self.cancel_all_subscriptions,
            write_off_unpaid_invoices: self.write_off_unpaid_invoices,
            item_adjust_unpaid_invoices: self.item_adjust_unpaid_invoices,
            remove_future_notifications: self.remove_future_notifications,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateAccountRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::Account,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateAccountRequest<'a> {
    pub fn builder() -> CreateAccountRequestBuilder<'a> {
        CreateAccountRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateAccountRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::Account>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateAccountRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::Account) -> Self {
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

    pub fn build(self) -> Result<CreateAccountRequest<'a>, &'static str> {
        Ok(CreateAccountRequest {
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
pub struct CreateAccountCustomFieldsRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateAccountCustomFieldsRequest<'a> {
    pub fn builder() -> CreateAccountCustomFieldsRequestBuilder<'a> {
        CreateAccountCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateAccountCustomFieldsRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateAccountCustomFieldsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
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

    pub fn build(self) -> Result<CreateAccountCustomFieldsRequest<'a>, &'static str> {
        Ok(CreateAccountCustomFieldsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
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
pub struct CreateAccountTagsRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<Uuid>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateAccountTagsRequest<'a> {
    pub fn builder() -> CreateAccountTagsRequestBuilder<'a> {
        CreateAccountTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateAccountTagsRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateAccountTagsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
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

    pub fn build(self) -> Result<CreateAccountTagsRequest<'a>, &'static str> {
        Ok(CreateAccountTagsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
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
pub struct CreatePaymentMethodRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentMethod,
    pub(crate) is_default: Option<bool>,
    pub(crate) pay_all_unpaid_invoices: Option<bool>,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreatePaymentMethodRequest<'a> {
    pub fn builder() -> CreatePaymentMethodRequestBuilder<'a> {
        CreatePaymentMethodRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreatePaymentMethodRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentMethod>,
    is_default: Option<bool>,
    pay_all_unpaid_invoices: Option<bool>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreatePaymentMethodRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::PaymentMethod) -> Self {
        self.body = Some(body);
        self
    }

    pub fn is_default(mut self, is_default: bool) -> Self {
        self.is_default = Some(is_default);
        self
    }

    pub fn pay_all_unpaid_invoices(mut self, pay_all_unpaid_invoices: bool) -> Self {
        self.pay_all_unpaid_invoices = Some(pay_all_unpaid_invoices);
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

    pub fn build(self) -> Result<CreatePaymentMethodRequest<'a>, &'static str> {
        Ok(CreatePaymentMethodRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            is_default: self.is_default,
            pay_all_unpaid_invoices: self.pay_all_unpaid_invoices,
            control_plugin_name: self.control_plugin_name,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteAccountCustomFieldsRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) custom_field: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteAccountCustomFieldsRequest<'a> {
    pub fn builder() -> DeleteAccountCustomFieldsRequestBuilder<'a> {
        DeleteAccountCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteAccountCustomFieldsRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    custom_field: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteAccountCustomFieldsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
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

    pub fn build(self) -> Result<DeleteAccountCustomFieldsRequest<'a>, &'static str> {
        Ok(DeleteAccountCustomFieldsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
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
pub struct DeleteAccountTagsRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) tag_def: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteAccountTagsRequest<'a> {
    pub fn builder() -> DeleteAccountTagsRequestBuilder<'a> {
        DeleteAccountTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteAccountTagsRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    tag_def: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteAccountTagsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
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

    pub fn build(self) -> Result<DeleteAccountTagsRequest<'a>, &'static str> {
        Ok(DeleteAccountTagsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
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
pub struct GetAccountRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) account_with_balance: Option<bool>,
    pub(crate) account_with_balance_and_cba: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetAccountRequest<'a> {
    pub fn builder() -> GetAccountRequestBuilder<'a> {
        GetAccountRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAccountRequestBuilder<'a> {
    account_id: Option<&'a str>,
    account_with_balance: Option<bool>,
    account_with_balance_and_cba: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetAccountRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn account_with_balance(mut self, account_with_balance: bool) -> Self {
        self.account_with_balance = Some(account_with_balance);
        self
    }

    pub fn account_with_balance_and_cba(mut self, account_with_balance_and_cba: bool) -> Self {
        self.account_with_balance_and_cba = Some(account_with_balance_and_cba);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetAccountRequest<'a>, &'static str> {
        Ok(GetAccountRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            account_with_balance: self.account_with_balance,
            account_with_balance_and_cba: self.account_with_balance_and_cba,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAccountAuditLogsRequest<'a> {
    pub(crate) account_id: &'a str,
}

impl<'a> GetAccountAuditLogsRequest<'a> {
    pub fn builder() -> GetAccountAuditLogsRequestBuilder<'a> {
        GetAccountAuditLogsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAccountAuditLogsRequestBuilder<'a> {
    account_id: Option<&'a str>,
}

impl<'a> GetAccountAuditLogsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn build(self) -> Result<GetAccountAuditLogsRequest<'a>, &'static str> {
        Ok(GetAccountAuditLogsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAccountAuditLogsWithHistoryRequest<'a> {
    pub(crate) account_id: &'a str,
}

impl<'a> GetAccountAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetAccountAuditLogsWithHistoryRequestBuilder<'a> {
        GetAccountAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAccountAuditLogsWithHistoryRequestBuilder<'a> {
    account_id: Option<&'a str>,
}

impl<'a> GetAccountAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn build(self) -> Result<GetAccountAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetAccountAuditLogsWithHistoryRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAccountBundlesRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) external_key: Option<&'a str>,
    pub(crate) bundles_filter: Option<&'a str>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetAccountBundlesRequest<'a> {
    pub fn builder() -> GetAccountBundlesRequestBuilder<'a> {
        GetAccountBundlesRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAccountBundlesRequestBuilder<'a> {
    account_id: Option<&'a str>,
    external_key: Option<&'a str>,
    bundles_filter: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetAccountBundlesRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn external_key(mut self, external_key: &'a str) -> Self {
        self.external_key = Some(external_key);
        self
    }

    pub fn bundles_filter(mut self, bundles_filter: &'a str) -> Self {
        self.bundles_filter = Some(bundles_filter);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetAccountBundlesRequest<'a>, &'static str> {
        Ok(GetAccountBundlesRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            external_key: self.external_key,
            bundles_filter: self.bundles_filter,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAccountBundlesPaginatedRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetAccountBundlesPaginatedRequest<'a> {
    pub fn builder() -> GetAccountBundlesPaginatedRequestBuilder<'a> {
        GetAccountBundlesPaginatedRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAccountBundlesPaginatedRequestBuilder<'a> {
    account_id: Option<&'a str>,
    offset: Option<i64>,
    limit: Option<i64>,
    audit: Option<&'a str>,
}

impl<'a> GetAccountBundlesPaginatedRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
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

    pub fn build(self) -> Result<GetAccountBundlesPaginatedRequest<'a>, &'static str> {
        Ok(GetAccountBundlesPaginatedRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            offset: self.offset,
            limit: self.limit,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAccountByKeyRequest<'a> {
    pub(crate) external_key: &'a str,
    pub(crate) account_with_balance: Option<bool>,
    pub(crate) account_with_balance_and_cba: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetAccountByKeyRequest<'a> {
    pub fn builder() -> GetAccountByKeyRequestBuilder<'a> {
        GetAccountByKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAccountByKeyRequestBuilder<'a> {
    external_key: Option<&'a str>,
    account_with_balance: Option<bool>,
    account_with_balance_and_cba: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetAccountByKeyRequestBuilder<'a> {
    pub fn external_key(mut self, external_key: &'a str) -> Self {
        self.external_key = Some(external_key);
        self
    }

    pub fn account_with_balance(mut self, account_with_balance: bool) -> Self {
        self.account_with_balance = Some(account_with_balance);
        self
    }

    pub fn account_with_balance_and_cba(mut self, account_with_balance_and_cba: bool) -> Self {
        self.account_with_balance_and_cba = Some(account_with_balance_and_cba);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetAccountByKeyRequest<'a>, &'static str> {
        Ok(GetAccountByKeyRequest {
            external_key: self.external_key.ok_or("external_key is required")?,
            account_with_balance: self.account_with_balance,
            account_with_balance_and_cba: self.account_with_balance_and_cba,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAccountCustomFieldsRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetAccountCustomFieldsRequest<'a> {
    pub fn builder() -> GetAccountCustomFieldsRequestBuilder<'a> {
        GetAccountCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAccountCustomFieldsRequestBuilder<'a> {
    account_id: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetAccountCustomFieldsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetAccountCustomFieldsRequest<'a>, &'static str> {
        Ok(GetAccountCustomFieldsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAccountEmailAuditLogsWithHistoryRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) account_email_id: &'a str,
}

impl<'a> GetAccountEmailAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetAccountEmailAuditLogsWithHistoryRequestBuilder<'a> {
        GetAccountEmailAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAccountEmailAuditLogsWithHistoryRequestBuilder<'a> {
    account_id: Option<&'a str>,
    account_email_id: Option<&'a str>,
}

impl<'a> GetAccountEmailAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn account_email_id(mut self, account_email_id: &'a str) -> Self {
        self.account_email_id = Some(account_email_id);
        self
    }

    pub fn build(self) -> Result<GetAccountEmailAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetAccountEmailAuditLogsWithHistoryRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            account_email_id: self.account_email_id.ok_or("account_email_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAccountTagsRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) included_deleted: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetAccountTagsRequest<'a> {
    pub fn builder() -> GetAccountTagsRequestBuilder<'a> {
        GetAccountTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAccountTagsRequestBuilder<'a> {
    account_id: Option<&'a str>,
    included_deleted: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetAccountTagsRequestBuilder<'a> {
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

    pub fn build(self) -> Result<GetAccountTagsRequest<'a>, &'static str> {
        Ok(GetAccountTagsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            included_deleted: self.included_deleted,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAccountTimelineRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) parallel: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetAccountTimelineRequest<'a> {
    pub fn builder() -> GetAccountTimelineRequestBuilder<'a> {
        GetAccountTimelineRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAccountTimelineRequestBuilder<'a> {
    account_id: Option<&'a str>,
    parallel: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetAccountTimelineRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn parallel(mut self, parallel: bool) -> Self {
        self.parallel = Some(parallel);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetAccountTimelineRequest<'a>, &'static str> {
        Ok(GetAccountTimelineRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            parallel: self.parallel,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAccountsRequest<'a> {
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) account_with_balance: Option<bool>,
    pub(crate) account_with_balance_and_cba: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetAccountsRequest<'a> {
    pub fn builder() -> GetAccountsRequestBuilder<'a> {
        GetAccountsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAccountsRequestBuilder<'a> {
    offset: Option<i64>,
    limit: Option<i64>,
    account_with_balance: Option<bool>,
    account_with_balance_and_cba: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetAccountsRequestBuilder<'a> {
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn account_with_balance(mut self, account_with_balance: bool) -> Self {
        self.account_with_balance = Some(account_with_balance);
        self
    }

    pub fn account_with_balance_and_cba(mut self, account_with_balance_and_cba: bool) -> Self {
        self.account_with_balance_and_cba = Some(account_with_balance_and_cba);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetAccountsRequest<'a>, &'static str> {
        Ok(GetAccountsRequest {
            offset: self.offset,
            limit: self.limit,
            account_with_balance: self.account_with_balance,
            account_with_balance_and_cba: self.account_with_balance_and_cba,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAllCustomFieldsRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) object_type: Option<&'a str>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetAllCustomFieldsRequest<'a> {
    pub fn builder() -> GetAllCustomFieldsRequestBuilder<'a> {
        GetAllCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAllCustomFieldsRequestBuilder<'a> {
    account_id: Option<&'a str>,
    object_type: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetAllCustomFieldsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn object_type(mut self, object_type: &'a str) -> Self {
        self.object_type = Some(object_type);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetAllCustomFieldsRequest<'a>, &'static str> {
        Ok(GetAllCustomFieldsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            object_type: self.object_type,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAllTagsRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) object_type: Option<&'a str>,
    pub(crate) included_deleted: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetAllTagsRequest<'a> {
    pub fn builder() -> GetAllTagsRequestBuilder<'a> {
        GetAllTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAllTagsRequestBuilder<'a> {
    account_id: Option<&'a str>,
    object_type: Option<&'a str>,
    included_deleted: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetAllTagsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn object_type(mut self, object_type: &'a str) -> Self {
        self.object_type = Some(object_type);
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

    pub fn build(self) -> Result<GetAllTagsRequest<'a>, &'static str> {
        Ok(GetAllTagsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            object_type: self.object_type,
            included_deleted: self.included_deleted,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetBlockingStateAuditLogsWithHistoryRequest<'a> {
    pub(crate) blocking_id: &'a str,
}

impl<'a> GetBlockingStateAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetBlockingStateAuditLogsWithHistoryRequestBuilder<'a> {
        GetBlockingStateAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetBlockingStateAuditLogsWithHistoryRequestBuilder<'a> {
    blocking_id: Option<&'a str>,
}

impl<'a> GetBlockingStateAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn blocking_id(mut self, blocking_id: &'a str) -> Self {
        self.blocking_id = Some(blocking_id);
        self
    }

    pub fn build(self) -> Result<GetBlockingStateAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetBlockingStateAuditLogsWithHistoryRequest {
            blocking_id: self.blocking_id.ok_or("blocking_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetBlockingStatesRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) blocking_state_types: Option<Vec<String>>,
    pub(crate) blocking_state_svcs: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetBlockingStatesRequest<'a> {
    pub fn builder() -> GetBlockingStatesRequestBuilder<'a> {
        GetBlockingStatesRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetBlockingStatesRequestBuilder<'a> {
    account_id: Option<&'a str>,
    blocking_state_types: Option<Vec<String>>,
    blocking_state_svcs: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetBlockingStatesRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn blocking_state_types(mut self, blocking_state_types: Vec<String>) -> Self {
        self.blocking_state_types = Some(blocking_state_types);
        self
    }

    pub fn blocking_state_svcs(mut self, blocking_state_svcs: Vec<String>) -> Self {
        self.blocking_state_svcs = Some(blocking_state_svcs);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetBlockingStatesRequest<'a>, &'static str> {
        Ok(GetBlockingStatesRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            blocking_state_types: self.blocking_state_types,
            blocking_state_svcs: self.blocking_state_svcs,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetChildrenAccountsRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) account_with_balance: Option<bool>,
    pub(crate) account_with_balance_and_cba: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetChildrenAccountsRequest<'a> {
    pub fn builder() -> GetChildrenAccountsRequestBuilder<'a> {
        GetChildrenAccountsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetChildrenAccountsRequestBuilder<'a> {
    account_id: Option<&'a str>,
    account_with_balance: Option<bool>,
    account_with_balance_and_cba: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetChildrenAccountsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn account_with_balance(mut self, account_with_balance: bool) -> Self {
        self.account_with_balance = Some(account_with_balance);
        self
    }

    pub fn account_with_balance_and_cba(mut self, account_with_balance_and_cba: bool) -> Self {
        self.account_with_balance_and_cba = Some(account_with_balance_and_cba);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetChildrenAccountsRequest<'a>, &'static str> {
        Ok(GetChildrenAccountsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            account_with_balance: self.account_with_balance,
            account_with_balance_and_cba: self.account_with_balance_and_cba,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetEmailsRequest<'a> {
    pub(crate) account_id: &'a str,
}

impl<'a> GetEmailsRequest<'a> {
    pub fn builder() -> GetEmailsRequestBuilder<'a> {
        GetEmailsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetEmailsRequestBuilder<'a> {
    account_id: Option<&'a str>,
}

impl<'a> GetEmailsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn build(self) -> Result<GetEmailsRequest<'a>, &'static str> {
        Ok(GetEmailsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoicePaymentsRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) with_attempts: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoicePaymentsRequest<'a> {
    pub fn builder() -> GetInvoicePaymentsRequestBuilder<'a> {
        GetInvoicePaymentsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoicePaymentsRequestBuilder<'a> {
    account_id: Option<&'a str>,
    with_plugin_info: Option<bool>,
    with_attempts: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoicePaymentsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
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

    pub fn build(self) -> Result<GetInvoicePaymentsRequest<'a>, &'static str> {
        Ok(GetInvoicePaymentsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            with_plugin_info: self.with_plugin_info,
            with_attempts: self.with_attempts,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoicesForAccountRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) start_date: Option<String>,
    pub(crate) end_date: Option<String>,
    pub(crate) with_migration_invoices: Option<bool>,
    pub(crate) unpaid_invoices_only: Option<bool>,
    pub(crate) include_voided_invoices: Option<bool>,
    pub(crate) include_invoice_components: Option<bool>,
    pub(crate) invoices_filter: Option<&'a str>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoicesForAccountRequest<'a> {
    pub fn builder() -> GetInvoicesForAccountRequestBuilder<'a> {
        GetInvoicesForAccountRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoicesForAccountRequestBuilder<'a> {
    account_id: Option<&'a str>,
    start_date: Option<String>,
    end_date: Option<String>,
    with_migration_invoices: Option<bool>,
    unpaid_invoices_only: Option<bool>,
    include_voided_invoices: Option<bool>,
    include_invoice_components: Option<bool>,
    invoices_filter: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoicesForAccountRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
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

    pub fn with_migration_invoices(mut self, with_migration_invoices: bool) -> Self {
        self.with_migration_invoices = Some(with_migration_invoices);
        self
    }

    pub fn unpaid_invoices_only(mut self, unpaid_invoices_only: bool) -> Self {
        self.unpaid_invoices_only = Some(unpaid_invoices_only);
        self
    }

    pub fn include_voided_invoices(mut self, include_voided_invoices: bool) -> Self {
        self.include_voided_invoices = Some(include_voided_invoices);
        self
    }

    pub fn include_invoice_components(mut self, include_invoice_components: bool) -> Self {
        self.include_invoice_components = Some(include_invoice_components);
        self
    }

    pub fn invoices_filter(mut self, invoices_filter: &'a str) -> Self {
        self.invoices_filter = Some(invoices_filter);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetInvoicesForAccountRequest<'a>, &'static str> {
        Ok(GetInvoicesForAccountRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            start_date: self.start_date,
            end_date: self.end_date,
            with_migration_invoices: self.with_migration_invoices,
            unpaid_invoices_only: self.unpaid_invoices_only,
            include_voided_invoices: self.include_voided_invoices,
            include_invoice_components: self.include_invoice_components,
            invoices_filter: self.invoices_filter,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoicesForAccountPaginatedRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoicesForAccountPaginatedRequest<'a> {
    pub fn builder() -> GetInvoicesForAccountPaginatedRequestBuilder<'a> {
        GetInvoicesForAccountPaginatedRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoicesForAccountPaginatedRequestBuilder<'a> {
    account_id: Option<&'a str>,
    offset: Option<i64>,
    limit: Option<i64>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoicesForAccountPaginatedRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
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

    pub fn build(self) -> Result<GetInvoicesForAccountPaginatedRequest<'a>, &'static str> {
        Ok(GetInvoicesForAccountPaginatedRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            offset: self.offset,
            limit: self.limit,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentsForAccountRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) with_attempts: Option<bool>,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentsForAccountRequest<'a> {
    pub fn builder() -> GetPaymentsForAccountRequestBuilder<'a> {
        GetPaymentsForAccountRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentsForAccountRequestBuilder<'a> {
    account_id: Option<&'a str>,
    with_attempts: Option<bool>,
    with_plugin_info: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentsForAccountRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn with_attempts(mut self, with_attempts: bool) -> Self {
        self.with_attempts = Some(with_attempts);
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

    pub fn build(self) -> Result<GetPaymentsForAccountRequest<'a>, &'static str> {
        Ok(GetPaymentsForAccountRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            with_attempts: self.with_attempts,
            with_plugin_info: self.with_plugin_info,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModifyAccountCustomFieldsRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyAccountCustomFieldsRequest<'a> {
    pub fn builder() -> ModifyAccountCustomFieldsRequestBuilder<'a> {
        ModifyAccountCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ModifyAccountCustomFieldsRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyAccountCustomFieldsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
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

    pub fn build(self) -> Result<ModifyAccountCustomFieldsRequest<'a>, &'static str> {
        Ok(ModifyAccountCustomFieldsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            body: self.body.ok_or("body is required")?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RemoveEmailRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) email: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> RemoveEmailRequest<'a> {
    pub fn builder() -> RemoveEmailRequestBuilder<'a> {
        RemoveEmailRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct RemoveEmailRequestBuilder<'a> {
    account_id: Option<&'a str>,
    email: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> RemoveEmailRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn email(mut self, email: &'a str) -> Self {
        self.email = Some(email);
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

    pub fn build(self) -> Result<RemoveEmailRequest<'a>, &'static str> {
        Ok(RemoveEmailRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            email: self.email.ok_or("email is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SearchAccountsRequest<'a> {
    pub(crate) search_key: &'a str,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) account_with_balance: Option<bool>,
    pub(crate) account_with_balance_and_cba: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> SearchAccountsRequest<'a> {
    pub fn builder() -> SearchAccountsRequestBuilder<'a> {
        SearchAccountsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SearchAccountsRequestBuilder<'a> {
    search_key: Option<&'a str>,
    offset: Option<i64>,
    limit: Option<i64>,
    account_with_balance: Option<bool>,
    account_with_balance_and_cba: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> SearchAccountsRequestBuilder<'a> {
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

    pub fn account_with_balance(mut self, account_with_balance: bool) -> Self {
        self.account_with_balance = Some(account_with_balance);
        self
    }

    pub fn account_with_balance_and_cba(mut self, account_with_balance_and_cba: bool) -> Self {
        self.account_with_balance_and_cba = Some(account_with_balance_and_cba);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<SearchAccountsRequest<'a>, &'static str> {
        Ok(SearchAccountsRequest {
            search_key: self.search_key.ok_or("search_key is required")?,
            offset: self.offset,
            limit: self.limit,
            account_with_balance: self.account_with_balance,
            account_with_balance_and_cba: self.account_with_balance_and_cba,
            audit: self.audit,
        })
    }
}
#[derive(Debug, Clone)]
pub struct SetDefaultPaymentMethodRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) payment_method_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) pay_all_unpaid_invoices: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> SetDefaultPaymentMethodRequest<'a> {
    pub fn builder() -> SetDefaultPaymentMethodRequestBuilder<'a> {
        SetDefaultPaymentMethodRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SetDefaultPaymentMethodRequestBuilder<'a> {
    account_id: Option<&'a str>,
    payment_method_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    pay_all_unpaid_invoices: Option<bool>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> SetDefaultPaymentMethodRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn payment_method_id(mut self, payment_method_id: &'a str) -> Self {
        self.payment_method_id = Some(payment_method_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn pay_all_unpaid_invoices(mut self, pay_all_unpaid_invoices: bool) -> Self {
        self.pay_all_unpaid_invoices = Some(pay_all_unpaid_invoices);
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

    pub fn build(self) -> Result<SetDefaultPaymentMethodRequest<'a>, &'static str> {
        Ok(SetDefaultPaymentMethodRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            payment_method_id: self.payment_method_id.ok_or("payment_method_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            pay_all_unpaid_invoices: self.pay_all_unpaid_invoices,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}


#[derive(Debug, Clone)]
pub struct TransferChildCreditToParentRequest<'a> {
    pub(crate) child_account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> TransferChildCreditToParentRequest<'a> {
    pub fn builder() -> TransferChildCreditToParentRequestBuilder<'a> {
        TransferChildCreditToParentRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct TransferChildCreditToParentRequestBuilder<'a> {
    child_account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> TransferChildCreditToParentRequestBuilder<'a> {
    pub fn child_account_id(mut self, child_account_id: &'a str) -> Self {
        self.child_account_id = Some(child_account_id);
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

    pub fn build(self) -> Result<TransferChildCreditToParentRequest<'a>, &'static str> {
        Ok(TransferChildCreditToParentRequest {
            child_account_id: self.child_account_id.ok_or("child_account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UpdateAccountRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::Account,
    pub(crate) treat_null_as_reset: Option<bool>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdateAccountRequest<'a> {
    pub fn builder() -> UpdateAccountRequestBuilder<'a> {
        UpdateAccountRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UpdateAccountRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::Account>,
    treat_null_as_reset: Option<bool>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdateAccountRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::Account) -> Self {
        self.body = Some(body);
        self
    }

    pub fn treat_null_as_reset(mut self, treat_null_as_reset: bool) -> Self {
        self.treat_null_as_reset = Some(treat_null_as_reset);
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

    pub fn build(self) -> Result<UpdateAccountRequest<'a>, &'static str> {
        Ok(UpdateAccountRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            body: self.body.ok_or("body is required")?,
            treat_null_as_reset: self.treat_null_as_reset,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ProcessPaymentRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) payment_method_id: Option<&'a str>,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ProcessPaymentRequest<'a> {
    pub fn builder() -> ProcessPaymentRequestBuilder<'a> {
        ProcessPaymentRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ProcessPaymentRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    payment_method_id: Option<&'a str>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ProcessPaymentRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
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

    pub fn payment_method_id(mut self, payment_method_id: &'a str) -> Self {
        self.payment_method_id = Some(payment_method_id);
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

    pub fn build(self) -> Result<ProcessPaymentRequest<'a>, &'static str> {
        Ok(ProcessPaymentRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            body: self.body.ok_or("body is required")?,
            payment_method_id: self.payment_method_id,
            control_plugin_name: self.control_plugin_name,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ProcessPaymentByExternalKeyRequest<'a> {
    pub(crate) external_key: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::PaymentTransaction,
    pub(crate) payment_method_id: Option<&'a str>,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ProcessPaymentByExternalKeyRequest<'a> {
    pub fn builder() -> ProcessPaymentByExternalKeyRequestBuilder<'a> {
        ProcessPaymentByExternalKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ProcessPaymentByExternalKeyRequestBuilder<'a> {
    external_key: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::PaymentTransaction>,
    payment_method_id: Option<&'a str>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ProcessPaymentByExternalKeyRequestBuilder<'a> {
    pub fn external_key(mut self, external_key: &'a str) -> Self {
        self.external_key = Some(external_key);
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

    pub fn payment_method_id(mut self, payment_method_id: &'a str) -> Self {
        self.payment_method_id = Some(payment_method_id);
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

    pub fn build(self) -> Result<ProcessPaymentByExternalKeyRequest<'a>, &'static str> {
        Ok(ProcessPaymentByExternalKeyRequest {
            external_key: self.external_key.ok_or("external_key is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            body: self.body.ok_or("body is required")?,
            payment_method_id: self.payment_method_id,
            control_plugin_name: self.control_plugin_name,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RebalanceExistingCbaonAccountRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> RebalanceExistingCbaonAccountRequest<'a> {
    pub fn builder() -> RebalanceExistingCbaonAccountRequestBuilder<'a> {
        RebalanceExistingCbaonAccountRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct RebalanceExistingCbaonAccountRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> RebalanceExistingCbaonAccountRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
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

    pub fn build(self) -> Result<RebalanceExistingCbaonAccountRequest<'a>, &'static str> {
        Ok(RebalanceExistingCbaonAccountRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RefreshPaymentMethodsRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) plugin_name: Option<&'a str>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> RefreshPaymentMethodsRequest<'a> {
    pub fn builder() -> RefreshPaymentMethodsRequestBuilder<'a> {
        RefreshPaymentMethodsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct RefreshPaymentMethodsRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    plugin_name: Option<&'a str>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> RefreshPaymentMethodsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
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

    pub fn x_killbill_reason(mut self, x_killbill_reason: &'a str) -> Self {
        self.x_killbill_reason = Some(x_killbill_reason);
        self
    }

    pub fn x_killbill_comment(mut self, x_killbill_comment: &'a str) -> Self {
        self.x_killbill_comment = Some(x_killbill_comment);
        self
    }

    pub fn build(self) -> Result<RefreshPaymentMethodsRequest<'a>, &'static str> {
        Ok(RefreshPaymentMethodsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            plugin_name: self.plugin_name,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetOverdueAccountRequest<'a> {
    pub(crate) account_id: &'a str,
}

impl<'a> GetOverdueAccountRequest<'a> {
    pub fn builder() -> GetOverdueAccountRequestBuilder<'a> {
        GetOverdueAccountRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetOverdueAccountRequestBuilder<'a> {
    account_id: Option<&'a str>,
}

impl<'a> GetOverdueAccountRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn build(self) -> Result<GetOverdueAccountRequest<'a>, &'static str> {
        Ok(GetOverdueAccountRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentMethodsForAccountRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) included_deleted: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentMethodsForAccountRequest<'a> {
    pub fn builder() -> GetPaymentMethodsForAccountRequestBuilder<'a> {
        GetPaymentMethodsForAccountRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentMethodsForAccountRequestBuilder<'a> {
    account_id: Option<&'a str>,
    with_plugin_info: Option<bool>,
    included_deleted: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentMethodsForAccountRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn with_plugin_info(mut self, with_plugin_info: bool) -> Self {
        self.with_plugin_info = Some(with_plugin_info);
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

    pub fn build(self) -> Result<GetPaymentMethodsForAccountRequest<'a>, &'static str> {
        Ok(GetPaymentMethodsForAccountRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            with_plugin_info: self.with_plugin_info,
            included_deleted: self.included_deleted,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}


#[derive(Debug, Clone)]
pub struct PayAllInvoicesRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) payment_method_id: Option<&'a str>,
    pub(crate) external_payment: Option<bool>,
    pub(crate) payment_amount: Option<f64>,
    pub(crate) target_date: Option<String>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> PayAllInvoicesRequest<'a> {
    pub fn builder() -> PayAllInvoicesRequestBuilder<'a> {
        PayAllInvoicesRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct PayAllInvoicesRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    payment_method_id: Option<&'a str>,
    external_payment: Option<bool>,
    payment_amount: Option<f64>,
    target_date: Option<String>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> PayAllInvoicesRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn payment_method_id(mut self, payment_method_id: &'a str) -> Self {
        self.payment_method_id = Some(payment_method_id);
        self
    }

    pub fn external_payment(mut self, external_payment: bool) -> Self {
        self.external_payment = Some(external_payment);
        self
    }

    pub fn payment_amount(mut self, payment_amount: f64) -> Self {
        self.payment_amount = Some(payment_amount);
        self
    }

    pub fn target_date(mut self, target_date: impl Into<String>) -> Self {
        self.target_date = Some(target_date.into());
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

    pub fn build(self) -> Result<PayAllInvoicesRequest<'a>, &'static str> {
        Ok(PayAllInvoicesRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            payment_method_id: self.payment_method_id,
            external_payment: self.external_payment,
            payment_amount: self.payment_amount,
            target_date: self.target_date,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}



#[derive(Debug, thiserror::Error)]
pub enum AccountApiError {
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

impl AccountApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{
        BlockingState,
        AccountEmail,
        Account,
        CustomField,
        Tag,
        AuditLog,
        Bundle,
        PaymentMethod,
        InvoicePayment,
        Payment,
        AccountTimeline,
        Invoice,
    };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_add_account_blocking_state() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/accounts/test-account-id/block")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"state_name": "test-state"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = AddAccountBlockingStateRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .body(BlockingState {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.add_account_blocking_state(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_add_email() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/accounts/test-account-id/emails")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"email": "test@test.com"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = AddEmailRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .body(AccountEmail {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.add_email(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_close_account() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/accounts/test-account-id")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = CloseAccountRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.close_account(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_account() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/accounts")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"account_id": "test-account-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = CreateAccountRequest::builder()
            .x_killbill_created_by("test")
            .body(Account {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.create_account(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_account_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/accounts/test-account-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = CreateAccountCustomFieldsRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_account_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_account_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/accounts/test-account-id/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tag_definition_id": "test-tag-def-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = CreateAccountTagsRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .body(vec![Uuid::new_v4()])
            .build()
            .unwrap();

        let result = api.create_account_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_payment_method() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/accounts/test-account-id/paymentMethods")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"payment_method_id": "test-payment-method-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = CreatePaymentMethodRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .body(PaymentMethod {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.create_payment_method(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_account_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/accounts/test-account-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = DeleteAccountCustomFieldsRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_account_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_account_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/accounts/test-account-id/tags")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = DeleteAccountTagsRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_account_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_account() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"account_id": "test-account-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetAccountRequest::builder().account_id("test-account-id").build().unwrap();

        let result = api.get_account(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_account_audit_logs() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/auditLogs")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"change_type": "test-change-type"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetAccountAuditLogsRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_account_audit_logs(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_account_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"change_type": "test-change-type"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetAccountAuditLogsWithHistoryRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_account_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_account_bundles() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/bundles")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"bundle_id": "test-bundle-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetAccountBundlesRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_account_bundles(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_account_bundles_paginated() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/bundles/pagination")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"bundle_id": "test-bundle-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetAccountBundlesPaginatedRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_account_bundles_paginated(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_account_by_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts?externalKey=test-external-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"account_id": "test-account-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetAccountByKeyRequest::builder()
            .external_key("test-external-key")
            .build()
            .unwrap();

        let result = api.get_account_by_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_account_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetAccountCustomFieldsRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_account_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_account_email_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock(
                "GET",
                "/1.0/kb/accounts/test-account-id/emails/test-account-email-id/auditLogsWithHistory"
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"change_type": "test-change-type"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetAccountEmailAuditLogsWithHistoryRequest::builder()
            .account_id("test-account-id")
            .account_email_id("test-account-email-id")
            .build()
            .unwrap();

        let result = api.get_account_email_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_account_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tag_definition_id": "test-tag-def-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetAccountTagsRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_account_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_account_timeline() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/timeline")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"account_id": "test-account-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetAccountTimelineRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_account_timeline(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_accounts() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/pagination")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"account_id": "test-account-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetAccountsRequest::builder().build().unwrap();

        let result = api.get_accounts(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_all_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/allCustomFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetAllCustomFieldsRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_all_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_all_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/allTags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tag_definition_id": "test-tag-def-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetAllTagsRequest::builder().account_id("test-account-id").build().unwrap();

        let result = api.get_all_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_blocking_state_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/block/test-blocking-id/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"change_type": "test-change-type"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetBlockingStateAuditLogsWithHistoryRequest::builder()
            .blocking_id("test-blocking-id")
            .build()
            .unwrap();

        let result = api.get_blocking_state_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_blocking_states() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/block")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"state_name": "test-state"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetBlockingStatesRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_blocking_states(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_children_accounts() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/children")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"account_id": "test-account-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetChildrenAccountsRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_children_accounts(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_emails() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/emails")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"email": "test@test.com"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetEmailsRequest::builder().account_id("test-account-id").build().unwrap();

        let result = api.get_emails(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_payments() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/invoicePayments")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"payment_id": "test-payment-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetInvoicePaymentsRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_invoice_payments(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoices_for_account() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/invoices")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"invoice_id": "test-invoice-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetInvoicesForAccountRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_invoices_for_account(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoices_for_account_paginated() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/invoices/pagination")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"invoice_id": "test-invoice-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetInvoicesForAccountPaginatedRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_invoices_for_account_paginated(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_overdue_account() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/overdue")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"state_name": "test-state"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetOverdueAccountRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_overdue_account(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payment_methods_for_account() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/paymentMethods")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"payment_method_id": "test-payment-method-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetPaymentMethodsForAccountRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_payment_methods_for_account(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payments_for_account() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/test-account-id/payments")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"payment_id": "test-payment-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = GetPaymentsForAccountRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_payments_for_account(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_modify_account_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/accounts/test-account-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = ModifyAccountCustomFieldsRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.modify_account_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_pay_all_invoices() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/accounts/test-account-id/invoicePayments")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"invoice_id": "test-invoice-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = PayAllInvoicesRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.pay_all_invoices(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_process_payment() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/accounts/test-account-id/payments")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"payment_id": "test-payment-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = ProcessPaymentRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .body(models::PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.process_payment(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_process_payment_by_external_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/accounts/payments?externalKey=test-external-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"payment_id": "test-payment-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = ProcessPaymentByExternalKeyRequest::builder()
            .external_key("test-external-key")
            .x_killbill_created_by("test")
            .body(models::PaymentTransaction {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.process_payment_by_external_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_rebalance_existing_cbaon_account() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/accounts/test-account-id/cbaRebalancing")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = RebalanceExistingCbaonAccountRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.rebalance_existing_cbaon_account(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_refresh_payment_methods() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/accounts/test-account-id/paymentMethods/refresh")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = RefreshPaymentMethodsRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.refresh_payment_methods(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_remove_email() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/accounts/test-account-id/emails/test@test.com")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = RemoveEmailRequest::builder()
            .account_id("test-account-id")
            .email("test@test.com")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.remove_email(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_search_accounts() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/accounts/search/test-search-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"account_id": "test-account-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = SearchAccountsRequest::builder()
            .search_key("test-search-key")
            .build()
            .unwrap();

        let result = api.search_accounts(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_set_default_payment_method() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock(
                "PUT",
                "/1.0/kb/accounts/test-account-id/paymentMethods/test-payment-method-id/setDefault"
            )
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = SetDefaultPaymentMethodRequest::builder()
            .payment_method_id("test-payment-method-id")
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.set_default_payment_method(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_transfer_child_credit_to_parent() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/accounts/test-child-account-id/transferCredit")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = TransferChildCreditToParentRequest::builder()
            .child_account_id("test-child-account-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.transfer_child_credit_to_parent(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_update_account() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/accounts/test-account-id")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AccountApi::new(config);
        let request = UpdateAccountRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .body(Account {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.update_account(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
