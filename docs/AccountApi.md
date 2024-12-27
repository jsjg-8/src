# \AccountApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_account_blocking_state**](AccountApi.md#add_account_blocking_state) | **POST** /1.0/kb/accounts/{accountId}/block | Block an account
[**add_email**](AccountApi.md#add_email) | **POST** /1.0/kb/accounts/{accountId}/emails | Add account email
[**close_account**](AccountApi.md#close_account) | **DELETE** /1.0/kb/accounts/{accountId} | Close account
[**create_account**](AccountApi.md#create_account) | **POST** /1.0/kb/accounts | Create account
[**create_account_custom_fields**](AccountApi.md#create_account_custom_fields) | **POST** /1.0/kb/accounts/{accountId}/customFields | Add custom fields to account
[**create_account_tags**](AccountApi.md#create_account_tags) | **POST** /1.0/kb/accounts/{accountId}/tags | Add tags to account
[**create_payment_method**](AccountApi.md#create_payment_method) | **POST** /1.0/kb/accounts/{accountId}/paymentMethods | Add a payment method
[**delete_account_custom_fields**](AccountApi.md#delete_account_custom_fields) | **DELETE** /1.0/kb/accounts/{accountId}/customFields | Remove custom fields from account
[**delete_account_tags**](AccountApi.md#delete_account_tags) | **DELETE** /1.0/kb/accounts/{accountId}/tags | Remove tags from account
[**get_account**](AccountApi.md#get_account) | **GET** /1.0/kb/accounts/{accountId} | Retrieve an account by id
[**get_account_audit_logs**](AccountApi.md#get_account_audit_logs) | **GET** /1.0/kb/accounts/{accountId}/auditLogs | Retrieve audit logs by account id
[**get_account_audit_logs_with_history**](AccountApi.md#get_account_audit_logs_with_history) | **GET** /1.0/kb/accounts/{accountId}/auditLogsWithHistory | Retrieve account audit logs with history by account id
[**get_account_bundles**](AccountApi.md#get_account_bundles) | **GET** /1.0/kb/accounts/{accountId}/bundles | Retrieve bundles for account
[**get_account_bundles_paginated**](AccountApi.md#get_account_bundles_paginated) | **GET** /1.0/kb/accounts/{accountId}/bundles/pagination | Retrieve paginated bundles for account
[**get_account_by_key**](AccountApi.md#get_account_by_key) | **GET** /1.0/kb/accounts | Retrieve an account by external key
[**get_account_custom_fields**](AccountApi.md#get_account_custom_fields) | **GET** /1.0/kb/accounts/{accountId}/customFields | Retrieve account custom fields
[**get_account_email_audit_logs_with_history**](AccountApi.md#get_account_email_audit_logs_with_history) | **GET** /1.0/kb/accounts/{accountId}/emails/{accountEmailId}/auditLogsWithHistory | Retrieve account email audit logs with history by id
[**get_account_tags**](AccountApi.md#get_account_tags) | **GET** /1.0/kb/accounts/{accountId}/tags | Retrieve account tags
[**get_account_timeline**](AccountApi.md#get_account_timeline) | **GET** /1.0/kb/accounts/{accountId}/timeline | Retrieve account timeline
[**get_accounts**](AccountApi.md#get_accounts) | **GET** /1.0/kb/accounts/pagination | List accounts
[**get_all_custom_fields**](AccountApi.md#get_all_custom_fields) | **GET** /1.0/kb/accounts/{accountId}/allCustomFields | Retrieve account customFields
[**get_all_tags**](AccountApi.md#get_all_tags) | **GET** /1.0/kb/accounts/{accountId}/allTags | Retrieve account tags
[**get_blocking_state_audit_logs_with_history**](AccountApi.md#get_blocking_state_audit_logs_with_history) | **GET** /1.0/kb/accounts/block/{blockingId}/auditLogsWithHistory | Retrieve blocking state audit logs with history by id
[**get_blocking_states**](AccountApi.md#get_blocking_states) | **GET** /1.0/kb/accounts/{accountId}/block | Retrieve blocking states for account
[**get_children_accounts**](AccountApi.md#get_children_accounts) | **GET** /1.0/kb/accounts/{accountId}/children | List children accounts
[**get_emails**](AccountApi.md#get_emails) | **GET** /1.0/kb/accounts/{accountId}/emails | Retrieve an account emails
[**get_invoice_payments**](AccountApi.md#get_invoice_payments) | **GET** /1.0/kb/accounts/{accountId}/invoicePayments | Retrieve account invoice payments
[**get_invoices_for_account**](AccountApi.md#get_invoices_for_account) | **GET** /1.0/kb/accounts/{accountId}/invoices | Retrieve account invoices
[**get_invoices_for_account_paginated**](AccountApi.md#get_invoices_for_account_paginated) | **GET** /1.0/kb/accounts/{accountId}/invoices/pagination | Retrieve paginated invoices for account
[**get_overdue_account**](AccountApi.md#get_overdue_account) | **GET** /1.0/kb/accounts/{accountId}/overdue | Retrieve overdue state for account
[**get_payment_methods_for_account**](AccountApi.md#get_payment_methods_for_account) | **GET** /1.0/kb/accounts/{accountId}/paymentMethods | Retrieve account payment methods
[**get_payments_for_account**](AccountApi.md#get_payments_for_account) | **GET** /1.0/kb/accounts/{accountId}/payments | Retrieve account payments
[**modify_account_custom_fields**](AccountApi.md#modify_account_custom_fields) | **PUT** /1.0/kb/accounts/{accountId}/customFields | Modify custom fields to account
[**pay_all_invoices**](AccountApi.md#pay_all_invoices) | **POST** /1.0/kb/accounts/{accountId}/invoicePayments | Trigger a payment for all unpaid invoices
[**process_payment**](AccountApi.md#process_payment) | **POST** /1.0/kb/accounts/{accountId}/payments | Trigger a payment (authorization, purchase or credit)
[**process_payment_by_external_key**](AccountApi.md#process_payment_by_external_key) | **POST** /1.0/kb/accounts/payments | Trigger a payment using the account external key (authorization, purchase or credit)
[**rebalance_existing_cbaon_account**](AccountApi.md#rebalance_existing_cbaon_account) | **PUT** /1.0/kb/accounts/{accountId}/cbaRebalancing | Rebalance account CBA
[**refresh_payment_methods**](AccountApi.md#refresh_payment_methods) | **PUT** /1.0/kb/accounts/{accountId}/paymentMethods/refresh | Refresh account payment methods
[**remove_email**](AccountApi.md#remove_email) | **DELETE** /1.0/kb/accounts/{accountId}/emails/{email} | Delete email from account
[**search_accounts**](AccountApi.md#search_accounts) | **GET** /1.0/kb/accounts/search/{searchKey} | Search accounts
[**set_default_payment_method**](AccountApi.md#set_default_payment_method) | **PUT** /1.0/kb/accounts/{accountId}/paymentMethods/{paymentMethodId}/setDefault | Set the default payment method
[**transfer_child_credit_to_parent**](AccountApi.md#transfer_child_credit_to_parent) | **PUT** /1.0/kb/accounts/{childAccountId}/transferCredit | Move a given child credit to the parent level
[**update_account**](AccountApi.md#update_account) | **PUT** /1.0/kb/accounts/{accountId} | Update account



## add_account_blocking_state

> Vec<models::BlockingState> add_account_blocking_state(account_id, x_killbill_created_by, body, requested_date, plugin_property, x_killbill_reason, x_killbill_comment)
Block an account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
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


## add_email

> Vec<models::AccountEmail> add_email(account_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add account email

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**AccountEmail**](AccountEmail.md) |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**Vec<models::AccountEmail>**](AccountEmail.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## close_account

> close_account(account_id, x_killbill_created_by, cancel_all_subscriptions, write_off_unpaid_invoices, item_adjust_unpaid_invoices, remove_future_notifications, x_killbill_reason, x_killbill_comment)
Close account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**cancel_all_subscriptions** | Option<**bool**> |  |  |[default to false]
**write_off_unpaid_invoices** | Option<**bool**> |  |  |[default to false]
**item_adjust_unpaid_invoices** | Option<**bool**> |  |  |[default to false]
**remove_future_notifications** | Option<**bool**> |  |  |[default to true]
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


## create_account

> models::Account create_account(x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Create account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Account**](Account.md) |  | [required] |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::Account**](Account.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_account_custom_fields

> Vec<models::CustomField> create_account_custom_fields(account_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add custom fields to account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
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


## create_account_tags

> Vec<models::Tag> create_account_tags(account_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add tags to account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
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


## create_payment_method

> models::PaymentMethod create_payment_method(account_id, x_killbill_created_by, body, is_default, pay_all_unpaid_invoices, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Add a payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentMethod**](PaymentMethod.md) |  | [required] |
**is_default** | Option<**bool**> |  |  |[default to false]
**pay_all_unpaid_invoices** | Option<**bool**> |  |  |[default to false]
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::PaymentMethod**](PaymentMethod.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_account_custom_fields

> delete_account_custom_fields(account_id, x_killbill_created_by, custom_field, x_killbill_reason, x_killbill_comment)
Remove custom fields from account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
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


## delete_account_tags

> delete_account_tags(account_id, x_killbill_created_by, tag_def, x_killbill_reason, x_killbill_comment)
Remove tags from account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
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


## get_account

> models::Account get_account(account_id, account_with_balance, account_with_balance_and_cba, audit)
Retrieve an account by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**account_with_balance** | Option<**bool**> |  |  |[default to false]
**account_with_balance_and_cba** | Option<**bool**> |  |  |[default to false]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::Account**](Account.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_audit_logs

> Vec<models::AuditLog> get_account_audit_logs(account_id)
Retrieve audit logs by account id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_audit_logs_with_history

> Vec<models::AuditLog> get_account_audit_logs_with_history(account_id)
Retrieve account audit logs with history by account id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_bundles

> Vec<models::Bundle> get_account_bundles(account_id, external_key, bundles_filter, audit)
Retrieve bundles for account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**external_key** | Option<**String**> |  |  |
**bundles_filter** | Option<**String**> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Bundle>**](Bundle.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_bundles_paginated

> Vec<models::Bundle> get_account_bundles_paginated(account_id, offset, limit, audit)
Retrieve paginated bundles for account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
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


## get_account_by_key

> models::Account get_account_by_key(external_key, account_with_balance, account_with_balance_and_cba, audit)
Retrieve an account by external key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_key** | **String** |  | [required] |
**account_with_balance** | Option<**bool**> |  |  |[default to false]
**account_with_balance_and_cba** | Option<**bool**> |  |  |[default to false]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::Account**](Account.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_custom_fields

> Vec<models::CustomField> get_account_custom_fields(account_id, audit)
Retrieve account custom fields

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::CustomField>**](CustomField.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_email_audit_logs_with_history

> Vec<models::AuditLog> get_account_email_audit_logs_with_history(account_id, account_email_id)
Retrieve account email audit logs with history by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**account_email_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_tags

> Vec<models::Tag> get_account_tags(account_id, included_deleted, audit)
Retrieve account tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
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


## get_account_timeline

> models::AccountTimeline get_account_timeline(account_id, parallel, audit)
Retrieve account timeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**parallel** | Option<**bool**> |  |  |[default to false]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::AccountTimeline**](AccountTimeline.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_accounts

> Vec<models::Account> get_accounts(offset, limit, account_with_balance, account_with_balance_and_cba, audit)
List accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i64**> |  |  |[default to 0]
**limit** | Option<**i64**> |  |  |[default to 100]
**account_with_balance** | Option<**bool**> |  |  |[default to false]
**account_with_balance_and_cba** | Option<**bool**> |  |  |[default to false]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Account>**](Account.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_custom_fields

> Vec<models::CustomField> get_all_custom_fields(account_id, object_type, audit)
Retrieve account customFields

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**object_type** | Option<**String**> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::CustomField>**](CustomField.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_tags

> Vec<models::Tag> get_all_tags(account_id, object_type, included_deleted, audit)
Retrieve account tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**object_type** | Option<**String**> |  |  |
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


## get_blocking_state_audit_logs_with_history

> Vec<models::AuditLog> get_blocking_state_audit_logs_with_history(blocking_id)
Retrieve blocking state audit logs with history by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blocking_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blocking_states

> Vec<models::BlockingState> get_blocking_states(account_id, blocking_state_types, blocking_state_svcs, audit)
Retrieve blocking states for account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**blocking_state_types** | Option<[**Vec<String>**](String.md)> |  |  |
**blocking_state_svcs** | Option<[**Vec<String>**](String.md)> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::BlockingState>**](BlockingState.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_children_accounts

> Vec<models::Account> get_children_accounts(account_id, account_with_balance, account_with_balance_and_cba, audit)
List children accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**account_with_balance** | Option<**bool**> |  |  |[default to false]
**account_with_balance_and_cba** | Option<**bool**> |  |  |[default to false]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Account>**](Account.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_emails

> Vec<models::AccountEmail> get_emails(account_id)
Retrieve an account emails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AccountEmail>**](AccountEmail.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice_payments

> Vec<models::InvoicePayment> get_invoice_payments(account_id, with_plugin_info, with_attempts, plugin_property, audit)
Retrieve account invoice payments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**with_plugin_info** | Option<**bool**> |  |  |[default to false]
**with_attempts** | Option<**bool**> |  |  |[default to false]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::InvoicePayment>**](InvoicePayment.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoices_for_account

> Vec<models::Invoice> get_invoices_for_account(account_id, start_date, end_date, with_migration_invoices, unpaid_invoices_only, include_voided_invoices, include_invoice_components, invoices_filter, audit)
Retrieve account invoices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**start_date** | Option<**String**> |  |  |
**end_date** | Option<**String**> |  |  |
**with_migration_invoices** | Option<**bool**> |  |  |[default to false]
**unpaid_invoices_only** | Option<**bool**> |  |  |[default to false]
**include_voided_invoices** | Option<**bool**> |  |  |[default to false]
**include_invoice_components** | Option<**bool**> |  |  |[default to false]
**invoices_filter** | Option<**String**> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Invoice>**](Invoice.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoices_for_account_paginated

> Vec<models::Invoice> get_invoices_for_account_paginated(account_id, offset, limit, audit)
Retrieve paginated invoices for account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**offset** | Option<**i64**> |  |  |[default to 0]
**limit** | Option<**i64**> |  |  |[default to 100]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Invoice>**](Invoice.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_overdue_account

> models::OverdueState get_overdue_account(account_id)
Retrieve overdue state for account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::OverdueState**](OverdueState.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_methods_for_account

> Vec<models::PaymentMethod> get_payment_methods_for_account(account_id, with_plugin_info, included_deleted, plugin_property, audit)
Retrieve account payment methods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**with_plugin_info** | Option<**bool**> |  |  |[default to false]
**included_deleted** | Option<**bool**> |  |  |[default to false]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::PaymentMethod>**](PaymentMethod.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payments_for_account

> Vec<models::Payment> get_payments_for_account(account_id, with_attempts, with_plugin_info, plugin_property, audit)
Retrieve account payments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**with_attempts** | Option<**bool**> |  |  |[default to false]
**with_plugin_info** | Option<**bool**> |  |  |[default to false]
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


## modify_account_custom_fields

> modify_account_custom_fields(account_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Modify custom fields to account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
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


## pay_all_invoices

> Vec<models::Invoice> pay_all_invoices(account_id, x_killbill_created_by, payment_method_id, external_payment, payment_amount, target_date, plugin_property, x_killbill_reason, x_killbill_comment)
Trigger a payment for all unpaid invoices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**payment_method_id** | Option<**uuid::Uuid**> |  |  |
**external_payment** | Option<**bool**> |  |  |[default to false]
**payment_amount** | Option<**f64**> |  |  |
**target_date** | Option<**String**> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**Vec<models::Invoice>**](Invoice.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## process_payment

> models::Payment process_payment(account_id, x_killbill_created_by, body, payment_method_id, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Trigger a payment (authorization, purchase or credit)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**payment_method_id** | Option<**uuid::Uuid**> |  |  |
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


## process_payment_by_external_key

> models::Payment process_payment_by_external_key(external_key, x_killbill_created_by, body, payment_method_id, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Trigger a payment using the account external key (authorization, purchase or credit)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_key** | **String** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**PaymentTransaction**](PaymentTransaction.md) |  | [required] |
**payment_method_id** | Option<**uuid::Uuid**> |  |  |
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


## rebalance_existing_cbaon_account

> rebalance_existing_cbaon_account(account_id, x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Rebalance account CBA

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
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


## refresh_payment_methods

> refresh_payment_methods(account_id, x_killbill_created_by, plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Refresh account payment methods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**plugin_name** | Option<**String**> |  |  |
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


## remove_email

> remove_email(account_id, email, x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Delete email from account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**email** | **String** |  | [required] |
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


## search_accounts

> Vec<models::Account> search_accounts(search_key, offset, limit, account_with_balance, account_with_balance_and_cba, audit)
Search accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_key** | **String** |  | [required] |
**offset** | Option<**i64**> |  |  |[default to 0]
**limit** | Option<**i64**> |  |  |[default to 100]
**account_with_balance** | Option<**bool**> |  |  |[default to false]
**account_with_balance_and_cba** | Option<**bool**> |  |  |[default to false]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Account>**](Account.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_payment_method

> set_default_payment_method(account_id, payment_method_id, x_killbill_created_by, pay_all_unpaid_invoices, plugin_property, x_killbill_reason, x_killbill_comment)
Set the default payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**payment_method_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**pay_all_unpaid_invoices** | Option<**bool**> |  |  |[default to false]
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


## transfer_child_credit_to_parent

> transfer_child_credit_to_parent(child_account_id, x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Move a given child credit to the parent level

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_account_id** | **uuid::Uuid** |  | [required] |
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


## update_account

> update_account(account_id, x_killbill_created_by, body, treat_null_as_reset, x_killbill_reason, x_killbill_comment)
Update account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Account**](Account.md) |  | [required] |
**treat_null_as_reset** | Option<**bool**> |  |  |[default to false]
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

