# DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerMembership

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the Customer the CustomerMembership is for. | 
**email** | **String** | Email used by the invited user. | 
**expires_at** | Option<**String**> | Time the CustomerMembership should expire at. | [optional]
**id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the CustomerMembership. | 
**invite_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | ID of the CustomerInvite the membership was created from. | [optional]
**member_since** | Option<**String**> | Date the CustomerMembership was created at. | [optional]
**mfa** | **bool** | MFA activated by the user. | 
**role** | [**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerRoles**](de.mittwald.v1.membership.CustomerRoles.md) |  | 
**user_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the user the CustomerMembership is for. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


