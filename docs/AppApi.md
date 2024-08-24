# \AppApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_execute_action**](AppApi.md#app_execute_action) | **POST** /v2/app-installations/{appInstallationId}/actions/{action} | Trigger a runtime action belonging to an AppInstallation.
[**app_get_app**](AppApi.md#app_get_app) | **GET** /v2/apps/{appId} | Get an App.
[**app_get_appinstallation**](AppApi.md#app_get_appinstallation) | **GET** /v2/app-installations/{appInstallationId} | Get an AppInstallation.
[**app_get_appversion**](AppApi.md#app_get_appversion) | **GET** /v2/apps/{appId}/versions/{appVersionId} | Get an AppVersion.
[**app_get_installed_systemsoftware_for_appinstallation**](AppApi.md#app_get_installed_systemsoftware_for_appinstallation) | **GET** /v2/app-installations/{appInstallationId}/systemSoftware | Get the installed `SystemSoftware' for a specific `AppInstallation`.
[**app_get_missing_dependencies_for_appinstallation**](AppApi.md#app_get_missing_dependencies_for_appinstallation) | **GET** /v2/app-installations/{appInstallationId}/missing-dependencies | Get the missing requirements of an appInstallation for a specific target AppVersion.
[**app_get_systemsoftware**](AppApi.md#app_get_systemsoftware) | **GET** /v2/system-softwares/{systemSoftwareId} | Get a SystemSoftware.
[**app_get_systemsoftwareversion**](AppApi.md#app_get_systemsoftwareversion) | **GET** /v2/system-software/{systemSoftwareId}/versions/{systemSoftwareVersionId} | Get a SystemSoftwareVersion.
[**app_link_database**](AppApi.md#app_link_database) | **PATCH** /v2/app-installations/{appInstallationId}/database | Create linkage between an AppInstallation and a MySQLDatabase.
[**app_list_appinstallations**](AppApi.md#app_list_appinstallations) | **GET** /v2/projects/{projectId}/app-installations | List AppInstallations belonging to a Project.
[**app_list_appinstallations_for_user**](AppApi.md#app_list_appinstallations_for_user) | **GET** /v2/app-installations | List AppInstallations that a user has access to.
[**app_list_apps**](AppApi.md#app_list_apps) | **GET** /v2/apps | List Apps.
[**app_list_appversions**](AppApi.md#app_list_appversions) | **GET** /v2/apps/{appId}/versions | List AppVersions belonging to an App.
[**app_list_systemsoftwares**](AppApi.md#app_list_systemsoftwares) | **GET** /v2/system-softwares | List SystemSoftwares.
[**app_list_systemsoftwareversions**](AppApi.md#app_list_systemsoftwareversions) | **GET** /v2/system-software/{systemSoftwareId}/versions | List SystemSoftwareVersions belonging to a SystemSoftware.
[**app_list_update_candidates_for_appversion**](AppApi.md#app_list_update_candidates_for_appversion) | **GET** /v2/apps/{appId}/versions/{baseAppVersionId}/update-candidates | List update candidates belonging to an AppVersion.
[**app_patch_appinstallation**](AppApi.md#app_patch_appinstallation) | **PATCH** /v2/app-installations/{appInstallationId} | Update properties belonging to an AppInstallation.
[**app_replace_database**](AppApi.md#app_replace_database) | **PATCH** /v2/app-installations/{appInstallationId}/database/replace | Replace a MySQL Database with another MySQL Database.
[**app_request_appinstallation**](AppApi.md#app_request_appinstallation) | **POST** /v2/projects/{projectId}/app-installations | Request an AppInstallation.
[**app_request_appinstallation_copy**](AppApi.md#app_request_appinstallation_copy) | **POST** /v2/app-installations/{appInstallationId}/actions/copy | Request a copy of an AppInstallation.
[**app_retrieve_status**](AppApi.md#app_retrieve_status) | **GET** /v2/app-installations/{appInstallationId}/status | Get runtime status belonging to an AppInstallation.
[**app_set_database_users**](AppApi.md#app_set_database_users) | **PUT** /v2/app-installations/{appInstallationId}/databases/{databaseId}/users | Create linkage between an AppInstallation and DatabaseUsers.
[**app_uninstall_appinstallation**](AppApi.md#app_uninstall_appinstallation) | **DELETE** /v2/app-installations/{appInstallationId} | Trigger an uninstallation process for an AppInstallation.
[**app_unlink_database**](AppApi.md#app_unlink_database) | **DELETE** /v2/app-installations/{appInstallationId}/databases/{databaseId} | Remove linkage between an AppInstallation and a Database.



## app_execute_action

> app_execute_action(app_installation_id, action, body)
Trigger a runtime action belonging to an AppInstallation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_installation_id** | **String** |  | [required] |
**action** | [**DePeriodMittwaldPeriodV1PeriodAppPeriodAction**](.md) |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_get_app

> models::DePeriodMittwaldPeriodV1PeriodAppPeriodApp app_get_app(app_id)
Get an App.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodAppPeriodApp**](de.mittwald.v1.app.App.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_get_appinstallation

> models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppInstallation app_get_appinstallation(app_installation_id)
Get an AppInstallation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_installation_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppInstallation**](de.mittwald.v1.app.AppInstallation.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_get_appversion

> models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppVersion app_get_appversion(app_id, app_version_id)
Get an AppVersion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **uuid::Uuid** |  | [required] |
**app_version_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppVersion**](de.mittwald.v1.app.AppVersion.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_get_installed_systemsoftware_for_appinstallation

> Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodSystemSoftware> app_get_installed_systemsoftware_for_appinstallation(app_installation_id, tag_filter)
Get the installed `SystemSoftware' for a specific `AppInstallation`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_installation_id** | **uuid::Uuid** |  | [required] |
**tag_filter** | Option<**String**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodSystemSoftware>**](de.mittwald.v1.app.SystemSoftware.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_get_missing_dependencies_for_appinstallation

> models::AppGetMissingDependenciesForAppinstallation200Response app_get_missing_dependencies_for_appinstallation(app_installation_id, target_app_version_id)
Get the missing requirements of an appInstallation for a specific target AppVersion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_installation_id** | **uuid::Uuid** |  | [required] |
**target_app_version_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::AppGetMissingDependenciesForAppinstallation200Response**](app_get_missing_dependencies_for_appinstallation_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_get_systemsoftware

> models::DePeriodMittwaldPeriodV1PeriodAppPeriodSystemSoftware app_get_systemsoftware(system_software_id)
Get a SystemSoftware.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_software_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodAppPeriodSystemSoftware**](de.mittwald.v1.app.SystemSoftware.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_get_systemsoftwareversion

> models::DePeriodMittwaldPeriodV1PeriodAppPeriodSystemSoftwareVersion app_get_systemsoftwareversion(system_software_id, system_software_version_id)
Get a SystemSoftwareVersion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_software_id** | **uuid::Uuid** |  | [required] |
**system_software_version_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodAppPeriodSystemSoftwareVersion**](de.mittwald.v1.app.SystemSoftwareVersion.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_link_database

> app_link_database(app_installation_id, app_link_database_request)
Create linkage between an AppInstallation and a MySQLDatabase.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_installation_id** | **uuid::Uuid** |  | [required] |
**app_link_database_request** | [**AppLinkDatabaseRequest**](AppLinkDatabaseRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_list_appinstallations

> Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppInstallation> app_list_appinstallations(project_id, limit, page, skip)
List AppInstallations belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**limit** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppInstallation>**](de.mittwald.v1.app.AppInstallation.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_list_appinstallations_for_user

> Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppInstallation> app_list_appinstallations_for_user(limit, page, skip)
List AppInstallations that a user has access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppInstallation>**](de.mittwald.v1.app.AppInstallation.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_list_apps

> Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodApp> app_list_apps(limit, page, skip)
List Apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodApp>**](de.mittwald.v1.app.App.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_list_appversions

> Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppVersion> app_list_appversions(app_id, recommended)
List AppVersions belonging to an App.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **uuid::Uuid** |  | [required] |
**recommended** | Option<**bool**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppVersion>**](de.mittwald.v1.app.AppVersion.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_list_systemsoftwares

> Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodSystemSoftware> app_list_systemsoftwares(limit, page, skip)
List SystemSoftwares.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodSystemSoftware>**](de.mittwald.v1.app.SystemSoftware.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_list_systemsoftwareversions

> Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodSystemSoftwareVersion> app_list_systemsoftwareversions(system_software_id, version_range, recommended)
List SystemSoftwareVersions belonging to a SystemSoftware.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_software_id** | **uuid::Uuid** |  | [required] |
**version_range** | Option<**String**> |  |  |
**recommended** | Option<**bool**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodSystemSoftwareVersion>**](de.mittwald.v1.app.SystemSoftwareVersion.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_list_update_candidates_for_appversion

> Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppVersion> app_list_update_candidates_for_appversion(app_id, base_app_version_id)
List update candidates belonging to an AppVersion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **uuid::Uuid** |  | [required] |
**base_app_version_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppVersion>**](de.mittwald.v1.app.AppVersion.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_patch_appinstallation

> app_patch_appinstallation(app_installation_id, app_patch_appinstallation_request)
Update properties belonging to an AppInstallation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_installation_id** | **uuid::Uuid** |  | [required] |
**app_patch_appinstallation_request** | [**AppPatchAppinstallationRequest**](AppPatchAppinstallationRequest.md) | Properties to update for an AppInstallation. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_replace_database

> app_replace_database(app_installation_id, app_replace_database_request)
Replace a MySQL Database with another MySQL Database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_installation_id** | **uuid::Uuid** | The ID of the appinstallation you want the database to be replaced for. | [required] |
**app_replace_database_request** | [**AppReplaceDatabaseRequest**](AppReplaceDatabaseRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_request_appinstallation

> models::AppRequestAppinstallation201Response app_request_appinstallation(project_id, app_request_appinstallation_request)
Request an AppInstallation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**app_request_appinstallation_request** | [**AppRequestAppinstallationRequest**](AppRequestAppinstallationRequest.md) |  | [required] |

### Return type

[**models::AppRequestAppinstallation201Response**](app_request_appinstallation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_request_appinstallation_copy

> models::AppRequestAppinstallation201Response app_request_appinstallation_copy(app_installation_id, app_request_appinstallation_copy_request)
Request a copy of an AppInstallation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_installation_id** | **uuid::Uuid** |  | [required] |
**app_request_appinstallation_copy_request** | [**AppRequestAppinstallationCopyRequest**](AppRequestAppinstallationCopyRequest.md) | Properties to copy an app installation. | [required] |

### Return type

[**models::AppRequestAppinstallation201Response**](app_request_appinstallation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_retrieve_status

> models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppInstallationStatus app_retrieve_status(app_installation_id)
Get runtime status belonging to an AppInstallation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_installation_id** | **String** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodAppPeriodAppInstallationStatus**](de.mittwald.v1.app.AppInstallationStatus.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_set_database_users

> app_set_database_users(app_installation_id, database_id, app_set_database_users_request)
Create linkage between an AppInstallation and DatabaseUsers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_installation_id** | **uuid::Uuid** |  | [required] |
**database_id** | **uuid::Uuid** |  | [required] |
**app_set_database_users_request** | [**AppSetDatabaseUsersRequest**](AppSetDatabaseUsersRequest.md) | Properties to update for the specified system software. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_uninstall_appinstallation

> app_uninstall_appinstallation(app_installation_id)
Trigger an uninstallation process for an AppInstallation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_installation_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_unlink_database

> app_unlink_database(app_installation_id, database_id)
Remove linkage between an AppInstallation and a Database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_installation_id** | **uuid::Uuid** |  | [required] |
**database_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

