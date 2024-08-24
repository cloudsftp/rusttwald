# \SshsftpUserApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sftp_user_create_sftp_user**](SshsftpUserApi.md#sftp_user_create_sftp_user) | **POST** /v2/projects/{projectId}/sftp-users | Create an SFTPUser for a Project.
[**sftp_user_delete_sftp_user**](SshsftpUserApi.md#sftp_user_delete_sftp_user) | **DELETE** /v2/sftp-users/{sftpUserId} | Delete an SFTPUser.
[**sftp_user_get_sftp_user**](SshsftpUserApi.md#sftp_user_get_sftp_user) | **GET** /v2/sftp-users/{sftpUserId} | Get an SFTPUser.
[**sftp_user_list_sftp_users**](SshsftpUserApi.md#sftp_user_list_sftp_users) | **GET** /v2/projects/{projectId}/sftp-users | Get all SFTPUsers for a Project.
[**sftp_user_update_sftp_user**](SshsftpUserApi.md#sftp_user_update_sftp_user) | **PATCH** /v2/sftp-users/{sftpUserId} | Update an SFTPUser.
[**ssh_user_create_ssh_user**](SshsftpUserApi.md#ssh_user_create_ssh_user) | **POST** /v2/projects/{projectId}/ssh-users | Create an SSHUser for a Project.
[**ssh_user_delete_ssh_user**](SshsftpUserApi.md#ssh_user_delete_ssh_user) | **DELETE** /v2/ssh-users/{sshUserId} | Delete an SSHUser.
[**ssh_user_get_ssh_user**](SshsftpUserApi.md#ssh_user_get_ssh_user) | **GET** /v2/ssh-users/{sshUserId} | Get an SSHUser.
[**ssh_user_list_ssh_users**](SshsftpUserApi.md#ssh_user_list_ssh_users) | **GET** /v2/projects/{projectId}/ssh-users | Get all SSHUsers for a Project.
[**ssh_user_update_ssh_user**](SshsftpUserApi.md#ssh_user_update_ssh_user) | **PATCH** /v2/ssh-users/{sshUserId} | Update an SSHUser.
[**v2_project_project_id_sftp_users_get**](SshsftpUserApi.md#v2_project_project_id_sftp_users_get) | **GET** /v2/project/{projectId}/sftp-users | Get all SFTPUsers for a Project.
[**v2_sshusers_ssh_user_id_patch**](SshsftpUserApi.md#v2_sshusers_ssh_user_id_patch) | **PATCH** /v2/sshusers/{sshUserId} | Update an SSHUser.



## sftp_user_create_sftp_user

> models::DePeriodMittwaldPeriodV1PeriodSshuserPeriodSftpUser sftp_user_create_sftp_user(project_id, sftp_user_create_sftp_user_request)
Create an SFTPUser for a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to create an SFTPUser for. | [required] |
**sftp_user_create_sftp_user_request** | [**SftpUserCreateSftpUserRequest**](SftpUserCreateSftpUserRequest.md) |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodSshuserPeriodSftpUser**](de.mittwald.v1.sshuser.SftpUser.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sftp_user_delete_sftp_user

> sftp_user_delete_sftp_user(sftp_user_id)
Delete an SFTPUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sftp_user_id** | **String** | ID of the SFTPUser to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sftp_user_get_sftp_user

> models::DePeriodMittwaldPeriodV1PeriodSshuserPeriodSftpUser sftp_user_get_sftp_user(sftp_user_id)
Get an SFTPUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sftp_user_id** | **String** | ID of the SFTPUser to get. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodSshuserPeriodSftpUser**](de.mittwald.v1.sshuser.SftpUser.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sftp_user_list_sftp_users

> Vec<models::DePeriodMittwaldPeriodV1PeriodSshuserPeriodSftpUser> sftp_user_list_sftp_users(project_id, limit, skip)
Get all SFTPUsers for a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to request SFTPUsers for. | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodSshuserPeriodSftpUser>**](de.mittwald.v1.sshuser.SftpUser.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sftp_user_update_sftp_user

> sftp_user_update_sftp_user(sftp_user_id, sftp_user_update_sftp_user_request)
Update an SFTPUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sftp_user_id** | **String** | ID of the SFTPUser to be updated. | [required] |
**sftp_user_update_sftp_user_request** | Option<[**SftpUserUpdateSftpUserRequest**](SftpUserUpdateSftpUserRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssh_user_create_ssh_user

> models::DePeriodMittwaldPeriodV1PeriodSshuserPeriodSshUser ssh_user_create_ssh_user(project_id, ssh_user_create_ssh_user_request)
Create an SSHUser for a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to create an SSHUser for. | [required] |
**ssh_user_create_ssh_user_request** | [**SshUserCreateSshUserRequest**](SshUserCreateSshUserRequest.md) |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodSshuserPeriodSshUser**](de.mittwald.v1.sshuser.SshUser.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssh_user_delete_ssh_user

> ssh_user_delete_ssh_user(ssh_user_id)
Delete an SSHUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_user_id** | **String** | ID of the SSHUser to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssh_user_get_ssh_user

> models::DePeriodMittwaldPeriodV1PeriodSshuserPeriodSshUser ssh_user_get_ssh_user(ssh_user_id)
Get an SSHUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_user_id** | **String** | ID of the SSHUser to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodSshuserPeriodSshUser**](de.mittwald.v1.sshuser.SshUser.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssh_user_list_ssh_users

> Vec<models::DePeriodMittwaldPeriodV1PeriodSshuserPeriodSshUser> ssh_user_list_ssh_users(project_id, limit, skip)
Get all SSHUsers for a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to retrieve SSHUsers for. | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodSshuserPeriodSshUser>**](de.mittwald.v1.sshuser.SshUser.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssh_user_update_ssh_user

> ssh_user_update_ssh_user(ssh_user_id, ssh_user_update_ssh_user_request)
Update an SSHUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_user_id** | **String** | ID of the SSHUser to be updated. | [required] |
**ssh_user_update_ssh_user_request** | Option<[**SshUserUpdateSshUserRequest**](SshUserUpdateSshUserRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_project_project_id_sftp_users_get

> v2_project_project_id_sftp_users_get(project_id, limit, skip)
Get all SFTPUsers for a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to request SFTPUsers for. | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_sshusers_ssh_user_id_patch

> v2_sshusers_ssh_user_id_patch(ssh_user_id, ssh_user_update_ssh_user_request)
Update an SSHUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_user_id** | **String** | ID of the SSHUser to be updated. | [required] |
**ssh_user_update_ssh_user_request** | Option<[**SshUserUpdateSshUserRequest**](SshUserUpdateSshUserRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

