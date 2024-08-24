# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**servicetoken_authenticate_service**](DefaultApi.md#servicetoken_authenticate_service) | **POST** /v2/services/{accessKeyId}/actions/authenticate | Obtain a service token.



## servicetoken_authenticate_service

> models::ServicetokenAuthenticateService200Response servicetoken_authenticate_service(access_key_id, servicetoken_authenticate_service_request)
Obtain a service token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key_id** | **String** |  | [required] |
**servicetoken_authenticate_service_request** | [**ServicetokenAuthenticateServiceRequest**](ServicetokenAuthenticateServiceRequest.md) |  | [required] |

### Return type

[**models::ServicetokenAuthenticateService200Response**](servicetoken_authenticate_service_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

