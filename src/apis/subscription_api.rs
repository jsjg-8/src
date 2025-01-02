use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;
use uuid::Uuid;

pub struct SubscriptionApi {
    config: Configuration,
}

impl SubscriptionApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn add_subscription_blocking_state(
        &self,
        request: AddSubscriptionBlockingStateRequest<'_>
    ) -> Result<Vec<models::BlockingState>, SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}/block",
            self.config.base_path,
            request.subscription_id
        );

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

    pub async fn cancel_subscription_plan(
        &self,
        request: CancelSubscriptionPlanRequest<'_>
    ) -> Result<(), SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}",
            self.config.base_path,
            request.subscription_id
        );

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("requestedDate", request.requested_date),
                    ("callCompletion", request.call_completion.map(|b| b.to_string())),
                    ("callTimeoutSec", request.call_timeout_sec.map(|s| s.to_string())),
                    ("entitlementPolicy", request.entitlement_policy.map(|s| s.to_string())),
                    ("billingPolicy", request.billing_policy.map(|s| s.to_string())),
                    (
                        "useRequestedDateForBilling",
                        request.use_requested_date_for_billing.map(|b| b.to_string()),
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

    pub async fn change_subscription_plan(
        &self,
        request: ChangeSubscriptionPlanRequest<'_>
    ) -> Result<(), SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}",
            self.config.base_path,
            request.subscription_id
        );

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("requestedDate", request.requested_date),
                    ("callCompletion", request.call_completion.map(|b| b.to_string())),
                    ("callTimeoutSec", request.call_timeout_sec.map(|s| s.to_string())),
                    ("billingPolicy", request.billing_policy.map(|s| s.to_string())),
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
        Self::handle_empty_response(response).await
    }

    pub async fn create_subscription(
        &self,
        request: CreateSubscriptionRequest<'_>
    ) -> Result<models::Subscription, SubscriptionApiError> {
        let url = format!("{}/1.0/kb/subscriptions", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("entitlementDate", request.entitlement_date),
                    ("billingDate", request.billing_date),
                    (
                        "renameKeyIfExistsAndUnused",
                        request.rename_key_if_exists_and_unused.map(|b| b.to_string()),
                    ),
                    ("migrated", request.migrated.map(|b| b.to_string())),
                    ("skipResponse", request.skip_response.map(|b| b.to_string())),
                    ("callCompletion", request.call_completion.map(|b| b.to_string())),
                    ("callTimeoutSec", request.call_timeout_sec.map(|s| s.to_string())),
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

    pub async fn create_subscription_custom_fields(
        &self,
        request: CreateSubscriptionCustomFieldsRequest<'_>
    ) -> Result<(), SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}/customFields",
            self.config.base_path,
            request.subscription_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn create_subscription_tags(
        &self,
        request: CreateSubscriptionTagsRequest<'_>
    ) -> Result<(), SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}/tags",
            self.config.base_path,
            request.subscription_id
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn create_subscription_with_add_ons(
        &self,
        request: CreateSubscriptionWithAddOnsRequest<'_>
    ) -> Result<models::Bundle, SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/createSubscriptionWithAddOns",
            self.config.base_path
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("entitlementDate", request.entitlement_date),
                    ("billingDate", request.billing_date),
                    ("migrated", request.migrated.map(|b| b.to_string())),
                    ("skipResponse", request.skip_response.map(|b| b.to_string())),
                    (
                        "renameKeyIfExistsAndUnused",
                        request.rename_key_if_exists_and_unused.map(|b| b.to_string()),
                    ),
                    ("callCompletion", request.call_completion.map(|b| b.to_string())),
                    ("callTimeoutSec", request.call_timeout_sec.map(|s| s.to_string())),
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

    pub async fn create_subscriptions_with_add_ons(
        &self,
        request: CreateSubscriptionsWithAddOnsRequest<'_>
    ) -> Result<Vec<models::Bundle>, SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/createSubscriptionsWithAddOns",
            self.config.base_path
        );

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("entitlementDate", request.entitlement_date),
                    ("billingDate", request.billing_date),
                    ("migrated", request.migrated.map(|b| b.to_string())),
                    ("skipResponse", request.skip_response.map(|b| b.to_string())),
                    (
                        "renameKeyIfExistsAndUnused",
                        request.rename_key_if_exists_and_unused.map(|b| b.to_string()),
                    ),
                    ("callCompletion", request.call_completion.map(|b| b.to_string())),
                    ("callTimeoutSec", request.call_timeout_sec.map(|s| s.to_string())),
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

    pub async fn delete_subscription_custom_fields(
        &self,
        request: DeleteSubscriptionCustomFieldsRequest<'_>
    ) -> Result<(), SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}/customFields",
            self.config.base_path,
            request.subscription_id
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

    pub async fn delete_subscription_tags(
        &self,
        request: DeleteSubscriptionTagsRequest<'_>
    ) -> Result<(), SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}/tags",
            self.config.base_path,
            request.subscription_id
        );

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

    pub async fn get_subscription(
        &self,
        request: GetSubscriptionRequest<'_>
    ) -> Result<models::Subscription, SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}",
            self.config.base_path,
            request.subscription_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("audit", request.audit)]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_subscription_audit_logs_with_history(
        &self,
        request: GetSubscriptionAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}/auditLogsWithHistory",
            self.config.base_path,
            request.subscription_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_subscription_by_key(
        &self,
        request: GetSubscriptionByKeyRequest<'_>
    ) -> Result<models::Subscription, SubscriptionApiError> {
        let url = format!("{}/1.0/kb/subscriptions", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("externalKey", request.external_key),
                    ("audit", request.audit.unwrap_or_default()),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_subscription_custom_fields(
        &self,
        request: GetSubscriptionCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}/customFields",
            self.config.base_path,
            request.subscription_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("audit", request.audit)]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_subscription_event_audit_logs_with_history(
        &self,
        request: GetSubscriptionEventAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/events/{}/auditLogsWithHistory",
            self.config.base_path,
            request.event_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_subscription_tags(
        &self,
        request: GetSubscriptionTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}/tags",
            self.config.base_path,
            request.subscription_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("includedDeleted", request.included_deleted.map(|b| b.to_string())),
                    ("audit", request.audit.map(|s| s.to_string())),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn modify_subscription_custom_fields(
        &self,
        request: ModifySubscriptionCustomFieldsRequest<'_>
    ) -> Result<(), SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}/customFields",
            self.config.base_path,
            request.subscription_id
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

    pub async fn uncancel_subscription_plan(
        &self,
        request: UncancelSubscriptionPlanRequest<'_>
    ) -> Result<(), SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}/uncancel",
            self.config.base_path,
            request.subscription_id
        );

        let req = self.config.client
            .request(Method::PUT, &url)
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
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn undo_change_subscription_plan(
        &self,
        request: UndoChangeSubscriptionPlanRequest<'_>
    ) -> Result<(), SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}/undoChangePlan",
            self.config.base_path,
            request.subscription_id
        );

        let req = self.config.client
            .request(Method::PUT, &url)
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
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn update_subscription_bcd(
        &self,
        request: UpdateSubscriptionBcdRequest<'_>
    ) -> Result<(), SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}/bcd",
            self.config.base_path,
            request.subscription_id
        );

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("effectiveFromDate", request.effective_from_date),
                    (
                        "forceNewBcdWithPastEffectiveDate",
                        request.force_new_bcd_with_past_effective_date.map(|b| b.to_string()),
                    ),
                ]
            )
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .json(&request.body);

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn update_subscription_quantity(
        &self,
        request: UpdateSubscriptionQuantityRequest<'_>
    ) -> Result<(), SubscriptionApiError> {
        let url = format!(
            "{}/1.0/kb/subscriptions/{}/quantity",
            self.config.base_path,
            request.subscription_id
        );

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("effectiveFromDate", request.effective_from_date),
                    (
                        "forceNewQuantityWithPastEffectiveDate",
                        request.force_new_quantity_with_past_effective_date.map(|b| b.to_string()),
                    ),
                ]
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
    ) -> Result<T, SubscriptionApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(SubscriptionApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(SubscriptionApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), SubscriptionApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(SubscriptionApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddSubscriptionBlockingStateRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::BlockingState,
    pub(crate) requested_date: Option<String>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> AddSubscriptionBlockingStateRequest<'a> {
    pub fn builder() -> AddSubscriptionBlockingStateRequestBuilder<'a> {
        AddSubscriptionBlockingStateRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct AddSubscriptionBlockingStateRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::BlockingState>,
    requested_date: Option<String>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> AddSubscriptionBlockingStateRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
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

    pub fn build(self) -> Result<AddSubscriptionBlockingStateRequest<'a>, &'static str> {
        Ok(AddSubscriptionBlockingStateRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
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
pub struct CancelSubscriptionPlanRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) requested_date: Option<String>,
    pub(crate) call_completion: Option<bool>,
    pub(crate) call_timeout_sec: Option<i64>,
    pub(crate) entitlement_policy: Option<&'a str>,
    pub(crate) billing_policy: Option<&'a str>,
    pub(crate) use_requested_date_for_billing: Option<bool>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CancelSubscriptionPlanRequest<'a> {
    pub fn builder() -> CancelSubscriptionPlanRequestBuilder<'a> {
        CancelSubscriptionPlanRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CancelSubscriptionPlanRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    requested_date: Option<String>,
    call_completion: Option<bool>,
    call_timeout_sec: Option<i64>,
    entitlement_policy: Option<&'a str>,
    billing_policy: Option<&'a str>,
    use_requested_date_for_billing: Option<bool>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CancelSubscriptionPlanRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn requested_date(mut self, requested_date: impl Into<String>) -> Self {
        self.requested_date = Some(requested_date.into());
        self
    }

    pub fn call_completion(mut self, call_completion: bool) -> Self {
        self.call_completion = Some(call_completion);
        self
    }

    pub fn call_timeout_sec(mut self, call_timeout_sec: i64) -> Self {
        self.call_timeout_sec = Some(call_timeout_sec);
        self
    }

    pub fn entitlement_policy(mut self, entitlement_policy: &'a str) -> Self {
        self.entitlement_policy = Some(entitlement_policy);
        self
    }

    pub fn billing_policy(mut self, billing_policy: &'a str) -> Self {
        self.billing_policy = Some(billing_policy);
        self
    }

    pub fn use_requested_date_for_billing(mut self, use_requested_date_for_billing: bool) -> Self {
        self.use_requested_date_for_billing = Some(use_requested_date_for_billing);
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

    pub fn build(self) -> Result<CancelSubscriptionPlanRequest<'a>, &'static str> {
        Ok(CancelSubscriptionPlanRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            requested_date: self.requested_date,
            call_completion: self.call_completion,
            call_timeout_sec: self.call_timeout_sec,
            entitlement_policy: self.entitlement_policy,
            billing_policy: self.billing_policy,
            use_requested_date_for_billing: self.use_requested_date_for_billing,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ChangeSubscriptionPlanRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::Subscription,
    pub(crate) requested_date: Option<String>,
    pub(crate) call_completion: Option<bool>,
    pub(crate) call_timeout_sec: Option<i64>,
    pub(crate) billing_policy: Option<&'a str>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ChangeSubscriptionPlanRequest<'a> {
    pub fn builder() -> ChangeSubscriptionPlanRequestBuilder<'a> {
        ChangeSubscriptionPlanRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ChangeSubscriptionPlanRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::Subscription>,
    requested_date: Option<String>,
    call_completion: Option<bool>,
    call_timeout_sec: Option<i64>,
    billing_policy: Option<&'a str>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ChangeSubscriptionPlanRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::Subscription) -> Self {
        self.body = Some(body);
        self
    }

    pub fn requested_date(mut self, requested_date: impl Into<String>) -> Self {
        self.requested_date = Some(requested_date.into());
        self
    }

    pub fn call_completion(mut self, call_completion: bool) -> Self {
        self.call_completion = Some(call_completion);
        self
    }

    pub fn call_timeout_sec(mut self, call_timeout_sec: i64) -> Self {
        self.call_timeout_sec = Some(call_timeout_sec);
        self
    }

    pub fn billing_policy(mut self, billing_policy: &'a str) -> Self {
        self.billing_policy = Some(billing_policy);
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

    pub fn build(self) -> Result<ChangeSubscriptionPlanRequest<'a>, &'static str> {
        Ok(ChangeSubscriptionPlanRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            requested_date: self.requested_date,
            call_completion: self.call_completion,
            call_timeout_sec: self.call_timeout_sec,
            billing_policy: self.billing_policy,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateSubscriptionRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::Subscription,
    pub(crate) entitlement_date: Option<String>,
    pub(crate) billing_date: Option<String>,
    pub(crate) rename_key_if_exists_and_unused: Option<bool>,
    pub(crate) migrated: Option<bool>,
    pub(crate) skip_response: Option<bool>,
    pub(crate) call_completion: Option<bool>,
    pub(crate) call_timeout_sec: Option<i64>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateSubscriptionRequest<'a> {
    pub fn builder() -> CreateSubscriptionRequestBuilder<'a> {
        CreateSubscriptionRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateSubscriptionRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::Subscription>,
    entitlement_date: Option<String>,
    billing_date: Option<String>,
    rename_key_if_exists_and_unused: Option<bool>,
    migrated: Option<bool>,
    skip_response: Option<bool>,
    call_completion: Option<bool>,
    call_timeout_sec: Option<i64>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateSubscriptionRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::Subscription) -> Self {
        self.body = Some(body);
        self
    }

    pub fn entitlement_date(mut self, entitlement_date: impl Into<String>) -> Self {
        self.entitlement_date = Some(entitlement_date.into());
        self
    }

    pub fn billing_date(mut self, billing_date: impl Into<String>) -> Self {
        self.billing_date = Some(billing_date.into());
        self
    }

    pub fn rename_key_if_exists_and_unused(
        mut self,
        rename_key_if_exists_and_unused: bool
    ) -> Self {
        self.rename_key_if_exists_and_unused = Some(rename_key_if_exists_and_unused);
        self
    }

    pub fn migrated(mut self, migrated: bool) -> Self {
        self.migrated = Some(migrated);
        self
    }

    pub fn skip_response(mut self, skip_response: bool) -> Self {
        self.skip_response = Some(skip_response);
        self
    }

    pub fn call_completion(mut self, call_completion: bool) -> Self {
        self.call_completion = Some(call_completion);
        self
    }

    pub fn call_timeout_sec(mut self, call_timeout_sec: i64) -> Self {
        self.call_timeout_sec = Some(call_timeout_sec);
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

    pub fn build(self) -> Result<CreateSubscriptionRequest<'a>, &'static str> {
        Ok(CreateSubscriptionRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            entitlement_date: self.entitlement_date,
            billing_date: self.billing_date,
            rename_key_if_exists_and_unused: self.rename_key_if_exists_and_unused,
            migrated: self.migrated,
            skip_response: self.skip_response,
            call_completion: self.call_completion,
            call_timeout_sec: self.call_timeout_sec,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateSubscriptionCustomFieldsRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateSubscriptionCustomFieldsRequest<'a> {
    pub fn builder() -> CreateSubscriptionCustomFieldsRequestBuilder<'a> {
        CreateSubscriptionCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateSubscriptionCustomFieldsRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateSubscriptionCustomFieldsRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
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

    pub fn build(self) -> Result<CreateSubscriptionCustomFieldsRequest<'a>, &'static str> {
        Ok(CreateSubscriptionCustomFieldsRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
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
pub struct CreateSubscriptionTagsRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<Uuid>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateSubscriptionTagsRequest<'a> {
    pub fn builder() -> CreateSubscriptionTagsRequestBuilder<'a> {
        CreateSubscriptionTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateSubscriptionTagsRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateSubscriptionTagsRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
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

    pub fn build(self) -> Result<CreateSubscriptionTagsRequest<'a>, &'static str> {
        Ok(CreateSubscriptionTagsRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
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
pub struct CreateSubscriptionWithAddOnsRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::Subscription>,
    pub(crate) entitlement_date: Option<String>,
    pub(crate) billing_date: Option<String>,
    pub(crate) migrated: Option<bool>,
    pub(crate) skip_response: Option<bool>,
    pub(crate) rename_key_if_exists_and_unused: Option<bool>,
    pub(crate) call_completion: Option<bool>,
    pub(crate) call_timeout_sec: Option<i64>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateSubscriptionWithAddOnsRequest<'a> {
    pub fn builder() -> CreateSubscriptionWithAddOnsRequestBuilder<'a> {
        CreateSubscriptionWithAddOnsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateSubscriptionWithAddOnsRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::Subscription>>,
    entitlement_date: Option<String>,
    billing_date: Option<String>,
    migrated: Option<bool>,
    skip_response: Option<bool>,
    rename_key_if_exists_and_unused: Option<bool>,
    call_completion: Option<bool>,
    call_timeout_sec: Option<i64>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateSubscriptionWithAddOnsRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: Vec<models::Subscription>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn entitlement_date(mut self, entitlement_date: impl Into<String>) -> Self {
        self.entitlement_date = Some(entitlement_date.into());
        self
    }

    pub fn billing_date(mut self, billing_date: impl Into<String>) -> Self {
        self.billing_date = Some(billing_date.into());
        self
    }

    pub fn migrated(mut self, migrated: bool) -> Self {
        self.migrated = Some(migrated);
        self
    }

    pub fn skip_response(mut self, skip_response: bool) -> Self {
        self.skip_response = Some(skip_response);
        self
    }

    pub fn rename_key_if_exists_and_unused(
        mut self,
        rename_key_if_exists_and_unused: bool
    ) -> Self {
        self.rename_key_if_exists_and_unused = Some(rename_key_if_exists_and_unused);
        self
    }

    pub fn call_completion(mut self, call_completion: bool) -> Self {
        self.call_completion = Some(call_completion);
        self
    }

    pub fn call_timeout_sec(mut self, call_timeout_sec: i64) -> Self {
        self.call_timeout_sec = Some(call_timeout_sec);
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

    pub fn build(self) -> Result<CreateSubscriptionWithAddOnsRequest<'a>, &'static str> {
        Ok(CreateSubscriptionWithAddOnsRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            entitlement_date: self.entitlement_date,
            billing_date: self.billing_date,
            migrated: self.migrated,
            skip_response: self.skip_response,
            rename_key_if_exists_and_unused: self.rename_key_if_exists_and_unused,
            call_completion: self.call_completion,
            call_timeout_sec: self.call_timeout_sec,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateSubscriptionsWithAddOnsRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::BulkSubscriptionsBundle>,
    pub(crate) entitlement_date: Option<String>,
    pub(crate) billing_date: Option<String>,
    pub(crate) rename_key_if_exists_and_unused: Option<bool>,
    pub(crate) migrated: Option<bool>,
    pub(crate) skip_response: Option<bool>,
    pub(crate) call_completion: Option<bool>,
    pub(crate) call_timeout_sec: Option<i64>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateSubscriptionsWithAddOnsRequest<'a> {
    pub fn builder() -> CreateSubscriptionsWithAddOnsRequestBuilder<'a> {
        CreateSubscriptionsWithAddOnsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateSubscriptionsWithAddOnsRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::BulkSubscriptionsBundle>>,
    entitlement_date: Option<String>,
    billing_date: Option<String>,
    rename_key_if_exists_and_unused: Option<bool>,
    migrated: Option<bool>,
    skip_response: Option<bool>,
    call_completion: Option<bool>,
    call_timeout_sec: Option<i64>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateSubscriptionsWithAddOnsRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: Vec<models::BulkSubscriptionsBundle>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn entitlement_date(mut self, entitlement_date: impl Into<String>) -> Self {
        self.entitlement_date = Some(entitlement_date.into());
        self
    }

    pub fn billing_date(mut self, billing_date: impl Into<String>) -> Self {
        self.billing_date = Some(billing_date.into());
        self
    }

    pub fn rename_key_if_exists_and_unused(
        mut self,
        rename_key_if_exists_and_unused: bool
    ) -> Self {
        self.rename_key_if_exists_and_unused = Some(rename_key_if_exists_and_unused);
        self
    }

    pub fn migrated(mut self, migrated: bool) -> Self {
        self.migrated = Some(migrated);
        self
    }

    pub fn skip_response(mut self, skip_response: bool) -> Self {
        self.skip_response = Some(skip_response);
        self
    }

    pub fn call_completion(mut self, call_completion: bool) -> Self {
        self.call_completion = Some(call_completion);
        self
    }

    pub fn call_timeout_sec(mut self, call_timeout_sec: i64) -> Self {
        self.call_timeout_sec = Some(call_timeout_sec);
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

    pub fn build(self) -> Result<CreateSubscriptionsWithAddOnsRequest<'a>, &'static str> {
        Ok(CreateSubscriptionsWithAddOnsRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            entitlement_date: self.entitlement_date,
            billing_date: self.billing_date,
            rename_key_if_exists_and_unused: self.rename_key_if_exists_and_unused,
            migrated: self.migrated,
            skip_response: self.skip_response,
            call_completion: self.call_completion,
            call_timeout_sec: self.call_timeout_sec,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteSubscriptionCustomFieldsRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) custom_field: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteSubscriptionCustomFieldsRequest<'a> {
    pub fn builder() -> DeleteSubscriptionCustomFieldsRequestBuilder<'a> {
        DeleteSubscriptionCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteSubscriptionCustomFieldsRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    custom_field: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteSubscriptionCustomFieldsRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
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

    pub fn build(self) -> Result<DeleteSubscriptionCustomFieldsRequest<'a>, &'static str> {
        Ok(DeleteSubscriptionCustomFieldsRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
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
pub struct DeleteSubscriptionTagsRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) tag_def: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteSubscriptionTagsRequest<'a> {
    pub fn builder() -> DeleteSubscriptionTagsRequestBuilder<'a> {
        DeleteSubscriptionTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteSubscriptionTagsRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    tag_def: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteSubscriptionTagsRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
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

    pub fn build(self) -> Result<DeleteSubscriptionTagsRequest<'a>, &'static str> {
        Ok(DeleteSubscriptionTagsRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
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
pub struct GetSubscriptionRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetSubscriptionRequest<'a> {
    pub fn builder() -> GetSubscriptionRequestBuilder<'a> {
        GetSubscriptionRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetSubscriptionRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetSubscriptionRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetSubscriptionRequest<'a>, &'static str> {
        Ok(GetSubscriptionRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetSubscriptionAuditLogsWithHistoryRequest<'a> {
    pub(crate) subscription_id: &'a str,
}

impl<'a> GetSubscriptionAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetSubscriptionAuditLogsWithHistoryRequestBuilder<'a> {
        GetSubscriptionAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetSubscriptionAuditLogsWithHistoryRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
}

impl<'a> GetSubscriptionAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn build(self) -> Result<GetSubscriptionAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetSubscriptionAuditLogsWithHistoryRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetSubscriptionByKeyRequest<'a> {
    pub(crate) external_key: &'a str,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetSubscriptionByKeyRequest<'a> {
    pub fn builder() -> GetSubscriptionByKeyRequestBuilder<'a> {
        GetSubscriptionByKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetSubscriptionByKeyRequestBuilder<'a> {
    external_key: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetSubscriptionByKeyRequestBuilder<'a> {
    pub fn external_key(mut self, external_key: &'a str) -> Self {
        self.external_key = Some(external_key);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetSubscriptionByKeyRequest<'a>, &'static str> {
        Ok(GetSubscriptionByKeyRequest {
            external_key: self.external_key.ok_or("external_key is required")?,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetSubscriptionCustomFieldsRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetSubscriptionCustomFieldsRequest<'a> {
    pub fn builder() -> GetSubscriptionCustomFieldsRequestBuilder<'a> {
        GetSubscriptionCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetSubscriptionCustomFieldsRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetSubscriptionCustomFieldsRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetSubscriptionCustomFieldsRequest<'a>, &'static str> {
        Ok(GetSubscriptionCustomFieldsRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetSubscriptionEventAuditLogsWithHistoryRequest<'a> {
    pub(crate) event_id: &'a str,
}

impl<'a> GetSubscriptionEventAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetSubscriptionEventAuditLogsWithHistoryRequestBuilder<'a> {
        GetSubscriptionEventAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetSubscriptionEventAuditLogsWithHistoryRequestBuilder<'a> {
    event_id: Option<&'a str>,
}

impl<'a> GetSubscriptionEventAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn event_id(mut self, event_id: &'a str) -> Self {
        self.event_id = Some(event_id);
        self
    }

    pub fn build(
        self
    ) -> Result<GetSubscriptionEventAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetSubscriptionEventAuditLogsWithHistoryRequest {
            event_id: self.event_id.ok_or("event_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetSubscriptionTagsRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) included_deleted: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetSubscriptionTagsRequest<'a> {
    pub fn builder() -> GetSubscriptionTagsRequestBuilder<'a> {
        GetSubscriptionTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetSubscriptionTagsRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    included_deleted: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetSubscriptionTagsRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
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

    pub fn build(self) -> Result<GetSubscriptionTagsRequest<'a>, &'static str> {
        Ok(GetSubscriptionTagsRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
            included_deleted: self.included_deleted,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModifySubscriptionCustomFieldsRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifySubscriptionCustomFieldsRequest<'a> {
    pub fn builder() -> ModifySubscriptionCustomFieldsRequestBuilder<'a> {
        ModifySubscriptionCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ModifySubscriptionCustomFieldsRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifySubscriptionCustomFieldsRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
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

    pub fn build(self) -> Result<ModifySubscriptionCustomFieldsRequest<'a>, &'static str> {
        Ok(ModifySubscriptionCustomFieldsRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
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
pub struct UncancelSubscriptionPlanRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UncancelSubscriptionPlanRequest<'a> {
    pub fn builder() -> UncancelSubscriptionPlanRequestBuilder<'a> {
        UncancelSubscriptionPlanRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UncancelSubscriptionPlanRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UncancelSubscriptionPlanRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
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

    pub fn build(self) -> Result<UncancelSubscriptionPlanRequest<'a>, &'static str> {
        Ok(UncancelSubscriptionPlanRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UndoChangeSubscriptionPlanRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UndoChangeSubscriptionPlanRequest<'a> {
    pub fn builder() -> UndoChangeSubscriptionPlanRequestBuilder<'a> {
        UndoChangeSubscriptionPlanRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UndoChangeSubscriptionPlanRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UndoChangeSubscriptionPlanRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
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

    pub fn build(self) -> Result<UndoChangeSubscriptionPlanRequest<'a>, &'static str> {
        Ok(UndoChangeSubscriptionPlanRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UpdateSubscriptionBcdRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::Subscription,
    pub(crate) effective_from_date: Option<String>,
    pub(crate) force_new_bcd_with_past_effective_date: Option<bool>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdateSubscriptionBcdRequest<'a> {
    pub fn builder() -> UpdateSubscriptionBcdRequestBuilder<'a> {
        UpdateSubscriptionBcdRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UpdateSubscriptionBcdRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::Subscription>,
    effective_from_date: Option<String>,
    force_new_bcd_with_past_effective_date: Option<bool>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdateSubscriptionBcdRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::Subscription) -> Self {
        self.body = Some(body);
        self
    }

    pub fn effective_from_date(mut self, effective_from_date: impl Into<String>) -> Self {
        self.effective_from_date = Some(effective_from_date.into());
        self
    }

    pub fn force_new_bcd_with_past_effective_date(
        mut self,
        force_new_bcd_with_past_effective_date: bool
    ) -> Self {
        self.force_new_bcd_with_past_effective_date = Some(force_new_bcd_with_past_effective_date);
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

    pub fn build(self) -> Result<UpdateSubscriptionBcdRequest<'a>, &'static str> {
        Ok(UpdateSubscriptionBcdRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            effective_from_date: self.effective_from_date,
            force_new_bcd_with_past_effective_date: self.force_new_bcd_with_past_effective_date,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UpdateSubscriptionQuantityRequest<'a> {
    pub(crate) subscription_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::Subscription,
    pub(crate) effective_from_date: Option<String>,
    pub(crate) force_new_quantity_with_past_effective_date: Option<bool>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdateSubscriptionQuantityRequest<'a> {
    pub fn builder() -> UpdateSubscriptionQuantityRequestBuilder<'a> {
        UpdateSubscriptionQuantityRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UpdateSubscriptionQuantityRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::Subscription>,
    effective_from_date: Option<String>,
    force_new_quantity_with_past_effective_date: Option<bool>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdateSubscriptionQuantityRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::Subscription) -> Self {
        self.body = Some(body);
        self
    }

    pub fn effective_from_date(mut self, effective_from_date: impl Into<String>) -> Self {
        self.effective_from_date = Some(effective_from_date.into());
        self
    }

    pub fn force_new_quantity_with_past_effective_date(
        mut self,
        force_new_quantity_with_past_effective_date: bool
    ) -> Self {
        self.force_new_quantity_with_past_effective_date = Some(
            force_new_quantity_with_past_effective_date
        );
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

    pub fn build(self) -> Result<UpdateSubscriptionQuantityRequest<'a>, &'static str> {
        Ok(UpdateSubscriptionQuantityRequest {
            subscription_id: self.subscription_id.ok_or("subscription_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            effective_from_date: self.effective_from_date,
            force_new_quantity_with_past_effective_date: self.force_new_quantity_with_past_effective_date,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SubscriptionApiError {
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

impl SubscriptionApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{ BlockingState, Subscription, CustomField, Tag, BulkSubscriptionsBundle };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_add_subscription_blocking_state() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/subscriptions/test-subscription/block")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"stateName": "test-state"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = AddSubscriptionBlockingStateRequest::builder()
            .subscription_id("test-subscription")
            .x_killbill_created_by("test")
            .body(BlockingState {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.add_subscription_blocking_state(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_cancel_subscription_plan() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/subscriptions/test-subscription")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = CancelSubscriptionPlanRequest::builder()
            .subscription_id("test-subscription")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.cancel_subscription_plan(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_change_subscription_plan() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/subscriptions/test-subscription")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = ChangeSubscriptionPlanRequest::builder()
            .subscription_id("test-subscription")
            .x_killbill_created_by("test")
            .body(Subscription {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.change_subscription_plan(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_subscription() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/subscriptions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"subscriptionId": "test-subscription"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = CreateSubscriptionRequest::builder()
            .x_killbill_created_by("test")
            .body(Subscription {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.create_subscription(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_subscription_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/subscriptions/test-subscription/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = CreateSubscriptionCustomFieldsRequest::builder()
            .subscription_id("test-subscription")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_subscription_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_subscription_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/subscriptions/test-subscription/tags")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = CreateSubscriptionTagsRequest::builder()
            .subscription_id("test-subscription")
            .x_killbill_created_by("test")
            .body(vec![Uuid::new_v4()])
            .build()
            .unwrap();

        let result = api.create_subscription_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_subscription_with_add_ons() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/subscriptions/createSubscriptionWithAddOns")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"bundleId": "test-bundle"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = CreateSubscriptionWithAddOnsRequest::builder()
            .x_killbill_created_by("test")
            .body(
                vec![Subscription {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_subscription_with_add_ons(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_subscriptions_with_add_ons() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/subscriptions/createSubscriptionsWithAddOns")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"bundleId": "test-bundle"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = CreateSubscriptionsWithAddOnsRequest::builder()
            .x_killbill_created_by("test")
            .body(
                vec![BulkSubscriptionsBundle {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_subscriptions_with_add_ons(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_subscription_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/subscriptions/test-subscription/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = DeleteSubscriptionCustomFieldsRequest::builder()
            .subscription_id("test-subscription")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_subscription_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_subscription_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/subscriptions/test-subscription/tags")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = DeleteSubscriptionTagsRequest::builder()
            .subscription_id("test-subscription")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_subscription_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_subscription() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/subscriptions/test-subscription")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"subscriptionId": "test-subscription"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = GetSubscriptionRequest::builder()
            .subscription_id("test-subscription")
            .build()
            .unwrap();

        let result = api.get_subscription(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_subscription_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/subscriptions/test-subscription/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"changeType": "INSERT"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = GetSubscriptionAuditLogsWithHistoryRequest::builder()
            .subscription_id("test-subscription")
            .build()
            .unwrap();

        let result = api.get_subscription_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_subscription_by_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/subscriptions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"subscriptionId": "test-subscription"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = GetSubscriptionByKeyRequest::builder()
            .external_key("test-subscription")
            .build()
            .unwrap();

        let result = api.get_subscription_by_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_subscription_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/subscriptions/test-subscription/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = GetSubscriptionCustomFieldsRequest::builder()
            .subscription_id("test-subscription")
            .build()
            .unwrap();

        let result = api.get_subscription_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_subscription_event_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/subscriptions/events/test-event/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"changeType": "INSERT"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = GetSubscriptionEventAuditLogsWithHistoryRequest::builder()
            .event_id("test-event")
            .build()
            .unwrap();

        let result = api.get_subscription_event_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_subscription_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/subscriptions/test-subscription/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tagId": "test-tag"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = GetSubscriptionTagsRequest::builder()
            .subscription_id("test-subscription")
            .build()
            .unwrap();

        let result = api.get_subscription_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_modify_subscription_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/subscriptions/test-subscription/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = ModifySubscriptionCustomFieldsRequest::builder()
            .subscription_id("test-subscription")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.modify_subscription_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_uncancel_subscription_plan() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/subscriptions/test-subscription/uncancel")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = UncancelSubscriptionPlanRequest::builder()
            .subscription_id("test-subscription")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.uncancel_subscription_plan(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_undo_change_subscription_plan() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/subscriptions/test-subscription/undoChangePlan")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = UndoChangeSubscriptionPlanRequest::builder()
            .subscription_id("test-subscription")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.undo_change_subscription_plan(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_update_subscription_bcd() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/subscriptions/test-subscription/bcd")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = UpdateSubscriptionBcdRequest::builder()
            .subscription_id("test-subscription")
            .x_killbill_created_by("test")
            .body(Subscription {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.update_subscription_bcd(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_update_subscription_quantity() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/subscriptions/test-subscription/quantity")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = SubscriptionApi::new(config);
        let request = UpdateSubscriptionQuantityRequest::builder()
            .subscription_id("test-subscription")
            .x_killbill_created_by("test")
            .body(Subscription {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.update_subscription_quantity(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
