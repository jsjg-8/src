# PaymentAttempt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**payment_method_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**payment_external_key** | Option<**String**> |  | [optional]
**transaction_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**transaction_external_key** | Option<**String**> |  | [optional]
**transaction_type** | Option<**String**> |  | [optional]
**effective_date** | Option<**String**> |  | [optional]
**state_name** | Option<**String**> |  | [optional]
**amount** | Option<**f64**> | Transaction amount, required except for void operations | [optional]
**currency** | Option<**String**> | Amount currency (account currency unless specified) | [optional]
**plugin_name** | Option<**String**> |  | [optional]
**plugin_properties** | Option<[**Vec<models::PluginProperty>**](PluginProperty.md)> |  | [optional]
**audit_logs** | Option<[**Vec<models::AuditLog>**](AuditLog.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


