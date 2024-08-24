# \ProjectFileSystemApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**project_file_system_get_directories**](ProjectFileSystemApi.md#project_file_system_get_directories) | **GET** /v2/projects/{projectId}/filesystem-directories | List directories belonging to a Project.
[**project_file_system_get_disk_usage**](ProjectFileSystemApi.md#project_file_system_get_disk_usage) | **GET** /v2/projects/{projectId}/filesystem-disk-usage | Get a Project directory filesystem usage.
[**project_file_system_get_file_content**](ProjectFileSystemApi.md#project_file_system_get_file_content) | **GET** /v2/projects/{projectId}/filesystem-file-content | Get a Project file's content.
[**project_file_system_get_jwt**](ProjectFileSystemApi.md#project_file_system_get_jwt) | **GET** /v2/projects/{projectId}/jwt | Get a Project's file/filesystem authorization token.
[**project_file_system_list_files**](ProjectFileSystemApi.md#project_file_system_list_files) | **GET** /v2/projects/{projectId}/filesystem-files | Get a Project file's information.



## project_file_system_get_directories

> models::DePeriodMittwaldPeriodV1PeriodProjectPeriodFilesystemDirectoryListing project_file_system_get_directories(project_id, directory, name, max_depth, r#type, executable, hidden)
List directories belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to list the directories for. | [required] |
**directory** | Option<**String**> | The directory to start listing subdirectories from. |  |
**name** | Option<**String**> | Search for specific filenames, only. The name may be a glob expression. |  |
**max_depth** | Option<**i32**> | The nesting depth for recursively listing directory contents. |  |
**r#type** | Option<[**Vec<String>**](String.md)> | One of file or directory (may be specified multiple times), to restrict results to directory items of this specific type. |  |
**executable** | Option<**bool**> | Set to true to constrain search results to executable files. |  |
**hidden** | Option<**bool**> | Set to true to include hidden files. (with . prefix in the search results). |  |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodProjectPeriodFilesystemDirectoryListing**](de.mittwald.v1.project.FilesystemDirectoryListing.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_file_system_get_disk_usage

> models::DePeriodMittwaldPeriodV1PeriodProjectPeriodFilesystemUsagesDisk project_file_system_get_disk_usage(project_id, directory)
Get a Project directory filesystem usage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project. | [required] |
**directory** | Option<**String**> | Starting directory for the disk usage calculation. |  |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodProjectPeriodFilesystemUsagesDisk**](de.mittwald.v1.project.FilesystemUsagesDisk.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_file_system_get_file_content

> String project_file_system_get_file_content(project_id, file, inline)
Get a Project file's content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project. | [required] |
**file** | Option<**String**> | Name of the file. |  |
**inline** | Option<**bool**> | Set to true to serve the file with Content-Disposition: inline. Otherwise, it will be served with Content-Disposition: attachment. filename=... |  |

### Return type

**String**

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_file_system_get_jwt

> models::DePeriodMittwaldPeriodV1PeriodProjectPeriodFsApiJwt project_file_system_get_jwt(project_id)
Get a Project's file/filesystem authorization token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to retrieve an authorization token for. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodProjectPeriodFsApiJwt**](de.mittwald.v1.project.FsApiJwt.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_file_system_list_files

> models::DePeriodMittwaldPeriodV1PeriodProjectPeriodFilesystemDirectoryListing project_file_system_list_files(project_id, file)
Get a Project file's information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project. | [required] |
**file** | Option<**String**> | Name of the file to retrieve. |  |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodProjectPeriodFilesystemDirectoryListing**](de.mittwald.v1.project.FilesystemDirectoryListing.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

