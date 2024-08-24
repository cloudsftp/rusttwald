# ProjectListProjects200ResponseInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** |  | 
**customer_id** | **String** |  | 
**customer_meta** | [**models::ProjectListProjects200ResponseInnerCustomerMeta**](project_list_projects_200_response_inner_customerMeta.md) |  | 
**description** | **String** |  | 
**disable_reason** | Option<[**models::DePeriodMittwaldPeriodV1PeriodProjectPeriodDisableReason**](de.mittwald.v1.project.DisableReason.md)> |  | [optional]
**disabled_at** | Option<**String**> |  | [optional]
**enabled** | **bool** |  | 
**id** | **String** |  | 
**image_ref_id** | Option<**String**> |  | [optional]
**is_ready** | **bool** | deprecated | 
**project_hosting_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**readiness** | [**models::DePeriodMittwaldPeriodV1PeriodProjectPeriodDeprecatedProjectReadinessStatus**](de.mittwald.v1.project.DeprecatedProjectReadinessStatus.md) |  | 
**server_id** | Option<**String**> |  | [optional]
**short_id** | **String** |  | 
**status** | [**models::DePeriodMittwaldPeriodV1PeriodProjectPeriodProjectStatus**](de.mittwald.v1.project.ProjectStatus.md) |  | 
**status_set_at** | **String** |  | 
**web_storage_usage_in_bytes** | **i32** |  | 
**web_storage_usage_in_bytes_set_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


