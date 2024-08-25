# DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectInvite

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avatar_ref_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Reference to the Project's avatar. | [optional]
**id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the ProjectInvite. | 
**information** | [**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodInviteInformation**](de.mittwald.v1.membership.InviteInformation.md) |  | 
**mail_address** | **String** | Mail-address of the user the ProjectInvite is for. | 
**membership_expires_at** | Option<**String**> | Time the ProjectMembership should expire at. | [optional]
**message** | Option<**String**> | Message contained in the ProjectInvite. | [optional]
**project_description** | **String** | Description of the Project the invite is created for. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the Project the invitation is for. | 
**role** | [**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectRoles**](de.mittwald.v1.membership.ProjectRoles.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


