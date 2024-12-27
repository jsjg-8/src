# Account

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**first_name_length** | Option<**i32**> |  | [optional]
**external_key** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**bill_cycle_day_local** | Option<**i32**> |  | [optional]
**currency** | Option<**String**> |  | [optional]
**parent_account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**is_payment_delegated_to_parent** | Option<**bool**> |  | [optional]
**payment_method_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**reference_time** | Option<**String**> |  | [optional]
**time_zone** | Option<**String**> |  | [optional]
**address1** | Option<**String**> |  | [optional]
**address2** | Option<**String**> |  | [optional]
**postal_code** | Option<**String**> |  | [optional]
**company** | Option<**String**> |  | [optional]
**city** | Option<**String**> |  | [optional]
**state** | Option<**String**> |  | [optional]
**country** | Option<**String**> |  | [optional]
**locale** | Option<**String**> |  | [optional]
**phone** | Option<**String**> |  | [optional]
**notes** | Option<**String**> |  | [optional]
**is_migrated** | Option<**bool**> |  | [optional]
**account_balance** | Option<**f64**> |  | [optional]
**account_cba** | Option<**f64**> |  | [optional]
**audit_logs** | Option<[**Vec<models::AuditLog>**](AuditLog.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


