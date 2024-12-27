# PaymentMethod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payment_method_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**external_key** | Option<**String**> |  | [optional]
**account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**is_default** | Option<**bool**> |  | [optional]
**plugin_name** | Option<**String**> |  | [optional]
**plugin_info** | Option<[**models::PaymentMethodPluginDetail**](PaymentMethodPluginDetail.md)> |  | [optional]
**audit_logs** | Option<[**Vec<models::AuditLog>**](AuditLog.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


