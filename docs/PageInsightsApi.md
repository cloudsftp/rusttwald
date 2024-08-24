# \PageInsightsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pageinsights_get_performance_data**](PageInsightsApi.md#pageinsights_get_performance_data) | **GET** /v2/page-insights | Get detailed performance data for a given domain and path.
[**pageinsights_get_strace_data**](PageInsightsApi.md#pageinsights_get_strace_data) | **GET** /v2/projects/{projectId}/straces/{straceId} | Get all data for a given strace.
[**pageinsights_list_performance_data_for_project**](PageInsightsApi.md#pageinsights_list_performance_data_for_project) | **GET** /v2/projects/{projectId}/page-insights | List websites (specified as domain and path) from a project where performance data is available.
[**pageinsights_schedule_strace**](PageInsightsApi.md#pageinsights_schedule_strace) | **POST** /v2/projects/{projectId}/straces | Schedule a strace measurement for a single http request.



## pageinsights_get_performance_data

> models::PageinsightsGetPerformanceData200Response pageinsights_get_performance_data(domain, path, date)
Get detailed performance data for a given domain and path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | A domain or subdomain. | [required] |
**path** | **String** | A path of the domain. | [required] |
**date** | Option<**String**> | Query data for a specific date, defaults to date today. |  |

### Return type

[**models::PageinsightsGetPerformanceData200Response**](pageinsights_get_performance_data_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pageinsights_get_strace_data

> models::PageinsightsGetStraceData200Response pageinsights_get_strace_data(strace_id, project_id)
Get all data for a given strace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**strace_id** | **uuid::Uuid** | ID of the strace to get the data for. | [required] |
**project_id** | **uuid::Uuid** | ID of the project the strace belongs to. | [required] |

### Return type

[**models::PageinsightsGetStraceData200Response**](pageinsights_get_strace_data_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pageinsights_list_performance_data_for_project

> Vec<models::PageinsightsListPerformanceDataForProject200ResponseInner> pageinsights_list_performance_data_for_project(project_id, domain)
List websites (specified as domain and path) from a project where performance data is available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to list domains for. | [required] |
**domain** | Option<**String**> | Filter for a specific domain. |  |

### Return type

[**Vec<models::PageinsightsListPerformanceDataForProject200ResponseInner>**](pageinsights_list_performance_data_for_project_200_response_inner.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pageinsights_schedule_strace

> models::AppRequestAppinstallation201Response pageinsights_schedule_strace(project_id, pageinsights_schedule_strace_request)
Schedule a strace measurement for a single http request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of a project to own the created strace. It must exist a ingress with the same hostname as in the url to be measured. | [required] |
**pageinsights_schedule_strace_request** | Option<[**PageinsightsScheduleStraceRequest**](PageinsightsScheduleStraceRequest.md)> |  |  |

### Return type

[**models::AppRequestAppinstallation201Response**](app_request_appinstallation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

