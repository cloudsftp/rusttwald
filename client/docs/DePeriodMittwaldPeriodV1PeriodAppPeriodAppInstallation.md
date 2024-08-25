# DePeriodMittwaldPeriodV1PeriodAppPeriodAppInstallation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**app_version** | [**models::DePeriodMittwaldPeriodV1PeriodAppPeriodVersionStatus**](de.mittwald.v1.app.VersionStatus.md) |  | 
**custom_document_root** | Option<**String**> |  | [optional]
**description** | **String** |  | 
**disabled** | **bool** |  | [default to false]
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**installation_path** | **String** |  | 
**linked_databases** | Option<[**Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodLinkedDatabase>**](de.mittwald.v1.app.LinkedDatabase.md)> |  | [optional]
**processes** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**project_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**screenshot_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**screenshot_ref** | Option<**String**> |  | [optional]
**short_id** | **String** |  | 
**system_software** | Option<[**Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodInstalledSystemSoftware>**](de.mittwald.v1.app.InstalledSystemSoftware.md)> |  | [optional]
**update_policy** | Option<[**models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppUpdatePolicy**](de.mittwald.v1.app.AppUpdatePolicy.md)> |  | [optional]
**user_inputs** | Option<[**Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodSavedUserInput>**](de.mittwald.v1.app.SavedUserInput.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


