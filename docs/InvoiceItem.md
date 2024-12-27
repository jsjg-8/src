# InvoiceItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoice_item_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**invoice_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**linked_invoice_item_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**account_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**child_account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**bundle_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**subscription_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**product_name** | Option<**String**> |  | [optional]
**plan_name** | Option<**String**> |  | [optional]
**phase_name** | Option<**String**> |  | [optional]
**usage_name** | Option<**String**> |  | [optional]
**pretty_product_name** | Option<**String**> |  | [optional]
**pretty_plan_name** | Option<**String**> |  | [optional]
**pretty_phase_name** | Option<**String**> |  | [optional]
**pretty_usage_name** | Option<**String**> |  | [optional]
**item_type** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**start_date** | Option<[**String**](string.md)> |  | [optional]
**end_date** | Option<[**String**](string.md)> |  | [optional]
**amount** | Option<**f64**> |  | [optional]
**rate** | Option<**f64**> |  | [optional]
**currency** | Option<**String**> |  | [optional]
**quantity** | Option<**f64**> |  | [optional]
**item_details** | Option<**String**> |  | [optional]
**catalog_effective_date** | Option<**String**> |  | [optional]
**child_items** | Option<[**Vec<models::InvoiceItem>**](InvoiceItem.md)> |  | [optional]
**audit_logs** | Option<[**Vec<models::AuditLog>**](AuditLog.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


