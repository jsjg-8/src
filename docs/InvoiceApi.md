# \InvoiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**adjust_invoice_item**](InvoiceApi.md#adjust_invoice_item) | **POST** /1.0/kb/invoices/{invoiceId} | Adjust an invoice item
[**commit_invoice**](InvoiceApi.md#commit_invoice) | **PUT** /1.0/kb/invoices/{invoiceId}/commitInvoice | Perform the invoice status transition from DRAFT to COMMITTED
[**create_external_charges**](InvoiceApi.md#create_external_charges) | **POST** /1.0/kb/invoices/charges/{accountId} | Create external charge(s)
[**create_future_invoice**](InvoiceApi.md#create_future_invoice) | **POST** /1.0/kb/invoices | Trigger an invoice generation
[**create_future_invoice_group**](InvoiceApi.md#create_future_invoice_group) | **POST** /1.0/kb/invoices/group | Trigger an invoice generation
[**create_instant_payment**](InvoiceApi.md#create_instant_payment) | **POST** /1.0/kb/invoices/{invoiceId}/payments | Trigger a payment for invoice
[**create_invoice_custom_fields**](InvoiceApi.md#create_invoice_custom_fields) | **POST** /1.0/kb/invoices/{invoiceId}/customFields | Add custom fields to invoice
[**create_invoice_tags**](InvoiceApi.md#create_invoice_tags) | **POST** /1.0/kb/invoices/{invoiceId}/tags | Add tags to invoice
[**create_migration_invoice**](InvoiceApi.md#create_migration_invoice) | **POST** /1.0/kb/invoices/migration/{accountId} | Create a migration invoice
[**create_tax_items**](InvoiceApi.md#create_tax_items) | **POST** /1.0/kb/invoices/taxes/{accountId} | Create tax items
[**delete_cba**](InvoiceApi.md#delete_cba) | **DELETE** /1.0/kb/invoices/{invoiceId}/{invoiceItemId}/cba | Delete a CBA item
[**delete_invoice_custom_fields**](InvoiceApi.md#delete_invoice_custom_fields) | **DELETE** /1.0/kb/invoices/{invoiceId}/customFields | Remove custom fields from invoice
[**delete_invoice_tags**](InvoiceApi.md#delete_invoice_tags) | **DELETE** /1.0/kb/invoices/{invoiceId}/tags | Remove tags from invoice
[**generate_dry_run_invoice**](InvoiceApi.md#generate_dry_run_invoice) | **POST** /1.0/kb/invoices/dryRun | Generate a dryRun invoice
[**get_catalog_translation**](InvoiceApi.md#get_catalog_translation) | **GET** /1.0/kb/invoices/catalogTranslation/{locale} | Retrieves the catalog translation for the tenant
[**get_invoice**](InvoiceApi.md#get_invoice) | **GET** /1.0/kb/invoices/{invoiceId} | Retrieve an invoice by id
[**get_invoice_as_html**](InvoiceApi.md#get_invoice_as_html) | **GET** /1.0/kb/invoices/{invoiceId}/html | Render an invoice as HTML
[**get_invoice_audit_logs_with_history**](InvoiceApi.md#get_invoice_audit_logs_with_history) | **GET** /1.0/kb/invoices/{invoiceId}/auditLogsWithHistory | Retrieve invoice audit logs with history by id
[**get_invoice_by_item_id**](InvoiceApi.md#get_invoice_by_item_id) | **GET** /1.0/kb/invoices/byItemId/{itemId} | Retrieve an invoice by invoice item id
[**get_invoice_by_number**](InvoiceApi.md#get_invoice_by_number) | **GET** /1.0/kb/invoices/byNumber/{invoiceNumber} | Retrieve an invoice by number
[**get_invoice_custom_fields**](InvoiceApi.md#get_invoice_custom_fields) | **GET** /1.0/kb/invoices/{invoiceId}/customFields | Retrieve invoice custom fields
[**get_invoice_mp_template**](InvoiceApi.md#get_invoice_mp_template) | **GET** /1.0/kb/invoices/manualPayTemplate/{locale} | Retrieves the manualPay invoice template for the tenant
[**get_invoice_tags**](InvoiceApi.md#get_invoice_tags) | **GET** /1.0/kb/invoices/{invoiceId}/tags | Retrieve invoice tags
[**get_invoice_template**](InvoiceApi.md#get_invoice_template) | **GET** /1.0/kb/invoices/template | Retrieves the invoice template for the tenant
[**get_invoice_translation**](InvoiceApi.md#get_invoice_translation) | **GET** /1.0/kb/invoices/translation/{locale} | Retrieves the invoice translation for the tenant
[**get_invoices**](InvoiceApi.md#get_invoices) | **GET** /1.0/kb/invoices/pagination | List invoices
[**get_invoices_group**](InvoiceApi.md#get_invoices_group) | **GET** /1.0/kb/invoices/{groupId}/group | Retrieve a set of invoices by group id
[**get_payments_for_invoice**](InvoiceApi.md#get_payments_for_invoice) | **GET** /1.0/kb/invoices/{invoiceId}/payments | Retrieve payments associated with an invoice
[**modify_invoice_custom_fields**](InvoiceApi.md#modify_invoice_custom_fields) | **PUT** /1.0/kb/invoices/{invoiceId}/customFields | Modify custom fields to invoice
[**search_invoices**](InvoiceApi.md#search_invoices) | **GET** /1.0/kb/invoices/search/{searchKey} | Search invoices
[**upload_catalog_translation**](InvoiceApi.md#upload_catalog_translation) | **POST** /1.0/kb/invoices/catalogTranslation/{locale} | Upload the catalog translation for the tenant
[**upload_invoice_mp_template**](InvoiceApi.md#upload_invoice_mp_template) | **POST** /1.0/kb/invoices/manualPayTemplate | Upload the manualPay invoice template for the tenant
[**upload_invoice_template**](InvoiceApi.md#upload_invoice_template) | **POST** /1.0/kb/invoices/template | Upload the invoice template for the tenant
[**upload_invoice_translation**](InvoiceApi.md#upload_invoice_translation) | **POST** /1.0/kb/invoices/translation/{locale} | Upload the invoice translation for the tenant
[**void_invoice**](InvoiceApi.md#void_invoice) | **PUT** /1.0/kb/invoices/{invoiceId}/voidInvoice | Perform the action of voiding an invoice



## adjust_invoice_item

> models::Invoice adjust_invoice_item(invoice_id, x_killbill_created_by, body, requested_date, plugin_property, x_killbill_reason, x_killbill_comment)
Adjust an invoice item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**InvoiceItem**](InvoiceItem.md) |  | [required] |
**requested_date** | Option<**String**> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commit_invoice

> commit_invoice(invoice_id, x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Perform the invoice status transition from DRAFT to COMMITTED

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
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


## create_external_charges

> Vec<models::InvoiceItem> create_external_charges(account_id, x_killbill_created_by, body, requested_date, auto_commit, plugin_property, x_killbill_reason, x_killbill_comment)
Create external charge(s)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Vec<models::InvoiceItem>**](InvoiceItem.md) |  | [required] |
**requested_date** | Option<**String**> |  |  |
**auto_commit** | Option<**bool**> |  |  |[default to false]
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**Vec<models::InvoiceItem>**](InvoiceItem.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_future_invoice

> models::Invoice create_future_invoice(account_id, x_killbill_created_by, target_date, plugin_property, x_killbill_reason, x_killbill_comment)
Trigger an invoice generation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**target_date** | Option<**String**> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_future_invoice_group

> Vec<models::Invoice> create_future_invoice_group(account_id, x_killbill_created_by, target_date, plugin_property, x_killbill_reason, x_killbill_comment)
Trigger an invoice generation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
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


## create_instant_payment

> models::InvoicePayment create_instant_payment(invoice_id, x_killbill_created_by, body, external_payment, control_plugin_name, plugin_property, x_killbill_reason, x_killbill_comment)
Trigger a payment for invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**InvoicePayment**](InvoicePayment.md) |  | [required] |
**external_payment** | Option<**bool**> |  |  |[default to false]
**control_plugin_name** | Option<[**Vec<String>**](String.md)> |  |  |
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


## create_invoice_custom_fields

> Vec<models::CustomField> create_invoice_custom_fields(invoice_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add custom fields to invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
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


## create_invoice_tags

> Vec<models::Tag> create_invoice_tags(invoice_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Add tags to invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
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


## create_migration_invoice

> models::Invoice create_migration_invoice(account_id, x_killbill_created_by, body, target_date, x_killbill_reason, x_killbill_comment)
Create a migration invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Vec<models::InvoiceItem>**](InvoiceItem.md) |  | [required] |
**target_date** | Option<**String**> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tax_items

> Vec<models::InvoiceItem> create_tax_items(account_id, x_killbill_created_by, body, auto_commit, requested_date, plugin_property, x_killbill_reason, x_killbill_comment)
Create tax items

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**Vec<models::InvoiceItem>**](InvoiceItem.md) |  | [required] |
**auto_commit** | Option<**bool**> |  |  |[default to false]
**requested_date** | Option<**String**> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**Vec<models::InvoiceItem>**](InvoiceItem.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cba

> delete_cba(invoice_id, invoice_item_id, account_id, x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Delete a CBA item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
**invoice_item_id** | **uuid::Uuid** |  | [required] |
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


## delete_invoice_custom_fields

> delete_invoice_custom_fields(invoice_id, x_killbill_created_by, custom_field, x_killbill_reason, x_killbill_comment)
Remove custom fields from invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
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


## delete_invoice_tags

> delete_invoice_tags(invoice_id, x_killbill_created_by, tag_def, x_killbill_reason, x_killbill_comment)
Remove tags from invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
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


## generate_dry_run_invoice

> models::Invoice generate_dry_run_invoice(account_id, x_killbill_created_by, body, target_date, plugin_property, x_killbill_reason, x_killbill_comment)
Generate a dryRun invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**InvoiceDryRun**](InvoiceDryRun.md) |  | [required] |
**target_date** | Option<**String**> |  |  |
**plugin_property** | Option<[**Vec<String>**](String.md)> |  |  |
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_catalog_translation

> String get_catalog_translation(locale)
Retrieves the catalog translation for the tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** |  | [required] |

### Return type

**String**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice

> models::Invoice get_invoice(invoice_id, with_children_items, audit)
Retrieve an invoice by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
**with_children_items** | Option<**bool**> |  |  |[default to false]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice_as_html

> String get_invoice_as_html(invoice_id)
Render an invoice as HTML

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |

### Return type

**String**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice_audit_logs_with_history

> Vec<models::AuditLog> get_invoice_audit_logs_with_history(invoice_id)
Retrieve invoice audit logs with history by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AuditLog>**](AuditLog.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice_by_item_id

> models::Invoice get_invoice_by_item_id(item_id, with_children_items, audit)
Retrieve an invoice by invoice item id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** |  | [required] |
**with_children_items** | Option<**bool**> |  |  |[default to false]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice_by_number

> models::Invoice get_invoice_by_number(invoice_number, with_children_items, audit)
Retrieve an invoice by number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_number** | **i32** |  | [required] |
**with_children_items** | Option<**bool**> |  |  |[default to false]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice_custom_fields

> Vec<models::CustomField> get_invoice_custom_fields(invoice_id, audit)
Retrieve invoice custom fields

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::CustomField>**](CustomField.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice_mp_template

> String get_invoice_mp_template(locale)
Retrieves the manualPay invoice template for the tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** |  | [required] |

### Return type

**String**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice_tags

> Vec<models::Tag> get_invoice_tags(invoice_id, included_deleted, audit)
Retrieve invoice tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
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


## get_invoice_template

> String get_invoice_template()
Retrieves the invoice template for the tenant

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice_translation

> String get_invoice_translation(locale)
Retrieves the invoice translation for the tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** |  | [required] |

### Return type

**String**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoices

> Vec<models::Invoice> get_invoices(offset, limit, audit)
List invoices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## get_invoices_group

> Vec<models::Invoice> get_invoices_group(group_id, account_id, with_children_items, audit)
Retrieve a set of invoices by group id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** |  | [required] |
**account_id** | **uuid::Uuid** |  | [required] |
**with_children_items** | Option<**bool**> |  |  |[default to false]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::Invoice>**](Invoice.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payments_for_invoice

> Vec<models::InvoicePayment> get_payments_for_invoice(invoice_id, with_plugin_info, with_attempts, audit)
Retrieve payments associated with an invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
**with_plugin_info** | Option<**bool**> |  |  |[default to false]
**with_attempts** | Option<**bool**> |  |  |[default to false]
**audit** | Option<**String**> |  |  |[default to NONE]

### Return type

[**Vec<models::InvoicePayment>**](InvoicePayment.md)

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_invoice_custom_fields

> modify_invoice_custom_fields(invoice_id, x_killbill_created_by, body, x_killbill_reason, x_killbill_comment)
Modify custom fields to invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
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


## search_invoices

> Vec<models::Invoice> search_invoices(search_key, offset, limit, audit)
Search invoices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_key** | **String** |  | [required] |
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


## upload_catalog_translation

> String upload_catalog_translation(locale, x_killbill_created_by, body, delete_if_exists, x_killbill_reason, x_killbill_comment)
Upload the catalog translation for the tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | **String** |  | [required] |
**delete_if_exists** | Option<**bool**> |  |  |[default to false]
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_invoice_mp_template

> String upload_invoice_mp_template(x_killbill_created_by, body, delete_if_exists, x_killbill_reason, x_killbill_comment)
Upload the manualPay invoice template for the tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | **String** |  | [required] |
**delete_if_exists** | Option<**bool**> |  |  |[default to false]
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: text/html
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_invoice_template

> String upload_invoice_template(x_killbill_created_by, body, delete_if_exists, x_killbill_reason, x_killbill_comment)
Upload the invoice template for the tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | **String** |  | [required] |
**delete_if_exists** | Option<**bool**> |  |  |[default to false]
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: text/html
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_invoice_translation

> String upload_invoice_translation(locale, x_killbill_created_by, body, delete_if_exists, x_killbill_reason, x_killbill_comment)
Upload the invoice translation for the tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** |  | [required] |
**x_killbill_created_by** | **String** |  | [required] |
**body** | **String** |  | [required] |
**delete_if_exists** | Option<**bool**> |  |  |[default to false]
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[Killbill Api Key](../README.md#Killbill Api Key), [Killbill Api Secret](../README.md#Killbill Api Secret), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## void_invoice

> void_invoice(invoice_id, x_killbill_created_by, x_killbill_reason, x_killbill_comment)
Perform the action of voiding an invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **uuid::Uuid** |  | [required] |
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

