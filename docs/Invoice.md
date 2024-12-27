# Invoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | Option<**f64**> |  | [optional]
**currency** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**credit_adj** | Option<**f64**> |  | [optional]
**refund_adj** | Option<**f64**> |  | [optional]
**invoice_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**invoice_date** | Option<[**String**](string.md)> |  | [optional]
**target_date** | Option<[**String**](string.md)> |  | [optional]
**invoice_number** | Option<**String**> |  | [optional]
**balance** | Option<**f64**> |  | [optional]
**account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**bundle_keys** | Option<**String**> |  | [optional]
**credits** | Option<[**Vec<models::InvoiceItem>**](InvoiceItem.md)> |  | [optional]
**items** | Option<[**Vec<models::InvoiceItem>**](InvoiceItem.md)> |  | [optional]
**tracking_ids** | Option<**Vec<String>**> |  | [optional]
**is_parent_invoice** | Option<**bool**> |  | [optional]
**parent_invoice_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**parent_account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**audit_logs** | Option<[**Vec<models::AuditLog>**](AuditLog.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


