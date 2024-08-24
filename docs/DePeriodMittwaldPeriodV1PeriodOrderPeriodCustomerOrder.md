# DePeriodMittwaldPeriodV1PeriodOrderPeriodCustomerOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contract_change_contract_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**customer_id** | **String** |  | 
**due_date** | Option<**String**> |  | [optional]
**invoicing_period** | **f64** | Invoicing period in months | 
**items** | [**Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderItem>**](de.mittwald.v1.order.OrderItem.md) |  | 
**order_date** | Option<**String**> |  | [optional]
**order_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**order_number** | **String** |  | 
**profile** | Option<[**models::DePeriodMittwaldPeriodV1PeriodOrderPeriodProfile**](de.mittwald.v1.order.Profile.md)> |  | [optional]
**status** | [**models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderStatus**](de.mittwald.v1.order.OrderStatus.md) |  | 
**summary** | [**models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderSummary**](de.mittwald.v1.order.OrderSummary.md) |  | 
**r#type** | [**models::DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderType**](de.mittwald.v1.order.OrderType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


