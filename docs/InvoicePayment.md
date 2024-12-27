# InvoicePayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target_invoice_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**payment_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**payment_number** | Option<**String**> |  | [optional]
**payment_external_key** | Option<**String**> |  | [optional]
**auth_amount** | Option<**f64**> |  | [optional]
**captured_amount** | Option<**f64**> |  | [optional]
**purchased_amount** | Option<**f64**> |  | [optional]
**refunded_amount** | Option<**f64**> |  | [optional]
**credited_amount** | Option<**f64**> |  | [optional]
**currency** | Option<**String**> |  | [optional]
**payment_method_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**transactions** | Option<[**Vec<models::PaymentTransaction>**](PaymentTransaction.md)> |  | [optional]
**payment_attempts** | Option<[**Vec<models::PaymentAttempt>**](PaymentAttempt.md)> |  | [optional]
**audit_logs** | Option<[**Vec<models::AuditLog>**](AuditLog.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


