# \RelocationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**relocation_create_legacy_tariff_change**](RelocationApi.md#relocation_create_legacy_tariff_change) | **POST** /v2/legacy-tariff-change | Register a tariff change for a legacy tariff.
[**relocation_create_relocation**](RelocationApi.md#relocation_create_relocation) | **POST** /v2/relocation | Relocate an external Project to mittwald.



## relocation_create_legacy_tariff_change

> models::ConversationUpdateMessage200Response relocation_create_legacy_tariff_change(relocation_create_legacy_tariff_change_request)
Register a tariff change for a legacy tariff.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**relocation_create_legacy_tariff_change_request** | [**RelocationCreateLegacyTariffChangeRequest**](RelocationCreateLegacyTariffChangeRequest.md) |  | [required] |

### Return type

[**models::ConversationUpdateMessage200Response**](conversation_update_message_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## relocation_create_relocation

> relocation_create_relocation(relocation_create_relocation_request)
Relocate an external Project to mittwald.

Give mittwald access to your Provider and let them move your Project to mittwald.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**relocation_create_relocation_request** | [**RelocationCreateRelocationRequest**](RelocationCreateRelocationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

