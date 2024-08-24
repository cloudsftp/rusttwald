# \MailApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mail_create_deliverybox**](MailApi.md#mail_create_deliverybox) | **POST** /v2/projects/{projectId}/delivery-boxes | Create a DeliveryBox.
[**mail_create_mail_address**](MailApi.md#mail_create_mail_address) | **POST** /v2/projects/{projectId}/mail-addresses | Create a MailAddress.
[**mail_delete_delivery_box**](MailApi.md#mail_delete_delivery_box) | **DELETE** /v2/delivery-boxes/{deliveryBoxId} | Delete a DeliveryBox.
[**mail_delete_mail_address**](MailApi.md#mail_delete_mail_address) | **DELETE** /v2/mail-addresses/{mailAddressId} | Delete a MailAddress.
[**mail_deliverybox_update_description_deprecated**](MailApi.md#mail_deliverybox_update_description_deprecated) | **PUT** /v2/deliveryboxes/{id}/description | Update the description of an deliverybox
[**mail_deliverybox_update_password_deprecated**](MailApi.md#mail_deliverybox_update_password_deprecated) | **PUT** /v2/deliveryboxes/{id}/password | Update the password for a specific deliverybox
[**mail_get_delivery_box**](MailApi.md#mail_get_delivery_box) | **GET** /v2/delivery-boxes/{deliveryBoxId} | Get a DeliveryBox.
[**mail_get_mail_address**](MailApi.md#mail_get_mail_address) | **GET** /v2/mail-addresses/{mailAddressId} | Get a MailAddress.
[**mail_list_delivery_boxes**](MailApi.md#mail_list_delivery_boxes) | **GET** /v2/projects/{projectId}/delivery-boxes | List DeliveryBoxes belonging to a Project.
[**mail_list_mail_addresses**](MailApi.md#mail_list_mail_addresses) | **GET** /v2/projects/{projectId}/mail-addresses | List MailAddresses belonging to a Project.
[**mail_list_project_mail_settings**](MailApi.md#mail_list_project_mail_settings) | **GET** /v2/projects/{projectId}/mail-settings | List mail settings of a Project.
[**mail_mailaddress_update_address_deprecated**](MailApi.md#mail_mailaddress_update_address_deprecated) | **PUT** /v2/mailaddresses/{id}/address | Update mail-address
[**mail_projectsetting_update_blacklist_deprecated**](MailApi.md#mail_projectsetting_update_blacklist_deprecated) | **PUT** /v2/projects/{projectId}/mailsettings/blacklist | Update blacklist for a given project ID
[**mail_projectsetting_update_whitelist_deprecated**](MailApi.md#mail_projectsetting_update_whitelist_deprecated) | **PUT** /v2/projects/{projectId}/mailsettings/whitelist | Update whitelist for a given project ID
[**mail_update_delivery_box_description**](MailApi.md#mail_update_delivery_box_description) | **PATCH** /v2/delivery-boxes/{deliveryBoxId}/description | Update the description of a DeliveryBox.
[**mail_update_delivery_box_password**](MailApi.md#mail_update_delivery_box_password) | **PATCH** /v2/delivery-boxes/{deliveryBoxId}/password | Update the password of a DeliveryBox.
[**mail_update_mail_address_address**](MailApi.md#mail_update_mail_address_address) | **PATCH** /v2/mail-addresses/{mailAddressId}/address | Update a MailAddress.
[**mail_update_mail_address_autoresponder**](MailApi.md#mail_update_mail_address_autoresponder) | **PATCH** /v2/mail-addresses/{mailAddressId}/autoresponder | Update the autoresponder of a MailAddress.
[**mail_update_mail_address_autoresponder_v2_deprecated**](MailApi.md#mail_update_mail_address_autoresponder_v2_deprecated) | **PUT** /v2/mail-addresses/{mailAddressId}/autoresponder | Update the autoresponder of a MailAddress.
[**mail_update_mail_address_catch_all**](MailApi.md#mail_update_mail_address_catch_all) | **PATCH** /v2/mail-addresses/{mailAddressId}/catch-all | Update the catchall of a MailAddress.
[**mail_update_mail_address_catchall_v2_deprecated**](MailApi.md#mail_update_mail_address_catchall_v2_deprecated) | **PUT** /v2/mail-addresses/{mailAddressId}/catchall | Update the catchall of a MailAddress.
[**mail_update_mail_address_forward_addresses**](MailApi.md#mail_update_mail_address_forward_addresses) | **PATCH** /v2/mail-addresses/{mailAddressId}/forward-addresses | Update the forward addresses of a MailAddresses.
[**mail_update_mail_address_forward_addresses_v2_deprecated**](MailApi.md#mail_update_mail_address_forward_addresses_v2_deprecated) | **PUT** /v2/mail-addresses/{mailAddressId}/forward-addresses | Update the forward addresses of a MailAddresses.
[**mail_update_mail_address_password**](MailApi.md#mail_update_mail_address_password) | **PATCH** /v2/mail-addresses/{mailAddressId}/password | Update the password for a MailAddress.
[**mail_update_mail_address_password_v2_deprecated**](MailApi.md#mail_update_mail_address_password_v2_deprecated) | **PUT** /v2/mail-addresses/{mailAddressId}/password | Update the password for a MailAddress.
[**mail_update_mail_address_quota**](MailApi.md#mail_update_mail_address_quota) | **PATCH** /v2/mail-addresses/{mailAddressId}/quota | Update the quota of a MailAddress.
[**mail_update_mail_address_quota_v2_deprecated**](MailApi.md#mail_update_mail_address_quota_v2_deprecated) | **PUT** /v2/mail-addresses/{mailAddressId}/quota | Update the quota of a MailAddress.
[**mail_update_mail_address_spam_protection**](MailApi.md#mail_update_mail_address_spam_protection) | **PATCH** /v2/mail-addresses/{mailAddressId}/spam-protection | Update the spam protection of a MailAddress.
[**mail_update_mail_address_spam_protection_v2_deprecated**](MailApi.md#mail_update_mail_address_spam_protection_v2_deprecated) | **PUT** /v2/mail-addresses/{mailAddressId}/spam-protection | Update the spam protection of a MailAddress.
[**mail_update_project_mail_setting**](MailApi.md#mail_update_project_mail_setting) | **PATCH** /v2/projects/{projectId}/mail-settings/{mailSetting} | Update a mail setting of a Project.
[**mail_update_project_mail_setting_v2_deprecated**](MailApi.md#mail_update_project_mail_setting_v2_deprecated) | **PUT** /v2/projects/{projectId}/mail-settings/{setting} | Update a mail setting of a Project.
[**v2_deliveryboxes_delivery_box_id_delete**](MailApi.md#v2_deliveryboxes_delivery_box_id_delete) | **DELETE** /v2/deliveryboxes/{deliveryBoxId} | Delete a DeliveryBox.
[**v2_deliveryboxes_delivery_box_id_get**](MailApi.md#v2_deliveryboxes_delivery_box_id_get) | **GET** /v2/deliveryboxes/{deliveryBoxId} | Get a DeliveryBox.
[**v2_mailaddresses_mail_address_id_auto_responder_put**](MailApi.md#v2_mailaddresses_mail_address_id_auto_responder_put) | **PUT** /v2/mailaddresses/{mailAddressId}/autoResponder | Update the autoresponder of a MailAddress.
[**v2_mailaddresses_mail_address_id_catch_all_put**](MailApi.md#v2_mailaddresses_mail_address_id_catch_all_put) | **PUT** /v2/mailaddresses/{mailAddressId}/catchAll | Update the catchall of a MailAddress.
[**v2_mailaddresses_mail_address_id_delete**](MailApi.md#v2_mailaddresses_mail_address_id_delete) | **DELETE** /v2/mailaddresses/{mailAddressId} | Delete a MailAddress.
[**v2_mailaddresses_mail_address_id_forwardaddresses_put**](MailApi.md#v2_mailaddresses_mail_address_id_forwardaddresses_put) | **PUT** /v2/mailaddresses/{mailAddressId}/forwardaddresses | Update the forward addresses of a MailAddresses.
[**v2_mailaddresses_mail_address_id_get**](MailApi.md#v2_mailaddresses_mail_address_id_get) | **GET** /v2/mailaddresses/{mailAddressId} | Get a MailAddress.
[**v2_mailaddresses_mail_address_id_password_put**](MailApi.md#v2_mailaddresses_mail_address_id_password_put) | **PUT** /v2/mailaddresses/{mailAddressId}/password | Update the password for a MailAddress.
[**v2_mailaddresses_mail_address_id_quota_put**](MailApi.md#v2_mailaddresses_mail_address_id_quota_put) | **PUT** /v2/mailaddresses/{mailAddressId}/quota | Update the quota of a MailAddress.
[**v2_mailaddresses_mail_address_id_spamprotection_put**](MailApi.md#v2_mailaddresses_mail_address_id_spamprotection_put) | **PUT** /v2/mailaddresses/{mailAddressId}/spamprotection | Update the spam protection of a MailAddress.
[**v2_projects_project_id_deliveryboxes_get**](MailApi.md#v2_projects_project_id_deliveryboxes_get) | **GET** /v2/projects/{projectId}/deliveryboxes | List DeliveryBoxes belonging to a Project.
[**v2_projects_project_id_deliveryboxes_post**](MailApi.md#v2_projects_project_id_deliveryboxes_post) | **POST** /v2/projects/{projectId}/deliveryboxes | Create a DeliveryBox.
[**v2_projects_project_id_mailaddresses_get**](MailApi.md#v2_projects_project_id_mailaddresses_get) | **GET** /v2/projects/{projectId}/mailaddresses | List MailAddresses belonging to a Project.
[**v2_projects_project_id_mailaddresses_post**](MailApi.md#v2_projects_project_id_mailaddresses_post) | **POST** /v2/projects/{projectId}/mailaddresses | Create a MailAddress.
[**v2_projects_project_id_mailsettings_get**](MailApi.md#v2_projects_project_id_mailsettings_get) | **GET** /v2/projects/{projectId}/mailsettings | List mail settings of a Project.



## mail_create_deliverybox

> models::AppRequestAppinstallation201Response mail_create_deliverybox(project_id, v2_projects_project_id_deliveryboxes_post_request)
Create a DeliveryBox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to create the DeliveryBox in. | [required] |
**v2_projects_project_id_deliveryboxes_post_request** | Option<[**V2ProjectsProjectIdDeliveryboxesPostRequest**](V2ProjectsProjectIdDeliveryboxesPostRequest.md)> | DeliveryBox |  |

### Return type

[**models::AppRequestAppinstallation201Response**](app_request_appinstallation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_create_mail_address

> models::AppRequestAppinstallation201Response mail_create_mail_address(project_id, v2_projects_project_id_mailaddresses_post_request)
Create a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the project to create a MailAddress for. | [required] |
**v2_projects_project_id_mailaddresses_post_request** | Option<[**V2ProjectsProjectIdMailaddressesPostRequest**](V2ProjectsProjectIdMailaddressesPostRequest.md)> |  |  |

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


## mail_deliverybox_update_description_deprecated

> mail_deliverybox_update_description_deprecated(id, database_update_mysql_database_description_request)
Update the description of an deliverybox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the deliverybox | [required] |
**database_update_mysql_database_description_request** | Option<[**DatabaseUpdateMysqlDatabaseDescriptionRequest**](DatabaseUpdateMysqlDatabaseDescriptionRequest.md)> | Description of the deliverybox |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_deliverybox_update_password_deprecated

> mail_deliverybox_update_password_deprecated(id, database_update_mysql_user_password_request)
Update the password for a specific deliverybox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the deliverybox | [required] |
**database_update_mysql_user_password_request** | Option<[**DatabaseUpdateMysqlUserPasswordRequest**](DatabaseUpdateMysqlUserPasswordRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
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


## mail_mailaddress_update_address_deprecated

> mail_mailaddress_update_address_deprecated(id, mail_mailaddress_update_address_deprecated_request)
Update mail-address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the mail address | [required] |
**mail_mailaddress_update_address_deprecated_request** | Option<[**MailMailaddressUpdateAddressDeprecatedRequest**](MailMailaddressUpdateAddressDeprecatedRequest.md)> | set mail-address |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_projectsetting_update_blacklist_deprecated

> mail_projectsetting_update_blacklist_deprecated(project_id, mail_projectsetting_update_blacklist_deprecated_request)
Update blacklist for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the project you want to update blacklist for | [required] |
**mail_projectsetting_update_blacklist_deprecated_request** | Option<[**MailProjectsettingUpdateBlacklistDeprecatedRequest**](MailProjectsettingUpdateBlacklistDeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_projectsetting_update_whitelist_deprecated

> mail_projectsetting_update_whitelist_deprecated(project_id, mail_projectsetting_update_whitelist_deprecated_request)
Update whitelist for a given project ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the project you want to update whitelist for | [required] |
**mail_projectsetting_update_whitelist_deprecated_request** | Option<[**MailProjectsettingUpdateWhitelistDeprecatedRequest**](MailProjectsettingUpdateWhitelistDeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
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

> mail_update_mail_address_address(mail_address_id, mail_mailaddress_update_address_deprecated_request)
Update a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the address for. | [required] |
**mail_mailaddress_update_address_deprecated_request** | Option<[**MailMailaddressUpdateAddressDeprecatedRequest**](MailMailaddressUpdateAddressDeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_mail_address_autoresponder

> mail_update_mail_address_autoresponder(mail_address_id, mail_update_mail_address_autoresponder_v2_deprecated_request)
Update the autoresponder of a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the autoresponder for. | [required] |
**mail_update_mail_address_autoresponder_v2_deprecated_request** | Option<[**MailUpdateMailAddressAutoresponderV2DeprecatedRequest**](MailUpdateMailAddressAutoresponderV2DeprecatedRequest.md)> | Autoresponder for the MailAddress. |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_mail_address_autoresponder_v2_deprecated

> mail_update_mail_address_autoresponder_v2_deprecated(mail_address_id, mail_update_mail_address_autoresponder_v2_deprecated_request)
Update the autoresponder of a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the autoresponder for. | [required] |
**mail_update_mail_address_autoresponder_v2_deprecated_request** | Option<[**MailUpdateMailAddressAutoresponderV2DeprecatedRequest**](MailUpdateMailAddressAutoresponderV2DeprecatedRequest.md)> | Autoresponder for the MailAddress. |  |

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


## mail_update_mail_address_catchall_v2_deprecated

> mail_update_mail_address_catchall_v2_deprecated(mail_address_id, mail_update_mail_address_catch_all_request)
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

> mail_update_mail_address_forward_addresses(mail_address_id, mail_update_mail_address_forward_addresses_v2_deprecated_request)
Update the forward addresses of a MailAddresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the forward addresses for. | [required] |
**mail_update_mail_address_forward_addresses_v2_deprecated_request** | Option<[**MailUpdateMailAddressForwardAddressesV2DeprecatedRequest**](MailUpdateMailAddressForwardAddressesV2DeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_mail_address_forward_addresses_v2_deprecated

> mail_update_mail_address_forward_addresses_v2_deprecated(mail_address_id, mail_update_mail_address_forward_addresses_v2_deprecated_request)
Update the forward addresses of a MailAddresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the forward addresses for. | [required] |
**mail_update_mail_address_forward_addresses_v2_deprecated_request** | Option<[**MailUpdateMailAddressForwardAddressesV2DeprecatedRequest**](MailUpdateMailAddressForwardAddressesV2DeprecatedRequest.md)> |  |  |

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


## mail_update_mail_address_password_v2_deprecated

> mail_update_mail_address_password_v2_deprecated(mail_address_id, database_update_mysql_user_password_request)
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

> mail_update_mail_address_quota(mail_address_id, mail_update_mail_address_quota_v2_deprecated_request)
Update the quota of a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the quota for. | [required] |
**mail_update_mail_address_quota_v2_deprecated_request** | Option<[**MailUpdateMailAddressQuotaV2DeprecatedRequest**](MailUpdateMailAddressQuotaV2DeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_mail_address_quota_v2_deprecated

> mail_update_mail_address_quota_v2_deprecated(mail_address_id, mail_update_mail_address_quota_v2_deprecated_request)
Update the quota of a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the quota for. | [required] |
**mail_update_mail_address_quota_v2_deprecated_request** | Option<[**MailUpdateMailAddressQuotaV2DeprecatedRequest**](MailUpdateMailAddressQuotaV2DeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_mail_address_spam_protection

> mail_update_mail_address_spam_protection(mail_address_id, mail_update_mail_address_spam_protection_v2_deprecated_request)
Update the spam protection of a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the spam protection for. | [required] |
**mail_update_mail_address_spam_protection_v2_deprecated_request** | Option<[**MailUpdateMailAddressSpamProtectionV2DeprecatedRequest**](MailUpdateMailAddressSpamProtectionV2DeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mail_update_mail_address_spam_protection_v2_deprecated

> mail_update_mail_address_spam_protection_v2_deprecated(mail_address_id, mail_update_mail_address_spam_protection_v2_deprecated_request)
Update the spam protection of a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the spam protection for. | [required] |
**mail_update_mail_address_spam_protection_v2_deprecated_request** | Option<[**MailUpdateMailAddressSpamProtectionV2DeprecatedRequest**](MailUpdateMailAddressSpamProtectionV2DeprecatedRequest.md)> |  |  |

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


## mail_update_project_mail_setting_v2_deprecated

> mail_update_project_mail_setting_v2_deprecated(project_id, setting, mail_update_project_mail_setting_request)
Update a mail setting of a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to update a mail setting for. | [required] |
**setting** | **String** | The setting to update. | [required] |
**mail_update_project_mail_setting_request** | Option<[**MailUpdateProjectMailSettingRequest**](MailUpdateProjectMailSettingRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_deliveryboxes_delivery_box_id_delete

> v2_deliveryboxes_delivery_box_id_delete(delivery_box_id)
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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_deliveryboxes_delivery_box_id_get

> v2_deliveryboxes_delivery_box_id_get(delivery_box_id)
Get a DeliveryBox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_box_id** | **uuid::Uuid** | ID of the DeliveryBox to be retrieved. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_mailaddresses_mail_address_id_auto_responder_put

> v2_mailaddresses_mail_address_id_auto_responder_put(mail_address_id, mail_update_mail_address_autoresponder_v2_deprecated_request)
Update the autoresponder of a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the autoresponder for. | [required] |
**mail_update_mail_address_autoresponder_v2_deprecated_request** | Option<[**MailUpdateMailAddressAutoresponderV2DeprecatedRequest**](MailUpdateMailAddressAutoresponderV2DeprecatedRequest.md)> | Autoresponder for the MailAddress. |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_mailaddresses_mail_address_id_catch_all_put

> v2_mailaddresses_mail_address_id_catch_all_put(mail_address_id, mail_update_mail_address_catch_all_request)
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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_mailaddresses_mail_address_id_delete

> v2_mailaddresses_mail_address_id_delete(mail_address_id)
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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_mailaddresses_mail_address_id_forwardaddresses_put

> v2_mailaddresses_mail_address_id_forwardaddresses_put(mail_address_id, mail_update_mail_address_forward_addresses_v2_deprecated_request)
Update the forward addresses of a MailAddresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the forward addresses for. | [required] |
**mail_update_mail_address_forward_addresses_v2_deprecated_request** | Option<[**MailUpdateMailAddressForwardAddressesV2DeprecatedRequest**](MailUpdateMailAddressForwardAddressesV2DeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_mailaddresses_mail_address_id_get

> v2_mailaddresses_mail_address_id_get(mail_address_id)
Get a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to be retrieved. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_mailaddresses_mail_address_id_password_put

> v2_mailaddresses_mail_address_id_password_put(mail_address_id, database_update_mysql_user_password_request)
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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_mailaddresses_mail_address_id_quota_put

> v2_mailaddresses_mail_address_id_quota_put(mail_address_id, mail_update_mail_address_quota_v2_deprecated_request)
Update the quota of a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the quota for. | [required] |
**mail_update_mail_address_quota_v2_deprecated_request** | Option<[**MailUpdateMailAddressQuotaV2DeprecatedRequest**](MailUpdateMailAddressQuotaV2DeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_mailaddresses_mail_address_id_spamprotection_put

> v2_mailaddresses_mail_address_id_spamprotection_put(mail_address_id, mail_update_mail_address_spam_protection_v2_deprecated_request)
Update the spam protection of a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mail_address_id** | **uuid::Uuid** | ID of the MailAddress to update the spam protection for. | [required] |
**mail_update_mail_address_spam_protection_v2_deprecated_request** | Option<[**MailUpdateMailAddressSpamProtectionV2DeprecatedRequest**](MailUpdateMailAddressSpamProtectionV2DeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_projects_project_id_deliveryboxes_get

> v2_projects_project_id_deliveryboxes_get(project_id)
List DeliveryBoxes belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to list DeliveryBoxes for. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_projects_project_id_deliveryboxes_post

> v2_projects_project_id_deliveryboxes_post(project_id, v2_projects_project_id_deliveryboxes_post_request)
Create a DeliveryBox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to create the DeliveryBox in. | [required] |
**v2_projects_project_id_deliveryboxes_post_request** | Option<[**V2ProjectsProjectIdDeliveryboxesPostRequest**](V2ProjectsProjectIdDeliveryboxesPostRequest.md)> | DeliveryBox |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_projects_project_id_mailaddresses_get

> v2_projects_project_id_mailaddresses_get(project_id)
List MailAddresses belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to list MailAddresses for. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_projects_project_id_mailaddresses_post

> v2_projects_project_id_mailaddresses_post(project_id, v2_projects_project_id_mailaddresses_post_request)
Create a MailAddress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the project to create a MailAddress for. | [required] |
**v2_projects_project_id_mailaddresses_post_request** | Option<[**V2ProjectsProjectIdMailaddressesPostRequest**](V2ProjectsProjectIdMailaddressesPostRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_projects_project_id_mailsettings_get

> v2_projects_project_id_mailsettings_get(project_id)
List mail settings of a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | ID of the Project to list mail settings for. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

