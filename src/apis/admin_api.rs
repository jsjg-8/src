use reqwest::{ Method, Response, StatusCode };
use serde::{ Deserialize, de::DeserializeOwned };
use crate::{ apis::configuration::Configuration, models };
use thiserror::Error;

pub struct AdminApi {
    config: Configuration,
}

impl AdminApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn get_queue_entries(
        &self,
        request: GetQueueEntriesRequest<'_>
    ) -> Result<(), AdminApiError> {
        let url = format!("{}/1.0/kb/admin/queues", self.config.base_path);

        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(
                &[
                    ("accountId", request.account_id),
                    ("queueName", request.queue_name),
                    ("serviceName", request.service_name),
                    ("withHistory", request.with_history.map(|b| if b { "true" } else { "false" })),
                    ("minDate", request.min_date),
                    ("maxDate", request.max_date),
                    ("withInProcessing", request.with_in_processing.map(|b| if b { "true" } else { "false" })),
                    ("withBusEvents", request.with_bus_events.map(|b| b.to_string()).as_deref()),
                    ("withNotifications", request.with_notifications.map(|b| if b { "true" } else { "false" })),
                ]
            );

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn invalidates_cache(
        &self,
        request: InvalidatesCacheRequest<'_>
    ) -> Result<(), AdminApiError> {
        let url = format!("{}/1.0/kb/admin/cache", self.config.base_path);

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .query(&[("cacheName", request.cache_name)]);

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn invalidates_cache_by_account(
        &self,
        request: InvalidatesCacheByAccountRequest<'_>
    ) -> Result<(), AdminApiError> {
        let url = format!(
            "{}/1.0/kb/admin/cache/accounts/{}",
            self.config.base_path,
            request.account_id
        );

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn invalidates_cache_by_tenant(&self) -> Result<(), AdminApiError> {
        let url = format!("{}/1.0/kb/admin/cache/tenants", self.config.base_path);

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn put_in_rotation(&self) -> Result<(), AdminApiError> {
        let url = format!("{}/1.0/kb/admin/healthcheck", self.config.base_path);

        let req = self.config.client
            .request(Method::PUT, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn put_out_of_rotation(&self) -> Result<(), AdminApiError> {
        let url = format!("{}/1.0/kb/admin/healthcheck", self.config.base_path);

        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers());

        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn trigger_invoice_generation_for_parked_accounts(
        &self,
        request: TriggerInvoiceGenerationForParkedAccountsRequest<'_>
    ) -> Result<(), AdminApiError> {
        let url = format!("{}/1.0/kb/admin/invoices", self.config.base_path);

        let req = self.config.client
            .request(Method::POST, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default())
            .query(
                &[
                    ("offset", request.offset.map(|o| o.to_string())),
                    ("limit", request.limit.map(|l| l.to_string())),
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
        Self::handle_empty_response(response).await
    }

    pub async fn update_payment_transaction_state(
        &self,
        request: UpdatePaymentTransactionStateRequest<'_>
    ) -> Result<(), AdminApiError> {
        let url = format!(
            "{}/1.0/kb/admin/payments/{}/transactions/{}",
            self.config.base_path,
            request.payment_id,
            request.payment_transaction_id
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

    async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T, AdminApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(AdminApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(AdminApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(response: Response) -> Result<(), AdminApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(AdminApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetQueueEntriesRequest<'a> {
    pub(crate) account_id: Option<&'a str>,
    pub(crate) queue_name: Option<&'a str>,
    pub(crate) service_name: Option<&'a str>,
    pub(crate) with_history: Option<bool>,
    pub(crate) min_date: Option<&'a str>,
    pub(crate) max_date: Option<&'a str>,
    pub(crate) with_in_processing: Option<bool>,
    pub(crate) with_bus_events: Option<bool>,
    pub(crate) with_notifications: Option<bool>,
}

impl<'a> GetQueueEntriesRequest<'a> {
    pub fn builder() -> GetQueueEntriesRequestBuilder<'a> {
        GetQueueEntriesRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetQueueEntriesRequestBuilder<'a> {
    account_id: Option<&'a str>,
    queue_name: Option<&'a str>,
    service_name: Option<&'a str>,
    with_history: Option<bool>,
    min_date: Option<&'a str>,
    max_date: Option<&'a str>,
    with_in_processing: Option<bool>,
    with_bus_events: Option<bool>,
    with_notifications: Option<bool>,
}

impl<'a> GetQueueEntriesRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn queue_name(mut self, queue_name: &'a str) -> Self {
        self.queue_name = Some(queue_name);
        self
    }

    pub fn service_name(mut self, service_name: &'a str) -> Self {
        self.service_name = Some(service_name);
        self
    }

    pub fn with_history(mut self, with_history: bool) -> Self {
        self.with_history = Some(with_history);
        self
    }

    pub fn min_date(mut self, min_date: &'a str) -> Self {
        self.min_date = Some(min_date);
        self
    }

    pub fn max_date(mut self, max_date: &'a str) -> Self {
        self.max_date = Some(max_date);
        self
    }

    pub fn with_in_processing(mut self, with_in_processing: bool) -> Self {
        self.with_in_processing = Some(with_in_processing);
        self
    }

    pub fn with_bus_events(mut self, with_bus_events: bool) -> Self {
        self.with_bus_events = Some(with_bus_events);
        self
    }

    pub fn with_notifications(mut self, with_notifications: bool) -> Self {
        self.with_notifications = Some(with_notifications);
        self
    }

    pub fn build(self) -> Result<GetQueueEntriesRequest<'a>, &'static str> {
        Ok(GetQueueEntriesRequest {
            account_id: self.account_id,
            queue_name: self.queue_name,
            service_name: self.service_name,
            with_history: self.with_history,
            min_date: self.min_date,
            max_date: self.max_date,
            with_in_processing: self.with_in_processing,
            with_bus_events: self.with_bus_events,
            with_notifications: self.with_notifications,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InvalidatesCacheRequest<'a> {
    pub(crate) cache_name: Option<&'a str>,
}

impl<'a> InvalidatesCacheRequest<'a> {
    pub fn builder() -> InvalidatesCacheRequestBuilder<'a> {
        InvalidatesCacheRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct InvalidatesCacheRequestBuilder<'a> {
    cache_name: Option<&'a str>,
}

impl<'a> InvalidatesCacheRequestBuilder<'a> {
    pub fn cache_name(mut self, cache_name: &'a str) -> Self {
        self.cache_name = Some(cache_name);
        self
    }

    pub fn build(self) -> Result<InvalidatesCacheRequest<'a>, &'static str> {
        Ok(InvalidatesCacheRequest {
            cache_name: self.cache_name,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InvalidatesCacheByAccountRequest<'a> {
    pub(crate) account_id: &'a str,
}

impl<'a> InvalidatesCacheByAccountRequest<'a> {
    pub fn builder() -> InvalidatesCacheByAccountRequestBuilder<'a> {
        InvalidatesCacheByAccountRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct InvalidatesCacheByAccountRequestBuilder<'a> {
    account_id: Option<&'a str>,
}

impl<'a> InvalidatesCacheByAccountRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn build(self) -> Result<InvalidatesCacheByAccountRequest<'a>, &'static str> {
        Ok(InvalidatesCacheByAccountRequest {
            account_id: self.account_id.ok_or("account_id is required")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TriggerInvoiceGenerationForParkedAccountsRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<i64>,
    pub(crate) plugin_property: Option<Vec<String>>,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> TriggerInvoiceGenerationForParkedAccountsRequest<'a> {
    pub fn builder() -> TriggerInvoiceGenerationForParkedAccountsRequestBuilder<'a> {
        TriggerInvoiceGenerationForParkedAccountsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct TriggerInvoiceGenerationForParkedAccountsRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    offset: Option<i64>,
    limit: Option<i64>,
    plugin_property: Option<Vec<String>>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> TriggerInvoiceGenerationForParkedAccountsRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
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

    pub fn build(
        self
    ) -> Result<TriggerInvoiceGenerationForParkedAccountsRequest<'a>, &'static str> {
        Ok(TriggerInvoiceGenerationForParkedAccountsRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or(
                "x_killbill_created_by is required"
            )?,
            offset: self.offset,
            limit: self.limit,
            plugin_property: self.plugin_property,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UpdatePaymentTransactionStateRequest<'a> {
    pub(crate) payment_id: &'a str,
    pub(crate) payment_transaction_id: &'a str,
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::AdminPayment,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdatePaymentTransactionStateRequest<'a> {
    pub fn builder() -> UpdatePaymentTransactionStateRequestBuilder<'a> {
        UpdatePaymentTransactionStateRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UpdatePaymentTransactionStateRequestBuilder<'a> {
    payment_id: Option<&'a str>,
    payment_transaction_id: Option<&'a str>,
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::AdminPayment>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UpdatePaymentTransactionStateRequestBuilder<'a> {
    pub fn payment_id(mut self, payment_id: &'a str) -> Self {
        self.payment_id = Some(payment_id);
        self
    }

    pub fn payment_transaction_id(mut self, payment_transaction_id: &'a str) -> Self {
        self.payment_transaction_id = Some(payment_transaction_id);
        self
    }

    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::AdminPayment) -> Self {
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

    pub fn build(self) -> Result<UpdatePaymentTransactionStateRequest<'a>, &'static str> {
        Ok(UpdatePaymentTransactionStateRequest {
            payment_id: self.payment_id.ok_or("payment_id is required")?,
            payment_transaction_id: self.payment_transaction_id.ok_or(
                "payment_transaction_id is required"
            )?,
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
pub enum AdminApiError {
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

impl AdminApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::AdminPayment;

    #[tokio::test]
    async fn test_get_queue_entries() {
        let mut server = Server::new_async().await;
        let mock = server.mock("GET", "/1.0/kb/admin/queues").with_status(204).create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AdminApi::new(config);
        let request = GetQueueEntriesRequest::builder().build().unwrap();

        let result = api.get_queue_entries(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_invalidates_cache() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/admin/cache")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AdminApi::new(config);
        let request = InvalidatesCacheRequest::builder().build().unwrap();

        let result = api.invalidates_cache(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_invalidates_cache_by_account() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/admin/cache/accounts/test-account-id")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AdminApi::new(config);
        let request = InvalidatesCacheByAccountRequest::builder()
            .account_id("test-account-id")
            .build()
            .unwrap();

        let result = api.invalidates_cache_by_account(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_invalidates_cache_by_tenant() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/admin/cache/tenants")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AdminApi::new(config);
        let result = api.invalidates_cache_by_tenant().await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_put_in_rotation() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/1.0/kb/admin/healthcheck")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AdminApi::new(config);
        let result = api.put_in_rotation().await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_put_out_of_rotation() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/1.0/kb/admin/healthcheck")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AdminApi::new(config);
        let result = api.put_out_of_rotation().await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_trigger_invoice_generation_for_parked_accounts() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/admin/invoices")
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AdminApi::new(config);
        let request = TriggerInvoiceGenerationForParkedAccountsRequest::builder()
            .x_killbill_created_by("test")
            .build()
            .unwrap();

        let result = api.trigger_invoice_generation_for_parked_accounts(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_update_payment_transaction_state() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock(
                "PUT",
                "/1.0/kb/admin/payments/test-payment-id/transactions/test-payment-transaction-id"
            )
            .with_status(204)
            .create_async().await;

        let config = Configuration::builder().base_path(server.url()).build().unwrap();

        let api = AdminApi::new(config);
        let request = UpdatePaymentTransactionStateRequest::builder()
            .payment_id("test-payment-id")
            .payment_transaction_id("test-payment-transaction-id")
            .x_killbill_created_by("test")
            .body(AdminPayment {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.update_payment_transaction_state(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
