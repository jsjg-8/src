# \CatalogApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_simple_plan**](CatalogApi.md#add_simple_plan) | **POST** /1.0/kb/catalog/simplePlan | Add a simple plan entry in the current version of the catalog
[**delete_catalog**](CatalogApi.md#delete_catalog) | **DELETE** /1.0/kb/catalog | Delete all versions for a per tenant catalog
[**get_available_addons**](CatalogApi.md#get_available_addons) | **GET** /1.0/kb/catalog/availableAddons | Retrieve available add-ons for a given product
[**get_available_base_plans**](CatalogApi.md#get_available_base_plans) | **GET** /1.0/kb/catalog/availableBasePlans | Retrieve available base plans
[**get_catalog_json**](CatalogApi.md#get_catalog_json) | **GET** /1.0/kb/catalog | Retrieve the catalog as JSON
[**get_catalog_versions**](CatalogApi.md#get_catalog_versions) | **GET** /1.0/kb/catalog/versions | Retrieve a list of catalog versions
[**get_catalog_xml**](CatalogApi.md#get_catalog_xml) | **GET** /1.0/kb/catalog/xml | Retrieve the full catalog as XML
[**get_phase_for_subscription_and_date**](CatalogApi.md#get_phase_for_subscription_and_date) | **GET** /1.0/kb/catalog/phase | Retrieve phase for a given subscription and date
[**get_plan_for_subscription_and_date**](CatalogApi.md#get_plan_for_subscription_and_date) | **GET** /1.0/kb/catalog/plan | Retrieve plan for a given subscription and date
[**get_price_list_for_subscription_and_date**](CatalogApi.md#get_price_list_for_subscription_and_date) | **GET** /1.0/kb/catalog/priceList | Retrieve priceList for a given subscription and date
[**get_product_for_subscription_and_date**](CatalogApi.md#get_product_for_subscription_and_date) | **GET** /1.0/kb/catalog/product | Retrieve product for a given subscription and date
[**upload_catalog_xml**](CatalogApi.md#upload_catalog_xml) | **POST** /1.0/kb/catalog/xml | Upload the full catalog as XML
[**validate_catalog_xml**](CatalogApi.md#validate_catalog_xml) | **POST** /1.0/kb/catalog/xml/validate | Validate a XML catalog



## add_simple_plan

> String add_simple_plan(x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add a simple plan entry in the current version of the catalog

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**SimplePlan**](SimplePlan.md) |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_catalog

> delete_catalog(x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Delete all versions for a per tenant catalog

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_available_addons

> Vec<models::PlanDetail> get_available_addons(base_product_name, price_list_name, account_id)
Retrieve available add-ons for a given product

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**base_product_name** | Option<**String**> |  |  |
**price_list_name** | Option<**String**> |  |  |
**account_id** | Option<**uuid::Uuid**> |  |  |

### Return type

[**Vec<models::PlanDetail>**](PlanDetail.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_available_base_plans

> Vec<models::PlanDetail> get_available_base_plans(account_id)
Retrieve available base plans

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**uuid::Uuid**> |  |  |

### Return type

[**Vec<models::PlanDetail>**](PlanDetail.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_catalog_json

> Vec<models::Catalog> get_catalog_json(requested_date, account_id)
Retrieve the catalog as JSON

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**requested_date** | Option<**String**> |  |  |
**account_id** | Option<**uuid::Uuid**> |  |  |

### Return type

[**Vec<models::Catalog>**](Catalog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_catalog_versions

> Vec<String> get_catalog_versions(account_id)
Retrieve a list of catalog versions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**uuid::Uuid**> |  |  |

### Return type

**Vec<String>**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_catalog_xml

> String get_catalog_xml(requested_date, account_id)
Retrieve the full catalog as XML

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**requested_date** | Option<**String**> |  |  |
**account_id** | Option<**uuid::Uuid**> |  |  |

### Return type

**String**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_phase_for_subscription_and_date

> models::Phase get_phase_for_subscription_and_date(subscription_id, requested_date)
Retrieve phase for a given subscription and date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | Option<**uuid::Uuid**> |  |  |
**requested_date** | Option<**String**> |  |  |

### Return type

[**models::Phase**](Phase.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plan_for_subscription_and_date

> models::Plan get_plan_for_subscription_and_date(subscription_id, requested_date)
Retrieve plan for a given subscription and date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | Option<**uuid::Uuid**> |  |  |
**requested_date** | Option<**String**> |  |  |

### Return type

[**models::Plan**](Plan.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_price_list_for_subscription_and_date

> models::PriceList get_price_list_for_subscription_and_date(subscription_id, requested_date)
Retrieve priceList for a given subscription and date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | Option<**uuid::Uuid**> |  |  |
**requested_date** | Option<**String**> |  |  |

### Return type

[**models::PriceList**](PriceList.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_for_subscription_and_date

> models::Product get_product_for_subscription_and_date(subscription_id, requested_date)
Retrieve product for a given subscription and date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | Option<**uuid::Uuid**> |  |  |
**requested_date** | Option<**String**> |  |  |

### Return type

[**models::Product**](Product.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_catalog_xml

> String upload_catalog_xml(x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Upload the full catalog as XML

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | **String** |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: text/xml
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_catalog_xml

> models::CatalogValidation validate_catalog_xml(x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Validate a XML catalog

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | **String** |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::CatalogValidation**](CatalogValidation.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: text/xml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

