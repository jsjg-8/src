# InvoicePaymentTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**transaction_external_key** | Option<**String**> |  | [optional]
**payment_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Associated payment id, required when notifying state transitions | [optional]
**payment_external_key** | Option<**String**> |  | [optional]
**transaction_type** | Option<**String**> |  | [optional]
**amount** | Option<**f64**> | Transaction amount, required except for void operations | [optional]
**currency** | Option<**String**> | Amount currency (account currency unless specified) | [optional]
**effective_date** | Option<**String**> |  | [optional]
**processed_amount** | Option<**f64**> |  | [optional]
**processed_currency** | Option<**String**> |  | [optional]
**status** | Option<**String**> | Transaction status, required for state change notifications | [optional]
**gateway_error_code** | Option<**String**> |  | [optional]
**gateway_error_msg** | Option<**String**> |  | [optional]
**first_payment_reference_id** | Option<**String**> |  | [optional]
**second_payment_reference_id** | Option<**String**> |  | [optional]
**properties** | Option<[**Vec<models::PluginProperty>**](PluginProperty.md)> |  | [optional]
**is_adjusted** | Option<**bool**> |  | [optional]
**adjustments** | Option<[**Vec<models::InvoiceItem>**](InvoiceItem.md)> |  | [optional]
**audit_logs** | Option<[**Vec<models::AuditLog>**](AuditLog.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


