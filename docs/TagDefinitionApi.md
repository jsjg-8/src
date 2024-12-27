# \TagDefinitionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tag_definition**](TagDefinitionApi.md#create_tag_definition) | **POST** /1.0/kb/tagDefinitions | Create a tag definition
[**delete_tag_definition**](TagDefinitionApi.md#delete_tag_definition) | **DELETE** /1.0/kb/tagDefinitions/{tagDefinitionId} | Delete a tag definition
[**get_tag_definition**](TagDefinitionApi.md#get_tag_definition) | **GET** /1.0/kb/tagDefinitions/{tagDefinitionId} | Retrieve a tag definition
[**get_tag_definition_audit_logs_with_history**](TagDefinitionApi.md#get_tag_definition_audit_logs_with_history) | **GET** /1.0/kb/tagDefinitions/{tagDefinitionId}/auditLogsWithHistory | Retrieve tag definition audit logs with history by id
[**get_tag_definitions**](TagDefinitionApi.md#get_tag_definitions) | **GET** /1.0/kb/tagDefinitions | List tag definitions



## create_tag_definition

> models::TagDefinition create_tag_definition(x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Create a tag definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**TagDefinition**](TagDefinition.md) |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::TagDefinition**](TagDefinition.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tag_definition

> delete_tag_definition(tag_definition_id, x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Delete a tag definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_definition_id** | **uuid::Uuid** |  | [required] |
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


## get_tag_definition

> models::TagDefinition get_tag_definition(tag_definition_id, audit)
Retrieve a tag definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_definition_id** | **uuid::Uuid** |  | [required] |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::TagDefinition**](TagDefinition.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag_definition_audit_logs_with_history

> Vec<models::AuditLog> get_tag_definition_audit_logs_with_history(tag_definition_id)
Retrieve tag definition audit logs with history by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_definition_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag_definitions

> Vec<models::TagDefinition> get_tag_definitions(audit)
List tag definitions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::TagDefinition>**](TagDefinition.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

