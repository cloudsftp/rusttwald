# DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avatar_ref_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Reference to the Project's avatar. | [optional]
**customer_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the Customer the invite is for. | 
**customer_name** | **String** | Name of the Customer the user is invited to. | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the CustomerInvite. | 
**information** | [**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodInviteInformation**](de.mittwald.v1.membership.InviteInformation.md) |  | 
**mail_address** | **String** | Mail-address of the user the invite is for. | 
**membership_expires_at** | Option<**String**> | Time the CustomerMembership should expire at. | [optional]
**message** | Option<**String**> | Message contained in the CustomerInvite. | [optional]
**role** | [**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerRoles**](de.mittwald.v1.membership.CustomerRoles.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


