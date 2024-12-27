# \CreditApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_credits**](CreditApi.md#create_credits) | **POST** /1.0/kb/credits | Create a credit
[**get_credit**](CreditApi.md#get_credit) | **GET** /1.0/kb/credits/{creditId} | Retrieve a credit by id



## create_credits

> Vec<models::InvoiceItem> create_credits(x_killbill_created_by, body, auto_commit, plugin_property, x_killbill_reason, x_killbill_comment)
Create a credit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Vec<models::InvoiceItem>**](InvoiceItem.md) |  | [required] |
**auto_commit** | Option<**bool**> |  |  |[default to false]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**Vec<models::InvoiceItem>**](InvoiceItem.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_credit

> models::InvoiceItem get_credit(credit_id)
Retrieve a credit by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credit_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::InvoiceItem**](InvoiceItem.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

