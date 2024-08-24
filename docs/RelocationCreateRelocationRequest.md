# RelocationCreateRelocationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**all_domains** | Option<**bool**> | Should all project releated domains should be transferred to mittwald? | [optional]
**allow_password_change** | **bool** | Has to be true. Do you accept that our mittwald team can change and get password from your current provider? | 
**contact** | [**models::RelocationCreateRelocationRequestContact**](relocation_create_relocation_request_contact.md) |  | 
**domains** | Option<[**Vec<models::DePeriodMittwaldPeriodV1PeriodDirectusPeriodDomain>**](de.mittwald.v1.directus.Domain.md)> | List of domains which should be transferred (when allDomains is not checked). | [optional]
**notes** | Option<**String**> | Anything our customer service needs to know for the relocation process. | [optional]
**provider** | [**models::RelocationCreateRelocationRequestProvider**](relocation_create_relocation_request_provider.md) |  | 
**target** | [**models::RelocationCreateRelocationRequestTarget**](relocation_create_relocation_request_target.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


