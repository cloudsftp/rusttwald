# \MailApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mail_create_deliverybox**](MailApi.md#mail_create_deliverybox) | **POST** /v2/projects/{projectId}/delivery-boxes | Create a DeliveryBox.
[**mail_create_mail_address**](MailApi.md#mail_create_mail_address) | **POST** /v2/projects/{projectId}/mail-addresses | Create a MailAddress.
[**mail_delete_delivery_box**](MailApi.md#mail_delete_delivery_box) | **DELETE** /v2/delivery-boxes/{deliveryBoxId} | Delete a DeliveryBox.
[**mail_delete_mail_address**](MailApi.md#mail_delete_mail_address) | **DELETE** /v2/mail-addresses/{mailAddressId} | Delete a MailAddress.
[**mail_get_delivery_box**](MailApi.md#mail_get_delivery_box) | **GET** /v2/delivery-boxes/{deliveryBoxId} | Get a DeliveryBox.
[**mail_get_mail_address**](MailApi.md#mail_get_mail_address) | **GET** /v2/mail-addresses/{mailAddressId} | Get a MailAddress.
[**mail_list_delivery_boxes**](MailApi.md#mail_list_delivery_boxes) | **GET** /v2/projects/{projectId}/delivery-boxes | List DeliveryBoxes belonging to a Project.
[**mail_list_mail_addresses**](MailApi.md#mail_list_mail_addresses) | **GET** /v2/projects/{projectId}/mail-addresses | List MailAddresses belonging to a Project.
[**mail_list_project_mail_settings**](MailApi.md#mail_list_project_mail_settings) | **GET** /v2/projects/{projectId}/mail-settings | List mail settings of a Project.
[**mail_update_delivery_box_description**](MailApi.md#mail_update_delivery_box_description) | **PATCH** /v2/delivery-boxes/{deliveryBoxId}/description | Update the description of a DeliveryBox.
[**mail_update_delivery_box_password**](MailApi.md#mail_update_delivery_box_password) | **PATCH** /v2/delivery-boxes/{deliveryBoxId}/password | Update the password of a DeliveryBox.
[**mail_update_mail_address_address**](MailApi.md#mail_update_mail_address_address) | **PATCH** /v2/mail-addresses/{mailAddressId}/address | Update a MailAddress.
[**mail_update_mail_address_autoresponder**](MailApi.md#mail_update_mail_address_autoresponder) | **PATCH** /v2/mail-addresses/{mailAddressId}/autoresponder | Update the autoresponder of a MailAddress.
[**mail_update_mail_address_catch_all**](MailApi.md#mail_update_mail_address_catch_all) | **PATCH** /v2/mail-addresses/{mailAddressId}/catch-all | Update the catchall of a MailAddress.
[**mail_update_mail_address_forward_addresses**](MailApi.md#mail_update_mail_address_forward_addresses) | **PATCH** /v2/mail-addresses/{mailAddressId}/forward-addresses | Update the forward addresses of a MailAddresses.
[**mail_update_mail_address_password**](MailApi.md#mail_update_mail_address_password) | **PATCH** /v2/mail-addresses/{mailAddressId}/password | Update the password for a MailAddress.
[**mail_update_mail_address_quota**](MailApi.md#mail_update_mail_address_quota) | **PATCH** /v2/mail-addresses/{mailAddressId}/quota | Update the quota of a MailAddress.
[**mail_update_mail_address_spam_protection**](MailApi.md#mail_update_mail_address_spam_protection) | **PATCH** /v2/mail-addresses/{mailAddressId}/spam-protection | Update the spam protection of a MailAddress.
[**mail_update_project_mail_setting**](MailApi.md#mail_update_project_mail_setting) | **PATCH** /v2/projects/{projectId}/mail-settings/{mailSetting} | Update a mail setting of a Project.



## mail_create_deliverybox

> models::AppRequestAppinstallation201Response mail_create_deliverybox(project_id, mail_create_deliverybox_request)
Create a DeliveryBox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to create the DeliveryBox in. | [required] |
**mail_create_deliverybox_request** | Option<[**MailCreateDeliveryboxRequest**](MailCreateDeliveryboxRequest.md)> | DeliveryBox |  |

### Return type

[**models::AppRequestAppinstallation201Response**](app_request_appinstallation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_create_mail_address

> models::AppRequestAppinstallation201Response mail_create_mail_address(project_id, mail_create_mail_address_request)
Create a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the project to create a MailAddress for. | [required] |
**mail_create_mail_address_request** | Option<[**MailCreateMailAddressRequest**](MailCreateMailAddressRequest.md)> |  |  |

### Return type

[**models::AppRequestAppinstallation201Response**](app_request_appinstallation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_delete_delivery_box

> mail_delete_delivery_box(delivery_box_id)
Delete a DeliveryBox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_box_id** | **uuid::Uuid** | ID of the DeliveryBox to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_delete_mail_address

> mail_delete_mail_address(mail_address_id)
Delete a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_get_delivery_box

> models::DePeriodMittwaldPeriodV1PeriodMailPeriodDeliverybox mail_get_delivery_box(delivery_box_id)
Get a DeliveryBox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_box_id** | **uuid::Uuid** | ID of the DeliveryBox to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMailPeriodDeliverybox**](de.mittwald.v1.mail.Deliverybox.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_get_mail_address

> models::DePeriodMittwaldPeriodV1PeriodMailPeriodMailAddress mail_get_mail_address(mail_address_id)
Get a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMailPeriodMailAddress**](de.mittwald.v1.mail.MailAddress.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_list_delivery_boxes

> Vec<models::DePeriodMittwaldPeriodV1PeriodMailPeriodDeliverybox> mail_list_delivery_boxes(project_id)
List DeliveryBoxes belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to list DeliveryBoxes for. | [required] |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMailPeriodDeliverybox>**](de.mittwald.v1.mail.Deliverybox.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_list_mail_addresses

> Vec<models::DePeriodMittwaldPeriodV1PeriodMailPeriodMailAddress> mail_list_mail_addresses(project_id)
List MailAddresses belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to list MailAddresses for. | [required] |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMailPeriodMailAddress>**](de.mittwald.v1.mail.MailAddress.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_list_project_mail_settings

> models::MailListProjectMailSettings200Response mail_list_project_mail_settings(project_id)
List mail settings of a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to list mail settings for. | [required] |

### Return type

[**models::MailListProjectMailSettings200Response**](mail_list_project_mail_settings_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_delivery_box_description

> mail_update_delivery_box_description(delivery_box_id, database_update_mysql_database_description_request)
Update the description of a DeliveryBox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_box_id** | **uuid::Uuid** | ID of the DeliveryBox to update the description for. | [required] |
**database_update_mysql_database_description_request** | Option<[**DatabaseUpdateMysqlDatabaseDescriptionRequest**](DatabaseUpdateMysqlDatabaseDescriptionRequest.md)> | Description of the DeliveryBox. |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_delivery_box_password

> mail_update_delivery_box_password(delivery_box_id, database_update_mysql_user_password_request)
Update the password of a DeliveryBox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_box_id** | **uuid::Uuid** | ID of the DeliveryBox to update the password for. | [required] |
**database_update_mysql_user_password_request** | Option<[**DatabaseUpdateMysqlUserPasswordRequest**](DatabaseUpdateMysqlUserPasswordRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_mail_address_address

> mail_update_mail_address_address(mail_address_id, mail_update_mail_address_address_request)
Update a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the address for. | [required] |
**mail_update_mail_address_address_request** | Option<[**MailUpdateMailAddressAddressRequest**](MailUpdateMailAddressAddressRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_mail_address_autoresponder

> mail_update_mail_address_autoresponder(mail_address_id, mail_update_mail_address_autoresponder_request)
Update the autoresponder of a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the autoresponder for. | [required] |
**mail_update_mail_address_autoresponder_request** | Option<[**MailUpdateMailAddressAutoresponderRequest**](MailUpdateMailAddressAutoresponderRequest.md)> | Autoresponder for the MailAddress. |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_mail_address_catch_all

> mail_update_mail_address_catch_all(mail_address_id, mail_update_mail_address_catch_all_request)
Update the catchall of a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the catchall for. | [required] |
**mail_update_mail_address_catch_all_request** | Option<[**MailUpdateMailAddressCatchAllRequest**](MailUpdateMailAddressCatchAllRequest.md)> | Catchall of the MailAddress. |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_mail_address_forward_addresses

> mail_update_mail_address_forward_addresses(mail_address_id, mail_update_mail_address_forward_addresses_request)
Update the forward addresses of a MailAddresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the forward addresses for. | [required] |
**mail_update_mail_address_forward_addresses_request** | Option<[**MailUpdateMailAddressForwardAddressesRequest**](MailUpdateMailAddressForwardAddressesRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_mail_address_password

> mail_update_mail_address_password(mail_address_id, database_update_mysql_user_password_request)
Update the password for a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the password for. | [required] |
**database_update_mysql_user_password_request** | Option<[**DatabaseUpdateMysqlUserPasswordRequest**](DatabaseUpdateMysqlUserPasswordRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_mail_address_quota

> mail_update_mail_address_quota(mail_address_id, mail_update_mail_address_quota_request)
Update the quota of a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the quota for. | [required] |
**mail_update_mail_address_quota_request** | Option<[**MailUpdateMailAddressQuotaRequest**](MailUpdateMailAddressQuotaRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_mail_address_spam_protection

> mail_update_mail_address_spam_protection(mail_address_id, mail_update_mail_address_spam_protection_request)
Update the spam protection of a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the spam protection for. | [required] |
**mail_update_mail_address_spam_protection_request** | Option<[**MailUpdateMailAddressSpamProtectionRequest**](MailUpdateMailAddressSpamProtectionRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_project_mail_setting

> mail_update_project_mail_setting(project_id, mail_setting, mail_update_project_mail_setting_request)
Update a mail setting of a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to update a mail setting for. | [required] |
**mail_setting** | **String** | The mail setting to update. | [required] |
**mail_update_project_mail_setting_request** | Option<[**MailUpdateProjectMailSettingRequest**](MailUpdateProjectMailSettingRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

