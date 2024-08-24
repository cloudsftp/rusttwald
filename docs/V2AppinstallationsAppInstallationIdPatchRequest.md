# V2AppinstallationsAppInstallationIdPatchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_version_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**custom_document_root** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**system_software** | Option<[**std::collections::HashMap<String, models::V2AppinstallationsAppInstallationIdPatchRequestSystemSoftwareValue>**](_v2_appinstallations__appInstallationId__patch_request_systemSoftware_value.md)> |  | [optional]
**update_policy** | Option<[**models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppUpdatePolicy**](de.mittwald.v1.app.AppUpdatePolicy.md)> |  | [optional]
**user_inputs** | Option<[**Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodSavedUserInput>**](de.mittwald.v1.app.SavedUserInput.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


