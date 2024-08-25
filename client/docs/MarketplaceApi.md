# \MarketplaceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**contributor_rotate_secret_for_extension_instance**](MarketplaceApi.md#contributor_rotate_secret_for_extension_instance) | **PUT** /v2/contributors/{contributorId}/extensions/{extensionId}/extension-instances/{extensionInstanceId}/secret | Rotate the secret for an extension instance.
[**extension_authenticate_instance**](MarketplaceApi.md#extension_authenticate_instance) | **POST** /v2/extension-instances/{extensionInstanceId}/tokens | Authenticate your external application using the extensionInstanceSecret.
[**extension_consent_to_extension_scopes**](MarketplaceApi.md#extension_consent_to_extension_scopes) | **PATCH** /v2/extension-instances/{extensionInstanceId}/scopes | Consent to extension scopes.
[**extension_create_extension_instance**](MarketplaceApi.md#extension_create_extension_instance) | **POST** /v2/extension-instances | Create an ExtensionInstance.
[**extension_create_retrieval_key**](MarketplaceApi.md#extension_create_retrieval_key) | **POST** /v2/extension-instances/{extensionInstanceId}/actions/create-access-token-retrieval-key | Create an access token retrieval key for an extension instance.
[**extension_delete_extension_instance**](MarketplaceApi.md#extension_delete_extension_instance) | **DELETE** /v2/extension-instances/{extensionInstanceId} | Delete an ExtensionInstance.
[**extension_disable_extension_instance**](MarketplaceApi.md#extension_disable_extension_instance) | **POST** /v2/extension-instances/{extensionInstanceId}/actions/disable | Disable an ExtensionInstance.
[**extension_dry_run_webhook**](MarketplaceApi.md#extension_dry_run_webhook) | **POST** /v2/contributors/{contributorId}/extensions/{extensionId}/extension-instances/{extensionInstanceId}/actions/dry-run/{webhookKind} | Dry run a webhook with random or given values.
[**extension_enable_extension_instance**](MarketplaceApi.md#extension_enable_extension_instance) | **POST** /v2/extension-instances/{extensionInstanceId}/actions/enable | Enable an ExtensionInstance.
[**extension_get_contributor**](MarketplaceApi.md#extension_get_contributor) | **GET** /v2/contributors/{contributorId} | Get a Contributor.
[**extension_get_extension**](MarketplaceApi.md#extension_get_extension) | **GET** /v2/extensions/{extensionId} | Get an Extension.
[**extension_get_extension_instance**](MarketplaceApi.md#extension_get_extension_instance) | **GET** /v2/extension-instances/{extensionInstanceId} | Get an ExtensionInstance.
[**extension_get_public_key**](MarketplaceApi.md#extension_get_public_key) | **GET** /v2/webhook-public-keys/{serial} | Get the public key to verify the webhook signature.
[**extension_list_contributors**](MarketplaceApi.md#extension_list_contributors) | **GET** /v2/contributors | List Contributors.
[**extension_list_extension_instances**](MarketplaceApi.md#extension_list_extension_instances) | **GET** /v2/extension-instances | List ExtensionInstances.
[**extension_list_extensions**](MarketplaceApi.md#extension_list_extensions) | **GET** /v2/extensions | List Extensions.



## contributor_rotate_secret_for_extension_instance

> models::ContributorRotateSecretForExtensionInstance200Response contributor_rotate_secret_for_extension_instance(contributor_id, extension_id, extension_instance_id, contributor_rotate_secret_for_extension_instance_request)
Rotate the secret for an extension instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contributor_id** | **String** |  | [required] |
**extension_id** | **uuid::Uuid** |  | [required] |
**extension_instance_id** | **uuid::Uuid** |  | [required] |
**contributor_rotate_secret_for_extension_instance_request** | [**ContributorRotateSecretForExtensionInstanceRequest**](ContributorRotateSecretForExtensionInstanceRequest.md) |  | [required] |

### Return type

[**models::ContributorRotateSecretForExtensionInstance200Response**](contributor_rotate_secret_for_extension_instance_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_authenticate_instance

> models::ExtensionAuthenticateInstance201Response extension_authenticate_instance(extension_instance_id, extension_authenticate_instance_request)
Authenticate your external application using the extensionInstanceSecret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_instance_id** | **uuid::Uuid** |  | [required] |
**extension_authenticate_instance_request** | [**ExtensionAuthenticateInstanceRequest**](ExtensionAuthenticateInstanceRequest.md) |  | [required] |

### Return type

[**models::ExtensionAuthenticateInstance201Response**](extension_authenticate_instance_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_consent_to_extension_scopes

> serde_json::Value extension_consent_to_extension_scopes(extension_instance_id, extension_consent_to_extension_scopes_request)
Consent to extension scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_instance_id** | **uuid::Uuid** |  | [required] |
**extension_consent_to_extension_scopes_request** | [**ExtensionConsentToExtensionScopesRequest**](ExtensionConsentToExtensionScopesRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_create_extension_instance

> models::ExtensionCreateExtensionInstance201Response extension_create_extension_instance(extension_create_extension_instance_request)
Create an ExtensionInstance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_create_extension_instance_request** | [**ExtensionCreateExtensionInstanceRequest**](ExtensionCreateExtensionInstanceRequest.md) |  | [required] |

### Return type

[**models::ExtensionCreateExtensionInstance201Response**](extension_create_extension_instance_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_create_retrieval_key

> models::ExtensionCreateRetrievalKey200Response extension_create_retrieval_key(extension_instance_id)
Create an access token retrieval key for an extension instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_instance_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ExtensionCreateRetrievalKey200Response**](extension_create_retrieval_key_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_delete_extension_instance

> serde_json::Value extension_delete_extension_instance(extension_instance_id)
Delete an ExtensionInstance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_instance_id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_disable_extension_instance

> serde_json::Value extension_disable_extension_instance(extension_instance_id)
Disable an ExtensionInstance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_instance_id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_dry_run_webhook

> models::ExtensionDryRunWebhook200Response extension_dry_run_webhook(contributor_id, extension_id, extension_instance_id, webhook_kind, context_id, scopes, instance_disabled, created_at, secret)
Dry run a webhook with random or given values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contributor_id** | **String** |  | [required] |
**extension_id** | **uuid::Uuid** |  | [required] |
**extension_instance_id** | **uuid::Uuid** | The ID of an extension instance. For this route it can be made up. | [required] |
**webhook_kind** | [**DePeriodMittwaldPeriodV1PeriodMarketplacePeriodWebhookKind**](.md) |  | [required] |
**context_id** | Option<**String**> | Define the contextId. If left empty, the contextId will be random. |  |
**scopes** | Option<[**Vec<String>**](String.md)> | Define the scopes. If left empty, the scopes will be those defined in the extension. |  |
**instance_disabled** | Option<**bool**> | Define the `disabled` state of the extension instance. If left empty, the it will be random. |  |
**created_at** | Option<**String**> | Define the creation timestamp of the extension instance. If left empty, the it will be random. |  |
**secret** | Option<**String**> | Define the secret of the extension instance. If left empty, the it will be random. This secret will not be able to be used. |  |

### Return type

[**models::ExtensionDryRunWebhook200Response**](extension_dry_run_webhook_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_enable_extension_instance

> serde_json::Value extension_enable_extension_instance(extension_instance_id)
Enable an ExtensionInstance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_instance_id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_get_contributor

> models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodContributor extension_get_contributor(contributor_id)
Get a Contributor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contributor_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodContributor**](de.mittwald.v1.marketplace.Contributor.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_get_extension

> models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodExtension extension_get_extension(extension_id)
Get an Extension.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodExtension**](de.mittwald.v1.marketplace.Extension.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_get_extension_instance

> models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodExtensionInstance extension_get_extension_instance(extension_instance_id)
Get an ExtensionInstance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_instance_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodExtensionInstance**](de.mittwald.v1.marketplace.ExtensionInstance.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_get_public_key

> models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodPublicKey extension_get_public_key(serial)
Get the public key to verify the webhook signature.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**serial** | **String** | The serial id of a specific public key. Use `latest` to get the latest public key. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodPublicKey**](de.mittwald.v1.marketplace.PublicKey.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_list_contributors

> Vec<models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodContributor> extension_list_contributors(limit, skip, page)
List Contributors.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 1000]
**skip** | Option<**i32**> |  |  |[default to 0]
**page** | Option<**i32**> |  |  |[default to 1]

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodContributor>**](de.mittwald.v1.marketplace.Contributor.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_list_extension_instances

> Vec<models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodExtensionInstance> extension_list_extension_instances(context, context_id, limit, skip, page)
List ExtensionInstances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | [**DePeriodMittwaldPeriodV1PeriodMarketplacePeriodContext**](.md) |  | [required] |
**context_id** | **uuid::Uuid** | The ID of your customer, project or user. | [required] |
**limit** | Option<**i32**> |  |  |[default to 1000]
**skip** | Option<**i32**> |  |  |[default to 0]
**page** | Option<**i32**> |  |  |[default to 1]

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodExtensionInstance>**](de.mittwald.v1.marketplace.ExtensionInstance.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extension_list_extensions

> Vec<models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodExtension> extension_list_extensions(context, limit, skip, page)
List Extensions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | Option<[**DePeriodMittwaldPeriodV1PeriodMarketplacePeriodContext**](.md)> |  |  |
**limit** | Option<**i32**> |  |  |[default to 1000]
**skip** | Option<**i32**> |  |  |[default to 0]
**page** | Option<**i32**> |  |  |[default to 1]

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMarketplacePeriodExtension>**](de.mittwald.v1.marketplace.Extension.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

