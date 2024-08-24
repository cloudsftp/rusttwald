# \CustomerApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_accept_customer_invite**](CustomerApi.md#customer_accept_customer_invite) | **POST** /v2/customer-invites/{customerInviteId}/actions/accept | Accept a CustomerInvite.
[**customer_create_customer**](CustomerApi.md#customer_create_customer) | **POST** /v2/customers | Create a new customer profile.
[**customer_create_customer_invite**](CustomerApi.md#customer_create_customer_invite) | **POST** /v2/customer/{customerId}/invites | Create a CustomerInvite.
[**customer_decline_customer_invite**](CustomerApi.md#customer_decline_customer_invite) | **POST** /v2/customer-invites/{customerInviteId}/actions/decline | Decline a CustomerInvite.
[**customer_delete_customer**](CustomerApi.md#customer_delete_customer) | **DELETE** /v2/customers/{customerId} | Delete a customer profile.
[**customer_delete_customer_invite**](CustomerApi.md#customer_delete_customer_invite) | **DELETE** /v2/customer-invites/{customerInviteId} | Delete a CustomerInvite.
[**customer_delete_customer_membership**](CustomerApi.md#customer_delete_customer_membership) | **DELETE** /v2/customer-memberships/{customerMembershipId} | Delete a CustomerMembership.
[**customer_get_customer**](CustomerApi.md#customer_get_customer) | **GET** /v2/customers/{customerId} | Get a customer profile.
[**customer_get_customer_invite**](CustomerApi.md#customer_get_customer_invite) | **GET** /v2/customer-invites/{customerInviteId} | Get a CustomerInvite.
[**customer_get_customer_membership**](CustomerApi.md#customer_get_customer_membership) | **GET** /v2/customer-memberships/{customerMembershipId} | Get a CustomerMembership.
[**customer_get_customer_token_invite**](CustomerApi.md#customer_get_customer_token_invite) | **GET** /v2/customer-token-invite | Get a CustomerInvite by token.
[**customer_is_customer_legally_competent**](CustomerApi.md#customer_is_customer_legally_competent) | **GET** /v2/customers/{customerId}/legally-competent | Check if the customer profile has a valid contract partner configured.
[**customer_leave_customer**](CustomerApi.md#customer_leave_customer) | **POST** /v2/customer/{customerId}/actions/leave | Leave a Customer.
[**customer_list_customer_invites**](CustomerApi.md#customer_list_customer_invites) | **GET** /v2/customer-invites | List CustomerInvites belonging to the executing user.
[**customer_list_customer_memberships**](CustomerApi.md#customer_list_customer_memberships) | **GET** /v2/customer-memberships | List CustomerMemberships belonging to the executing user.
[**customer_list_customers**](CustomerApi.md#customer_list_customers) | **GET** /v2/customers | Get all customer profiles the authenticated user has access to.
[**customer_list_invites_for_customer**](CustomerApi.md#customer_list_invites_for_customer) | **GET** /v2/customers/{customerId}/invites | List Invites belonging to a Customer.
[**customer_list_memberships_for_customer**](CustomerApi.md#customer_list_memberships_for_customer) | **GET** /v2/customers/{customerId}/memberships | List Memberships belonging to a Customer.
[**customer_remove_avatar**](CustomerApi.md#customer_remove_avatar) | **DELETE** /v2/customers/{customerId}/avatar | Remove the avatar picture of the customer profile.
[**customer_request_avatar_upload**](CustomerApi.md#customer_request_avatar_upload) | **POST** /v2/customers/{customerId}/avatar | Request a new avatar upload for the customer profile.
[**customer_resend_customer_invite_mail**](CustomerApi.md#customer_resend_customer_invite_mail) | **POST** /v2/customer-invites/{customerInviteId}/actions/resend | Resend the mail for a CustomerInvite.
[**customer_update_customer**](CustomerApi.md#customer_update_customer) | **PUT** /v2/customers/{customerId} | Update a customer profile.
[**customer_update_customer_membership**](CustomerApi.md#customer_update_customer_membership) | **PATCH** /v2/customer-memberships/{customerMembershipId} | Update a CustomerMembership.



## customer_accept_customer_invite

> customer_accept_customer_invite(customer_invite_id, customer_accept_customer_invite_request)
Accept a CustomerInvite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_invite_id** | **uuid::Uuid** | ID of the CustomerInvite to be accepted. | [required] |
**customer_accept_customer_invite_request** | [**CustomerAcceptCustomerInviteRequest**](CustomerAcceptCustomerInviteRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_create_customer

> models::CustomerCreateCustomer201Response customer_create_customer(customer_create_customer_request)
Create a new customer profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_create_customer_request** | [**CustomerCreateCustomerRequest**](CustomerCreateCustomerRequest.md) | The customer to create | [required] |

### Return type

[**models::CustomerCreateCustomer201Response**](customer_create_customer_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_create_customer_invite

> models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite customer_create_customer_invite(customer_id, customer_create_customer_invite_request)
Create a CustomerInvite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **uuid::Uuid** | ID of the Customer to create a CustomerInvite for. | [required] |
**customer_create_customer_invite_request** | [**CustomerCreateCustomerInviteRequest**](CustomerCreateCustomerInviteRequest.md) |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite**](de.mittwald.v1.membership.CustomerInvite.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_decline_customer_invite

> customer_decline_customer_invite(customer_invite_id, body)
Decline a CustomerInvite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_invite_id** | **uuid::Uuid** | ID of the CustomerInvite to be declined. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_delete_customer

> models::CustomerDeleteCustomer200Response customer_delete_customer(customer_id)
Delete a customer profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The uuid of the customer to be deleted. Only customer without active contracts or unpaid invoices can be deleted. | [required] |

### Return type

[**models::CustomerDeleteCustomer200Response**](customer_delete_customer_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_delete_customer_invite

> customer_delete_customer_invite(customer_invite_id)
Delete a CustomerInvite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_invite_id** | **uuid::Uuid** | ID if the CustomerInvite to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_delete_customer_membership

> customer_delete_customer_membership(customer_membership_id)
Delete a CustomerMembership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_membership_id** | **uuid::Uuid** | ID of the CustomerMembership to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_get_customer

> models::DePeriodMittwaldPeriodV1PeriodCustomerPeriodCustomer customer_get_customer(customer_id)
Get a customer profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The uuid of the customer to be returned | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodCustomerPeriodCustomer**](de.mittwald.v1.customer.Customer.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_get_customer_invite

> models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite customer_get_customer_invite(customer_invite_id)
Get a CustomerInvite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_invite_id** | **String** | ID of the CustomerInvite to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite**](de.mittwald.v1.membership.CustomerInvite.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_get_customer_membership

> models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerMembership customer_get_customer_membership(customer_membership_id)
Get a CustomerMembership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_membership_id** | **String** | ID of the CustomerMembership to retrieve. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerMembership**](de.mittwald.v1.membership.CustomerMembership.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_get_customer_token_invite

> models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite customer_get_customer_token_invite(token)
Get a CustomerInvite by token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Token of the CustomerInvite to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite**](de.mittwald.v1.membership.CustomerInvite.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_is_customer_legally_competent

> models::CustomerIsCustomerLegallyCompetent200Response customer_is_customer_legally_competent(customer_id)
Check if the customer profile has a valid contract partner configured.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The uuid of the customer to be checked | [required] |

### Return type

[**models::CustomerIsCustomerLegallyCompetent200Response**](customer_is_customer_legally_competent_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_leave_customer

> customer_leave_customer(customer_id, body)
Leave a Customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **uuid::Uuid** | ID of the Customer to be left. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_list_customer_invites

> Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite> customer_list_customer_invites(limit, skip)
List CustomerInvites belonging to the executing user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite>**](de.mittwald.v1.membership.CustomerInvite.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_list_customer_memberships

> Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerMembership> customer_list_customer_memberships(limit, skip)
List CustomerMemberships belonging to the executing user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerMembership>**](de.mittwald.v1.membership.CustomerMembership.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_list_customers

> Vec<models::DePeriodMittwaldPeriodV1PeriodCustomerPeriodCustomer> customer_list_customers(role, search, limit, skip, page)
Get all customer profiles the authenticated user has access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | Option<**String**> | filter to list only customers where the authenticated user has the specified role. Can be a comma seperated list. |  |
**search** | Option<**String**> | text search for customer number, customer name and id. Simple regular expressions are allowed. |  |
**limit** | Option<**i32**> | The max count of resources to return in the list response. |  |[default to 1000]
**skip** | Option<**i32**> | Number of items to skip. Should be multiple of `limit`. |  |[default to 0]
**page** | Option<**i32**> | Page to display. `skip` and `page` end up doing the same. If both `page` and `skip` are set, skip is used. |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodCustomerPeriodCustomer>**](de.mittwald.v1.customer.Customer.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_list_invites_for_customer

> Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite> customer_list_invites_for_customer(customer_id, limit, skip)
List Invites belonging to a Customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the Customer to list invites for. | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite>**](de.mittwald.v1.membership.CustomerInvite.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_list_memberships_for_customer

> Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerMembership> customer_list_memberships_for_customer(customer_id, limit, skip)
List Memberships belonging to a Customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Customer to list memberships for. | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerMembership>**](de.mittwald.v1.membership.CustomerMembership.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_remove_avatar

> customer_remove_avatar(customer_id)
Remove the avatar picture of the customer profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The uuid of the customer whose avatar is to be deleted | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_request_avatar_upload

> models::CustomerRequestAvatarUpload200Response customer_request_avatar_upload(customer_id, body)
Request a new avatar upload for the customer profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The uuid of the customer for whom an avatar upload is to be requested | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::CustomerRequestAvatarUpload200Response**](customer_request_avatar_upload_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_resend_customer_invite_mail

> customer_resend_customer_invite_mail(customer_invite_id, body)
Resend the mail for a CustomerInvite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_invite_id** | **uuid::Uuid** | ID of the CustomerInvite to resend the mail for. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_update_customer

> models::CustomerCreateCustomer201Response customer_update_customer(customer_id, customer_update_customer_request)
Update a customer profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**customer_update_customer_request** | [**CustomerUpdateCustomerRequest**](CustomerUpdateCustomerRequest.md) | The customer object to update | [required] |

### Return type

[**models::CustomerCreateCustomer201Response**](customer_create_customer_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_update_customer_membership

> customer_update_customer_membership(customer_membership_id, customer_update_customer_membership_request)
Update a CustomerMembership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_membership_id** | **uuid::Uuid** | ID of the CustomerMembership to be updated. | [required] |
**customer_update_customer_membership_request** | [**CustomerUpdateCustomerMembershipRequest**](CustomerUpdateCustomerMembershipRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

