# \CronjobApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cronjob_abort_execution**](CronjobApi.md#cronjob_abort_execution) | **POST** /v2/cronjobs/{cronjobId}/executions/{executionId}/actions/abort | Abort a CronjobExecution.
[**cronjob_create_cronjob**](CronjobApi.md#cronjob_create_cronjob) | **POST** /v2/projects/{projectId}/cronjobs | Create a Cronjob.
[**cronjob_create_execution**](CronjobApi.md#cronjob_create_execution) | **POST** /v2/cronjobs/{cronjobId}/executions | Trigger a Cronjob.
[**cronjob_delete_cronjob**](CronjobApi.md#cronjob_delete_cronjob) | **DELETE** /v2/cronjobs/{cronjobId} | Delete a Cronjob.
[**cronjob_get_cronjob**](CronjobApi.md#cronjob_get_cronjob) | **GET** /v2/cronjobs/{cronjobId} | Get a Cronjob.
[**cronjob_get_execution**](CronjobApi.md#cronjob_get_execution) | **GET** /v2/cronjobs/{cronjobId}/executions/{executionId} | Get a CronjobExecution.
[**cronjob_list_cronjobs**](CronjobApi.md#cronjob_list_cronjobs) | **GET** /v2/projects/{projectId}/cronjobs | List Cronjobs belonging to a Project.
[**cronjob_list_executions**](CronjobApi.md#cronjob_list_executions) | **GET** /v2/cronjobs/{cronjobId}/executions | List CronjobExecutions belonging to a Cronjob.
[**cronjob_update_cronjob**](CronjobApi.md#cronjob_update_cronjob) | **PATCH** /v2/cronjobs/{cronjobId} | Update a Cronjob.
[**cronjob_update_cronjob_app_id**](CronjobApi.md#cronjob_update_cronjob_app_id) | **PATCH** /v2/cronjobs/{cronjobId}/app-id | Update a Cronjob's app id.



## cronjob_abort_execution

> cronjob_abort_execution(cronjob_id, execution_id, body)
Abort a CronjobExecution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cronjob_id** | **uuid::Uuid** | ID of the CronjobExecution to be aborted. | [required] |
**execution_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cronjob_create_cronjob

> models::AppRequestAppinstallation201Response cronjob_create_cronjob(project_id, de_period_mittwald_period_v1_period_cronjob_period_cronjob_request)
Create a Cronjob.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project you want to create a Cronjob for. | [required] |
**de_period_mittwald_period_v1_period_cronjob_period_cronjob_request** | Option<[**DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjobRequest**](DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjobRequest.md)> | cronjob |  |

### Return type

[**models::AppRequestAppinstallation201Response**](app_request_appinstallation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cronjob_create_execution

> models::CronjobCreateExecution201Response cronjob_create_execution(cronjob_id, body)
Trigger a Cronjob.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cronjob_id** | **uuid::Uuid** | ID of the Cronjob to trigger. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::CronjobCreateExecution201Response**](cronjob_create_execution_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cronjob_delete_cronjob

> cronjob_delete_cronjob(cronjob_id)
Delete a Cronjob.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cronjob_id** | **uuid::Uuid** | ID of the Cronjob to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cronjob_get_cronjob

> models::DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjob cronjob_get_cronjob(cronjob_id)
Get a Cronjob.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cronjob_id** | **uuid::Uuid** | ID of the Cronjob to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjob**](de.mittwald.v1.cronjob.Cronjob.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cronjob_get_execution

> models::DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjobExecution cronjob_get_execution(execution_id, cronjob_id)
Get a CronjobExecution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**execution_id** | **String** | ID of the CronjobExecution to be retrieved. | [required] |
**cronjob_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjobExecution**](de.mittwald.v1.cronjob.CronjobExecution.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cronjob_list_cronjobs

> Vec<models::DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjob> cronjob_list_cronjobs(project_id, limit, skip, page)
List Cronjobs belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project for which to list Cronjobs for. | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjob>**](de.mittwald.v1.cronjob.Cronjob.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cronjob_list_executions

> Vec<models::DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjobExecution> cronjob_list_executions(cronjob_id, limit, skip, page, since, until, status)
List CronjobExecutions belonging to a Cronjob.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cronjob_id** | **uuid::Uuid** | ID of the Cronjob for which to list CronjobExecutions for. | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |
**since** | Option<**String**> |  |  |
**until** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjobExecution>**](de.mittwald.v1.cronjob.CronjobExecution.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cronjob_update_cronjob

> cronjob_update_cronjob(cronjob_id, cronjob_update_cronjob_request)
Update a Cronjob.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cronjob_id** | **uuid::Uuid** | ID of the Cronjob to be updated. | [required] |
**cronjob_update_cronjob_request** | Option<[**CronjobUpdateCronjobRequest**](CronjobUpdateCronjobRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cronjob_update_cronjob_app_id

> cronjob_update_cronjob_app_id(cronjob_id, cronjob_update_cronjob_app_id_request)
Update a Cronjob's app id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cronjob_id** | **uuid::Uuid** | ID of the Cronjob to update. | [required] |
**cronjob_update_cronjob_app_id_request** | Option<[**CronjobUpdateCronjobAppIdRequest**](CronjobUpdateCronjobAppIdRequest.md)> | ID of the app to set. |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

