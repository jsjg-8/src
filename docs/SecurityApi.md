# \SecurityApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_role_definition**](SecurityApi.md#add_role_definition) | **POST** /1.0/kb/security/roles | Add a new role definition)
[**add_user_roles**](SecurityApi.md#add_user_roles) | **POST** /1.0/kb/security/users | Add a new user with roles (to make api requests)
[**get_current_user_permissions**](SecurityApi.md#get_current_user_permissions) | **GET** /1.0/kb/security/permissions | List user permissions
[**get_current_user_subject**](SecurityApi.md#get_current_user_subject) | **GET** /1.0/kb/security/subject | Get user information
[**get_role_definition**](SecurityApi.md#get_role_definition) | **GET** /1.0/kb/security/roles/{role} | Get role definition
[**get_user_roles**](SecurityApi.md#get_user_roles) | **GET** /1.0/kb/security/users/{username}/roles | Get roles associated to a user
[**invalidate_user**](SecurityApi.md#invalidate_user) | **DELETE** /1.0/kb/security/users/{username} | Invalidate an existing user
[**update_role_definition**](SecurityApi.md#update_role_definition) | **PUT** /1.0/kb/security/roles | Update a new role definition)
[**update_user_password**](SecurityApi.md#update_user_password) | **PUT** /1.0/kb/security/users/{username}/password | Update a user password
[**update_user_roles**](SecurityApi.md#update_user_roles) | **PUT** /1.0/kb/security/users/{username}/roles | Update roles associated to a user



## add_role_definition

> models::RoleDefinition add_role_definition(x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add a new role definition)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**RoleDefinition**](RoleDefinition.md) |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::RoleDefinition**](RoleDefinition.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user_roles

> models::UserRoles add_user_roles(x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add a new user with roles (to make api requests)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**UserRoles**](UserRoles.md) |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::UserRoles**](UserRoles.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user_permissions

> Vec<String> get_current_user_permissions()
List user permissions

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user_subject

> models::Subject get_current_user_subject()
Get user information

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Subject**](Subject.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_definition

> models::RoleDefinition get_role_definition(role)
Get role definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** |  | [required] |

### Return type

[**models::RoleDefinition**](RoleDefinition.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_roles

> models::UserRoles get_user_roles(username)
Get roles associated to a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**models::UserRoles**](UserRoles.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invalidate_user

> invalidate_user(username, x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Invalidate an existing user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role_definition

> update_role_definition(x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Update a new role definition)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**RoleDefinition**](RoleDefinition.md) |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_password

> update_user_password(username, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Update a user password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**UserRoles**](UserRoles.md) |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_roles

> update_user_roles(username, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Update roles associated to a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**UserRoles**](UserRoles.md) |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

