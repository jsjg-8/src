# \UsageApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_usage**](UsageApi.md#get_all_usage) | **GET** /1.0/kb/usages/{subscriptionId} | Retrieve usage for a subscription
[**get_usage**](UsageApi.md#get_usage) | **GET** /1.0/kb/usages/{subscriptionId}/{unitType} | Retrieve usage for a subscription and unit type
[**record_usage**](UsageApi.md#record_usage) | **POST** /1.0/kb/usages | Record usage for a subscription



## get_all_usage

> models::RolledUpUsage get_all_usage(subscription_id, start_date, end_date, plugin_property)
Retrieve usage for a subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
**start_date** | Option<**String**> |  |  |
**end_date** | Option<**String**> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::RolledUpUsage**](RolledUpUsage.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage

> models::RolledUpUsage get_usage(subscription_id, unit_type, start_date, end_date, plugin_property)
Retrieve usage for a subscription and unit type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
**unit_type** | **String** |  | [required] |
**start_date** | Option<**String**> |  |  |
**end_date** | Option<**String**> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::RolledUpUsage**](RolledUpUsage.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## record_usage

> record_usage(x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Record usage for a subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**SubscriptionUsageRecord**](SubscriptionUsageRecord.md) |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

