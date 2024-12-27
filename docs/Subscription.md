# Subscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**bundle_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**bundle_external_key** | Option<**String**> |  | [optional]
**subscription_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**external_key** | Option<**String**> |  | [optional]
**start_date** | Option<**String**> |  | [optional]
**product_name** | **String** |  | 
**product_category** | Option<**String**> |  | [optional]
**billing_period** | **String** |  | 
**phase_type** | Option<**String**> |  | [optional]
**price_list** | **String** |  | 
**plan_name** | **String** |  | 
**state** | Option<**String**> |  | [optional]
**source_type** | Option<**String**> |  | [optional]
**cancelled_date** | Option<**String**> |  | [optional]
**charged_through_date** | Option<[**String**](string.md)> |  | [optional]
**billing_start_date** | Option<**String**> |  | [optional]
**billing_end_date** | Option<**String**> |  | [optional]
**bill_cycle_day_local** | Option<**i32**> |  | [optional]
**quantity** | Option<**i32**> |  | [optional]
**events** | Option<[**Vec<models::EventSubscription>**](EventSubscription.md)> |  | [optional]
**price_overrides** | Option<[**Vec<models::PhasePrice>**](PhasePrice.md)> |  | [optional]
**prices** | Option<[**Vec<models::PhasePrice>**](PhasePrice.md)> |  | [optional]
**audit_logs** | Option<[**Vec<models::AuditLog>**](AuditLog.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


