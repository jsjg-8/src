# \PaymentMethodApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payment_method_custom_fields**](PaymentMethodApi.md#create_payment_method_custom_fields) | **POST** /1.0/kb/paymentMethods/{paymentMethodId}/customFields | Add custom fields to payment method
[**delete_payment_method**](PaymentMethodApi.md#delete_payment_method) | **DELETE** /1.0/kb/paymentMethods/{paymentMethodId} | Delete a payment method
[**delete_payment_method_custom_fields**](PaymentMethodApi.md#delete_payment_method_custom_fields) | **DELETE** /1.0/kb/paymentMethods/{paymentMethodId}/customFields | Remove custom fields from payment method
[**get_payment_method**](PaymentMethodApi.md#get_payment_method) | **GET** /1.0/kb/paymentMethods/{paymentMethodId} | Retrieve a payment method by id
[**get_payment_method_audit_logs_with_history**](PaymentMethodApi.md#get_payment_method_audit_logs_with_history) | **GET** /1.0/kb/paymentMethods/{paymentMethodId}/auditLogsWithHistory | Retrieve payment method audit logs with history by id
[**get_payment_method_by_key**](PaymentMethodApi.md#get_payment_method_by_key) | **GET** /1.0/kb/paymentMethods | Retrieve a payment method by external key
[**get_payment_method_custom_fields**](PaymentMethodApi.md#get_payment_method_custom_fields) | **GET** /1.0/kb/paymentMethods/{paymentMethodId}/customFields | Retrieve payment method custom fields
[**get_payment_methods**](PaymentMethodApi.md#get_payment_methods) | **GET** /1.0/kb/paymentMethods/pagination | List payment methods
[**modify_payment_method_custom_fields**](PaymentMethodApi.md#modify_payment_method_custom_fields) | **PUT** /1.0/kb/paymentMethods/{paymentMethodId}/customFields | Modify custom fields to payment method
[**search_payment_methods**](PaymentMethodApi.md#search_payment_methods) | **GET** /1.0/kb/paymentMethods/search/{searchKey} | Search payment methods



## create_payment_method_custom_fields

> Vec<models::CustomField> create_payment_method_custom_fields(payment_method_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add custom fields to payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_method_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Vec<models::CustomField>**](CustomField.md) |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**Vec<models::CustomField>**](CustomField.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_payment_method

> delete_payment_method(payment_method_id, x_killbill_created_by, delete_default_pm_with_auto_pay_off, force_default_pm_deletion, plugin_property, x_killbill_reason, x_killbill_comment)
Delete a payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_method_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**delete_default_pm_with_auto_pay_off** | Option<**bool**> |  |  |[default to false]
**force_default_pm_deletion** | Option<**bool**> |  |  |[default to false]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
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


## delete_payment_method_custom_fields

> delete_payment_method_custom_fields(payment_method_id, x_killbill_created_by, custom_field, x_killbill_reason, x_killbill_comment)
Remove custom fields from payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_method_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**custom_field** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
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


## get_payment_method

> models::PaymentMethod get_payment_method(payment_method_id, included_deleted, with_plugin_info, plugin_property, audit)
Retrieve a payment method by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_method_id** | **uuid::Uuid** |  | [required] |
**included_deleted** | Option<**bool**> |  |  |[default to false]
**with_plugin_info** | Option<**bool**> |  |  |[default to false]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::PaymentMethod**](PaymentMethod.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_method_audit_logs_with_history

> Vec<models::AuditLog> get_payment_method_audit_logs_with_history(payment_method_id)
Retrieve payment method audit logs with history by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_method_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_method_by_key

> models::PaymentMethod get_payment_method_by_key(external_key, included_deleted, with_plugin_info, plugin_property, audit)
Retrieve a payment method by external key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_key** | **String** |  | [required] |
**included_deleted** | Option<**bool**> |  |  |[default to false]
**with_plugin_info** | Option<**bool**> |  |  |[default to false]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::PaymentMethod**](PaymentMethod.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_method_custom_fields

> Vec<models::CustomField> get_payment_method_custom_fields(payment_method_id, audit)
Retrieve payment method custom fields

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_method_id** | **uuid::Uuid** |  | [required] |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::CustomField>**](CustomField.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_methods

> Vec<models::PaymentMethod> get_payment_methods(offset, limit, plugin_name, with_plugin_info, plugin_property, audit)
List payment methods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i64**> |  |  |[default to 0]
**limit** | Option<**i64**> |  |  |[default to 100]
**plugin_name** | Option<**String**> |  |  |
**with_plugin_info** | Option<**bool**> |  |  |[default to false]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::PaymentMethod>**](PaymentMethod.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_payment_method_custom_fields

> modify_payment_method_custom_fields(payment_method_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Modify custom fields to payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_method_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Vec<models::CustomField>**](CustomField.md) |  | [required] |
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


## search_payment_methods

> Vec<models::PaymentMethod> search_payment_methods(search_key, offset, limit, plugin_name, with_plugin_info, plugin_property, audit)
Search payment methods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_key** | **String** |  | [required] |
**offset** | Option<**i64**> |  |  |[default to 0]
**limit** | Option<**i64**> |  |  |[default to 100]
**plugin_name** | Option<**String**> |  |  |
**with_plugin_info** | Option<**bool**> |  |  |[default to false]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::PaymentMethod>**](PaymentMethod.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

