# DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectMembership

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | Email used by the invited user. | 
**expires_at** | Option<**String**> | Time the ProjectMembership should expire at. | [optional]
**id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the ProjectMembership. | 
**inherited** | **bool** | Whether the ProjectMembership was inherited from a CustomerMembership. | 
**invite_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | ID of the ProjectInvite the membership was created from. | [optional]
**member_since** | Option<**String**> | Date the projectMembership was created at. | [optional]
**mfa** | **bool** | MFA activated by the user. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the Project the membership is for. | 
**role** | [**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectRoles**](de.mittwald.v1.membership.ProjectRoles.md) |  | 
**user_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the user the ProjectMembership is for. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


