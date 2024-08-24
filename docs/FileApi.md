# \FileApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**file_create_file**](FileApi.md#file_create_file) | **POST** /v2/files | Create a File.
[**file_get_file**](FileApi.md#file_get_file) | **GET** /v2/files/{fileId} | Get a File.
[**file_get_file_meta**](FileApi.md#file_get_file_meta) | **GET** /v2/files/{fileId}/meta | Get a File's meta.
[**file_get_file_upload_token_rules**](FileApi.md#file_get_file_upload_token_rules) | **GET** /v2/file-upload-tokens/{fileUploadToken}/rules | Get a FileUploadToken's rules.
[**file_get_file_upload_type_rules**](FileApi.md#file_get_file_upload_type_rules) | **GET** /v2/file-upload-types/{fileUploadType}/rules | Get a FileUploadType's rules.
[**file_get_file_with_name**](FileApi.md#file_get_file_with_name) | **GET** /v2/files/{fileId}/{fileName} | Get a File.



## file_create_file

> models::DePeriodMittwaldPeriodV1PeriodFilePeriodFileMeta file_create_file(token, file_create_file_request)
Create a File.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **uuid::Uuid** |  | [required] |
**file_create_file_request** | Option<[**FileCreateFileRequest**](FileCreateFileRequest.md)> |  |  |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodFilePeriodFileMeta**](de.mittwald.v1.file.FileMeta.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_get_file

> String file_get_file(file_id, accept, content_disposition, token)
Get a File.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **uuid::Uuid** | ID of the File to be retrieved. | [required] |
**accept** | Option<**String**> |  |  |[default to application/octet-stream]
**content_disposition** | Option<**String**> |  |  |[default to inline]
**token** | Option<**String**> | Only needed for protected Files. |  |

### Return type

**String**

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/plain;base64, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_get_file_meta

> models::DePeriodMittwaldPeriodV1PeriodFilePeriodFileMeta file_get_file_meta(file_id)
Get a File's meta.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **uuid::Uuid** | ID of the File to get the meta for. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodFilePeriodFileMeta**](de.mittwald.v1.file.FileMeta.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_get_file_upload_token_rules

> models::DePeriodMittwaldPeriodV1PeriodFilePeriodFileUploadRules file_get_file_upload_token_rules(file_upload_token)
Get a FileUploadToken's rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_upload_token** | **uuid::Uuid** | FileUploadToken to retrieve rules for. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodFilePeriodFileUploadRules**](de.mittwald.v1.file.FileUploadRules.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_get_file_upload_type_rules

> models::DePeriodMittwaldPeriodV1PeriodFilePeriodFileUploadRules file_get_file_upload_type_rules(file_upload_type)
Get a FileUploadType's rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_upload_type** | **String** | FileUploadType to retrieve rules for. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodFilePeriodFileUploadRules**](de.mittwald.v1.file.FileUploadRules.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_get_file_with_name

> String file_get_file_with_name(file_id, file_name, accept, content_disposition, token)
Get a File.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **uuid::Uuid** | ID of the File to be retrieved. | [required] |
**file_name** | **String** | Name of the File to be retrieved. | [required] |
**accept** | Option<**String**> |  |  |[default to application/octet-stream]
**content_disposition** | Option<**String**> |  |  |[default to inline]
**token** | Option<**String**> | Only needed for protected Files. |  |

### Return type

**String**

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/plain;base64, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

