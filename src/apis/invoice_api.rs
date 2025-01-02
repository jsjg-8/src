use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use uuid::Uuid;
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct InvoiceApi {
    config: Configuration,
}

impl InvoiceApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn adjust_invoice_item(
        &self,
        request: AdjustInvoiceItemRequest<'_>
    ) -> Result<models::Invoice, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/{}", self.config.base_path, request.invoice_id);

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

    pub async fn commit_invoice(
        &self,
        request: CommitInvoiceRequest<'_>
    ) -> Result<(), InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/{}/commitInvoice",
            self.config.base_path,
            request.invoice_id
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

    pub async fn create_external_charges(
        &self,
        request: CreateExternalChargesRequest<'_>
    ) -> Result<Vec<models::InvoiceItem>, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/charges/{}",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("requestedDate", request.requested_date),
                    ("autoCommit", request.auto_commit.map(|b| b.to_string())),
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

    pub async fn create_future_invoice(
        &self,
        request: CreateFutureInvoiceRequest<'_>
    ) -> Result<models::Invoice, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("accountId", request.account_id),
                    ("targetDate", &request.target_date.unwrap_or_default()),
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

    pub async fn create_future_invoice_group(
        &self,
        request: CreateFutureInvoiceGroupRequest<'_>
    ) -> Result<Vec<models::Invoice>, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/group", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("accountId", request.account_id),
                    ("targetDate", request.target_date.unwrap_or_default().as_str()),
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

    pub async fn create_instant_payment(
        &self,
        request: CreateInstantPaymentRequest<'_>
    ) -> Result<models::InvoicePayment, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/{}/payments",
            self.config.base_path,
            request.invoice_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("externalPayment", request.external_payment.map(|b| b.to_string()))])
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

    pub async fn create_invoice_custom_fields(
        &self,
        request: CreateInvoiceCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/{}/customFields",
            self.config.base_path,
            request.invoice_id
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

    pub async fn create_invoice_tags(
        &self,
        request: CreateInvoiceTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/{}/tags", self.config.base_path, request.invoice_id);

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

    pub async fn create_migration_invoice(
        &self,
        request: CreateMigrationInvoiceRequest<'_>
    ) -> Result<models::Invoice, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/migration/{}",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("targetDate", request.target_date)])
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn create_tax_items(
        &self,
        request: CreateTaxItemsRequest<'_>
    ) -> Result<Vec<models::InvoiceItem>, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/taxes/{}", self.config.base_path, request.account_id);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("autoCommit", request.auto_commit.map(|b| b.to_string())),
                    ("requestedDate", request.requested_date),
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

    pub async fn delete_cba(&self, request: DeleteCbaRequest<'_>) -> Result<(), InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/{}/{}/cba",
            self.config.base_path,
            request.invoice_id,
            request.invoice_item_id
        );

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("accountId", request.account_id)])
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn delete_invoice_custom_fields(
        &self,
        request: DeleteInvoiceCustomFieldsRequest<'_>
    ) -> Result<(), InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/{}/customFields",
            self.config.base_path,
            request.invoice_id
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

    pub async fn delete_invoice_tags(
        &self,
        request: DeleteInvoiceTagsRequest<'_>
    ) -> Result<(), InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/{}/tags", self.config.base_path, request.invoice_id);

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

    pub async fn generate_dry_run_invoice(
        &self,
        request: GenerateDryRunInvoiceRequest<'_>
    ) -> Result<models::Invoice, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/dryRun", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("accountId", request.account_id),
                    ("targetDate", request.target_date.as_deref().unwrap_or("")),
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

    pub async fn get_catalog_translation(
        &self,
        request: GetCatalogTranslationRequest<'_>
    ) -> Result<String, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/catalogTranslation/{}",
            self.config.base_path,
            request.locale
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice(
        &self,
        request: GetInvoiceRequest<'_>
    ) -> Result<models::Invoice, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/{}", self.config.base_path, request.invoice_id);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("withChildrenItems", request.with_children_items.map(|b| b.to_string())),
                    ("audit", request.audit.map(|s| s.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice_as_html(
        &self,
        request: GetInvoiceAsHtmlRequest<'_>
    ) -> Result<String, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/{}/html", self.config.base_path, request.invoice_id);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice_audit_logs_with_history(
        &self,
        request: GetInvoiceAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/{}/auditLogsWithHistory",
            self.config.base_path,
            request.invoice_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice_by_item_id(
        &self,
        request: GetInvoiceByItemIdRequest<'_>
    ) -> Result<models::Invoice, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/byItemId/{}", self.config.base_path, request.item_id);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("withChildrenItems", request.with_children_items.map(|b| b.to_string())),
                    ("audit", request.audit.map(|b| b.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice_by_number(
        &self,
        request: GetInvoiceByNumberRequest<'_>
    ) -> Result<models::Invoice, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/byNumber/{}",
            self.config.base_path,
            request.invoice_number
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("withChildrenItems", request.with_children_items.map(|b| b.to_string())),
                    ("audit", request.audit.map(|b| b.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice_custom_fields(
        &self,
        request: GetInvoiceCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/{}/customFields",
            self.config.base_path,
            request.invoice_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("audit", request.audit)]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice_mp_template(
        &self,
        request: GetInvoiceMpTemplateRequest<'_>
    ) -> Result<String, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/manualPayTemplate/{}",
            self.config.base_path,
            request.locale
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice_tags(
        &self,
        request: GetInvoiceTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/{}/tags", self.config.base_path, request.invoice_id);

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

    pub async fn get_invoice_template(&self) -> Result<String, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/template", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoice_translation(
        &self,
        request: GetInvoiceTranslationRequest<'_>
    ) -> Result<String, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/translation/{}",
            self.config.base_path,
            request.locale
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_invoices(
        &self,
        request: GetInvoicesRequest<'_>
    ) -> Result<Vec<models::Invoice>, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/pagination", self.config.base_path);

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

    pub async fn get_invoices_group(
        &self,
        request: GetInvoicesGroupRequest<'_>
    ) -> Result<Vec<models::Invoice>, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/{}/group", self.config.base_path, request.group_id);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("accountId", request.account_id),
                    ("withChildrenItems", request.with_children_items.map(|b| if b { "true" } else { "false" }).unwrap_or("false")),
                    ("audit", request.audit.unwrap_or("")),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_payments_for_invoice(
        &self,
        request: GetPaymentsForInvoiceRequest<'_>
    ) -> Result<Vec<models::InvoicePayment>, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/{}/payments",
            self.config.base_path,
            request.invoice_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("withPluginInfo", request.with_plugin_info.map(|b| b.to_string())),
                    ("withAttempts", request.with_attempts.map(|b| b.to_string())),
                    ("audit", request.audit.map(|l| l.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn modify_invoice_custom_fields(
        &self,
        request: ModifyInvoiceCustomFieldsRequest<'_>
    ) -> Result<(), InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/{}/customFields",
            self.config.base_path,
            request.invoice_id
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

    pub async fn search_invoices(
        &self,
        request: SearchInvoicesRequest<'_>
    ) -> Result<Vec<models::Invoice>, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/search/{}",
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
                    ("audit", request.audit.map(|l| l.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn upload_catalog_translation(
        &self,
        request: UploadCatalogTranslationRequest<'_>
    ) -> Result<String, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/catalogTranslation/{}",
            self.config.base_path,
            request.locale
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("deleteIfExists", request.delete_if_exists.map(|b| b.to_string()))])
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn upload_invoice_mp_template(
        &self,
        request: UploadInvoiceMpTemplateRequest<'_>
    ) -> Result<String, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/manualPayTemplate", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("deleteIfExists", request.delete_if_exists.map(|b| b.to_string()))])
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn upload_invoice_template(
        &self,
        request: UploadInvoiceTemplateRequest<'_>
    ) -> Result<String, InvoiceApiError> {
        let url = format!("{}/1.0/kb/invoices/template", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("deleteIfExists", request.delete_if_exists.map(|b| b.to_string()))])
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn upload_invoice_translation(
        &self,
        request: UploadInvoiceTranslationRequest<'_>
    ) -> Result<String, InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/translation/{}",
            self.config.base_path,
            request.locale
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("deleteIfExists", request.delete_if_exists.map(|b| b.to_string()))])
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn void_invoice(
        &self,
        request: VoidInvoiceRequest<'_>
    ) -> Result<(), InvoiceApiError> {
        let url = format!(
            "{}/1.0/kb/invoices/{}/voidInvoice",
            self.config.base_path,
            request.invoice_id
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

    async fn handle_response<T: DeserializeOwned>(
        response: Response
    ) -> Result<T, InvoiceApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(InvoiceApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(InvoiceApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), InvoiceApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(InvoiceApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdjustInvoiceItemRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::InvoiceItem,
    pub(crate) requested_date: Option<String>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> AdjustInvoiceItemRequest<'a> {
    pub fn builder() -> AdjustInvoiceItemRequestBuilder<'a> {
        AdjustInvoiceItemRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct AdjustInvoiceItemRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::InvoiceItem>,
    requested_date: Option<String>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> AdjustInvoiceItemRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::InvoiceItem) -> Self {
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

    pub fn build(self) -> Result<AdjustInvoiceItemRequest<'a>, &'static str> {
        Ok(AdjustInvoiceItemRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
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
pub struct CommitInvoiceRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CommitInvoiceRequest<'a> {
    pub fn builder() -> CommitInvoiceRequestBuilder<'a> {
        CommitInvoiceRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CommitInvoiceRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CommitInvoiceRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
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

    pub fn build(self) -> Result<CommitInvoiceRequest<'a>, &'static str> {
        Ok(CommitInvoiceRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateExternalChargesRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::InvoiceItem>,
    pub(crate) requested_date: Option<String>,
    pub(crate) auto_commit: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateExternalChargesRequest<'a> {
    pub fn builder() -> CreateExternalChargesRequestBuilder<'a> {
        CreateExternalChargesRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateExternalChargesRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::InvoiceItem>>,
    requested_date: Option<String>,
    auto_commit: Option<bool>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateExternalChargesRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: Vec<models::InvoiceItem>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn requested_date(mut self, requested_date: impl Into<String>) -> Self {
        self.requested_date = Some(requested_date.into());
        self
    }

    pub fn auto_commit(mut self, auto_commit: bool) -> Self {
        self.auto_commit = Some(auto_commit);
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

    pub fn build(self) -> Result<CreateExternalChargesRequest<'a>, &'static str> {
        Ok(CreateExternalChargesRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            requested_date: self.requested_date,
            auto_commit: self.auto_commit,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateFutureInvoiceRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) target_date: Option<String>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateFutureInvoiceRequest<'a> {
    pub fn builder() -> CreateFutureInvoiceRequestBuilder<'a> {
        CreateFutureInvoiceRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateFutureInvoiceRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    target_date: Option<String>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateFutureInvoiceRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
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

    pub fn build(self) -> Result<CreateFutureInvoiceRequest<'a>, &'static str> {
        Ok(CreateFutureInvoiceRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            target_date: self.target_date,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateFutureInvoiceGroupRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) target_date: Option<String>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateFutureInvoiceGroupRequest<'a> {
    pub fn builder() -> CreateFutureInvoiceGroupRequestBuilder<'a> {
        CreateFutureInvoiceGroupRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateFutureInvoiceGroupRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    target_date: Option<String>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateFutureInvoiceGroupRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
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

    pub fn build(self) -> Result<CreateFutureInvoiceGroupRequest<'a>, &'static str> {
        Ok(CreateFutureInvoiceGroupRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            target_date: self.target_date,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateInstantPaymentRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::InvoicePayment,
    pub(crate) external_payment: Option<bool>,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInstantPaymentRequest<'a> {
    pub fn builder() -> CreateInstantPaymentRequestBuilder<'a> {
        CreateInstantPaymentRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateInstantPaymentRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::InvoicePayment>,
    external_payment: Option<bool>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInstantPaymentRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::InvoicePayment) -> Self {
        self.body = Some(body);
        self
    }

    pub fn external_payment(mut self, external_payment: bool) -> Self {
        self.external_payment = Some(external_payment);
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

    pub fn build(self) -> Result<CreateInstantPaymentRequest<'a>, &'static str> {
        Ok(CreateInstantPaymentRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            external_payment: self.external_payment,
            control_plugin_name: self.control_plugin_name,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateInvoiceCustomFieldsRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInvoiceCustomFieldsRequest<'a> {
    pub fn builder() -> CreateInvoiceCustomFieldsRequestBuilder<'a> {
        CreateInvoiceCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateInvoiceCustomFieldsRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInvoiceCustomFieldsRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
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

    pub fn build(self) -> Result<CreateInvoiceCustomFieldsRequest<'a>, &'static str> {
        Ok(CreateInvoiceCustomFieldsRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
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
pub struct CreateInvoiceTagsRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<Uuid>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInvoiceTagsRequest<'a> {
    pub fn builder() -> CreateInvoiceTagsRequestBuilder<'a> {
        CreateInvoiceTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateInvoiceTagsRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateInvoiceTagsRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
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

    pub fn build(self) -> Result<CreateInvoiceTagsRequest<'a>, &'static str> {
        Ok(CreateInvoiceTagsRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
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
pub struct CreateMigrationInvoiceRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::InvoiceItem>,
    pub(crate) target_date: Option<String>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateMigrationInvoiceRequest<'a> {
    pub fn builder() -> CreateMigrationInvoiceRequestBuilder<'a> {
        CreateMigrationInvoiceRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateMigrationInvoiceRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::InvoiceItem>>,
    target_date: Option<String>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateMigrationInvoiceRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: Vec<models::InvoiceItem>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn target_date(mut self, target_date: impl Into<String>) -> Self {
        self.target_date = Some(target_date.into());
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

    pub fn build(self) -> Result<CreateMigrationInvoiceRequest<'a>, &'static str> {
        Ok(CreateMigrationInvoiceRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            body: self.body.ok_or("body is required")?,
            target_date: self.target_date,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}
#[derive(Debug, Clone)]
pub struct DeleteInvoiceCustomFieldsRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) custom_field: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteInvoiceCustomFieldsRequest<'a> {
    pub fn builder() -> DeleteInvoiceCustomFieldsRequestBuilder<'a> {
        DeleteInvoiceCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteInvoiceCustomFieldsRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    custom_field: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteInvoiceCustomFieldsRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
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

    pub fn build(self) -> Result<DeleteInvoiceCustomFieldsRequest<'a>, &'static str> {
        Ok(DeleteInvoiceCustomFieldsRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
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
pub struct DeleteInvoiceTagsRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) tag_def: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteInvoiceTagsRequest<'a> {
    pub fn builder() -> DeleteInvoiceTagsRequestBuilder<'a> {
        DeleteInvoiceTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteInvoiceTagsRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    tag_def: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteInvoiceTagsRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
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

    pub fn build(self) -> Result<DeleteInvoiceTagsRequest<'a>, &'static str> {
        Ok(DeleteInvoiceTagsRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
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
pub struct GetInvoiceRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) with_children_items: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoiceRequest<'a> {
    pub fn builder() -> GetInvoiceRequestBuilder<'a> {
        GetInvoiceRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoiceRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    with_children_items: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoiceRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    pub fn with_children_items(mut self, with_children_items: bool) -> Self {
        self.with_children_items = Some(with_children_items);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetInvoiceRequest<'a>, &'static str> {
        Ok(GetInvoiceRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
            with_children_items: self.with_children_items,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateTaxItemsRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::InvoiceItem>,
    pub(crate) auto_commit: Option<bool>,
    pub(crate) requested_date: Option<String>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateTaxItemsRequest<'a> {
    pub fn builder() -> CreateTaxItemsRequestBuilder<'a> {
        CreateTaxItemsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateTaxItemsRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::InvoiceItem>>,
    auto_commit: Option<bool>,
    requested_date: Option<String>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateTaxItemsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: Vec<models::InvoiceItem>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn auto_commit(mut self, auto_commit: bool) -> Self {
        self.auto_commit = Some(auto_commit);
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

    pub fn build(self) -> Result<CreateTaxItemsRequest<'a>, &'static str> {
        Ok(CreateTaxItemsRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            body: self.body.ok_or("body is required")?,
            auto_commit: self.auto_commit,
            requested_date: self.requested_date,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteCbaRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) invoice_item_id: &'a str,
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteCbaRequest<'a> {
    pub fn builder() -> DeleteCbaRequestBuilder<'a> {
        DeleteCbaRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteCbaRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    invoice_item_id: Option<&'a str>,
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteCbaRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    pub fn invoice_item_id(mut self, invoice_item_id: &'a str) -> Self {
        self.invoice_item_id = Some(invoice_item_id);
        self
    }

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

    pub fn build(self) -> Result<DeleteCbaRequest<'a>, &'static str> {
        Ok(DeleteCbaRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
            invoice_item_id: self.invoice_item_id.ok_or("invoice_item_id is required")?,
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GenerateDryRunInvoiceRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::InvoiceDryRun,
    pub(crate) target_date: Option<String>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> GenerateDryRunInvoiceRequest<'a> {
    pub fn builder() -> GenerateDryRunInvoiceRequestBuilder<'a> {
        GenerateDryRunInvoiceRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GenerateDryRunInvoiceRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::InvoiceDryRun>,
    target_date: Option<String>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> GenerateDryRunInvoiceRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::InvoiceDryRun) -> Self {
        self.body = Some(body);
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

    pub fn build(self) -> Result<GenerateDryRunInvoiceRequest<'a>, &'static str> {
        Ok(GenerateDryRunInvoiceRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            body: self.body.ok_or("body is required")?,
            target_date: self.target_date,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetCatalogTranslationRequest<'a> {
    pub(crate) locale: &'a str,
}

impl<'a> GetCatalogTranslationRequest<'a> {
    pub fn builder() -> GetCatalogTranslationRequestBuilder<'a> {
        GetCatalogTranslationRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetCatalogTranslationRequestBuilder<'a> {
    locale: Option<&'a str>,
}

impl<'a> GetCatalogTranslationRequestBuilder<'a> {
    pub fn locale(mut self, locale: &'a str) -> Self {
        self.locale = Some(locale);
        self
    }

    pub fn build(self) -> Result<GetCatalogTranslationRequest<'a>, &'static str> {
        Ok(GetCatalogTranslationRequest {
            locale: self.locale.ok_or("locale is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoiceAsHtmlRequest<'a> {
    pub(crate) invoice_id: &'a str,
}

impl<'a> GetInvoiceAsHtmlRequest<'a> {
    pub fn builder() -> GetInvoiceAsHtmlRequestBuilder<'a> {
        GetInvoiceAsHtmlRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoiceAsHtmlRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
}

impl<'a> GetInvoiceAsHtmlRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    pub fn build(self) -> Result<GetInvoiceAsHtmlRequest<'a>, &'static str> {
        Ok(GetInvoiceAsHtmlRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoiceAuditLogsWithHistoryRequest<'a> {
    pub(crate) invoice_id: &'a str,
}

impl<'a> GetInvoiceAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetInvoiceAuditLogsWithHistoryRequestBuilder<'a> {
        GetInvoiceAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoiceAuditLogsWithHistoryRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
}

impl<'a> GetInvoiceAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    pub fn build(self) -> Result<GetInvoiceAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetInvoiceAuditLogsWithHistoryRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoiceByItemIdRequest<'a> {
    pub(crate) item_id: &'a str,
    pub(crate) with_children_items: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoiceByItemIdRequest<'a> {
    pub fn builder() -> GetInvoiceByItemIdRequestBuilder<'a> {
        GetInvoiceByItemIdRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoiceByItemIdRequestBuilder<'a> {
    item_id: Option<&'a str>,
    with_children_items: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoiceByItemIdRequestBuilder<'a> {
    pub fn item_id(mut self, item_id: &'a str) -> Self {
        self.item_id = Some(item_id);
        self
    }

    pub fn with_children_items(mut self, with_children_items: bool) -> Self {
        self.with_children_items = Some(with_children_items);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetInvoiceByItemIdRequest<'a>, &'static str> {
        Ok(GetInvoiceByItemIdRequest {
            item_id: self.item_id.ok_or("item_id is required")?,
            with_children_items: self.with_children_items,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoiceByNumberRequest<'a> {
    pub(crate) invoice_number: i32,
    pub(crate) with_children_items: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoiceByNumberRequest<'a> {
    pub fn builder() -> GetInvoiceByNumberRequestBuilder<'a> {
        GetInvoiceByNumberRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoiceByNumberRequestBuilder<'a> {
    invoice_number: Option<i32>,
    with_children_items: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoiceByNumberRequestBuilder<'a> {
    pub fn invoice_number(mut self, invoice_number: i32) -> Self {
        self.invoice_number = Some(invoice_number);
        self
    }

    pub fn with_children_items(mut self, with_children_items: bool) -> Self {
        self.with_children_items = Some(with_children_items);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetInvoiceByNumberRequest<'a>, &'static str> {
        Ok(GetInvoiceByNumberRequest {
            invoice_number: self.invoice_number.ok_or("invoice_number is required")?,
            with_children_items: self.with_children_items,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoiceCustomFieldsRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoiceCustomFieldsRequest<'a> {
    pub fn builder() -> GetInvoiceCustomFieldsRequestBuilder<'a> {
        GetInvoiceCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoiceCustomFieldsRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoiceCustomFieldsRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetInvoiceCustomFieldsRequest<'a>, &'static str> {
        Ok(GetInvoiceCustomFieldsRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoiceMpTemplateRequest<'a> {
    pub(crate) locale: &'a str,
}

impl<'a> GetInvoiceMpTemplateRequest<'a> {
    pub fn builder() -> GetInvoiceMpTemplateRequestBuilder<'a> {
        GetInvoiceMpTemplateRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoiceMpTemplateRequestBuilder<'a> {
    locale: Option<&'a str>,
}

impl<'a> GetInvoiceMpTemplateRequestBuilder<'a> {
    pub fn locale(mut self, locale: &'a str) -> Self {
        self.locale = Some(locale);
        self
    }

    pub fn build(self) -> Result<GetInvoiceMpTemplateRequest<'a>, &'static str> {
        Ok(GetInvoiceMpTemplateRequest {
            locale: self.locale.ok_or("locale is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoiceTagsRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) included_deleted: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoiceTagsRequest<'a> {
    pub fn builder() -> GetInvoiceTagsRequestBuilder<'a> {
        GetInvoiceTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoiceTagsRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    included_deleted: Option<bool>,
    plugin_property: Option<Vec<String>>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoiceTagsRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
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

    pub fn build(self) -> Result<GetInvoiceTagsRequest<'a>, &'static str> {
        Ok(GetInvoiceTagsRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
            included_deleted: self.included_deleted,
            plugin_property: self.plugin_property,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoiceTranslationRequest<'a> {
    pub(crate) locale: &'a str,
}

impl<'a> GetInvoiceTranslationRequest<'a> {
    pub fn builder() -> GetInvoiceTranslationRequestBuilder<'a> {
        GetInvoiceTranslationRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoiceTranslationRequestBuilder<'a> {
    locale: Option<&'a str>,
}

impl<'a> GetInvoiceTranslationRequestBuilder<'a> {
    pub fn locale(mut self, locale: &'a str) -> Self {
        self.locale = Some(locale);
        self
    }

    pub fn build(self) -> Result<GetInvoiceTranslationRequest<'a>, &'static str> {
        Ok(GetInvoiceTranslationRequest {
            locale: self.locale.ok_or("locale is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoicesRequest<'a> {
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoicesRequest<'a> {
    pub fn builder() -> GetInvoicesRequestBuilder<'a> {
        GetInvoicesRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoicesRequestBuilder<'a> {
    offset: Option<i64>,
    limit: Option<i64>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoicesRequestBuilder<'a> {
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

    pub fn build(self) -> Result<GetInvoicesRequest<'a>, &'static str> {
        Ok(GetInvoicesRequest {
            offset: self.offset,
            limit: self.limit,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoicesGroupRequest<'a> {
    pub(crate) group_id: &'a str,
    pub(crate) account_id: &'a str,
    pub(crate) with_children_items: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetInvoicesGroupRequest<'a> {
    pub fn builder() -> GetInvoicesGroupRequestBuilder<'a> {
        GetInvoicesGroupRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetInvoicesGroupRequestBuilder<'a> {
    group_id: Option<&'a str>,
    account_id: Option<&'a str>,
    with_children_items: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetInvoicesGroupRequestBuilder<'a> {
    pub fn group_id(mut self, group_id: &'a str) -> Self {
        self.group_id = Some(group_id);
        self
    }

    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn with_children_items(mut self, with_children_items: bool) -> Self {
        self.with_children_items = Some(with_children_items);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetInvoicesGroupRequest<'a>, &'static str> {
        Ok(GetInvoicesGroupRequest {
            group_id: self.group_id.ok_or("group_id is required")?,
            account_id: self.account_id.ok_or("account_id is required")?,
            with_children_items: self.with_children_items,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPaymentsForInvoiceRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) with_plugin_info: Option<bool>,
    pub(crate) with_attempts: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetPaymentsForInvoiceRequest<'a> {
    pub fn builder() -> GetPaymentsForInvoiceRequestBuilder<'a> {
        GetPaymentsForInvoiceRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPaymentsForInvoiceRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    with_plugin_info: Option<bool>,
    with_attempts: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetPaymentsForInvoiceRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
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

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetPaymentsForInvoiceRequest<'a>, &'static str> {
        Ok(GetPaymentsForInvoiceRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
            with_plugin_info: self.with_plugin_info,
            with_attempts: self.with_attempts,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModifyInvoiceCustomFieldsRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyInvoiceCustomFieldsRequest<'a> {
    pub fn builder() -> ModifyInvoiceCustomFieldsRequestBuilder<'a> {
        ModifyInvoiceCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ModifyInvoiceCustomFieldsRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyInvoiceCustomFieldsRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
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

    pub fn build(self) -> Result<ModifyInvoiceCustomFieldsRequest<'a>, &'static str> {
        Ok(ModifyInvoiceCustomFieldsRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
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
pub struct SearchInvoicesRequest<'a> {
    pub(crate) search_key: &'a str,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> SearchInvoicesRequest<'a> {
    pub fn builder() -> SearchInvoicesRequestBuilder<'a> {
        SearchInvoicesRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SearchInvoicesRequestBuilder<'a> {
    search_key: Option<&'a str>,
    offset: Option<i64>,
    limit: Option<i64>,
    audit: Option<&'a str>,
}

impl<'a> SearchInvoicesRequestBuilder<'a> {
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

    pub fn build(self) -> Result<SearchInvoicesRequest<'a>, &'static str> {
        Ok(SearchInvoicesRequest {
            search_key: self.search_key.ok_or("search_key is required")?,
            offset: self.offset,
            limit: self.limit,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UploadCatalogTranslationRequest<'a> {
    pub(crate) locale: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: &'a str,
    pub(crate) delete_if_exists: Option<bool>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadCatalogTranslationRequest<'a> {
    pub fn builder() -> UploadCatalogTranslationRequestBuilder<'a> {
        UploadCatalogTranslationRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UploadCatalogTranslationRequestBuilder<'a> {
    locale: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<&'a str>,
    delete_if_exists: Option<bool>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadCatalogTranslationRequestBuilder<'a> {
    pub fn locale(mut self, locale: &'a str) -> Self {
        self.locale = Some(locale);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: &'a str) -> Self {
        self.body = Some(body);
        self
    }

    pub fn delete_if_exists(mut self, delete_if_exists: bool) -> Self {
        self.delete_if_exists = Some(delete_if_exists);
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

    pub fn build(self) -> Result<UploadCatalogTranslationRequest<'a>, &'static str> {
        Ok(UploadCatalogTranslationRequest {
            locale: self.locale.ok_or("locale is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            delete_if_exists: self.delete_if_exists,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UploadInvoiceMpTemplateRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: &'a str,
    pub(crate) delete_if_exists: Option<bool>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadInvoiceMpTemplateRequest<'a> {
    pub fn builder() -> UploadInvoiceMpTemplateRequestBuilder<'a> {
        UploadInvoiceMpTemplateRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UploadInvoiceMpTemplateRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<&'a str>,
    delete_if_exists: Option<bool>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadInvoiceMpTemplateRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: &'a str) -> Self {
        self.body = Some(body);
        self
    }

    pub fn delete_if_exists(mut self, delete_if_exists: bool) -> Self {
        self.delete_if_exists = Some(delete_if_exists);
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

    pub fn build(self) -> Result<UploadInvoiceMpTemplateRequest<'a>, &'static str> {
        Ok(UploadInvoiceMpTemplateRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            delete_if_exists: self.delete_if_exists,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UploadInvoiceTemplateRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: &'a str,
    pub(crate) delete_if_exists: Option<bool>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadInvoiceTemplateRequest<'a> {
    pub fn builder() -> UploadInvoiceTemplateRequestBuilder<'a> {
        UploadInvoiceTemplateRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UploadInvoiceTemplateRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<&'a str>,
    delete_if_exists: Option<bool>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadInvoiceTemplateRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: &'a str) -> Self {
        self.body = Some(body);
        self
    }

    pub fn delete_if_exists(mut self, delete_if_exists: bool) -> Self {
        self.delete_if_exists = Some(delete_if_exists);
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

    pub fn build(self) -> Result<UploadInvoiceTemplateRequest<'a>, &'static str> {
        Ok(UploadInvoiceTemplateRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            delete_if_exists: self.delete_if_exists,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UploadInvoiceTranslationRequest<'a> {
    pub(crate) locale: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: &'a str,
    pub(crate) delete_if_exists: Option<bool>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadInvoiceTranslationRequest<'a> {
    pub fn builder() -> UploadInvoiceTranslationRequestBuilder<'a> {
        UploadInvoiceTranslationRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UploadInvoiceTranslationRequestBuilder<'a> {
    locale: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<&'a str>,
    delete_if_exists: Option<bool>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadInvoiceTranslationRequestBuilder<'a> {
    pub fn locale(mut self, locale: &'a str) -> Self {
        self.locale = Some(locale);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: &'a str) -> Self {
        self.body = Some(body);
        self
    }

    pub fn delete_if_exists(mut self, delete_if_exists: bool) -> Self {
        self.delete_if_exists = Some(delete_if_exists);
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

    pub fn build(self) -> Result<UploadInvoiceTranslationRequest<'a>, &'static str> {
        Ok(UploadInvoiceTranslationRequest {
            locale: self.locale.ok_or("locale is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            delete_if_exists: self.delete_if_exists,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct VoidInvoiceRequest<'a> {
    pub(crate) invoice_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> VoidInvoiceRequest<'a> {
    pub fn builder() -> VoidInvoiceRequestBuilder<'a> {
        VoidInvoiceRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct VoidInvoiceRequestBuilder<'a> {
    invoice_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> VoidInvoiceRequestBuilder<'a> {
    pub fn invoice_id(mut self, invoice_id: &'a str) -> Self {
        self.invoice_id = Some(invoice_id);
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

    pub fn build(self) -> Result<VoidInvoiceRequest<'a>, &'static str> {
        Ok(VoidInvoiceRequest {
            invoice_id: self.invoice_id.ok_or("invoice_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InvoiceApiError {
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

impl InvoiceApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{
        InvoiceItem,
        Invoice,
        CustomField,
        Tag,
        AuditLog,
        InvoicePayment,
        PaymentTransaction,
    };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_adjust_invoice_item() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices/test-invoice-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"invoice_id": "test-invoice-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = AdjustInvoiceItemRequest::builder()
            .invoice_id("test-invoice-id")
            .x_killbill_created_by("test")
            .body(InvoiceItem {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.adjust_invoice_item(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_commit_invoice() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/invoices/test-invoice-id/commitInvoice")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = CommitInvoiceRequest::builder()
            .invoice_id("test-invoice-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.commit_invoice(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_external_charges() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices/charges/test-account-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"invoice_item_id": "test-invoice-item-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = CreateExternalChargesRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .body(
                vec![InvoiceItem {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_external_charges(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_future_invoice() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices?accountId=test-account-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"invoice_id": "test-invoice-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = CreateFutureInvoiceRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.create_future_invoice(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_future_invoice_group() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices/group?accountId=test-account-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"invoice_id": "test-invoice-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = CreateFutureInvoiceGroupRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.create_future_invoice_group(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_instant_payment() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices/test-invoice-id/payments")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"payment_id": "test-payment-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = CreateInstantPaymentRequest::builder()
            .invoice_id("test-invoice-id")
            .x_killbill_created_by("test")
            .body(InvoicePayment {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.create_instant_payment(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_invoice_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices/test-invoice-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = CreateInvoiceCustomFieldsRequest::builder()
            .invoice_id("test-invoice-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_invoice_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_invoice_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices/test-invoice-id/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tag_definition_id": "test-tag-def-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = CreateInvoiceTagsRequest::builder()
            .invoice_id("test-invoice-id")
            .x_killbill_created_by("test")
            .body(vec![Uuid::new_v4()])
            .build()
            .unwrap();

        let result = api.create_invoice_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_migration_invoice() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices/migration/test-account-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"invoice_id": "test-invoice-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = CreateMigrationInvoiceRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .body(
                vec![InvoiceItem {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_migration_invoice(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_tax_items() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices/taxes/test-account-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"invoice_item_id": "test-invoice-item-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = CreateTaxItemsRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .body(
                vec![InvoiceItem {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_tax_items(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_cba() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock(
                "DELETE",
                "/1.0/kb/invoices/test-invoice-id/test-invoice-item-id/cba?accountId=test-account-id"
            )
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = DeleteCbaRequest::builder()
            .invoice_id("test-invoice-id")
            .invoice_item_id("test-invoice-item-id")
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_cba(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_invoice_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/invoices/test-invoice-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = DeleteInvoiceCustomFieldsRequest::builder()
            .invoice_id("test-invoice-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_invoice_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_invoice_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/invoices/test-invoice-id/tags")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = DeleteInvoiceTagsRequest::builder()
            .invoice_id("test-invoice-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_invoice_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_generate_dry_run_invoice() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices/dryRun?accountId=test-account-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"invoice_id": "test-invoice-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GenerateDryRunInvoiceRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .body(models::InvoiceDryRun {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.generate_dry_run_invoice(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_catalog_translation() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/catalogTranslation/fr_CA")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#""test-translation""#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GetCatalogTranslationRequest::builder().locale("fr_CA").build().unwrap();

        let result = api.get_catalog_translation(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/test-invoice-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"invoice_id": "test-invoice-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GetInvoiceRequest::builder().invoice_id("test-invoice-id").build().unwrap();

        let result = api.get_invoice(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_as_html() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/test-invoice-id/html")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body>test</body></html>")
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GetInvoiceAsHtmlRequest::builder()
            .invoice_id("test-invoice-id")
            .build()
            .unwrap();

        let result = api.get_invoice_as_html(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/test-invoice-id/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"change_type": "test-change-type"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GetInvoiceAuditLogsWithHistoryRequest::builder()
            .invoice_id("test-invoice-id")
            .build()
            .unwrap();

        let result = api.get_invoice_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_by_item_id() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/byItemId/test-item-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"invoice_id": "test-invoice-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GetInvoiceByItemIdRequest::builder().item_id("test-item-id").build().unwrap();

        let result = api.get_invoice_by_item_id(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_by_number() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/byNumber/1234")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"invoice_id": "test-invoice-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GetInvoiceByNumberRequest::builder().invoice_number(1234).build().unwrap();

        let result = api.get_invoice_by_number(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/test-invoice-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GetInvoiceCustomFieldsRequest::builder()
            .invoice_id("test-invoice-id")
            .build()
            .unwrap();

        let result = api.get_invoice_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_mp_template() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/manualPayTemplate/fr_CA")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body>test</body></html>")
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GetInvoiceMpTemplateRequest::builder().locale("fr_CA").build().unwrap();

        let result = api.get_invoice_mp_template(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/test-invoice-id/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tag_definition_id": "test-tag-def-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GetInvoiceTagsRequest::builder()
            .invoice_id("test-invoice-id")
            .build()
            .unwrap();

        let result = api.get_invoice_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_template() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/template")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body>test</body></html>")
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let result = api.get_invoice_template().await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoice_translation() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/translation/fr_CA")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#""test-translation""#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GetInvoiceTranslationRequest::builder().locale("fr_CA").build().unwrap();

        let result = api.get_invoice_translation(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoices() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/pagination")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"invoice_id": "test-invoice-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GetInvoicesRequest::builder().build().unwrap();

        let result = api.get_invoices(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_invoices_group() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/test-group-id/group?accountId=test-account-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"invoice_id": "test-invoice-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GetInvoicesGroupRequest::builder()
            .group_id("test-group-id")
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.get_invoices_group(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_payments_for_invoice() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/test-invoice-id/payments")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"payment_id": "test-payment-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = GetPaymentsForInvoiceRequest::builder()
            .invoice_id("test-invoice-id")
            .build()
            .unwrap();

        let result = api.get_payments_for_invoice(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_modify_invoice_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/invoices/test-invoice-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = ModifyInvoiceCustomFieldsRequest::builder()
            .invoice_id("test-invoice-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.modify_invoice_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_search_invoices() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/invoices/search/test-search-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"invoice_id": "test-invoice-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = SearchInvoicesRequest::builder()
            .search_key("test-search-key")
            .build()
            .unwrap();

        let result = api.search_invoices(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_upload_catalog_translation() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices/catalogTranslation/fr_CA")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#""test-translation""#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = UploadCatalogTranslationRequest::builder()
            .locale("fr_CA")
            .x_killbill_created_by("test")
            .body("test-translation")
            .build()
            .unwrap();

        let result = api.upload_catalog_translation(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_upload_invoice_mp_template() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices/manualPayTemplate")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body>test</body></html>")
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = UploadInvoiceMpTemplateRequest::builder()
            .x_killbill_created_by("test")
            .body("<html><body>test</body></html>")
            .build()
            .unwrap();

        let result = api.upload_invoice_mp_template(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_upload_invoice_template() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices/template")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body>test</body></html>")
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = UploadInvoiceTemplateRequest::builder()
            .x_killbill_created_by("test")
            .body("<html><body>test</body></html>")
            .build()
            .unwrap();

        let result = api.upload_invoice_template(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_upload_invoice_translation() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/invoices/translation/fr_CA")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#""test-translation""#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = UploadInvoiceTranslationRequest::builder()
            .locale("fr_CA")
            .x_killbill_created_by("test")
            .body("test-translation")
            .build()
            .unwrap();

        let result = api.upload_invoice_translation(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_void_invoice() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/invoices/test-invoice-id/voidInvoice")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = InvoiceApi::new(config);
        let request = VoidInvoiceRequest::builder()
            .invoice_id("test-invoice-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.void_invoice(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
