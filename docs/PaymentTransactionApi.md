# \PaymentTransactionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_transaction_custom_fields**](PaymentTransactionApi.md#create_transaction_custom_fields) | **POST** /1.0/kb/paymentTransactions/{transactionId}/customFields | Add custom fields to payment transaction
[**create_transaction_tags**](PaymentTransactionApi.md#create_transaction_tags) | **POST** /1.0/kb/paymentTransactions/{transactionId}/tags | Add tags to payment transaction
[**delete_transaction_custom_fields**](PaymentTransactionApi.md#delete_transaction_custom_fields) | **DELETE** /1.0/kb/paymentTransactions/{transactionId}/customFields | Remove custom fields from payment transaction
[**delete_transaction_tags**](PaymentTransactionApi.md#delete_transaction_tags) | **DELETE** /1.0/kb/paymentTransactions/{transactionId}/tags | Remove tags from payment transaction
[**get_payment_by_transaction_external_key**](PaymentTransactionApi.md#get_payment_by_transaction_external_key) | **GET** /1.0/kb/paymentTransactions | Retrieve a payment by transaction external key
[**get_payment_by_transaction_id**](PaymentTransactionApi.md#get_payment_by_transaction_id) | **GET** /1.0/kb/paymentTransactions/{transactionId} | Retrieve a payment by transaction id
[**get_transaction_audit_logs_with_history**](PaymentTransactionApi.md#get_transaction_audit_logs_with_history) | **GET** /1.0/kb/paymentTransactions/{transactionId}/auditLogsWithHistory | Retrieve payment transaction audit logs with history by id
[**get_transaction_custom_fields**](PaymentTransactionApi.md#get_transaction_custom_fields) | **GET** /1.0/kb/paymentTransactions/{transactionId}/customFields | Retrieve payment transaction custom fields
[**get_transaction_tags**](PaymentTransactionApi.md#get_transaction_tags) | **GET** /1.0/kb/paymentTransactions/{transactionId}/tags | Retrieve payment transaction tags
[**modify_transaction_custom_fields**](PaymentTransactionApi.md#modify_transaction_custom_fields) | **PUT** /1.0/kb/paymentTransactions/{transactionId}/customFields | Modify custom fields to payment transaction
[**notify_state_changed**](PaymentTransactionApi.md#notify_state_changed) | **POST** /1.0/kb/paymentTransactions/{transactionId} | Mark a pending payment transaction as succeeded or failed



## create_transaction_custom_fields

> Vec<models::CustomField> create_transaction_custom_fields(transaction_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add custom fields to payment transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **uuid::Uuid** |  | [required] |
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


## create_transaction_tags

> Vec<models::Tag> create_transaction_tags(transaction_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add tags to payment transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**Vec<models::Tag>**](Tag.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_transaction_custom_fields

> delete_transaction_custom_fields(transaction_id, x_killbill_created_by, custom_field, x_killbill_reason, x_killbill_comment)
Remove custom fields from payment transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **uuid::Uuid** |  | [required] |
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


## delete_transaction_tags

> delete_transaction_tags(transaction_id, x_killbill_created_by, tag_def, x_killbill_reason, x_killbill_comment)
Remove tags from payment transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**tag_def** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
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


## get_payment_by_transaction_external_key

> models::Payment get_payment_by_transaction_external_key(transaction_external_key, with_plugin_info, with_attempts, plugin_property, audit)
Retrieve a payment by transaction external key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_external_key** | **String** |  | [required] |
**with_plugin_info** | Option<**bool**> |  |  |[default to false]
**with_attempts** | Option<**bool**> |  |  |[default to false]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::Payment**](Payment.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_by_transaction_id

> models::Payment get_payment_by_transaction_id(transaction_id, with_plugin_info, with_attempts, plugin_property, audit)
Retrieve a payment by transaction id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **uuid::Uuid** |  | [required] |
**with_plugin_info** | Option<**bool**> |  |  |[default to false]
**with_attempts** | Option<**bool**> |  |  |[default to false]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::Payment**](Payment.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_audit_logs_with_history

> Vec<models::AuditLog> get_transaction_audit_logs_with_history(transaction_id)
Retrieve payment transaction audit logs with history by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_custom_fields

> Vec<models::CustomField> get_transaction_custom_fields(transaction_id, audit)
Retrieve payment transaction custom fields

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **uuid::Uuid** |  | [required] |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::CustomField>**](CustomField.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_tags

> Vec<models::Tag> get_transaction_tags(transaction_id, included_deleted, audit)
Retrieve payment transaction tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **uuid::Uuid** |  | [required] |
**included_deleted** | Option<**bool**> |  |  |[default to false]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Tag>**](Tag.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_transaction_custom_fields

> modify_transaction_custom_fields(transaction_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Modify custom fields to payment transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **uuid::Uuid** |  | [required] |
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


## notify_state_changed

> models::Payment notify_state_changed(transaction_id, x_killbill_created_by, body, control_plugin_name, x_killbill_reason, x_killbill_comment)
Mark a pending payment transaction as succeeded or failed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::Payment**](Payment.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

