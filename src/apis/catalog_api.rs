use reqwest::{Method, Response, StatusCode};
use serde::{Deserialize, de::DeserializeOwned};
use crate::{
    apis::configuration::Configuration,
    models,
};
use thiserror::Error;

pub struct CatalogApi {
    config: Configuration,
}

impl CatalogApi {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    pub async fn add_simple_plan(
        &self,
        request: AddSimplePlanRequest<'_>,
    ) -> Result<String, CatalogApiError> {
        let url = format!("{}/1.0/kb/catalog/simplePlan", self.config.base_path);
        
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

    pub async fn delete_catalog(
        &self,
        request: DeleteCatalogRequest<'_>,
    ) -> Result<(), CatalogApiError> {
        let url = format!("{}/1.0/kb/catalog", self.config.base_path);
        
        let req = self.config.client
            .request(Method::DELETE, &url)
            .headers(self.config.get_auth_headers())
            .header("X-Killbill-CreatedBy", request.x_killbill_created_by)
            .header("X-Killbill-Reason", request.x_killbill_reason.unwrap_or_default())
            .header("X-Killbill-Comment", request.x_killbill_comment.unwrap_or_default());
        
        let response = req.send().await?;
        Self::handle_empty_response(response).await
    }

    pub async fn get_available_addons(
        &self,
        request: GetAvailableAddonsRequest<'_>,
    ) -> Result<Vec<models::PlanDetail>, CatalogApiError> {
        let url = format!("{}/1.0/kb/catalog/availableAddons", self.config.base_path);
        
        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[
                ("baseProductName", request.base_product_name),
                ("priceListName", request.price_list_name),
                ("accountId", request.account_id),
            ]);
        
        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_available_base_plans(
        &self,
        request: GetAvailableBasePlansRequest<'_>,
    ) -> Result<Vec<models::PlanDetail>, CatalogApiError> {
        let url = format!("{}/1.0/kb/catalog/availableBasePlans", self.config.base_path);
        
        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[
                ("accountId", request.account_id),
            ]);
        
        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_catalog_json(
        &self,
        request: GetCatalogJsonRequest<'_>,
    ) -> Result<Vec<models::Catalog>, CatalogApiError> {
        let url = format!("{}/1.0/kb/catalog", self.config.base_path);
        
        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[
                ("requestedDate", request.requested_date),
                ("accountId", request.account_id.map(|id| id.to_string())),
            ]);
        
        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_catalog_versions(
        &self,
        request: GetCatalogVersionsRequest<'_>,
    ) -> Result<Vec<String>, CatalogApiError> {
        let url = format!("{}/1.0/kb/catalog/versions", self.config.base_path);
        
        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[
                ("accountId", request.account_id),
            ]);
        
        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_catalog_xml(
        &self,
        request: GetCatalogXmlRequest<'_>,
    ) -> Result<String, CatalogApiError> {
        let url = format!("{}/1.0/kb/catalog/xml", self.config.base_path);
        
        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[
                ("requestedDate", request.requested_date),
                ("accountId", request.account_id.map(|id| id.to_string())),
            ]);
        
        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_phase_for_subscription_and_date(
        &self,
        request: GetPhaseForSubscriptionAndDateRequest<'_>,
    ) -> Result<models::Phase, CatalogApiError> {
        let url = format!("{}/1.0/kb/catalog/phase", self.config.base_path);
        
        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[
                ("subscriptionId", request.subscription_id),
                ("requestedDate", request.requested_date.as_deref()),
            ]);
        
        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_plan_for_subscription_and_date(
        &self,
        request: GetPlanForSubscriptionAndDateRequest<'_>,
    ) -> Result<models::Plan, CatalogApiError> {
        let url = format!("{}/1.0/kb/catalog/plan", self.config.base_path);
        
        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[
                ("subscriptionId", request.subscription_id),
                ("requestedDate", request.requested_date.as_deref()),
            ]);
        
        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_price_list_for_subscription_and_date(
        &self,
        request: GetPriceListForSubscriptionAndDateRequest<'_>,
    ) -> Result<models::PriceList, CatalogApiError> {
        let url = format!("{}/1.0/kb/catalog/priceList", self.config.base_path);
        
        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[
                ("subscriptionId", request.subscription_id),
                ("requestedDate", request.requested_date.as_deref()),
            ]);
        
        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn get_product_for_subscription_and_date(
        &self,
        request: GetProductForSubscriptionAndDateRequest<'_>,
    ) -> Result<models::Product, CatalogApiError> {
        let url = format!("{}/1.0/kb/catalog/product", self.config.base_path);
        
        let req = self.config.client
            .request(Method::GET, &url)
            .headers(self.config.get_auth_headers())
            .query(&[
                ("subscriptionId", request.subscription_id),
                ("requestedDate", request.requested_date.as_deref()),
            ]);
        
        let response = req.send().await?;
        Self::handle_response(response).await
    }

    pub async fn upload_catalog_xml(
        &self,
        request: UploadCatalogXmlRequest<'_>,
    ) -> Result<String, CatalogApiError> {
        let url = format!("{}/1.0/kb/catalog/xml", self.config.base_path);
        
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

    pub async fn validate_catalog_xml(
        &self,
        request: ValidateCatalogXmlRequest<'_>,
    ) -> Result<models::CatalogValidation, CatalogApiError> {
        let url = format!("{}/1.0/kb/catalog/xml/validate", self.config.base_path);
        
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

    async fn handle_response<T: DeserializeOwned>(
        response: Response
    ) -> Result<T, CatalogApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(CatalogApiError::from)
            }
            status => {
                let text = response.text().await?;
                Err(CatalogApiError::from_response(status, text))
            }
        }
    }

    async fn handle_empty_response(
        response: Response
    ) -> Result<(), CatalogApiError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => {
                let text = response.text().await?;
                Err(CatalogApiError::from_response(status, text))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddSimplePlanRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: models::SimplePlan,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> AddSimplePlanRequest<'a> {
    pub fn builder() -> AddSimplePlanRequestBuilder<'a> {
        AddSimplePlanRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct AddSimplePlanRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<models::SimplePlan>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> AddSimplePlanRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: models::SimplePlan) -> Self {
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

    pub fn build(self) -> Result<AddSimplePlanRequest<'a>, &'static str> {
        Ok(AddSimplePlanRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            body: self.body.ok_or("body is required")?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteCatalogRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteCatalogRequest<'a> {
    pub fn builder() -> DeleteCatalogRequestBuilder<'a> {
        DeleteCatalogRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DeleteCatalogRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> DeleteCatalogRequestBuilder<'a> {
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

    pub fn build(self) -> Result<DeleteCatalogRequest<'a>, &'static str> {
        Ok(DeleteCatalogRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAvailableAddonsRequest<'a> {
    pub(crate) base_product_name: Option<&'a str>,
    pub(crate) price_list_name: Option<&'a str>,
    pub(crate) account_id: Option<&'a str>,
}

impl<'a> GetAvailableAddonsRequest<'a> {
    pub fn builder() -> GetAvailableAddonsRequestBuilder<'a> {
        GetAvailableAddonsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAvailableAddonsRequestBuilder<'a> {
    base_product_name: Option<&'a str>,
    price_list_name: Option<&'a str>,
    account_id: Option<&'a str>,
}

impl<'a> GetAvailableAddonsRequestBuilder<'a> {
    pub fn base_product_name(mut self, base_product_name: &'a str) -> Self {
        self.base_product_name = Some(base_product_name);
        self
    }

    pub fn price_list_name(mut self, price_list_name: &'a str) -> Self {
        self.price_list_name = Some(price_list_name);
        self
    }

    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn build(self) -> Result<GetAvailableAddonsRequest<'a>, &'static str> {
        Ok(GetAvailableAddonsRequest {
            base_product_name: self.base_product_name,
            price_list_name: self.price_list_name,
            account_id: self.account_id,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAvailableBasePlansRequest<'a> {
    pub(crate) account_id: Option<&'a str>,
}

impl<'a> GetAvailableBasePlansRequest<'a> {
    pub fn builder() -> GetAvailableBasePlansRequestBuilder<'a> {
        GetAvailableBasePlansRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetAvailableBasePlansRequestBuilder<'a> {
    account_id: Option<&'a str>,
}

impl<'a> GetAvailableBasePlansRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn build(self) -> Result<GetAvailableBasePlansRequest<'a>, &'static str> {
        Ok(GetAvailableBasePlansRequest {
            account_id: self.account_id,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetCatalogJsonRequest<'a> {
    pub(crate) requested_date: Option<String>,
    pub(crate) account_id: Option<&'a str>,
}

impl<'a> GetCatalogJsonRequest<'a> {
    pub fn builder() -> GetCatalogJsonRequestBuilder<'a> {
        GetCatalogJsonRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetCatalogJsonRequestBuilder<'a> {
    requested_date: Option<String>,
    account_id: Option<&'a str>,
}

impl<'a> GetCatalogJsonRequestBuilder<'a> {
    pub fn requested_date(mut self, requested_date: impl Into<String>) -> Self {
        self.requested_date = Some(requested_date.into());
        self
    }

    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn build(self) -> Result<GetCatalogJsonRequest<'a>, &'static str> {
        Ok(GetCatalogJsonRequest {
            requested_date: self.requested_date,
            account_id: self.account_id,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetCatalogVersionsRequest<'a> {
    pub(crate) account_id: Option<&'a str>,
}

impl<'a> GetCatalogVersionsRequest<'a> {
    pub fn builder() -> GetCatalogVersionsRequestBuilder<'a> {
        GetCatalogVersionsRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetCatalogVersionsRequestBuilder<'a> {
    account_id: Option<&'a str>,
}

impl<'a> GetCatalogVersionsRequestBuilder<'a> {
    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn build(self) -> Result<GetCatalogVersionsRequest<'a>, &'static str> {
        Ok(GetCatalogVersionsRequest {
            account_id: self.account_id,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetCatalogXmlRequest<'a> {
    pub(crate) requested_date: Option<String>,
    pub(crate) account_id: Option<&'a str>,
}

impl<'a> GetCatalogXmlRequest<'a> {
    pub fn builder() -> GetCatalogXmlRequestBuilder<'a> {
        GetCatalogXmlRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetCatalogXmlRequestBuilder<'a> {
    requested_date: Option<String>,
    account_id: Option<&'a str>,
}

impl<'a> GetCatalogXmlRequestBuilder<'a> {
    pub fn requested_date(mut self, requested_date: impl Into<String>) -> Self {
        self.requested_date = Some(requested_date.into());
        self
    }

    pub fn account_id(mut self, account_id: &'a str) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn build(self) -> Result<GetCatalogXmlRequest<'a>, &'static str> {
        Ok(GetCatalogXmlRequest {
            requested_date: self.requested_date,
            account_id: self.account_id,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPhaseForSubscriptionAndDateRequest<'a> {
    pub(crate) subscription_id: Option<&'a str>,
    pub(crate) requested_date: Option<String>,
}

impl<'a> GetPhaseForSubscriptionAndDateRequest<'a> {
    pub fn builder() -> GetPhaseForSubscriptionAndDateRequestBuilder<'a> {
        GetPhaseForSubscriptionAndDateRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPhaseForSubscriptionAndDateRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    requested_date: Option<String>,
}

impl<'a> GetPhaseForSubscriptionAndDateRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn requested_date(mut self, requested_date: impl Into<String>) -> Self {
        self.requested_date = Some(requested_date.into());
        self
    }

    pub fn build(self) -> Result<GetPhaseForSubscriptionAndDateRequest<'a>, &'static str> {
        Ok(GetPhaseForSubscriptionAndDateRequest {
            subscription_id: self.subscription_id,
            requested_date: self.requested_date,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPlanForSubscriptionAndDateRequest<'a> {
    pub(crate) subscription_id: Option<&'a str>,
    pub(crate) requested_date: Option<String>,
}

impl<'a> GetPlanForSubscriptionAndDateRequest<'a> {
    pub fn builder() -> GetPlanForSubscriptionAndDateRequestBuilder<'a> {
        GetPlanForSubscriptionAndDateRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPlanForSubscriptionAndDateRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    requested_date: Option<String>,
}

impl<'a> GetPlanForSubscriptionAndDateRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn requested_date(mut self, requested_date: impl Into<String>) -> Self {
        self.requested_date = Some(requested_date.into());
        self
    }

    pub fn build(self) -> Result<GetPlanForSubscriptionAndDateRequest<'a>, &'static str> {
        Ok(GetPlanForSubscriptionAndDateRequest {
            subscription_id: self.subscription_id,
            requested_date: self.requested_date,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPriceListForSubscriptionAndDateRequest<'a> {
    pub(crate) subscription_id: Option<&'a str>,
    pub(crate) requested_date: Option<String>,
}

impl<'a> GetPriceListForSubscriptionAndDateRequest<'a> {
    pub fn builder() -> GetPriceListForSubscriptionAndDateRequestBuilder<'a> {
        GetPriceListForSubscriptionAndDateRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetPriceListForSubscriptionAndDateRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    requested_date: Option<String>,
}

impl<'a> GetPriceListForSubscriptionAndDateRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn requested_date(mut self, requested_date: impl Into<String>) -> Self {
        self.requested_date = Some(requested_date.into());
        self
    }

    pub fn build(self) -> Result<GetPriceListForSubscriptionAndDateRequest<'a>, &'static str> {
        Ok(GetPriceListForSubscriptionAndDateRequest {
            subscription_id: self.subscription_id,
            requested_date: self.requested_date,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetProductForSubscriptionAndDateRequest<'a> {
    pub(crate) subscription_id: Option<&'a str>,
    pub(crate) requested_date: Option<String>,
}

impl<'a> GetProductForSubscriptionAndDateRequest<'a> {
    pub fn builder() -> GetProductForSubscriptionAndDateRequestBuilder<'a> {
        GetProductForSubscriptionAndDateRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct GetProductForSubscriptionAndDateRequestBuilder<'a> {
    subscription_id: Option<&'a str>,
    requested_date: Option<String>,
}

impl<'a> GetProductForSubscriptionAndDateRequestBuilder<'a> {
    pub fn subscription_id(mut self, subscription_id: &'a str) -> Self {
        self.subscription_id = Some(subscription_id);
        self
    }

    pub fn requested_date(mut self, requested_date: impl Into<String>) -> Self {
        self.requested_date = Some(requested_date.into());
        self
    }

    pub fn build(self) -> Result<GetProductForSubscriptionAndDateRequest<'a>, &'static str> {
        Ok(GetProductForSubscriptionAndDateRequest {
            subscription_id: self.subscription_id,
            requested_date: self.requested_date,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UploadCatalogXmlRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadCatalogXmlRequest<'a> {
    pub fn builder() -> UploadCatalogXmlRequestBuilder<'a> {
        UploadCatalogXmlRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct UploadCatalogXmlRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> UploadCatalogXmlRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: &'a str) -> Self {
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

    pub fn build(self) -> Result<UploadCatalogXmlRequest<'a>, &'static str> {
        Ok(UploadCatalogXmlRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            body: self.body.ok_or("body is required")?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ValidateCatalogXmlRequest<'a> {
    pub(crate) x_killbill_created_by: &'a str,
    pub(crate) body: &'a str,
    pub(crate) x_killbill_reason: Option<&'a str>,
    pub(crate) x_killbill_comment: Option<&'a str>,
}

impl<'a> ValidateCatalogXmlRequest<'a> {
    pub fn builder() -> ValidateCatalogXmlRequestBuilder<'a> {
        ValidateCatalogXmlRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ValidateCatalogXmlRequestBuilder<'a> {
    x_killbill_created_by: Option<&'a str>,
    body: Option<&'a str>,
    x_killbill_reason: Option<&'a str>,
    x_killbill_comment: Option<&'a str>,
}

impl<'a> ValidateCatalogXmlRequestBuilder<'a> {
    pub fn x_killbill_created_by(mut self, x_killbill_created_by: &'a str) -> Self {
        self.x_killbill_created_by = Some(x_killbill_created_by);
        self
    }

    pub fn body(mut self, body: &'a str) -> Self {
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

    pub fn build(self) -> Result<ValidateCatalogXmlRequest<'a>, &'static str> {
        Ok(ValidateCatalogXmlRequest {
            x_killbill_created_by: self.x_killbill_created_by.ok_or("x_killbill_created_by is required")?,
            body: self.body.ok_or("body is required")?,
            x_killbill_reason: self.x_killbill_reason,
            x_killbill_comment: self.x_killbill_comment,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CatalogApiError {
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    
    #[error("API error ({status}): {message}")]
    ApiError {
        status: StatusCode,
        message: String,
    },
    
    #[error("Configuration error: {0}")]
    ConfigError(#[from] crate::apis::configuration::ConfigError),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl CatalogApiError {
    fn from_response(status: StatusCode, message: String) -> Self {
        Self::ApiError { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::models::{SimplePlan, PlanDetail, Catalog, Phase, Plan, PriceList, Product, CatalogValidation};

    #[tokio::test]
    async fn test_add_simple_plan() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/1.0/kb/catalog/simplePlan")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#""test-plan-id""#)
            .create_async()
            .await;

        let config = Configuration::builder()
            .base_path(server.url())
            .build()
            .unwrap();

        let api = CatalogApi::new(config);
        let request = AddSimplePlanRequest::builder()
            .x_killbill_created_by("test")
            .body(SimplePlan {
                ..Default::default()
            })
            .build()
            .unwrap();

        let result = api.add_simple_plan(request).await;
        assert!(result.is_ok());
        
        mock.assert_async().await;
    }

    // #[tokio::test]
    // async fn test_delete_catalog() {
    //     let mut server = Server::new_async().await;
    //     let mock = server
    //         .

}