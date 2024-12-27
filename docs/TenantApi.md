# \TenantApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tenant**](TenantApi.md#create_tenant) | **POST** /1.0/kb/tenants | Create a tenant
[**delete_per_tenant_configuration**](TenantApi.md#delete_per_tenant_configuration) | **DELETE** /1.0/kb/tenants/uploadPerTenantConfig | Delete a per tenant configuration (system properties)
[**delete_plugin_configuration**](TenantApi.md#delete_plugin_configuration) | **DELETE** /1.0/kb/tenants/uploadPluginConfig/{pluginName} | Delete a per tenant configuration for a plugin
[**delete_plugin_payment_state_machine_config**](TenantApi.md#delete_plugin_payment_state_machine_config) | **DELETE** /1.0/kb/tenants/uploadPluginPaymentStateMachineConfig/{pluginName} | Delete a per tenant payment state machine for a plugin
[**delete_push_notification_callbacks**](TenantApi.md#delete_push_notification_callbacks) | **DELETE** /1.0/kb/tenants/registerNotificationCallback | Delete a push notification
[**delete_user_key_value**](TenantApi.md#delete_user_key_value) | **DELETE** /1.0/kb/tenants/userKeyValue/{keyName} | Delete  a per tenant user key/value
[**get_all_plugin_configuration**](TenantApi.md#get_all_plugin_configuration) | **GET** /1.0/kb/tenants/uploadPerTenantConfig/{keyPrefix}/search | Retrieve a per tenant key value based on key prefix
[**get_per_tenant_configuration**](TenantApi.md#get_per_tenant_configuration) | **GET** /1.0/kb/tenants/uploadPerTenantConfig | Retrieve a per tenant configuration (system properties)
[**get_plugin_configuration**](TenantApi.md#get_plugin_configuration) | **GET** /1.0/kb/tenants/uploadPluginConfig/{pluginName} | Retrieve a per tenant configuration for a plugin
[**get_plugin_payment_state_machine_config**](TenantApi.md#get_plugin_payment_state_machine_config) | **GET** /1.0/kb/tenants/uploadPluginPaymentStateMachineConfig/{pluginName} | Retrieve a per tenant payment state machine for a plugin
[**get_push_notification_callbacks**](TenantApi.md#get_push_notification_callbacks) | **GET** /1.0/kb/tenants/registerNotificationCallback | Retrieve a push notification
[**get_tenant**](TenantApi.md#get_tenant) | **GET** /1.0/kb/tenants/{tenantId} | Retrieve a tenant by id
[**get_tenant_by_api_key**](TenantApi.md#get_tenant_by_api_key) | **GET** /1.0/kb/tenants | Retrieve a tenant by its API key
[**get_user_key_value**](TenantApi.md#get_user_key_value) | **GET** /1.0/kb/tenants/userKeyValue/{keyName} | Retrieve a per tenant user key/value
[**insert_user_key_value**](TenantApi.md#insert_user_key_value) | **POST** /1.0/kb/tenants/userKeyValue/{keyName} | Add a per tenant user key/value
[**register_push_notification_callback**](TenantApi.md#register_push_notification_callback) | **POST** /1.0/kb/tenants/registerNotificationCallback | Create a push notification
[**upload_per_tenant_configuration**](TenantApi.md#upload_per_tenant_configuration) | **POST** /1.0/kb/tenants/uploadPerTenantConfig | Add a per tenant configuration (system properties)
[**upload_plugin_configuration**](TenantApi.md#upload_plugin_configuration) | **POST** /1.0/kb/tenants/uploadPluginConfig/{pluginName} | Add a per tenant configuration for a plugin
[**upload_plugin_payment_state_machine_config**](TenantApi.md#upload_plugin_payment_state_machine_config) | **POST** /1.0/kb/tenants/uploadPluginPaymentStateMachineConfig/{pluginName} | Add a per tenant payment state machine for a plugin



## create_tenant

> models::Tenant create_tenant(x_killbill_created_by, body, use_global_default, x_killbill_reason, x_killbill_comment)
Create a tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Tenant**](Tenant.md) |  | [required] |
**use_global_default** | Option<**bool**> |  |  |[default to false]
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::Tenant**](Tenant.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_per_tenant_configuration

> delete_per_tenant_configuration(x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Delete a per tenant configuration (system properties)

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


## delete_plugin_configuration

> delete_plugin_configuration(plugin_name, x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Delete a per tenant configuration for a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_name** | **String** |  | [required] |
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


## delete_plugin_payment_state_machine_config

> delete_plugin_payment_state_machine_config(plugin_name, x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Delete a per tenant payment state machine for a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_name** | **String** |  | [required] |
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


## delete_push_notification_callbacks

> delete_push_notification_callbacks(x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Delete a push notification

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


## delete_user_key_value

> delete_user_key_value(key_name, x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Delete  a per tenant user key/value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_name** | **String** |  | [required] |
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


## get_all_plugin_configuration

> Vec<models::TenantKeyValue> get_all_plugin_configuration(key_prefix)
Retrieve a per tenant key value based on key prefix

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_prefix** | **String** |  | [required] |

### Return type

[**Vec<models::TenantKeyValue>**](TenantKeyValue.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_per_tenant_configuration

> models::TenantKeyValue get_per_tenant_configuration()
Retrieve a per tenant configuration (system properties)

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TenantKeyValue**](TenantKeyValue.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_configuration

> models::TenantKeyValue get_plugin_configuration(plugin_name)
Retrieve a per tenant configuration for a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_name** | **String** |  | [required] |

### Return type

[**models::TenantKeyValue**](TenantKeyValue.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_payment_state_machine_config

> models::TenantKeyValue get_plugin_payment_state_machine_config(plugin_name)
Retrieve a per tenant payment state machine for a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_name** | **String** |  | [required] |

### Return type

[**models::TenantKeyValue**](TenantKeyValue.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_push_notification_callbacks

> models::TenantKeyValue get_push_notification_callbacks()
Retrieve a push notification

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TenantKeyValue**](TenantKeyValue.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant

> models::Tenant get_tenant(tenant_id)
Retrieve a tenant by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Tenant**](Tenant.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_by_api_key

> models::Tenant get_tenant_by_api_key(api_key)
Retrieve a tenant by its API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | Option<**String**> |  |  |

### Return type

[**models::Tenant**](Tenant.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_key_value

> models::TenantKeyValue get_user_key_value(key_name)
Retrieve a per tenant user key/value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_name** | **String** |  | [required] |

### Return type

[**models::TenantKeyValue**](TenantKeyValue.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insert_user_key_value

> models::TenantKeyValue insert_user_key_value(key_name, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add a per tenant user key/value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_name** | **String** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | **String** |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::TenantKeyValue**](TenantKeyValue.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_push_notification_callback

> models::TenantKeyValue register_push_notification_callback(x_killbill_created_by, cb, x_killbill_reason, x_killbill_comment)
Create a push notification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**cb** | Option<**String**> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::TenantKeyValue**](TenantKeyValue.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_per_tenant_configuration

> models::TenantKeyValue upload_per_tenant_configuration(x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add a per tenant configuration (system properties)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | **String** |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::TenantKeyValue**](TenantKeyValue.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_plugin_configuration

> models::TenantKeyValue upload_plugin_configuration(plugin_name, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add a per tenant configuration for a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_name** | **String** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | **String** |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::TenantKeyValue**](TenantKeyValue.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_plugin_payment_state_machine_config

> models::TenantKeyValue upload_plugin_payment_state_machine_config(plugin_name, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add a per tenant payment state machine for a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_name** | **String** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | **String** |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::TenantKeyValue**](TenantKeyValue.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

