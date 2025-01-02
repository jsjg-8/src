use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct HostedPaymentPageApi {
    config: Configuration,
}

impl HostedPaymentPageApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn build_combo_form_descriptor(
        &self,
        request: BuildComboFormDescriptorRequest<'_>
    ) -> Result<models::HostedPaymentPageFormDescriptor, HostedPaymentPageApiError> {
        let url = format!("{}/1.0/kb/paymentGateways/hosted/form", self.config.base_path);

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

    pub async fn build_form_descriptor(
        &self,
        request: BuildFormDescriptorRequest<'_>
    ) -> Result<models::HostedPaymentPageFormDescriptor, HostedPaymentPageApiError> {
        let url = format!(
            "{}/1.0/kb/paymentGateways/hosted/form/{}",
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

    pub async fn process_notification(
        &self,
        request: ProcessNotificationRequest<'_>
    ) -> Result<(), HostedPaymentPageApiError> {
        let url = format!(
            "{}/1.0/kb/paymentGateways/notification/{}",
            self.config.base_path,
            request.plugin_name
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
    ) -> Result<T, HostedPaymentPageApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(HostedPaymentPageApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(HostedPaymentPageApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), HostedPaymentPageApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(HostedPaymentPageApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildComboFormDescriptorRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::ComboHostedPaymentPage,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> BuildComboFormDescriptorRequest<'a> {
    pub fn builder() -> BuildComboFormDescriptorRequestBuilder<'a> {
        BuildComboFormDescriptorRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct BuildComboFormDescriptorRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::ComboHostedPaymentPage>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> BuildComboFormDescriptorRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::ComboHostedPaymentPage) -> Self {
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

    pub fn build(self) -> Result<BuildComboFormDescriptorRequest<'a>, &'static str> {
        Ok(BuildComboFormDescriptorRequest {
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
pub struct BuildFormDescriptorRequest<'a> {
    pub(crate) account_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::HostedPaymentPageFields,
    pub(crate) payment_method_id: Option<&'a str>,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> BuildFormDescriptorRequest<'a> {
    pub fn builder() -> BuildFormDescriptorRequestBuilder<'a> {
        BuildFormDescriptorRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct BuildFormDescriptorRequestBuilder<'a> {
    account_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::HostedPaymentPageFields>,
    payment_method_id: Option<&'a str>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> BuildFormDescriptorRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::HostedPaymentPageFields) -> Self {
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

    pub fn build(self) -> Result<BuildFormDescriptorRequest<'a>, &'static str> {
        Ok(BuildFormDescriptorRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
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
pub struct ProcessNotificationRequest<'a> {
    pub(crate) plugin_name: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: &'a str,
    pub(crate) control_plugin_name: Option<Vec<String>>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ProcessNotificationRequest<'a> {
    pub fn builder() -> ProcessNotificationRequestBuilder<'a> {
        ProcessNotificationRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ProcessNotificationRequestBuilder<'a> {
    plugin_name: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<&'a str>,
    control_plugin_name: Option<Vec<String>>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ProcessNotificationRequestBuilder<'a> {
    pub fn plugin_name(mut self, plugin_name: &'a str) -> Self {
        self.plugin_name = Some(plugin_name);
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

    pub fn build(self) -> Result<ProcessNotificationRequest<'a>, &'static str> {
        Ok(ProcessNotificationRequest {
            plugin_name: self.plugin_name.ok_or("plugin_name is required")?,
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
pub enum HostedPaymentPageApiError {
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

impl HostedPaymentPageApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{
        HostedPaymentPageFormDescriptor,
        ComboHostedPaymentPage,
        HostedPaymentPageFields,
    };

    #[tokio::test]
    async fn test_build_combo_form_descriptor() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/paymentGateways/hosted/form")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"form_id": "test-form-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = HostedPaymentPageApi::new(config);
        let request = BuildComboFormDescriptorRequest::builder()
            .x_killbill_created_by("test")
            .body(ComboHostedPaymentPage {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.build_combo_form_descriptor(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_build_form_descriptor() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/paymentGateways/hosted/form/test-account-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"form_id": "test-form-id"}"#)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = HostedPaymentPageApi::new(config);
        let request = BuildFormDescriptorRequest::builder()
            .account_id("test-account-id")
            .x_killbill_created_by("test")
            .body(HostedPaymentPageFields {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.build_form_descriptor(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
