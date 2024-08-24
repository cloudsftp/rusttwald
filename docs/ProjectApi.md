# \ProjectApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**project_accept_project_invite**](ProjectApi.md#project_accept_project_invite) | **POST** /v2/project-invites/{projectInviteId}/actions/accept | Accept a ProjectInvite.
[**project_create_project**](ProjectApi.md#project_create_project) | **POST** /v2/servers/{serverId}/projects | Create a Project belonging to a Server.
[**project_create_project_invite**](ProjectApi.md#project_create_project_invite) | **POST** /v2/projects/{projectId}/invites | Create a ProjectInvite.
[**project_decline_project_invite**](ProjectApi.md#project_decline_project_invite) | **POST** /v2/project-invites/{projectInviteId}/actions/decline | Decline a ProjectInvite.
[**project_delete_project**](ProjectApi.md#project_delete_project) | **DELETE** /v2/projects/{projectId} | Delete a Project.
[**project_delete_project_avatar**](ProjectApi.md#project_delete_project_avatar) | **DELETE** /v2/projects/{projectId}/avatar | Delete a Project's avatar.
[**project_delete_project_invite**](ProjectApi.md#project_delete_project_invite) | **DELETE** /v2/project-invites/{projectInviteId} | Delete a ProjectInvite.
[**project_delete_project_membership**](ProjectApi.md#project_delete_project_membership) | **DELETE** /v2/project-memberships/{projectMembershipId} | Delete a ProjectMembership.
[**project_delete_server_avatar**](ProjectApi.md#project_delete_server_avatar) | **DELETE** /v2/servers/{serverId}/avatar | Delete a Server's avatar.
[**project_get_project**](ProjectApi.md#project_get_project) | **GET** /v2/projects/{projectId} | Get a Project.
[**project_get_project_invite**](ProjectApi.md#project_get_project_invite) | **GET** /v2/project-invites/{projectInviteId} | Get a ProjectInvite.
[**project_get_project_membership**](ProjectApi.md#project_get_project_membership) | **GET** /v2/project-memberships/{projectMembershipId} | Get a ProjectMembership
[**project_get_project_token_invite**](ProjectApi.md#project_get_project_token_invite) | **GET** /v2/project-token-invite | Get a ProjectInvite by token.
[**project_get_self_membership_for_project**](ProjectApi.md#project_get_self_membership_for_project) | **GET** /v2/projects/{projectId}/memberships/self | Get the executing user's membership in a Project.
[**project_get_server**](ProjectApi.md#project_get_server) | **GET** /v2/servers/{serverId} | Get a Server.
[**project_leave_project**](ProjectApi.md#project_leave_project) | **POST** /v2/projects/{projectId}/leave | Leave a Project.
[**project_list_invites_for_project**](ProjectApi.md#project_list_invites_for_project) | **GET** /v2/projects/{projectId}/invites | List Invites belonging to a Project.
[**project_list_memberships_for_project**](ProjectApi.md#project_list_memberships_for_project) | **GET** /v2/projects/{projectId}/memberships | List Memberships belonging to a Project.
[**project_list_project_invites**](ProjectApi.md#project_list_project_invites) | **GET** /v2/project-invites | List ProjectInvites belonging to the executing user.
[**project_list_project_memberships**](ProjectApi.md#project_list_project_memberships) | **GET** /v2/project-memberships | List ProjectMemberships belonging to the executing user.
[**project_list_projects**](ProjectApi.md#project_list_projects) | **GET** /v2/projects | List Projects belonging to the executing user.
[**project_list_servers**](ProjectApi.md#project_list_servers) | **GET** /v2/servers | List Servers belonging to the executing user.
[**project_request_project_avatar_upload**](ProjectApi.md#project_request_project_avatar_upload) | **POST** /v2/projects/{projectId}/avatar | Request a Project avatar upload.
[**project_request_server_avatar_upload**](ProjectApi.md#project_request_server_avatar_upload) | **POST** /v2/servers/{serverId}/avatar | Request a Server avatar upload.
[**project_resend_project_invite_mail**](ProjectApi.md#project_resend_project_invite_mail) | **POST** /v2/project-invites/{projectInviteId}/actions/resend | Resend the mail for a ProjectInvite.
[**project_update_project_description**](ProjectApi.md#project_update_project_description) | **PATCH** /v2/projects/{projectId}/description | Update a Project's description.
[**project_update_project_membership**](ProjectApi.md#project_update_project_membership) | **PATCH** /v2/project-memberships/{projectMembershipId} | Update a ProjectMembership.
[**project_update_server_description**](ProjectApi.md#project_update_server_description) | **PATCH** /v2/servers/{serverId}/description | Update a Servers's description.



## project_accept_project_invite

> project_accept_project_invite(project_invite_id, customer_accept_customer_invite_request)
Accept a ProjectInvite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_invite_id** | **uuid::Uuid** | ID of the ProjectInvite to be accepted. | [required] |
**customer_accept_customer_invite_request** | [**CustomerAcceptCustomerInviteRequest**](CustomerAcceptCustomerInviteRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_create_project

> models::AppRequestAppinstallation201Response project_create_project(server_id, project_create_project_request)
Create a Project belonging to a Server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **uuid::Uuid** | ID of the Server to create the Project for. | [required] |
**project_create_project_request** | Option<[**ProjectCreateProjectRequest**](ProjectCreateProjectRequest.md)> |  |  |

### Return type

[**models::AppRequestAppinstallation201Response**](app_request_appinstallation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_create_project_invite

> models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectInvite project_create_project_invite(project_id, project_create_project_invite_request)
Create a ProjectInvite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to create a ProjectInvite for. | [required] |
**project_create_project_invite_request** | [**ProjectCreateProjectInviteRequest**](ProjectCreateProjectInviteRequest.md) |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectInvite**](de.mittwald.v1.membership.ProjectInvite.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_decline_project_invite

> project_decline_project_invite(project_invite_id, body)
Decline a ProjectInvite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_invite_id** | **uuid::Uuid** | ID of the ProjectInvite to be declined. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_delete_project

> project_delete_project(project_id)
Delete a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the Project. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_delete_project_avatar

> project_delete_project_avatar(project_id)
Delete a Project's avatar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project the avatar should be deleted for. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_delete_project_invite

> project_delete_project_invite(project_invite_id)
Delete a ProjectInvite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_invite_id** | **uuid::Uuid** | ID of the ProjectInvite to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_delete_project_membership

> project_delete_project_membership(project_membership_id)
Delete a ProjectMembership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_membership_id** | **uuid::Uuid** | ID of the ProjectMembership to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_delete_server_avatar

> project_delete_server_avatar(server_id)
Delete a Server's avatar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **uuid::Uuid** | ID of the Server to delete the avatar for. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_get_project

> models::DePeriodMittwaldPeriodV1PeriodProjectPeriodProject project_get_project(project_id)
Get a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodProjectPeriodProject**](de.mittwald.v1.project.Project.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_get_project_invite

> models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectInvite project_get_project_invite(project_invite_id)
Get a ProjectInvite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_invite_id** | **String** | ID of the ProjectInvite to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectInvite**](de.mittwald.v1.membership.ProjectInvite.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_get_project_membership

> models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectMembership project_get_project_membership(project_membership_id)
Get a ProjectMembership

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_membership_id** | **String** | ID of the ProjectMembership to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectMembership**](de.mittwald.v1.membership.ProjectMembership.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_get_project_token_invite

> models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectInvite project_get_project_token_invite(token)
Get a ProjectInvite by token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Token of the ProjectInvite to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectInvite**](de.mittwald.v1.membership.ProjectInvite.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_get_self_membership_for_project

> models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectMembership project_get_self_membership_for_project(project_id)
Get the executing user's membership in a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to get the membership for. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectMembership**](de.mittwald.v1.membership.ProjectMembership.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_get_server

> models::DePeriodMittwaldPeriodV1PeriodProjectPeriodServer project_get_server(server_id)
Get a Server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** | ID of the Server to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodProjectPeriodServer**](de.mittwald.v1.project.Server.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_leave_project

> project_leave_project(project_id, body)
Leave a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to be left. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_list_invites_for_project

> Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectInvite> project_list_invites_for_project(project_id, limit, skip)
List Invites belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to list invites for. | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectInvite>**](de.mittwald.v1.membership.ProjectInvite.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_list_memberships_for_project

> Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectMembership> project_list_memberships_for_project(project_id, limit, skip)
List Memberships belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to list memberships for. | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectMembership>**](de.mittwald.v1.membership.ProjectMembership.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_list_project_invites

> Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectInvite> project_list_project_invites(limit, skip)
List ProjectInvites belonging to the executing user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectInvite>**](de.mittwald.v1.membership.ProjectInvite.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_list_project_memberships

> Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectMembership> project_list_project_memberships(limit, skip)
List ProjectMemberships belonging to the executing user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectMembership>**](de.mittwald.v1.membership.ProjectMembership.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_list_projects

> Vec<models::ProjectListProjects200ResponseInner> project_list_projects(customer_id, server_id, limit, skip, page)
List Projects belonging to the executing user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | ID of the Organization to list Projects for. |  |
**server_id** | Option<**uuid::Uuid**> | ID of the Server to list Projects for. |  |
**limit** | Option<**i32**> | The max count of resources to return in the list response. |  |[default to 10000]
**skip** | Option<**i32**> | Number of items to skip. Should be multiple of `limit`. |  |[default to 0]
**page** | Option<**i32**> | Page to display. `skip` and `page` end up doing the same. If both `page` and `skip` are set, skip is used. |  |

### Return type

[**Vec<models::ProjectListProjects200ResponseInner>**](project_list_projects_200_response_inner.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_list_servers

> Vec<models::DePeriodMittwaldPeriodV1PeriodProjectPeriodServer> project_list_servers(customer_id, limit, page, skip)
List Servers belonging to the executing user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | ID of the Organization to list PlacementGroups for. If no ID is provided, the ID of the executing User is used instead. |  |
**limit** | Option<**i32**> |  |  |[default to 10000]
**page** | Option<**i32**> |  |  |[default to 1]
**skip** | Option<**i32**> |  |  |[default to 0]

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodProjectPeriodServer>**](de.mittwald.v1.project.Server.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_request_project_avatar_upload

> models::ProjectRequestProjectAvatarUpload200Response project_request_project_avatar_upload(project_id)
Request a Project avatar upload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project you want to request an avatar upload for. | [required] |

### Return type

[**models::ProjectRequestProjectAvatarUpload200Response**](project_request_project_avatar_upload_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_request_server_avatar_upload

> models::ProjectRequestProjectAvatarUpload200Response project_request_server_avatar_upload(server_id)
Request a Server avatar upload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **uuid::Uuid** | ID of the Server to request an avatar upload for. | [required] |

### Return type

[**models::ProjectRequestProjectAvatarUpload200Response**](project_request_project_avatar_upload_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_resend_project_invite_mail

> project_resend_project_invite_mail(project_invite_id, body)
Resend the mail for a ProjectInvite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_invite_id** | **uuid::Uuid** | ID of the ProjectInvite to resend the mail for. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_update_project_description

> project_update_project_description(project_id, project_update_project_description_request)
Update a Project's description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to update the description for. | [required] |
**project_update_project_description_request** | Option<[**ProjectUpdateProjectDescriptionRequest**](ProjectUpdateProjectDescriptionRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_update_project_membership

> project_update_project_membership(project_membership_id, project_update_project_membership_request)
Update a ProjectMembership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_membership_id** | **uuid::Uuid** | ID of the ProjectMembership to be updated. | [required] |
**project_update_project_membership_request** | [**ProjectUpdateProjectMembershipRequest**](ProjectUpdateProjectMembershipRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_update_server_description

> project_update_server_description(server_id, project_update_server_description_request)
Update a Servers's description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **uuid::Uuid** | ID of the Server to update the description of. | [required] |
**project_update_server_description_request** | Option<[**ProjectUpdateServerDescriptionRequest**](ProjectUpdateServerDescriptionRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

