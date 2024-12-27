# \SubscriptionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_subscription_blocking_state**](SubscriptionApi.md#add_subscription_blocking_state) | **POST** /1.0/kb/subscriptions/{subscriptionId}/block | Block a subscription
[**cancel_subscription_plan**](SubscriptionApi.md#cancel_subscription_plan) | **DELETE** /1.0/kb/subscriptions/{subscriptionId} | Cancel an entitlement plan
[**change_subscription_plan**](SubscriptionApi.md#change_subscription_plan) | **PUT** /1.0/kb/subscriptions/{subscriptionId} | Change entitlement plan
[**create_subscription**](SubscriptionApi.md#create_subscription) | **POST** /1.0/kb/subscriptions | Create an subscription
[**create_subscription_custom_fields**](SubscriptionApi.md#create_subscription_custom_fields) | **POST** /1.0/kb/subscriptions/{subscriptionId}/customFields | Add custom fields to subscription
[**create_subscription_tags**](SubscriptionApi.md#create_subscription_tags) | **POST** /1.0/kb/subscriptions/{subscriptionId}/tags | 
[**create_subscription_with_add_ons**](SubscriptionApi.md#create_subscription_with_add_ons) | **POST** /1.0/kb/subscriptions/createSubscriptionWithAddOns | Create an entitlement with addOn products
[**create_subscriptions_with_add_ons**](SubscriptionApi.md#create_subscriptions_with_add_ons) | **POST** /1.0/kb/subscriptions/createSubscriptionsWithAddOns | Create multiple entitlements with addOn products
[**delete_subscription_custom_fields**](SubscriptionApi.md#delete_subscription_custom_fields) | **DELETE** /1.0/kb/subscriptions/{subscriptionId}/customFields | Remove custom fields from subscription
[**delete_subscription_tags**](SubscriptionApi.md#delete_subscription_tags) | **DELETE** /1.0/kb/subscriptions/{subscriptionId}/tags | Remove tags from subscription
[**get_subscription**](SubscriptionApi.md#get_subscription) | **GET** /1.0/kb/subscriptions/{subscriptionId} | Retrieve a subscription by id
[**get_subscription_audit_logs_with_history**](SubscriptionApi.md#get_subscription_audit_logs_with_history) | **GET** /1.0/kb/subscriptions/{subscriptionId}/auditLogsWithHistory | Retrieve subscription audit logs with history by id
[**get_subscription_by_key**](SubscriptionApi.md#get_subscription_by_key) | **GET** /1.0/kb/subscriptions | Retrieve a subscription by external key
[**get_subscription_custom_fields**](SubscriptionApi.md#get_subscription_custom_fields) | **GET** /1.0/kb/subscriptions/{subscriptionId}/customFields | Retrieve subscription custom fields
[**get_subscription_event_audit_logs_with_history**](SubscriptionApi.md#get_subscription_event_audit_logs_with_history) | **GET** /1.0/kb/subscriptions/events/{eventId}/auditLogsWithHistory | Retrieve subscription event audit logs with history by id
[**get_subscription_tags**](SubscriptionApi.md#get_subscription_tags) | **GET** /1.0/kb/subscriptions/{subscriptionId}/tags | Retrieve subscription tags
[**modify_subscription_custom_fields**](SubscriptionApi.md#modify_subscription_custom_fields) | **PUT** /1.0/kb/subscriptions/{subscriptionId}/customFields | Modify custom fields to subscription
[**uncancel_subscription_plan**](SubscriptionApi.md#uncancel_subscription_plan) | **PUT** /1.0/kb/subscriptions/{subscriptionId}/uncancel | Un-cancel an entitlement
[**undo_change_subscription_plan**](SubscriptionApi.md#undo_change_subscription_plan) | **PUT** /1.0/kb/subscriptions/{subscriptionId}/undoChangePlan | Undo a pending change plan on an entitlement
[**update_subscription_bcd**](SubscriptionApi.md#update_subscription_bcd) | **PUT** /1.0/kb/subscriptions/{subscriptionId}/bcd | Update the BCD associated to a subscription
[**update_subscription_quantity**](SubscriptionApi.md#update_subscription_quantity) | **PUT** /1.0/kb/subscriptions/{subscriptionId}/quantity | Update the quantity associated to a subscription



## add_subscription_blocking_state

> Vec<models::BlockingState> add_subscription_blocking_state(subscription_id, x_killbill_created_by, body, requested_date, plugin_property, x_killbill_reason, x_killbill_comment)
Block a subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**BlockingState**](BlockingState.md) |  | [required] |
**requested_date** | Option<**String**> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**Vec<models::BlockingState>**](BlockingState.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_subscription_plan

> cancel_subscription_plan(subscription_id, x_killbill_created_by, requested_date, call_completion, call_timeout_sec, entitlement_policy, billing_policy, use_requested_date_for_billing, plugin_property, x_killbill_reason, x_killbill_comment)
Cancel an entitlement plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**requested_date** | Option<**String**> |  |  |
**call_completion** | Option<**bool**> |  |  |[default to false]
**call_timeout_sec** | Option<**i64**> |  |  |[default to 5]
**entitlement_policy** | Option<**String**> |  |  |
**billing_policy** | Option<**String**> |  |  |
**use_requested_date_for_billing** | Option<**bool**> |  |  |[default to false]
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


## change_subscription_plan

> change_subscription_plan(subscription_id, x_killbill_created_by, body, requested_date, call_completion, call_timeout_sec, billing_policy, plugin_property, x_killbill_reason, x_killbill_comment)
Change entitlement plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Subscription**](Subscription.md) |  | [required] |
**requested_date** | Option<**String**> |  |  |
**call_completion** | Option<**bool**> |  |  |[default to false]
**call_timeout_sec** | Option<**i64**> |  |  |[default to 3]
**billing_policy** | Option<**String**> |  |  |
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


## create_subscription

> models::Subscription create_subscription(x_killbill_created_by, body, entitlement_date, billing_date, rename_key_if_exists_and_unused, migrated, skip_response, call_completion, call_timeout_sec, plugin_property, x_killbill_reason, x_killbill_comment)
Create an subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Subscription**](Subscription.md) |  | [required] |
**entitlement_date** | Option<**String**> |  |  |
**billing_date** | Option<**String**> |  |  |
**rename_key_if_exists_and_unused** | Option<**bool**> |  |  |[default to true]
**migrated** | Option<**bool**> |  |  |[default to false]
**skip_response** | Option<**bool**> |  |  |[default to false]
**call_completion** | Option<**bool**> |  |  |[default to false]
**call_timeout_sec** | Option<**i64**> |  |  |[default to 3]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subscription_custom_fields

> create_subscription_custom_fields(subscription_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add custom fields to subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
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


## create_subscription_tags

> create_subscription_tags(subscription_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) |  | [required] |
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


## create_subscription_with_add_ons

> models::Bundle create_subscription_with_add_ons(x_killbill_created_by, body, entitlement_date, billing_date, migrated, skip_response, rename_key_if_exists_and_unused, call_completion, call_timeout_sec, plugin_property, x_killbill_reason, x_killbill_comment)
Create an entitlement with addOn products

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Vec<models::Subscription>**](Subscription.md) |  | [required] |
**entitlement_date** | Option<**String**> |  |  |
**billing_date** | Option<**String**> |  |  |
**migrated** | Option<**bool**> |  |  |[default to false]
**skip_response** | Option<**bool**> |  |  |[default to false]
**rename_key_if_exists_and_unused** | Option<**bool**> |  |  |[default to true]
**call_completion** | Option<**bool**> |  |  |[default to false]
**call_timeout_sec** | Option<**i64**> |  |  |[default to 3]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::Bundle**](Bundle.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subscriptions_with_add_ons

> Vec<models::Bundle> create_subscriptions_with_add_ons(x_killbill_created_by, body, entitlement_date, billing_date, rename_key_if_exists_and_unused, migrated, skip_response, call_completion, call_timeout_sec, plugin_property, x_killbill_reason, x_killbill_comment)
Create multiple entitlements with addOn products

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Vec<models::BulkSubscriptionsBundle>**](BulkSubscriptionsBundle.md) |  | [required] |
**entitlement_date** | Option<**String**> |  |  |
**billing_date** | Option<**String**> |  |  |
**rename_key_if_exists_and_unused** | Option<**bool**> |  |  |[default to true]
**migrated** | Option<**bool**> |  |  |[default to false]
**skip_response** | Option<**bool**> |  |  |[default to false]
**call_completion** | Option<**bool**> |  |  |[default to false]
**call_timeout_sec** | Option<**i64**> |  |  |[default to 3]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**Vec<models::Bundle>**](Bundle.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subscription_custom_fields

> delete_subscription_custom_fields(subscription_id, x_killbill_created_by, custom_field, x_killbill_reason, x_killbill_comment)
Remove custom fields from subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
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


## delete_subscription_tags

> delete_subscription_tags(subscription_id, x_killbill_created_by, tag_def, x_killbill_reason, x_killbill_comment)
Remove tags from subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
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


## get_subscription

> models::Subscription get_subscription(subscription_id, audit)
Retrieve a subscription by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_audit_logs_with_history

> Vec<models::AuditLog> get_subscription_audit_logs_with_history(subscription_id)
Retrieve subscription audit logs with history by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_by_key

> models::Subscription get_subscription_by_key(external_key, audit)
Retrieve a subscription by external key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_key** | **String** |  | [required] |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_custom_fields

> Vec<models::CustomField> get_subscription_custom_fields(subscription_id, audit)
Retrieve subscription custom fields

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::CustomField>**](CustomField.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_event_audit_logs_with_history

> Vec<models::AuditLog> get_subscription_event_audit_logs_with_history(event_id)
Retrieve subscription event audit logs with history by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_tags

> Vec<models::Tag> get_subscription_tags(subscription_id, included_deleted, audit)
Retrieve subscription tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
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


## modify_subscription_custom_fields

> modify_subscription_custom_fields(subscription_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Modify custom fields to subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
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


## uncancel_subscription_plan

> uncancel_subscription_plan(subscription_id, x_killbill_created_by, plugin_property, x_killbill_reason, x_killbill_comment)
Un-cancel an entitlement

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
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


## undo_change_subscription_plan

> undo_change_subscription_plan(subscription_id, x_killbill_created_by, plugin_property, x_killbill_reason, x_killbill_comment)
Undo a pending change plan on an entitlement

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
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


## update_subscription_bcd

> update_subscription_bcd(subscription_id, x_killbill_created_by, body, effective_from_date, force_new_bcd_with_past_effective_date, x_killbill_reason, x_killbill_comment)
Update the BCD associated to a subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Subscription**](Subscription.md) |  | [required] |
**effective_from_date** | Option<**String**> |  |  |
**force_new_bcd_with_past_effective_date** | Option<**bool**> |  |  |[default to false]
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


## update_subscription_quantity

> update_subscription_quantity(subscription_id, x_killbill_created_by, body, effective_from_date, force_new_quantity_with_past_effective_date, x_killbill_reason, x_killbill_comment)
Update the quantity associated to a subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Subscription**](Subscription.md) |  | [required] |
**effective_from_date** | Option<**String**> |  |  |
**force_new_quantity_with_past_effective_date** | Option<**bool**> |  |  |[default to false]
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

