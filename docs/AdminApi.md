# \AdminApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_queue_entries**](AdminApi.md#get_queue_entries) | **GET** /1.0/kb/admin/queues | Get queues entries
[**invalidates_cache**](AdminApi.md#invalidates_cache) | **DELETE** /1.0/kb/admin/cache | Invalidates the given Cache if specified, otherwise invalidates all caches
[**invalidates_cache_by_account**](AdminApi.md#invalidates_cache_by_account) | **DELETE** /1.0/kb/admin/cache/accounts/{accountId} | Invalidates Caches per account level
[**invalidates_cache_by_tenant**](AdminApi.md#invalidates_cache_by_tenant) | **DELETE** /1.0/kb/admin/cache/tenants | Invalidates Caches per tenant level
[**put_in_rotation**](AdminApi.md#put_in_rotation) | **PUT** /1.0/kb/admin/healthcheck | Put the host back into rotation
[**put_out_of_rotation**](AdminApi.md#put_out_of_rotation) | **DELETE** /1.0/kb/admin/healthcheck | Put the host out of rotation
[**trigger_invoice_generation_for_parked_accounts**](AdminApi.md#trigger_invoice_generation_for_parked_accounts) | **POST** /1.0/kb/admin/invoices | Trigger an invoice generation for all parked accounts
[**update_payment_transaction_state**](AdminApi.md#update_payment_transaction_state) | **PUT** /1.0/kb/admin/payments/{paymentId}/transactions/{paymentTransactionId} | Update existing paymentTransaction and associated payment state



## get_queue_entries

> get_queue_entries(account_id, queue_name, service_name, with_history, min_date, max_date, with_in_processing, with_bus_events, with_notifications)
Get queues entries

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**uuid::Uuid**> |  |  |
**queue_name** | Option<**String**> |  |  |
**service_name** | Option<**String**> |  |  |
**with_history** | Option<**bool**> |  |  |[default to true]
**min_date** | Option<**String**> |  |  |
**max_date** | Option<**String**> |  |  |
**with_in_processing** | Option<**bool**> |  |  |[default to true]
**with_bus_events** | Option<**bool**> |  |  |[default to true]
**with_notifications** | Option<**bool**> |  |  |[default to true]

### Return type

 (empty response body)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invalidates_cache

> invalidates_cache(cache_name)
Invalidates the given Cache if specified, otherwise invalidates all caches

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cache_name** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invalidates_cache_by_account

> invalidates_cache_by_account(account_id)
Invalidates Caches per account level

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invalidates_cache_by_tenant

> invalidates_cache_by_tenant()
Invalidates Caches per tenant level

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_in_rotation

> put_in_rotation()
Put the host back into rotation

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_out_of_rotation

> put_out_of_rotation()
Put the host out of rotation

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_invoice_generation_for_parked_accounts

> trigger_invoice_generation_for_parked_accounts(x_killbill_created_by, offset, limit, plugin_property, x_killbill_reason, x_killbill_comment)
Trigger an invoice generation for all parked accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**offset** | Option<**i64**> |  |  |[default to 0]
**limit** | Option<**i64**> |  |  |[default to 100]
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


## update_payment_transaction_state

> update_payment_transaction_state(payment_id, payment_transaction_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Update existing paymentTransaction and associated payment state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **uuid::Uuid** |  | [required] |
**payment_transaction_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**AdminPayment**](AdminPayment.md) |  | [required] |
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

