# \UserApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**password_validation_get_password_policy**](UserApi.md#password_validation_get_password_policy) | **GET** /v2/password-policies/{passwordPolicy} | Get a PasswordPolicy.
[**user_add_phone_number**](UserApi.md#user_add_phone_number) | **POST** /v2/users/{userId}/phone | Add phone number and start verification process.
[**user_authenticate**](UserApi.md#user_authenticate) | **POST** /v2/authenticate | Authenticate yourself to get an access token.
[**user_authenticate_mfa**](UserApi.md#user_authenticate_mfa) | **POST** /v2/authenticate-mfa | Validate your second factor.
[**user_authenticate_with_access_token_retrieval_key**](UserApi.md#user_authenticate_with_access_token_retrieval_key) | **POST** /v2/authenticate-token-retrieval-key | Authenticate an user with an access token retrieval key.
[**user_change_email**](UserApi.md#user_change_email) | **PUT** /v2/users/self/credentials/email | Change your Email-Address.
[**user_change_password**](UserApi.md#user_change_password) | **PUT** /v2/users/self/credentials/password | Change your password.
[**user_check_token**](UserApi.md#user_check_token) | **POST** /v2/users/self/credentials/token | Check token for validity.
[**user_confirm_mfa**](UserApi.md#user_confirm_mfa) | **POST** /v2/users/self/credentials/mfa | Confirm Multi Factor Authentication.
[**user_confirm_password_reset**](UserApi.md#user_confirm_password_reset) | **POST** /v2/users/self/credentials/password/confirm-reset | Confirm password reset.
[**user_create_api_token**](UserApi.md#user_create_api_token) | **POST** /v2/users/self/api-tokens | Store a new ApiToken.
[**user_create_feedback**](UserApi.md#user_create_feedback) | **POST** /v2/users/self/feedback | Submit your user feedback.
[**user_create_ssh_key**](UserApi.md#user_create_ssh_key) | **POST** /v2/users/self/ssh-keys | Store a new ssh-key.
[**user_delete_api_token**](UserApi.md#user_delete_api_token) | **DELETE** /v2/users/self/api-tokens/{apiTokenId} | Deletes an ApiToken.
[**user_delete_ssh_key**](UserApi.md#user_delete_ssh_key) | **DELETE** /v2/users/self/ssh-keys/{sshKeyId} | Remove a ssh-key.
[**user_delete_user**](UserApi.md#user_delete_user) | **DELETE** /v2/users/self | Delete your account and all your personal data.
[**user_disable_mfa**](UserApi.md#user_disable_mfa) | **DELETE** /v2/users/self/credentials/mfa | Disable Multi Factor Authentication.
[**user_edit_api_token**](UserApi.md#user_edit_api_token) | **PUT** /v2/users/self/api-tokens/{apiTokenId} | Update an existing `ApiToken`.
[**user_edit_ssh_key**](UserApi.md#user_edit_ssh_key) | **PUT** /v2/users/self/ssh-keys/{sshKeyId} | Edit a stored ssh-key.
[**user_get_api_token**](UserApi.md#user_get_api_token) | **GET** /v2/users/self/api-tokens/{apiTokenId} | Get a specific ApiToken.
[**user_get_mfa_status**](UserApi.md#user_get_mfa_status) | **GET** /v2/users/self/credentials/mfa | Get your current multi factor auth status.
[**user_get_own_account**](UserApi.md#user_get_own_account) | **GET** /v2/users/self/personal-information | Get your account information.
[**user_get_own_email**](UserApi.md#user_get_own_email) | **GET** /v2/users/self/credentials/email | Get your verified Email-Address.
[**user_get_password_updated_at**](UserApi.md#user_get_password_updated_at) | **GET** /v2/users/self/credentials/password-updated-at | The timestamp of your latest password change.
[**user_get_personalized_settings**](UserApi.md#user_get_personalized_settings) | **GET** /v2/users/{userId}/settings | Get personalized settings.
[**user_get_poll_status**](UserApi.md#user_get_poll_status) | **GET** /v2/poll-settings/{userId} | Get poll settings for the specified user.
[**user_get_session**](UserApi.md#user_get_session) | **GET** /v2/users/self/sessions/{tokenId} | Get a specific session.
[**user_get_ssh_key**](UserApi.md#user_get_ssh_key) | **GET** /v2/users/self/ssh-keys/{sshKeyId} | Get a specific stored ssh-key.
[**user_get_user**](UserApi.md#user_get_user) | **GET** /v2/users/{userId} | Get profile information for a user.
[**user_init_mfa**](UserApi.md#user_init_mfa) | **POST** /v2/users/self/credentials/init-mfa | Initialize Multi Factor Authentication. If successful, it needs to be confirmed, before usage of mfa.
[**user_init_password_reset**](UserApi.md#user_init_password_reset) | **POST** /v2/users/self/credentials/actions/init-password-reset | Initialize password reset process.
[**user_list_api_tokens**](UserApi.md#user_list_api_tokens) | **GET** /v2/users/self/api-tokens | List all of your ApiTokens.
[**user_list_feedback**](UserApi.md#user_list_feedback) | **GET** /v2/users/{userId}/feedback | Submitted feedback of the given user.
[**user_list_sessions**](UserApi.md#user_list_sessions) | **GET** /v2/users/self/sessions | List all sessions.
[**user_list_ssh_keys**](UserApi.md#user_list_ssh_keys) | **GET** /v2/users/self/ssh-keys | Get your stored ssh-keys.
[**user_logout**](UserApi.md#user_logout) | **PUT** /v2/logout | Terminate session and invalidate access token.
[**user_oauth_get_authorization**](UserApi.md#user_oauth_get_authorization) | **GET** /v2/oauth2/authorize | Obtain authorization from the resource owner.
[**user_oauth_retrieve_access_token**](UserApi.md#user_oauth_retrieve_access_token) | **POST** /v2/oauth2/token | Retrieve Access Token from Authorization Code.
[**user_post_poll_status**](UserApi.md#user_post_poll_status) | **POST** /v2/poll-settings/{userId} | Store new or update poll settings.
[**user_refresh_session**](UserApi.md#user_refresh_session) | **PUT** /v2/users/self/sessions | Refresh a session.
[**user_register**](UserApi.md#user_register) | **POST** /v2/register | Register with email and password.
[**user_remove_avatar**](UserApi.md#user_remove_avatar) | **DELETE** /v2/users/{userId}/avatar | Remove Avatar.
[**user_remove_phone_number**](UserApi.md#user_remove_phone_number) | **DELETE** /v2/users/{userId}/phone | Remove phone number.
[**user_request_avatar_upload**](UserApi.md#user_request_avatar_upload) | **POST** /v2/users/{userId}/avatar | Request a new avatar image upload.
[**user_resend_verification_email**](UserApi.md#user_resend_verification_email) | **POST** /v2/users/self/credentials/email/actions/resend-email | Resend the Email-Address verification email.
[**user_reset_recoverycodes**](UserApi.md#user_reset_recoverycodes) | **PUT** /v2/users/self/credentials/mfa | Reset RecoveryCodes for MFA.
[**user_support_code_request**](UserApi.md#user_support_code_request) | **GET** /v2/users/self/credentials/support-code | Request a support code.
[**user_terminate_all_sessions**](UserApi.md#user_terminate_all_sessions) | **DELETE** /v2/users/self/sessions | Terminate all sessions, except the current session.
[**user_terminate_session**](UserApi.md#user_terminate_session) | **DELETE** /v2/users/self/sessions/{tokenId} | Terminate a specific Session.
[**user_update_account**](UserApi.md#user_update_account) | **PUT** /v2/users/self/personal-information | Update your account information.
[**user_update_personal_information**](UserApi.md#user_update_personal_information) | **PUT** /v2/users/{userId} | Change personal information.
[**user_update_personalized_settings**](UserApi.md#user_update_personalized_settings) | **PUT** /v2/users/{userId}/settings | Update personalized GUI settings.
[**user_verify_email**](UserApi.md#user_verify_email) | **POST** /v2/users/self/credentials/email/actions/verify-email | Verify an added Email-Address.
[**user_verify_phone_number**](UserApi.md#user_verify_phone_number) | **POST** /v2/users/{userId}/actions/verify-phone | Verify phone number.
[**user_verify_registration**](UserApi.md#user_verify_registration) | **POST** /v2/verify-registration | Verify your registration.



## password_validation_get_password_policy

> String password_validation_get_password_policy(password_policy)
Get a PasswordPolicy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_policy** | **String** | The name of the PasswordPolicy to be retrieved. | [required] |

### Return type

**String**

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_add_phone_number

> user_add_phone_number(user_id, user_add_phone_number_request)
Add phone number and start verification process.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | `self` or the id of a user. | [required] |
**user_add_phone_number_request** | [**UserAddPhoneNumberRequest**](UserAddPhoneNumberRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_authenticate

> models::UserAuthenticateMfa200Response user_authenticate(user_authenticate_request)
Authenticate yourself to get an access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_authenticate_request** | [**UserAuthenticateRequest**](UserAuthenticateRequest.md) |  | [required] |

### Return type

[**models::UserAuthenticateMfa200Response**](user_authenticate_mfa_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_authenticate_mfa

> models::UserAuthenticateMfa200Response user_authenticate_mfa(user_authenticate_mfa_request)
Validate your second factor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_authenticate_mfa_request** | [**UserAuthenticateMfaRequest**](UserAuthenticateMfaRequest.md) |  | [required] |

### Return type

[**models::UserAuthenticateMfa200Response**](user_authenticate_mfa_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_authenticate_with_access_token_retrieval_key

> models::UserAuthenticateWithAccessTokenRetrievalKey200Response user_authenticate_with_access_token_retrieval_key(user_authenticate_with_access_token_retrieval_key_request)
Authenticate an user with an access token retrieval key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_authenticate_with_access_token_retrieval_key_request** | [**UserAuthenticateWithAccessTokenRetrievalKeyRequest**](UserAuthenticateWithAccessTokenRetrievalKeyRequest.md) |  | [required] |

### Return type

[**models::UserAuthenticateWithAccessTokenRetrievalKey200Response**](user_authenticate_with_access_token_retrieval_key_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_change_email

> user_change_email(user_change_email_request)
Change your Email-Address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_change_email_request** | [**UserChangeEmailRequest**](UserChangeEmailRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_change_password

> models::UserChangePassword200Response user_change_password(user_change_password_request)
Change your password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_change_password_request** | [**UserChangePasswordRequest**](UserChangePasswordRequest.md) |  | [required] |

### Return type

[**models::UserChangePassword200Response**](user_change_password_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_check_token

> models::UserCheckToken200Response user_check_token(body)
Check token for validity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::UserCheckToken200Response**](user_check_token_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_confirm_mfa

> models::UserResetRecoverycodes200Response user_confirm_mfa(user_confirm_mfa_request)
Confirm Multi Factor Authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_confirm_mfa_request** | [**UserConfirmMfaRequest**](UserConfirmMfaRequest.md) |  | [required] |

### Return type

[**models::UserResetRecoverycodes200Response**](user_reset_recoverycodes_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_confirm_password_reset

> user_confirm_password_reset(user_confirm_password_reset_request)
Confirm password reset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_confirm_password_reset_request** | [**UserConfirmPasswordResetRequest**](UserConfirmPasswordResetRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_create_api_token

> models::UserCreateApiToken201Response user_create_api_token(user_create_api_token_request)
Store a new ApiToken.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_create_api_token_request** | [**UserCreateApiTokenRequest**](UserCreateApiTokenRequest.md) |  | [required] |

### Return type

[**models::UserCreateApiToken201Response**](user_create_api_token_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_create_feedback

> serde_json::Value user_create_feedback(user_create_feedback_request)
Submit your user feedback.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_create_feedback_request** | Option<[**UserCreateFeedbackRequest**](UserCreateFeedbackRequest.md)> | The feedback to give. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_create_ssh_key

> serde_json::Value user_create_ssh_key(user_create_ssh_key_request)
Store a new ssh-key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_create_ssh_key_request** | [**UserCreateSshKeyRequest**](UserCreateSshKeyRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_delete_api_token

> user_delete_api_token(api_token_id)
Deletes an ApiToken.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_token_id** | **uuid::Uuid** | The uuid of an ApiToken. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_delete_ssh_key

> user_delete_ssh_key(ssh_key_id)
Remove a ssh-key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_key_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_delete_user

> user_delete_user(user_delete_user_request)
Delete your account and all your personal data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_delete_user_request** | [**UserDeleteUserRequest**](UserDeleteUserRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_disable_mfa

> user_disable_mfa(user_disable_mfa_request)
Disable Multi Factor Authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_disable_mfa_request** | [**UserDisableMfaRequest**](UserDisableMfaRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_edit_api_token

> user_edit_api_token(api_token_id, user_edit_api_token_request)
Update an existing `ApiToken`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_token_id** | **uuid::Uuid** |  | [required] |
**user_edit_api_token_request** | [**UserEditApiTokenRequest**](UserEditApiTokenRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_edit_ssh_key

> user_edit_ssh_key(ssh_key_id, user_edit_ssh_key_request)
Edit a stored ssh-key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_key_id** | **uuid::Uuid** |  | [required] |
**user_edit_ssh_key_request** | [**UserEditSshKeyRequest**](UserEditSshKeyRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_api_token

> models::DePeriodMittwaldPeriodV1PeriodSignupPeriodApiToken user_get_api_token(api_token_id)
Get a specific ApiToken.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_token_id** | **uuid::Uuid** | The id of an ApiToken. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodSignupPeriodApiToken**](de.mittwald.v1.signup.ApiToken.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_mfa_status

> models::UserGetMfaStatus200Response user_get_mfa_status()
Get your current multi factor auth status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserGetMfaStatus200Response**](user_get_mfa_status_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_own_account

> models::DePeriodMittwaldPeriodV1PeriodSignupPeriodAccount user_get_own_account(body)
Get your account information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodSignupPeriodAccount**](de.mittwald.v1.signup.Account.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_own_email

> models::UserGetOwnEmail200Response user_get_own_email()
Get your verified Email-Address.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserGetOwnEmail200Response**](user_get_own_email_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_password_updated_at

> models::UserGetPasswordUpdatedAt200Response user_get_password_updated_at(body)
The timestamp of your latest password change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::UserGetPasswordUpdatedAt200Response**](user_get_password_updated_at_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_personalized_settings

> models::UserGetPersonalizedSettings200Response user_get_personalized_settings(user_id)
Get personalized settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | `self` or the id of a user. | [required] |

### Return type

[**models::UserGetPersonalizedSettings200Response**](user_get_personalized_settings_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_poll_status

> models::DePeriodMittwaldPeriodV1PeriodPollPeriodUserPollSettings user_get_poll_status(user_id)
Get poll settings for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodPollPeriodUserPollSettings**](de.mittwald.v1.poll.UserPollSettings.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_session

> models::DePeriodMittwaldPeriodV1PeriodSignupPeriodUserSession user_get_session(token_id)
Get a specific session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | TokenId to identify a specific session. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodSignupPeriodUserSession**](de.mittwald.v1.signup.UserSession.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_ssh_key

> models::UserGetSshKey200Response user_get_ssh_key(ssh_key_id)
Get a specific stored ssh-key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_key_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::UserGetSshKey200Response**](user_get_ssh_key_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_user

> models::DePeriodMittwaldPeriodV1PeriodUserPeriodUser user_get_user(user_id)
Get profile information for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | `self` or the id of a user. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodUserPeriodUser**](de.mittwald.v1.user.User.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_init_mfa

> models::UserInitMfa200Response user_init_mfa(body)
Initialize Multi Factor Authentication. If successful, it needs to be confirmed, before usage of mfa.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::UserInitMfa200Response**](user_init_mfa_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_init_password_reset

> serde_json::Value user_init_password_reset(user_init_password_reset_request)
Initialize password reset process.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_init_password_reset_request** | [**UserInitPasswordResetRequest**](UserInitPasswordResetRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list_api_tokens

> Vec<models::DePeriodMittwaldPeriodV1PeriodSignupPeriodApiToken> user_list_api_tokens()
List all of your ApiTokens.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodSignupPeriodApiToken>**](de.mittwald.v1.signup.ApiToken.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list_feedback

> Vec<models::DePeriodMittwaldPeriodV1PeriodUserPeriodUserFeedback> user_list_feedback(user_id, subject)
Submitted feedback of the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | `self` or the id of a user. | [required] |
**subject** | Option<**String**> | Filter for subject content. |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodUserPeriodUserFeedback>**](de.mittwald.v1.user.UserFeedback.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list_sessions

> Vec<models::DePeriodMittwaldPeriodV1PeriodSignupPeriodUserSession> user_list_sessions()
List all sessions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodSignupPeriodUserSession>**](de.mittwald.v1.signup.UserSession.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list_ssh_keys

> models::UserListSshKeys200Response user_list_ssh_keys()
Get your stored ssh-keys.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserListSshKeys200Response**](user_list_ssh_keys_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_logout

> user_logout(body)
Terminate session and invalidate access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_oauth_get_authorization

> models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError user_oauth_get_authorization(response_type, client_id, x_access_token, grant_consent, grant_type, redirect_uri, scope, state, code_challenge, code_challenge_method)
Obtain authorization from the resource owner.

The OAuth 2.0 client requests authorization from the resource owner according to [RFC6749](https://datatracker.ietf.org/doc/html/rfc6749). The authorization request is made indirectly via the authorization server as an intermediary. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_type** | **String** | The response type. Must be set to `code` for this endpoint.  | [required] |
**client_id** | **String** | The client identifier as described in [RFC6749](https://datatracker.ietf.org/doc/html/rfc6749#section-2.2).  | [required] |
**x_access_token** | Option<**String**> | Access Token of the Resource Owner.  |  |
**grant_consent** | Option<**bool**> | Can be set in combination with `Authorization`-Header to grant consent for the requested scopes. If set to `true`, the scopes will be marked as consented and the authorization server will redirect the user-agent back to the client. If set to `false`, the authorization server will redirect the user-agent back to the client with an access denied error. If not set and `Authorization`-Header is set, the user will be asked for consent.  |  |
**grant_type** | Option<**String**> | The grant type. Must be set to `authorization_code` for this endpoint.  |  |
**redirect_uri** | Option<**String**> | The redirection URI to which the authorization server directs the user-agent back after the authorization endpoint. Must be a valid URI.  |  |
**scope** | Option<**String**> | The scope of the access request as described by [RFC6749](https://datatracker.ietf.org/doc/html/rfc6749#section-3.3).  |  |
**state** | Option<**String**> | An opaque value used by the client to maintain state between the request and callback. The authorization server includes this value when redirecting the user-agent back to the client. The parameter SHOULD be used for preventing cross-site request forgery as described in [RFC6749](https://datatracker.ietf.org/doc/html/rfc6749#section-10.12).  |  |
**code_challenge** | Option<**String**> | The code challenge as described by [RFC7636](https://datatracker.ietf.org/doc/html/rfc7636#section-4.2). If the authorization code flow should be used with Proof Key for Code Exchange (PKCE), this parameter   must be set and the code verifier has to be given to the token endpoint.  |  |
**code_challenge_method** | Option<**String**> | The code challenge method as described in [RFC7636](https://datatracker.ietf.org/doc/html/rfc7636#section-4.3). If used, the code_challenge must be set as well and the code verifier has to be given to the token endpoint  |  |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError**](de.mittwald.v1.commons.Error.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_oauth_retrieve_access_token

> models::UserOauthRetrieveAccessToken200Response user_oauth_retrieve_access_token(code, grant_type, redirect_uri, authorization, code_verifier)
Retrieve Access Token from Authorization Code.

The OAuth 2.0 client retrieves an Access Token from an existing authorization code according to [RFC6749](https://datatracker.ietf.org/doc/html/rfc6749). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The authorization code received from the authorization server.  | [required] |
**grant_type** | **String** | The grant type as described in [RFC6749](https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.3).  | [required] |
**redirect_uri** | **String** | The redirection URI used in the authorization request. Must be a valid URI.  | [required] |
**authorization** | Option<**String**> | The client credentials (`client_id` and `client_secret`), separated with a colon and base64 encoded as described in [RFC6749](https://datatracker.ietf.org/doc/html/rfc6749#section-2.3.1). Submitting the `client_id` and `client_secret` as request-body as suggested is forbidden.  |  |
**code_verifier** | Option<**String**> | The code verifier used to generate the code challenge as described in [RFC7636](https://datatracker.ietf.org/doc/html/rfc7636#section-4.1). If the authorization flow was initiated with a code challenge, this parameter is required.  |  |

### Return type

[**models::UserOauthRetrieveAccessToken200Response**](user_oauth_retrieve_access_token_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_post_poll_status

> models::DePeriodMittwaldPeriodV1PeriodPollPeriodUserPollSettings user_post_poll_status(user_id, user_post_poll_status_request)
Store new or update poll settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**user_post_poll_status_request** | [**UserPostPollStatusRequest**](UserPostPollStatusRequest.md) |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodPollPeriodUserPollSettings**](de.mittwald.v1.poll.UserPollSettings.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_refresh_session

> models::UserAuthenticateWithAccessTokenRetrievalKey200Response user_refresh_session(user_refresh_session_request)
Refresh a session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_refresh_session_request** | [**UserRefreshSessionRequest**](UserRefreshSessionRequest.md) |  | [required] |

### Return type

[**models::UserAuthenticateWithAccessTokenRetrievalKey200Response**](user_authenticate_with_access_token_retrieval_key_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_register

> models::UserRegister201Response user_register(user_register_request)
Register with email and password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_register_request** | [**UserRegisterRequest**](UserRegisterRequest.md) |  | [required] |

### Return type

[**models::UserRegister201Response**](user_register_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_remove_avatar

> user_remove_avatar(user_id)
Remove Avatar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | `self` or the id of a user. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_remove_phone_number

> user_remove_phone_number(user_id)
Remove phone number.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | `self` or the id of a user. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_request_avatar_upload

> models::UserRequestAvatarUpload200Response user_request_avatar_upload(user_id, body)
Request a new avatar image upload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | `self` or the id of a user. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::UserRequestAvatarUpload200Response**](user_request_avatar_upload_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_resend_verification_email

> user_resend_verification_email(user_resend_verification_email_request)
Resend the Email-Address verification email.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_resend_verification_email_request** | [**UserResendVerificationEmailRequest**](UserResendVerificationEmailRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_reset_recoverycodes

> models::UserResetRecoverycodes200Response user_reset_recoverycodes(user_reset_recoverycodes_request)
Reset RecoveryCodes for MFA.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_reset_recoverycodes_request** | [**UserResetRecoverycodesRequest**](UserResetRecoverycodesRequest.md) |  | [required] |

### Return type

[**models::UserResetRecoverycodes200Response**](user_reset_recoverycodes_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_support_code_request

> models::UserSupportCodeRequest200Response user_support_code_request(force_recreate)
Request a support code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_recreate** | Option<**bool**> |  |  |

### Return type

[**models::UserSupportCodeRequest200Response**](user_support_code_request_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_terminate_all_sessions

> user_terminate_all_sessions()
Terminate all sessions, except the current session.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_terminate_session

> user_terminate_session(token_id)
Terminate a specific Session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | The sessionId is the id of the token used to create the session. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_update_account

> user_update_account(user_update_account_request)
Update your account information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_update_account_request** | [**UserUpdateAccountRequest**](UserUpdateAccountRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_update_personal_information

> user_update_personal_information(user_id, user_update_account_request)
Change personal information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | `self` or the id of a user. | [required] |
**user_update_account_request** | [**UserUpdateAccountRequest**](UserUpdateAccountRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_update_personalized_settings

> user_update_personalized_settings(user_id, user_update_personalized_settings_request)
Update personalized GUI settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | `self` or the id of a user. | [required] |
**user_update_personalized_settings_request** | [**UserUpdatePersonalizedSettingsRequest**](UserUpdatePersonalizedSettingsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_verify_email

> user_verify_email(user_verify_email_request)
Verify an added Email-Address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_verify_email_request** | [**UserVerifyEmailRequest**](UserVerifyEmailRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_verify_phone_number

> user_verify_phone_number(user_id, user_verify_phone_number_request)
Verify phone number.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | `self` or the id of a user. | [required] |
**user_verify_phone_number_request** | [**UserVerifyPhoneNumberRequest**](UserVerifyPhoneNumberRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_verify_registration

> serde_json::Value user_verify_registration(user_verify_registration_request)
Verify your registration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_verify_registration_request** | [**UserVerifyRegistrationRequest**](UserVerifyRegistrationRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

