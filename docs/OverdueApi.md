# \OverdueApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_overdue_config_json**](OverdueApi.md#get_overdue_config_json) | **GET** /1.0/kb/overdue | Retrieve the overdue config as JSON
[**get_overdue_config_xml**](OverdueApi.md#get_overdue_config_xml) | **GET** /1.0/kb/overdue/xml | Retrieve the overdue config as XML
[**upload_overdue_config_json**](OverdueApi.md#upload_overdue_config_json) | **POST** /1.0/kb/overdue | Upload the full overdue config as JSON
[**upload_overdue_config_xml**](OverdueApi.md#upload_overdue_config_xml) | **POST** /1.0/kb/overdue/xml | Upload the full overdue config as XML



## get_overdue_config_json

> models::Overdue get_overdue_config_json()
Retrieve the overdue config as JSON

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Overdue**](Overdue.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_overdue_config_xml

> String get_overdue_config_xml()
Retrieve the overdue config as XML

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_overdue_config_json

> models::Overdue upload_overdue_config_json(x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Upload the full overdue config as JSON

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Overdue**](Overdue.md) |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::Overdue**](Overdue.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_overdue_config_xml

> String upload_overdue_config_xml(x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Upload the full overdue config as XML

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

