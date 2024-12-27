# \InvoicePaymentApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**complete_invoice_payment_transaction**](InvoicePaymentApi.md#complete_invoice_payment_transaction) | **PUT** /1.0/kb/invoicePayments/{paymentId} | Complete an existing transaction
[**create_chargeback**](InvoicePaymentApi.md#create_chargeback) | **POST** /1.0/kb/invoicePayments/{paymentId}/chargebacks | Record a chargeback
[**create_chargeback_reversal**](InvoicePaymentApi.md#create_chargeback_reversal) | **POST** /1.0/kb/invoicePayments/{paymentId}/chargebackReversals | Record a chargebackReversal
[**create_invoice_payment_custom_fields**](InvoicePaymentApi.md#create_invoice_payment_custom_fields) | **POST** /1.0/kb/invoicePayments/{paymentId}/customFields | Add custom fields to payment
[**create_invoice_payment_tags**](InvoicePaymentApi.md#create_invoice_payment_tags) | **POST** /1.0/kb/invoicePayments/{paymentId}/tags | Add tags to payment
[**create_refund_with_adjustments**](InvoicePaymentApi.md#create_refund_with_adjustments) | **POST** /1.0/kb/invoicePayments/{paymentId}/refunds | Refund a payment, and adjust the invoice if needed
[**delete_invoice_payment_custom_fields**](InvoicePaymentApi.md#delete_invoice_payment_custom_fields) | **DELETE** /1.0/kb/invoicePayments/{paymentId}/customFields | Remove custom fields from payment
[**delete_invoice_payment_tags**](InvoicePaymentApi.md#delete_invoice_payment_tags) | **DELETE** /1.0/kb/invoicePayments/{paymentId}/tags | Remove tags from payment
[**get_invoice_payment**](InvoicePaymentApi.md#get_invoice_payment) | **GET** /1.0/kb/invoicePayments/{paymentId} | Retrieve a payment by id
[**get_invoice_payment_audit_logs_with_history**](InvoicePaymentApi.md#get_invoice_payment_audit_logs_with_history) | **GET** /1.0/kb/invoicePayments/{invoicePaymentId}/auditLogsWithHistory | Retrieve invoice payment audit logs with history by id
[**get_invoice_payment_custom_fields**](InvoicePaymentApi.md#get_invoice_payment_custom_fields) | **GET** /1.0/kb/invoicePayments/{paymentId}/customFields | Retrieve payment custom fields
[**get_invoice_payment_tags**](InvoicePaymentApi.md#get_invoice_payment_tags) | **GET** /1.0/kb/invoicePayments/{paymentId}/tags | Retrieve payment tags
[**modify_invoice_payment_custom_fields**](InvoicePaymentApi.md#modify_invoice_payment_custom_fields) | **PUT** /1.0/kb/invoicePayments/{paymentId}/customFields | Modify custom fields to payment



## complete_invoice_payment_transaction

> complete_invoice_payment_transaction(payment_id, x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
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


## create_chargeback

> models::InvoicePayment create_chargeback(payment_id, x_killbill_created_by, body, plugin_property, x_killbill_reason, x_killbill_comment)
Record a chargeback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**InvoicePaymentTransaction**](InvoicePaymentTransaction.md) |  | [required] |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::InvoicePayment**](InvoicePayment.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_chargeback_reversal

> models::InvoicePayment create_chargeback_reversal(payment_id, x_killbill_created_by, body, plugin_property, x_killbill_reason, x_killbill_comment)
Record a chargebackReversal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**InvoicePaymentTransaction**](InvoicePaymentTransaction.md) |  | [required] |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::InvoicePayment**](InvoicePayment.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_invoice_payment_custom_fields

> Vec<models::CustomField> create_invoice_payment_custom_fields(payment_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
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


## create_invoice_payment_tags

> Vec<models::Tag> create_invoice_payment_tags(payment_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add tags to payment

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


## create_refund_with_adjustments

> models::InvoicePayment create_refund_with_adjustments(payment_id, x_killbill_created_by, body, external_payment, payment_method_id, plugin_property, x_killbill_reason, x_killbill_comment)
Refund a payment, and adjust the invoice if needed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**InvoicePaymentTransaction**](InvoicePaymentTransaction.md) |  | [required] |
**external_payment** | Option<**bool**> |  |  |[default to false]
**payment_method_id** | Option<**uuid::Uuid**> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::InvoicePayment**](InvoicePayment.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_invoice_payment_custom_fields

> delete_invoice_payment_custom_fields(payment_id, x_killbill_created_by, custom_field, x_killbill_reason, x_killbill_comment)
Remove custom fields from payment

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


## delete_invoice_payment_tags

> delete_invoice_payment_tags(payment_id, x_killbill_created_by, tag_def, x_killbill_reason, x_killbill_comment)
Remove tags from payment

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


## get_invoice_payment

> models::InvoicePayment get_invoice_payment(payment_id, with_plugin_info, with_attempts, plugin_property, audit)
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

[**models::InvoicePayment**](InvoicePayment.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice_payment_audit_logs_with_history

> Vec<models::AuditLog> get_invoice_payment_audit_logs_with_history(invoice_payment_id)
Retrieve invoice payment audit logs with history by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_payment_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice_payment_custom_fields

> Vec<models::CustomField> get_invoice_payment_custom_fields(payment_id, audit)
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


## get_invoice_payment_tags

> Vec<models::Tag> get_invoice_payment_tags(payment_id, included_deleted, plugin_property, audit)
Retrieve payment tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
**included_deleted** | Option<**bool**> |  |  |[default to false]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Tag>**](Tag.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_invoice_payment_custom_fields

> modify_invoice_payment_custom_fields(payment_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
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

