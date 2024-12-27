# \PaymentGatewayApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**build_combo_form_descriptor**](PaymentGatewayApi.md#build_combo_form_descriptor) | **POST** /1.0/kb/paymentGateways/hosted/form | Combo API to generate form data to redirect the customer to the gateway
[**build_form_descriptor**](PaymentGatewayApi.md#build_form_descriptor) | **POST** /1.0/kb/paymentGateways/hosted/form/{accountId} | Generate form data to redirect the customer to the gateway
[**process_notification**](PaymentGatewayApi.md#process_notification) | **POST** /1.0/kb/paymentGateways/notification/{pluginName} | Process a gateway notification



## build_combo_form_descriptor

> models::HostedPaymentPageFormDescriptor build_combo_form_descriptor(x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Combo API to generate form data to redirect the customer to the gateway

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**ComboHostedPaymentPage**](ComboHostedPaymentPage.md) |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::HostedPaymentPageFormDescriptor**](HostedPaymentPageFormDescriptor.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_form_descriptor

> models::HostedPaymentPageFormDescriptor build_form_descriptor(account_id, x_killbill_created_by, body, payment_method_id, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Generate form data to redirect the customer to the gateway

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**HostedPaymentPageFields**](HostedPaymentPageFields.md) |  | [required] |
**payment_method_id** | Option<**uuid::Uuid**> |  |  |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::HostedPaymentPageFormDescriptor**](HostedPaymentPageFormDescriptor.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## process_notification

> process_notification(plugin_name, x_killbill_created_by, body, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Process a gateway notification

The response is built by the appropriate plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_name** | **String** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | **String** |  | [required] |
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
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

