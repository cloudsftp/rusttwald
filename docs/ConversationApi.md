# \ConversationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**conversation_create_conversation**](ConversationApi.md#conversation_create_conversation) | **POST** /v2/conversations | Create a conversation.
[**conversation_create_message**](ConversationApi.md#conversation_create_message) | **POST** /v2/conversations/{conversationId}/messages | Send a new message in the conversation.
[**conversation_get_category**](ConversationApi.md#conversation_get_category) | **GET** /v2/conversation-categories/{categoryId} | Get a specific conversation category.
[**conversation_get_conversation**](ConversationApi.md#conversation_get_conversation) | **GET** /v2/conversations/{conversationId} | Get a support conversation.
[**conversation_get_conversation_members**](ConversationApi.md#conversation_get_conversation_members) | **GET** /v2/conversations/{conversationId}/members | Get members of a support conversation.
[**conversation_get_conversation_preferences_of_customer**](ConversationApi.md#conversation_get_conversation_preferences_of_customer) | **GET** /v2/customers/{customerId}/conversation-preferences | Get preferences for customer conversations.
[**conversation_get_file_access_token**](ConversationApi.md#conversation_get_file_access_token) | **GET** /v2/conversations/{conversationId}/files/{fileId}/access-token | Request an access token for the File belonging to the Conversation.
[**conversation_list_categories**](ConversationApi.md#conversation_list_categories) | **GET** /v2/conversation-categories | Get all conversation categories.
[**conversation_list_conversations**](ConversationApi.md#conversation_list_conversations) | **GET** /v2/conversations | Get all conversation the authenticated user has created or has access to.
[**conversation_list_messages_by_conversation**](ConversationApi.md#conversation_list_messages_by_conversation) | **GET** /v2/conversations/{conversationId}/messages | Get all message of the conversation.
[**conversation_request_file_upload**](ConversationApi.md#conversation_request_file_upload) | **POST** /v2/conversations/{conversationId}/files | Request a file upload token for the conversation.
[**conversation_set_conversation_status**](ConversationApi.md#conversation_set_conversation_status) | **PUT** /v2/conversations/{conversationId}/status | Update the status of a conversation.
[**conversation_update_conversation**](ConversationApi.md#conversation_update_conversation) | **PUT** /v2/conversations/{conversationId} | Update the basic properties of the conversation.
[**conversation_update_message**](ConversationApi.md#conversation_update_message) | **PATCH** /v2/conversations/{conversationId}/messages/{messageId} | Update the content of the message



## conversation_create_conversation

> models::ConversationCreateConversation201Response conversation_create_conversation(conversation_create_conversation_request)
Create a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_create_conversation_request** | [**ConversationCreateConversationRequest**](ConversationCreateConversationRequest.md) |  | [required] |

### Return type

[**models::ConversationCreateConversation201Response**](conversation_create_conversation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversation_create_message

> models::ConversationCreateMessage201Response conversation_create_message(conversation_id, conversation_create_message_request)
Send a new message in the conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **uuid::Uuid** |  | [required] |
**conversation_create_message_request** | [**ConversationCreateMessageRequest**](ConversationCreateMessageRequest.md) |  | [required] |

### Return type

[**models::ConversationCreateMessage201Response**](conversation_create_message_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversation_get_category

> models::DePeriodMittwaldPeriodV1PeriodConversationPeriodCategory conversation_get_category(category_id)
Get a specific conversation category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodConversationPeriodCategory**](de.mittwald.v1.conversation.Category.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversation_get_conversation

> models::DePeriodMittwaldPeriodV1PeriodConversationPeriodConversation conversation_get_conversation(conversation_id)
Get a support conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodConversationPeriodConversation**](de.mittwald.v1.conversation.Conversation.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversation_get_conversation_members

> Vec<models::DeMittwaldV1ConversationConversationMembersInner> conversation_get_conversation_members(conversation_id)
Get members of a support conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::DeMittwaldV1ConversationConversationMembersInner>**](de_mittwald_v1_conversation_ConversationMembers_inner.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversation_get_conversation_preferences_of_customer

> models::DePeriodMittwaldPeriodV1PeriodConversationPeriodConversationPreferences conversation_get_conversation_preferences_of_customer(customer_id)
Get preferences for customer conversations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodConversationPeriodConversationPreferences**](de.mittwald.v1.conversation.ConversationPreferences.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversation_get_file_access_token

> models::ConversationGetFileAccessToken200Response conversation_get_file_access_token(conversation_id, file_id)
Request an access token for the File belonging to the Conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **uuid::Uuid** |  | [required] |
**file_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ConversationGetFileAccessToken200Response**](conversation_get_file_access_token_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversation_list_categories

> Vec<models::DePeriodMittwaldPeriodV1PeriodConversationPeriodCategory> conversation_list_categories()
Get all conversation categories.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodConversationPeriodCategory>**](de.mittwald.v1.conversation.Category.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversation_list_conversations

> Vec<models::DePeriodMittwaldPeriodV1PeriodConversationPeriodConversation> conversation_list_conversations()
Get all conversation the authenticated user has created or has access to.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodConversationPeriodConversation>**](de.mittwald.v1.conversation.Conversation.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversation_list_messages_by_conversation

> Vec<models::ConversationListMessagesByConversation200ResponseInner> conversation_list_messages_by_conversation(conversation_id)
Get all message of the conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::ConversationListMessagesByConversation200ResponseInner>**](conversation_list_messages_by_conversation_200_response_inner.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversation_request_file_upload

> models::ConversationRequestFileUpload201Response conversation_request_file_upload(conversation_id, body)
Request a file upload token for the conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **uuid::Uuid** |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::ConversationRequestFileUpload201Response**](conversation_request_file_upload_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversation_set_conversation_status

> models::ConversationCreateConversation201Response conversation_set_conversation_status(conversation_id, conversation_set_conversation_status_request)
Update the status of a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **uuid::Uuid** |  | [required] |
**conversation_set_conversation_status_request** | [**ConversationSetConversationStatusRequest**](ConversationSetConversationStatusRequest.md) | The object containing the new status of the conversation. The statuses `open` and `answered` are displayed identically to the users as an open (unresolved) conversation. A conversation with the status `closed` is a solved issue. Closed conversations can be reopened by setting the status to `open` or by sending a message in the conversation.  | [required] |

### Return type

[**models::ConversationCreateConversation201Response**](conversation_create_conversation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversation_update_conversation

> models::ConversationCreateConversation201Response conversation_update_conversation(conversation_id, conversation_update_conversation_request)
Update the basic properties of the conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **uuid::Uuid** |  | [required] |
**conversation_update_conversation_request** | [**ConversationUpdateConversationRequest**](ConversationUpdateConversationRequest.md) |  | [required] |

### Return type

[**models::ConversationCreateConversation201Response**](conversation_create_conversation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversation_update_message

> models::ConversationUpdateMessage200Response conversation_update_message(conversation_id, message_id, conversation_update_message_request)
Update the content of the message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **uuid::Uuid** |  | [required] |
**message_id** | **uuid::Uuid** |  | [required] |
**conversation_update_message_request** | [**ConversationUpdateMessageRequest**](ConversationUpdateMessageRequest.md) |  | [required] |

### Return type

[**models::ConversationUpdateMessage200Response**](conversation_update_message_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

