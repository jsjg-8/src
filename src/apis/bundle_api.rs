use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use uuid::Uuid;
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct BundleApi {
    config: Configuration,
}

impl BundleApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn add_bundle_blocking_state(
        &self,
        request: AddBundleBlockingStateRequest<'_>
    ) -> Result<Vec<models::BlockingState>, BundleApiError> {
        let url = format!("{}/1.0/kb/bundles/{}/block", self.config.base_path, request.bundle_id);

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

    pub async fn create_bundle_custom_fields(
        &self,
        request: CreateBundleCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, BundleApiError> {
        let url = format!(
            "{}/1.0/kb/bundles/{}/customFields",
            self.config.base_path,
            request.bundle_id
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

    pub async fn create_bundle_tags(
        &self,
        request: CreateBundleTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, BundleApiError> {
        let url = format!("{}/1.0/kb/bundles/{}/tags", self.config.base_path, request.bundle_id);

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

    pub async fn delete_bundle_custom_fields(
        &self,
        request: DeleteBundleCustomFieldsRequest<'_>
    ) -> Result<(), BundleApiError> {
        let url = format!(
            "{}/1.0/kb/bundles/{}/customFields",
            self.config.base_path,
            request.bundle_id
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

    pub async fn delete_bundle_tags(
        &self,
        request: DeleteBundleTagsRequest<'_>
    ) -> Result<(), BundleApiError> {
        let url = format!("{}/1.0/kb/bundles/{}/tags", self.config.base_path, request.bundle_id);

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

    pub async fn get_bundle(
        &self,
        request: GetBundleRequest<'_>
    ) -> Result<models::Bundle, BundleApiError> {
        let url = format!("{}/1.0/kb/bundles/{}", self.config.base_path, request.bundle_id);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("audit", request.audit)]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_bundle_audit_logs_with_history(
        &self,
        request: GetBundleAuditLogsWithHistoryRequest<'_>
    ) -> Result<Vec<models::AuditLog>, BundleApiError> {
        let url = format!(
            "{}/1.0/kb/bundles/{}/auditLogsWithHistory",
            self.config.base_path,
            request.bundle_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_bundle_by_key(
        &self,
        request: GetBundleByKeyRequest<'_>
    ) -> Result<Vec<models::Bundle>, BundleApiError> {
        let url = format!("{}/1.0/kb/bundles", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("externalKey", request.external_key),
                    ("includedDeleted", request.included_deleted.map(|b| if b { "true" } else { "false" }).unwrap_or("false")),
                    ("audit", request.audit.unwrap_or("")),
                ]
            );

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_bundle_custom_fields(
        &self,
        request: GetBundleCustomFieldsRequest<'_>
    ) -> Result<Vec<models::CustomField>, BundleApiError> {
        let url = format!(
            "{}/1.0/kb/bundles/{}/customFields",
            self.config.base_path,
            request.bundle_id
        );

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("audit", request.audit)]);

        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_bundle_tags(
        &self,
        request: GetBundleTagsRequest<'_>
    ) -> Result<Vec<models::Tag>, BundleApiError> {
        let url = format!("{}/1.0/kb/bundles/{}/tags", self.config.base_path, request.bundle_id);

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

    pub async fn get_bundles(
        &self,
        request: GetBundlesRequest<'_>
    ) -> Result<Vec<models::Bundle>, BundleApiError> {
        let url = format!("{}/1.0/kb/bundles/pagination", self.config.base_path);

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

    pub async fn modify_bundle_custom_fields(
        &self,
        request: ModifyBundleCustomFieldsRequest<'_>
    ) -> Result<(), BundleApiError> {
        let url = format!(
            "{}/1.0/kb/bundles/{}/customFields",
            self.config.base_path,
            request.bundle_id
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

    pub async fn pause_bundle(
        &self,
        request: PauseBundleRequest<'_>
    ) -> Result<(), BundleApiError> {
        let url = format!("{}/1.0/kb/bundles/{}/pause", self.config.base_path, request.bundle_id);

        let req = self.config.client
            .request(Method::PUT, &url)
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
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn rename_external_key(
        &self,
        request: RenameExternalKeyRequest<'_>
    ) -> Result<(), BundleApiError> {
        let url = format!(
            "{}/1.0/kb/bundles/{}/renameKey",
            self.config.base_path,
            request.bundle_id
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

    pub async fn resume_bundle(
        &self,
        request: ResumeBundleRequest<'_>
    ) -> Result<(), BundleApiError> {
        let url = format!("{}/1.0/kb/bundles/{}/resume", self.config.base_path, request.bundle_id);

        let req = self.config.client
            .request(Method::PUT, &url)
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
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn search_bundles(
        &self,
        request: SearchBundlesRequest<'_>
    ) -> Result<Vec<models::Bundle>, BundleApiError> {
        let url = format!("{}/1.0/kb/bundles/search/{}", self.config.base_path, request.search_key);

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

    pub async fn transfer_bundle(
        &self,
        request: TransferBundleRequest<'_>
    ) -> Result<models::Bundle, BundleApiError> {
        let url = format!("{}/1.0/kb/bundles/{}", self.config.base_path, request.bundle_id);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("requestedDate", request.requested_date),
                    ("billingPolicy", request.billing_policy.map(|l| l.to_string())),
                    ("bcdTransfer", request.bcd_transfer.map(|l| l.to_string())),
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

    async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T, BundleApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(BundleApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(BundleApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), BundleApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(BundleApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddBundleBlockingStateRequest<'a> {
    pub(crate) bundle_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::BlockingState,
    pub(crate) requested_date: Option<String>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> AddBundleBlockingStateRequest<'a> {
    pub fn builder() -> AddBundleBlockingStateRequestBuilder<'a> {
        AddBundleBlockingStateRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct AddBundleBlockingStateRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::BlockingState>,
    requested_date: Option<String>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> AddBundleBlockingStateRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
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

    pub fn build(self) -> Result<AddBundleBlockingStateRequest<'a>, &'static str> {
        Ok(AddBundleBlockingStateRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
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
pub struct CreateBundleCustomFieldsRequest<'a> {
    pub(crate) bundle_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateBundleCustomFieldsRequest<'a> {
    pub fn builder() -> CreateBundleCustomFieldsRequestBuilder<'a> {
        CreateBundleCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateBundleCustomFieldsRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateBundleCustomFieldsRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
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

    pub fn build(self) -> Result<CreateBundleCustomFieldsRequest<'a>, &'static str> {
        Ok(CreateBundleCustomFieldsRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
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
pub struct CreateBundleTagsRequest<'a> {
    pub(crate) bundle_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<Uuid>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateBundleTagsRequest<'a> {
    pub fn builder() -> CreateBundleTagsRequestBuilder<'a> {
        CreateBundleTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateBundleTagsRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> CreateBundleTagsRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
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

    pub fn build(self) -> Result<CreateBundleTagsRequest<'a>, &'static str> {
        Ok(CreateBundleTagsRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
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
pub struct DeleteBundleCustomFieldsRequest<'a> {
    pub(crate) bundle_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) custom_field: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteBundleCustomFieldsRequest<'a> {
    pub fn builder() -> DeleteBundleCustomFieldsRequestBuilder<'a> {
        DeleteBundleCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteBundleCustomFieldsRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    custom_field: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteBundleCustomFieldsRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
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

    pub fn build(self) -> Result<DeleteBundleCustomFieldsRequest<'a>, &'static str> {
        Ok(DeleteBundleCustomFieldsRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
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
pub struct DeleteBundleTagsRequest<'a> {
    pub(crate) bundle_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) tag_def: Option<Vec<Uuid>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteBundleTagsRequest<'a> {
    pub fn builder() -> DeleteBundleTagsRequestBuilder<'a> {
        DeleteBundleTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteBundleTagsRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    tag_def: Option<Vec<Uuid>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteBundleTagsRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
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

    pub fn build(self) -> Result<DeleteBundleTagsRequest<'a>, &'static str> {
        Ok(DeleteBundleTagsRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
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
pub struct GetBundleRequest<'a> {
    pub(crate) bundle_id: &'a str,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetBundleRequest<'a> {
    pub fn builder() -> GetBundleRequestBuilder<'a> {
        GetBundleRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetBundleRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetBundleRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetBundleRequest<'a>, &'static str> {
        Ok(GetBundleRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetBundleAuditLogsWithHistoryRequest<'a> {
    pub(crate) bundle_id: &'a str,
}

impl<'a> GetBundleAuditLogsWithHistoryRequest<'a> {
    pub fn builder() -> GetBundleAuditLogsWithHistoryRequestBuilder<'a> {
        GetBundleAuditLogsWithHistoryRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetBundleAuditLogsWithHistoryRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
}

impl<'a> GetBundleAuditLogsWithHistoryRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
        self
    }

    pub fn build(self) -> Result<GetBundleAuditLogsWithHistoryRequest<'a>, &'static str> {
        Ok(GetBundleAuditLogsWithHistoryRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetBundleByKeyRequest<'a> {
    pub(crate) external_key: &'a str,
    pub(crate) included_deleted: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetBundleByKeyRequest<'a> {
    pub fn builder() -> GetBundleByKeyRequestBuilder<'a> {
        GetBundleByKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetBundleByKeyRequestBuilder<'a> {
    external_key: Option<&'a str>,
    included_deleted: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetBundleByKeyRequestBuilder<'a> {
    pub fn external_key(mut self, external_key: &'a str) -> Self {
        self.external_key = Some(external_key);
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

    pub fn build(self) -> Result<GetBundleByKeyRequest<'a>, &'static str> {
        Ok(GetBundleByKeyRequest {
            external_key: self.external_key.ok_or("external_key is required")?,
            included_deleted: self.included_deleted,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetBundleCustomFieldsRequest<'a> {
    pub(crate) bundle_id: &'a str,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetBundleCustomFieldsRequest<'a> {
    pub fn builder() -> GetBundleCustomFieldsRequestBuilder<'a> {
        GetBundleCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetBundleCustomFieldsRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
    audit: Option<&'a str>,
}

impl<'a> GetBundleCustomFieldsRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
        self
    }

    pub fn audit(mut self, audit: &'a str) -> Self {
        self.audit = Some(audit);
        self
    }

    pub fn build(self) -> Result<GetBundleCustomFieldsRequest<'a>, &'static str> {
        Ok(GetBundleCustomFieldsRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetBundleTagsRequest<'a> {
    pub(crate) bundle_id: &'a str,
    pub(crate) included_deleted: Option<bool>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetBundleTagsRequest<'a> {
    pub fn builder() -> GetBundleTagsRequestBuilder<'a> {
        GetBundleTagsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetBundleTagsRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
    included_deleted: Option<bool>,
    audit: Option<&'a str>,
}

impl<'a> GetBundleTagsRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
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

    pub fn build(self) -> Result<GetBundleTagsRequest<'a>, &'static str> {
        Ok(GetBundleTagsRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
            included_deleted: self.included_deleted,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetBundlesRequest<'a> {
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> GetBundlesRequest<'a> {
    pub fn builder() -> GetBundlesRequestBuilder<'a> {
        GetBundlesRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetBundlesRequestBuilder<'a> {
    offset: Option<i64>,
    limit: Option<i64>,
    audit: Option<&'a str>,
}

impl<'a> GetBundlesRequestBuilder<'a> {
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

    pub fn build(self) -> Result<GetBundlesRequest<'a>, &'static str> {
        Ok(GetBundlesRequest {
            offset: self.offset,
            limit: self.limit,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModifyBundleCustomFieldsRequest<'a> {
    pub(crate) bundle_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: Vec<models::CustomField>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyBundleCustomFieldsRequest<'a> {
    pub fn builder() -> ModifyBundleCustomFieldsRequestBuilder<'a> {
        ModifyBundleCustomFieldsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ModifyBundleCustomFieldsRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<Vec<models::CustomField>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ModifyBundleCustomFieldsRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
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

    pub fn build(self) -> Result<ModifyBundleCustomFieldsRequest<'a>, &'static str> {
        Ok(ModifyBundleCustomFieldsRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
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
pub struct PauseBundleRequest<'a> {
    pub(crate) bundle_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) requested_date: Option<String>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> PauseBundleRequest<'a> {
    pub fn builder() -> PauseBundleRequestBuilder<'a> {
        PauseBundleRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct PauseBundleRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    requested_date: Option<String>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> PauseBundleRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
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

    pub fn build(self) -> Result<PauseBundleRequest<'a>, &'static str> {
        Ok(PauseBundleRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            requested_date: self.requested_date,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RenameExternalKeyRequest<'a> {
    pub(crate) bundle_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::Bundle,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> RenameExternalKeyRequest<'a> {
    pub fn builder() -> RenameExternalKeyRequestBuilder<'a> {
        RenameExternalKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct RenameExternalKeyRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::Bundle>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> RenameExternalKeyRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::Bundle) -> Self {
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

    pub fn build(self) -> Result<RenameExternalKeyRequest<'a>, &'static str> {
        Ok(RenameExternalKeyRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
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
pub struct ResumeBundleRequest<'a> {
    pub(crate) bundle_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) requested_date: Option<String>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ResumeBundleRequest<'a> {
    pub fn builder() -> ResumeBundleRequestBuilder<'a> {
        ResumeBundleRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ResumeBundleRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    requested_date: Option<String>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ResumeBundleRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
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

    pub fn build(self) -> Result<ResumeBundleRequest<'a>, &'static str> {
        Ok(ResumeBundleRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            requested_date: self.requested_date,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SearchBundlesRequest<'a> {
    pub(crate) search_key: &'a str,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) audit: Option<&'a str>,
}

impl<'a> SearchBundlesRequest<'a> {
    pub fn builder() -> SearchBundlesRequestBuilder<'a> {
        SearchBundlesRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SearchBundlesRequestBuilder<'a> {
    search_key: Option<&'a str>,
    offset: Option<i64>,
    limit: Option<i64>,
    audit: Option<&'a str>,
}

impl<'a> SearchBundlesRequestBuilder<'a> {
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

    pub fn build(self) -> Result<SearchBundlesRequest<'a>, &'static str> {
        Ok(SearchBundlesRequest {
            search_key: self.search_key.ok_or("search_key is required")?,
            offset: self.offset,
            limit: self.limit,
            audit: self.audit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TransferBundleRequest<'a> {
    pub(crate) bundle_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::Bundle,
    pub(crate) requested_date: Option<String>,
    pub(crate) billing_policy: Option<&'a str>,
    pub(crate) bcd_transfer: Option<&'a str>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> TransferBundleRequest<'a> {
    pub fn builder() -> TransferBundleRequestBuilder<'a> {
        TransferBundleRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct TransferBundleRequestBuilder<'a> {
    bundle_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::Bundle>,
    requested_date: Option<String>,
    billing_policy: Option<&'a str>,
    bcd_transfer: Option<&'a str>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> TransferBundleRequestBuilder<'a> {
    pub fn bundle_id(mut self, bundle_id: &'a str) -> Self {
        self.bundle_id = Some(bundle_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::Bundle) -> Self {
        self.body = Some(body);
        self
    }

    pub fn requested_date(mut self, requested_date: impl Into<String>) -> Self {
        self.requested_date = Some(requested_date.into());
        self
    }

    pub fn billing_policy(mut self, billing_policy: &'a str) -> Self {
        self.billing_policy = Some(billing_policy);
        self
    }

    pub fn bcd_transfer(mut self, bcd_transfer: &'a str) -> Self {
        self.bcd_transfer = Some(bcd_transfer);
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

    pub fn build(self) -> Result<TransferBundleRequest<'a>, &'static str> {
        Ok(TransferBundleRequest {
            bundle_id: self.bundle_id.ok_or("bundle_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            body: self.body.ok_or("body is required")?,
            requested_date: self.requested_date,
            billing_policy: self.billing_policy,
            bcd_transfer: self.bcd_transfer,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BundleApiError {
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

impl BundleApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{ BlockingState, CustomField, Tag, AuditLog, Bundle };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_add_bundle_blocking_state() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/bundles/test-bundle-id/block")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"state_name": "test-state"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = AddBundleBlockingStateRequest::builder()
            .bundle_id("test-bundle-id")
            .x_killbill_created_by("test")
            .body(BlockingState {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.add_bundle_blocking_state(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_bundle_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/bundles/test-bundle-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = CreateBundleCustomFieldsRequest::builder()
            .bundle_id("test-bundle-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.create_bundle_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_bundle_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/bundles/test-bundle-id/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tag_definition_id": "test-tag-def-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = CreateBundleTagsRequest::builder()
            .bundle_id("test-bundle-id")
            .x_killbill_created_by("test")
            .body(vec![Uuid::new_v4()])
            .build()
            .unwrap();

        let result = api.create_bundle_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_bundle_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/bundles/test-bundle-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = DeleteBundleCustomFieldsRequest::builder()
            .bundle_id("test-bundle-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_bundle_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_bundle_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/bundles/test-bundle-id/tags")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = DeleteBundleTagsRequest::builder()
            .bundle_id("test-bundle-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.delete_bundle_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_bundle() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/bundles/test-bundle-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"bundle_id": "test-bundle-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = GetBundleRequest::builder().bundle_id("test-bundle-id").build().unwrap();

        let result = api.get_bundle(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_bundle_audit_logs_with_history() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/bundles/test-bundle-id/auditLogsWithHistory")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"change_type": "test-change-type"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = GetBundleAuditLogsWithHistoryRequest::builder()
            .bundle_id("test-bundle-id")
            .build()
            .unwrap();

        let result = api.get_bundle_audit_logs_with_history(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_bundle_by_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/bundles?externalKey=test-external-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"bundle_id": "test-bundle-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = GetBundleByKeyRequest::builder()
            .external_key("test-external-key")
            .build()
            .unwrap();

        let result = api.get_bundle_by_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_bundle_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/bundles/test-bundle-id/customFields")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name": "test-name"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = GetBundleCustomFieldsRequest::builder()
            .bundle_id("test-bundle-id")
            .build()
            .unwrap();

        let result = api.get_bundle_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_bundle_tags() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/bundles/test-bundle-id/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"tag_definition_id": "test-tag-def-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = GetBundleTagsRequest::builder().bundle_id("test-bundle-id").build().unwrap();

        let result = api.get_bundle_tags(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_bundles() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/bundles/pagination")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"bundle_id": "test-bundle-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = GetBundlesRequest::builder().build().unwrap();

        let result = api.get_bundles(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_modify_bundle_custom_fields() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/bundles/test-bundle-id/customFields")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = ModifyBundleCustomFieldsRequest::builder()
            .bundle_id("test-bundle-id")
            .x_killbill_created_by("test")
            .body(
                vec![CustomField {
                    ..Default::default()
                }]
            )
            .build()
            .unwrap();

        let result = api.modify_bundle_custom_fields(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_pause_bundle() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/bundles/test-bundle-id/pause")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = PauseBundleRequest::builder()
            .bundle_id("test-bundle-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.pause_bundle(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_rename_external_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/bundles/test-bundle-id/renameKey")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = RenameExternalKeyRequest::builder()
            .bundle_id("test-bundle-id")
            .x_killbill_created_by("test")
            .body(Bundle {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.rename_external_key(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_resume_bundle() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/bundles/test-bundle-id/resume")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = ResumeBundleRequest::builder()
            .bundle_id("test-bundle-id")
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.resume_bundle(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_search_bundles() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/1.0/kb/bundles/search/test-search-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"bundle_id": "test-bundle-id"}]"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = SearchBundlesRequest::builder()
            .search_key("test-search-key")
            .build()
            .unwrap();

        let result = api.search_bundles(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_transfer_bundle() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/bundles/test-bundle-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"bundle_id": "test-bundle-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = BundleApi::new(config);
        let request = TransferBundleRequest::builder()
            .bundle_id("test-bundle-id")
            .x_killbill_created_by("test")
            .body(Bundle {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.transfer_bundle(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
