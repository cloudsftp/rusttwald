# DePeriodMittwaldPeriodV1PeriodContractPeriodContractItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activation_date** | Option<**String**> |  | [optional]
**aggregate_reference** | Option<[**models::DePeriodMittwaldPeriodV1PeriodContractPeriodAggregateReference**](de.mittwald.v1.contract.AggregateReference.md)> |  | [optional]
**articles** | [**Vec<models::DePeriodMittwaldPeriodV1PeriodContractPeriodArticle>**](de.mittwald.v1.contract.Article.md) |  | 
**contract_period** | **f64** |  | 
**description** | **String** |  | 
**free_trial_days** | Option<**f64**> |  | [optional]
**group_by_project_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**invoice_stop** | Option<**String**> | If this attribute is set, the contract item will currently only be invoiced until this date. | [optional]
**invoicing_period** | Option<**f64**> |  | [optional]
**is_activated** | **bool** |  | 
**is_base_item** | **bool** |  | 
**is_in_free_trial** | Option<**bool**> |  | [optional]
**is_inclusive** | Option<**bool**> |  | [optional]
**item_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**next_possible_downgrade_date** | Option<**String**> | If this attribute is not set, termination is not allowed. | [optional]
**next_possible_termination_date** | Option<**String**> | If this attribute is not set, a tariff change is not allowed. | [optional]
**next_possible_upgrade_date** | Option<**String**> | If this attribute is not set, a tariff change is not allowed. | [optional]
**order_date** | Option<**String**> |  | [optional]
**order_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**replaced_by_item** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**tariff_change** | Option<[**models::DePeriodMittwaldPeriodV1PeriodContractPeriodTariffChange**](de.mittwald.v1.contract.TariffChange.md)> |  | [optional]
**termination** | Option<[**models::DePeriodMittwaldPeriodV1PeriodContractPeriodTermination**](de.mittwald.v1.contract.Termination.md)> |  | [optional]
**total_price** | [**models::DePeriodMittwaldPeriodV1PeriodContractPeriodPrice**](de.mittwald.v1.contract.Price.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


