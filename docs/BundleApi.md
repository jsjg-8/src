# \BundleApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_bundle_blocking_state**](BundleApi.md#add_bundle_blocking_state) | **POST** /1.0/kb/bundles/{bundleId}/block | Block a bundle
[**create_bundle_custom_fields**](BundleApi.md#create_bundle_custom_fields) | **POST** /1.0/kb/bundles/{bundleId}/customFields | Add custom fields to bundle
[**create_bundle_tags**](BundleApi.md#create_bundle_tags) | **POST** /1.0/kb/bundles/{bundleId}/tags | Add tags to bundle
[**delete_bundle_custom_fields**](BundleApi.md#delete_bundle_custom_fields) | **DELETE** /1.0/kb/bundles/{bundleId}/customFields | Remove custom fields from bundle
[**delete_bundle_tags**](BundleApi.md#delete_bundle_tags) | **DELETE** /1.0/kb/bundles/{bundleId}/tags | Remove tags from bundle
[**get_bundle**](BundleApi.md#get_bundle) | **GET** /1.0/kb/bundles/{bundleId} | Retrieve a bundle by id
[**get_bundle_audit_logs_with_history**](BundleApi.md#get_bundle_audit_logs_with_history) | **GET** /1.0/kb/bundles/{bundleId}/auditLogsWithHistory | Retrieve bundle audit logs with history by id
[**get_bundle_by_key**](BundleApi.md#get_bundle_by_key) | **GET** /1.0/kb/bundles | Retrieve a bundle by external key
[**get_bundle_custom_fields**](BundleApi.md#get_bundle_custom_fields) | **GET** /1.0/kb/bundles/{bundleId}/customFields | Retrieve bundle custom fields
[**get_bundle_tags**](BundleApi.md#get_bundle_tags) | **GET** /1.0/kb/bundles/{bundleId}/tags | Retrieve bundle tags
[**get_bundles**](BundleApi.md#get_bundles) | **GET** /1.0/kb/bundles/pagination | List bundles
[**modify_bundle_custom_fields**](BundleApi.md#modify_bundle_custom_fields) | **PUT** /1.0/kb/bundles/{bundleId}/customFields | Modify custom fields to bundle
[**pause_bundle**](BundleApi.md#pause_bundle) | **PUT** /1.0/kb/bundles/{bundleId}/pause | Pause a bundle
[**rename_external_key**](BundleApi.md#rename_external_key) | **PUT** /1.0/kb/bundles/{bundleId}/renameKey | Update a bundle externalKey
[**resume_bundle**](BundleApi.md#resume_bundle) | **PUT** /1.0/kb/bundles/{bundleId}/resume | Resume a bundle
[**search_bundles**](BundleApi.md#search_bundles) | **GET** /1.0/kb/bundles/search/{searchKey} | Search bundles
[**transfer_bundle**](BundleApi.md#transfer_bundle) | **POST** /1.0/kb/bundles/{bundleId} | Transfer a bundle to another account



## add_bundle_blocking_state

> Vec<models::BlockingState> add_bundle_blocking_state(bundle_id, x_killbill_created_by, body, requested_date, plugin_property, x_killbill_reason, x_killbill_comment)
Block a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |
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


## create_bundle_custom_fields

> Vec<models::CustomField> create_bundle_custom_fields(bundle_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add custom fields to bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |
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


## create_bundle_tags

> Vec<models::Tag> create_bundle_tags(bundle_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add tags to bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |
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


## delete_bundle_custom_fields

> delete_bundle_custom_fields(bundle_id, x_killbill_created_by, custom_field, x_killbill_reason, x_killbill_comment)
Remove custom fields from bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |
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


## delete_bundle_tags

> delete_bundle_tags(bundle_id, x_killbill_created_by, tag_def, x_killbill_reason, x_killbill_comment)
Remove tags from bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |
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


## get_bundle

> models::Bundle get_bundle(bundle_id, audit)
Retrieve a bundle by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::Bundle**](Bundle.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bundle_audit_logs_with_history

> Vec<models::AuditLog> get_bundle_audit_logs_with_history(bundle_id)
Retrieve bundle audit logs with history by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bundle_by_key

> Vec<models::Bundle> get_bundle_by_key(external_key, included_deleted, audit)
Retrieve a bundle by external key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_key** | **String** |  | [required] |
**included_deleted** | Option<**bool**> |  |  |[default to false]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Bundle>**](Bundle.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bundle_custom_fields

> Vec<models::CustomField> get_bundle_custom_fields(bundle_id, audit)
Retrieve bundle custom fields

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::CustomField>**](CustomField.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bundle_tags

> Vec<models::Tag> get_bundle_tags(bundle_id, included_deleted, audit)
Retrieve bundle tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |
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


## get_bundles

> Vec<models::Bundle> get_bundles(offset, limit, audit)
List bundles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i64**> |  |  |[default to 0]
**limit** | Option<**i64**> |  |  |[default to 100]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Bundle>**](Bundle.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_bundle_custom_fields

> modify_bundle_custom_fields(bundle_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Modify custom fields to bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |
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


## pause_bundle

> pause_bundle(bundle_id, x_killbill_created_by, requested_date, plugin_property, x_killbill_reason, x_killbill_comment)
Pause a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**requested_date** | Option<**String**> |  |  |
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


## rename_external_key

> rename_external_key(bundle_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Update a bundle externalKey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Bundle**](Bundle.md) |  | [required] |
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


## resume_bundle

> resume_bundle(bundle_id, x_killbill_created_by, requested_date, plugin_property, x_killbill_reason, x_killbill_comment)
Resume a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**requested_date** | Option<**String**> |  |  |
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


## search_bundles

> Vec<models::Bundle> search_bundles(search_key, offset, limit, audit)
Search bundles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_key** | **String** |  | [required] |
**offset** | Option<**i64**> |  |  |[default to 0]
**limit** | Option<**i64**> |  |  |[default to 100]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Bundle>**](Bundle.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_bundle

> models::Bundle transfer_bundle(bundle_id, x_killbill_created_by, body, requested_date, billing_policy, bcd_transfer, plugin_property, x_killbill_reason, x_killbill_comment)
Transfer a bundle to another account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Bundle**](Bundle.md) |  | [required] |
**requested_date** | Option<**String**> |  |  |
**billing_policy** | Option<**String**> |  |  |[default to END_OF_TERM]
**bcd_transfer** | Option<**String**> |  |  |[default to USE_EXISTING]
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

