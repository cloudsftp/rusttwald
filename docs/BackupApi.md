# \BackupApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**backup_create_project_backup**](BackupApi.md#backup_create_project_backup) | **POST** /v2/projects/{projectId}/backups | Create a Backup of a Project.
[**backup_create_project_backup_export**](BackupApi.md#backup_create_project_backup_export) | **POST** /v2/project-backups/{projectBackupId}/export | Export a ProjectBackup for download.
[**backup_create_project_backup_schedule**](BackupApi.md#backup_create_project_backup_schedule) | **POST** /v2/projects/{projectId}/backup-schedules | Create a BackupSchedule for a Project.
[**backup_delete_project_backup**](BackupApi.md#backup_delete_project_backup) | **DELETE** /v2/project-backups/{projectBackupId} | Delete a ProjectBackup.
[**backup_delete_project_backup_export**](BackupApi.md#backup_delete_project_backup_export) | **DELETE** /v2/project-backups/{projectBackupId}/export | Delete a ProjectBackupExport.
[**backup_delete_project_backup_schedule**](BackupApi.md#backup_delete_project_backup_schedule) | **DELETE** /v2/project-backup-schedules/{projectBackupScheduleId} | Delete a ProjectBackupSchedule.
[**backup_get_project_backup**](BackupApi.md#backup_get_project_backup) | **GET** /v2/project-backups/{projectBackupId} | Get a ProjectBackup.
[**backup_get_project_backup_schedule**](BackupApi.md#backup_get_project_backup_schedule) | **GET** /v2/project-backup-schedules/{projectBackupScheduleId} | Get a ProjectBackupSchedule.
[**backup_list_project_backup_schedules**](BackupApi.md#backup_list_project_backup_schedules) | **GET** /v2/projects/{projectId}/backup-schedules | List BackupSchedules belonging to a Project.
[**backup_list_project_backups**](BackupApi.md#backup_list_project_backups) | **GET** /v2/projects/{projectId}/backups | List Backups belonging to a Project.
[**backup_update_project_backup_description**](BackupApi.md#backup_update_project_backup_description) | **PATCH** /v2/project-backups/{projectBackupId}/description | Change the description of a ProjectBackup.
[**backup_update_project_backup_schedule**](BackupApi.md#backup_update_project_backup_schedule) | **PATCH** /v2/project-backup-schedules/{projectBackupScheduleId} | Update a ProjectBackupSchedule.



## backup_create_project_backup

> models::DePeriodMittwaldPeriodV1PeriodBackupPeriodProjectBackup backup_create_project_backup(project_id, backup_create_project_backup_request)
Create a Backup of a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the project to create a Backup for. | [required] |
**backup_create_project_backup_request** | [**BackupCreateProjectBackupRequest**](BackupCreateProjectBackupRequest.md) |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodBackupPeriodProjectBackup**](de.mittwald.v1.backup.ProjectBackup.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## backup_create_project_backup_export

> backup_create_project_backup_export(project_backup_id, backup_create_project_backup_export_request)
Export a ProjectBackup for download.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_backup_id** | **uuid::Uuid** | ID of the ProjectBackup to export. | [required] |
**backup_create_project_backup_export_request** | [**BackupCreateProjectBackupExportRequest**](BackupCreateProjectBackupExportRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## backup_create_project_backup_schedule

> models::DePeriodMittwaldPeriodV1PeriodBackupPeriodProjectBackupSchedule backup_create_project_backup_schedule(project_id, backup_create_project_backup_schedule_request)
Create a BackupSchedule for a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to create a BackupSchedule for. | [required] |
**backup_create_project_backup_schedule_request** | [**BackupCreateProjectBackupScheduleRequest**](BackupCreateProjectBackupScheduleRequest.md) |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodBackupPeriodProjectBackupSchedule**](de.mittwald.v1.backup.ProjectBackupSchedule.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## backup_delete_project_backup

> backup_delete_project_backup(project_backup_id)
Delete a ProjectBackup.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_backup_id** | **uuid::Uuid** | ID of the ProjectBackup to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## backup_delete_project_backup_export

> backup_delete_project_backup_export(project_backup_id)
Delete a ProjectBackupExport.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_backup_id** | **uuid::Uuid** | ID if the ProjectBackup to delete the export of. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## backup_delete_project_backup_schedule

> backup_delete_project_backup_schedule(project_backup_schedule_id)
Delete a ProjectBackupSchedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_backup_schedule_id** | **uuid::Uuid** | ID of the ProjectBackupSchedule to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## backup_get_project_backup

> models::DePeriodMittwaldPeriodV1PeriodBackupPeriodProjectBackup backup_get_project_backup(project_backup_id)
Get a ProjectBackup.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_backup_id** | **uuid::Uuid** | ID of the ProjectBackup to retrieve. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodBackupPeriodProjectBackup**](de.mittwald.v1.backup.ProjectBackup.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## backup_get_project_backup_schedule

> models::DePeriodMittwaldPeriodV1PeriodBackupPeriodProjectBackupSchedule backup_get_project_backup_schedule(project_backup_schedule_id)
Get a ProjectBackupSchedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_backup_schedule_id** | **uuid::Uuid** | ID of the ProjectBackupSchedule to retrieve. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodBackupPeriodProjectBackupSchedule**](de.mittwald.v1.backup.ProjectBackupSchedule.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## backup_list_project_backup_schedules

> Vec<models::DePeriodMittwaldPeriodV1PeriodBackupPeriodProjectBackupSchedule> backup_list_project_backup_schedules(project_id)
List BackupSchedules belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to list BackupSchedules for. | [required] |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodBackupPeriodProjectBackupSchedule>**](de.mittwald.v1.backup.ProjectBackupSchedule.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## backup_list_project_backups

> Vec<models::DePeriodMittwaldPeriodV1PeriodBackupPeriodProjectBackup> backup_list_project_backups(project_id)
List Backups belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to get Backups for. | [required] |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodBackupPeriodProjectBackup>**](de.mittwald.v1.backup.ProjectBackup.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## backup_update_project_backup_description

> backup_update_project_backup_description(project_backup_id, backup_update_project_backup_description_request)
Change the description of a ProjectBackup.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_backup_id** | **uuid::Uuid** | ID of the ProjectBackup to change the description of. | [required] |
**backup_update_project_backup_description_request** | [**BackupUpdateProjectBackupDescriptionRequest**](BackupUpdateProjectBackupDescriptionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## backup_update_project_backup_schedule

> backup_update_project_backup_schedule(project_backup_schedule_id, backup_update_project_backup_schedule_request)
Update a ProjectBackupSchedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_backup_schedule_id** | **uuid::Uuid** | ID of the ProjectBackupSchedule to be updated. | [required] |
**backup_update_project_backup_schedule_request** | Option<[**BackupUpdateProjectBackupScheduleRequest**](BackupUpdateProjectBackupScheduleRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

