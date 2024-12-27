# \PaymentApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_scheduled_payment_transaction_by_external_key**](PaymentApi.md#cancel_scheduled_payment_transaction_by_external_key) | **DELETE** /1.0/kb/payments/cancelScheduledPaymentTransaction | Cancels a scheduled payment attempt retry
[**cancel_scheduled_payment_transaction_by_id**](PaymentApi.md#cancel_scheduled_payment_transaction_by_id) | **DELETE** /1.0/kb/payments/{paymentTransactionId}/cancelScheduledPaymentTransaction | Cancels a scheduled payment attempt retry
[**capture_authorization**](PaymentApi.md#capture_authorization) | **POST** /1.0/kb/payments/{paymentId} | Capture an existing authorization
[**capture_authorization_by_external_key**](PaymentApi.md#capture_authorization_by_external_key) | **POST** /1.0/kb/payments | Capture an existing authorization
[**chargeback_payment**](PaymentApi.md#chargeback_payment) | **POST** /1.0/kb/payments/{paymentId}/chargebacks | Record a chargeback
[**chargeback_payment_by_external_key**](PaymentApi.md#chargeback_payment_by_external_key) | **POST** /1.0/kb/payments/chargebacks | Record a chargeback
[**chargeback_reversal_payment**](PaymentApi.md#chargeback_reversal_payment) | **POST** /1.0/kb/payments/{paymentId}/chargebackReversals | Record a chargeback reversal
[**chargeback_reversal_payment_by_external_key**](PaymentApi.md#chargeback_reversal_payment_by_external_key) | **POST** /1.0/kb/payments/chargebackReversals | Record a chargeback reversal
[**complete_transaction**](PaymentApi.md#complete_transaction) | **PUT** /1.0/kb/payments/{paymentId} | Complete an existing transaction
[**complete_transaction_by_external_key**](PaymentApi.md#complete_transaction_by_external_key) | **PUT** /1.0/kb/payments | Complete an existing transaction
[**create_combo_payment**](PaymentApi.md#create_combo_payment) | **POST** /1.0/kb/payments/combo | Combo api to create a new payment transaction on a existing (or not) account 
[**create_payment_custom_fields**](PaymentApi.md#create_payment_custom_fields) | **POST** /1.0/kb/payments/{paymentId}/customFields | Add custom fields to payment
[**create_payment_tags**](PaymentApi.md#create_payment_tags) | **POST** /1.0/kb/payments/{paymentId}/tags | Add tags to payment payment
[**delete_payment_custom_fields**](PaymentApi.md#delete_payment_custom_fields) | **DELETE** /1.0/kb/payments/{paymentId}/customFields | Remove custom fields from payment payment
[**delete_payment_tags**](PaymentApi.md#delete_payment_tags) | **DELETE** /1.0/kb/payments/{paymentId}/tags | Remove tags from payment payment
[**get_payment**](PaymentApi.md#get_payment) | **GET** /1.0/kb/payments/{paymentId} | Retrieve a payment by id
[**get_payment_attempt_audit_logs_with_history**](PaymentApi.md#get_payment_attempt_audit_logs_with_history) | **GET** /1.0/kb/payments/attempts/{paymentAttemptId}/auditLogsWithHistory | Retrieve payment attempt audit logs with history by id
[**get_payment_audit_logs_with_history**](PaymentApi.md#get_payment_audit_logs_with_history) | **GET** /1.0/kb/payments/{paymentId}/auditLogsWithHistory | Retrieve payment audit logs with history by id
[**get_payment_by_external_key**](PaymentApi.md#get_payment_by_external_key) | **GET** /1.0/kb/payments | Retrieve a payment by external key
[**get_payment_custom_fields**](PaymentApi.md#get_payment_custom_fields) | **GET** /1.0/kb/payments/{paymentId}/customFields | Retrieve payment custom fields
[**get_payment_tags**](PaymentApi.md#get_payment_tags) | **GET** /1.0/kb/payments/{paymentId}/tags | Retrieve payment payment tags
[**get_payments**](PaymentApi.md#get_payments) | **GET** /1.0/kb/payments/pagination | Get payments
[**modify_payment_custom_fields**](PaymentApi.md#modify_payment_custom_fields) | **PUT** /1.0/kb/payments/{paymentId}/customFields | Modify custom fields to payment
[**refund_payment**](PaymentApi.md#refund_payment) | **POST** /1.0/kb/payments/{paymentId}/refunds | Refund an existing payment
[**refund_payment_by_external_key**](PaymentApi.md#refund_payment_by_external_key) | **POST** /1.0/kb/payments/refunds | Refund an existing payment
[**search_payments**](PaymentApi.md#search_payments) | **GET** /1.0/kb/payments/search/{searchKey} | Search payments
[**void_payment**](PaymentApi.md#void_payment) | **DELETE** /1.0/kb/payments/{paymentId} | Void an existing payment
[**void_payment_by_external_key**](PaymentApi.md#void_payment_by_external_key) | **DELETE** /1.0/kb/payments | Void an existing payment



## cancel_scheduled_payment_transaction_by_external_key

> cancel_scheduled_payment_transaction_by_external_key(transaction_external_key, x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Cancels a scheduled payment attempt retry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_external_key** | **String** |  | [required] |
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


## cancel_scheduled_payment_transaction_by_id

> cancel_scheduled_payment_transaction_by_id(payment_transaction_id, x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Cancels a scheduled payment attempt retry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_transaction_id** | **uuid::Uuid** |  | [required] |
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


## capture_authorization

> models::Payment capture_authorization(payment_id, x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Capture an existing authorization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
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


## capture_authorization_by_external_key

> models::Payment capture_authorization_by_external_key(x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Capture an existing authorization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
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


## chargeback_payment

> models::Payment chargeback_payment(payment_id, x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Record a chargeback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
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


## chargeback_payment_by_external_key

> models::Payment chargeback_payment_by_external_key(x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Record a chargeback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
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


## chargeback_reversal_payment

> models::Payment chargeback_reversal_payment(payment_id, x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Record a chargeback reversal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
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


## chargeback_reversal_payment_by_external_key

> models::Payment chargeback_reversal_payment_by_external_key(x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Record a chargeback reversal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
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


## complete_transaction

> complete_transaction(payment_id, x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Complete an existing transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
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


## complete_transaction_by_external_key

> complete_transaction_by_external_key(x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Complete an existing transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
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


## create_combo_payment

> models::Payment create_combo_payment(x_killbill_created_by, body, control_plugin_name, x_killbill_reason, x_killbill_comment)
Combo api to create a new payment transaction on a existing (or not) account 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**ComboPaymentTransaction**](ComboPaymentTransaction.md) |  | [required] |
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


## create_payment_custom_fields

> Vec<models::CustomField> create_payment_custom_fields(payment_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add custom fields to payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
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


## create_payment_tags

> Vec<models::Tag> create_payment_tags(payment_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add tags to payment payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
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


## delete_payment_custom_fields

> delete_payment_custom_fields(payment_id, x_killbill_created_by, custom_field, x_killbill_reason, x_killbill_comment)
Remove custom fields from payment payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
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


## delete_payment_tags

> delete_payment_tags(payment_id, x_killbill_created_by, tag_def, x_killbill_reason, x_killbill_comment)
Remove tags from payment payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
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


## get_payment

> models::Payment get_payment(payment_id, with_plugin_info, with_attempts, plugin_property, audit)
Retrieve a payment by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
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


## get_payment_attempt_audit_logs_with_history

> Vec<models::AuditLog> get_payment_attempt_audit_logs_with_history(payment_attempt_id)
Retrieve payment attempt audit logs with history by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_attempt_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_audit_logs_with_history

> Vec<models::AuditLog> get_payment_audit_logs_with_history(payment_id)
Retrieve payment audit logs with history by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_by_external_key

> models::Payment get_payment_by_external_key(external_key, with_plugin_info, with_attempts, plugin_property, audit)
Retrieve a payment by external key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_key** | **String** |  | [required] |
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


## get_payment_custom_fields

> Vec<models::CustomField> get_payment_custom_fields(payment_id, audit)
Retrieve payment custom fields

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::CustomField>**](CustomField.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_tags

> Vec<models::Tag> get_payment_tags(payment_id, included_deleted, audit)
Retrieve payment payment tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
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


## get_payments

> Vec<models::Payment> get_payments(offset, limit, plugin_name, with_plugin_info, with_attempts, plugin_property, audit)
Get payments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i64**> |  |  |[default to 0]
**limit** | Option<**i64**> |  |  |[default to 100]
**plugin_name** | Option<**String**> |  |  |
**with_plugin_info** | Option<**bool**> |  |  |[default to false]
**with_attempts** | Option<**bool**> |  |  |[default to false]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Payment>**](Payment.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_payment_custom_fields

> modify_payment_custom_fields(payment_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Modify custom fields to payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
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


## refund_payment

> models::Payment refund_payment(payment_id, x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Refund an existing payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
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


## refund_payment_by_external_key

> models::Payment refund_payment_by_external_key(x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Refund an existing payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
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


## search_payments

> Vec<models::Payment> search_payments(search_key, offset, limit, with_plugin_info, with_attempts, plugin_name, plugin_property, audit)
Search payments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_key** | **String** |  | [required] |
**offset** | Option<**i64**> |  |  |[default to 0]
**limit** | Option<**i64**> |  |  |[default to 100]
**with_plugin_info** | Option<**bool**> |  |  |[default to false]
**with_attempts** | Option<**bool**> |  |  |[default to false]
**plugin_name** | Option<**String**> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Payment>**](Payment.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## void_payment

> void_payment(payment_id, x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Void an existing payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
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


## void_payment_by_external_key

> void_payment_by_external_key(x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Void an existing payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
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

