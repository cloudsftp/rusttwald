# \NotificationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**newsletter_get_info**](NotificationApi.md#newsletter_get_info) | **GET** /v2/newsletter-subscriptions/self | Getting the subscription status of the subscription.
[**newsletter_subscribe_user**](NotificationApi.md#newsletter_subscribe_user) | **POST** /v2/newsletter-subscriptions | Subscribe a user to the mStudio newsletter.
[**newsletter_unsubscribe_user**](NotificationApi.md#newsletter_unsubscribe_user) | **DELETE** /v2/newsletter-subscriptions/self | Unsubscribe a user from the mStudio newsletter.
[**notifications_count_unread_notifications**](NotificationApi.md#notifications_count_unread_notifications) | **GET** /v2/notification-unread-counts | Get the counts for unread notifications of the user.
[**notifications_list_notifications**](NotificationApi.md#notifications_list_notifications) | **GET** /v2/notifications | List all unread notifications.
[**notifications_read_all_notifications**](NotificationApi.md#notifications_read_all_notifications) | **POST** /v2/notifications/actions/read-all | Mark all notifications as read.
[**notifications_read_all_notifications_deprecated**](NotificationApi.md#notifications_read_all_notifications_deprecated) | **PUT** /v2/notifications/status | Mark all notifications as read (deprecated).
[**notifications_read_notification**](NotificationApi.md#notifications_read_notification) | **PUT** /v2/notifications/{notificationId}/status | Mark notification as read.
[**v2_notifications_unread_counts_get**](NotificationApi.md#v2_notifications_unread_counts_get) | **GET** /v2/notifications/unread-counts | Get the counts for unread notifications of the user.



## newsletter_get_info

> models::NewsletterGetInfo200Response newsletter_get_info()
Getting the subscription status of the subscription.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NewsletterGetInfo200Response**](newsletter_get_info_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## newsletter_subscribe_user

> models::NewsletterSubscribeUser200Response newsletter_subscribe_user(newsletter_subscribe_user_request)
Subscribe a user to the mStudio newsletter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**newsletter_subscribe_user_request** | [**NewsletterSubscribeUserRequest**](NewsletterSubscribeUserRequest.md) |  | [required] |

### Return type

[**models::NewsletterSubscribeUser200Response**](newsletter_subscribe_user_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## newsletter_unsubscribe_user

> newsletter_unsubscribe_user()
Unsubscribe a user from the mStudio newsletter.

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


## notifications_count_unread_notifications

> models::NotificationsCountUnreadNotifications200Response notifications_count_unread_notifications()
Get the counts for unread notifications of the user.

The user is determined by the access token used. Lighter weight route instead of getting all unread notifications and count yourself. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NotificationsCountUnreadNotifications200Response**](notifications_count_unread_notifications_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_list_notifications

> Vec<models::DePeriodMittwaldPeriodV1PeriodMessagingPeriodNotification> notifications_list_notifications(status, limit, skip, page)
List all unread notifications.



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> |  |  |
**limit** | Option<**i32**> | The max count of resources to return in the list response. |  |[default to 500]
**skip** | Option<**i32**> | Number of items to skip. Should be multiple of `limit`. |  |[default to 0]
**page** | Option<**i32**> | Page to display. `skip` and `page` end up doing the same. If both `page` and `skip` are set, skip is used. |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodMessagingPeriodNotification>**](de.mittwald.v1.messaging.Notification.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_read_all_notifications

> models::NotificationsReadAllNotificationsDeprecated200Response notifications_read_all_notifications(body)
Mark all notifications as read.

Mark all notifications for the authenticated user as read.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::NotificationsReadAllNotificationsDeprecated200Response**](notifications_read_all_notifications_deprecated_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_read_all_notifications_deprecated

> models::NotificationsReadAllNotificationsDeprecated200Response notifications_read_all_notifications_deprecated(body)
Mark all notifications as read (deprecated).

Deprecated route. Please use /v2/notifications/actions/read-all instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::NotificationsReadAllNotificationsDeprecated200Response**](notifications_read_all_notifications_deprecated_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_read_notification

> models::NotificationsReadAllNotificationsDeprecated200Response notifications_read_notification(notification_id, notifications_read_all_notifications_deprecated200_response)
Mark notification as read.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **uuid::Uuid** |  | [required] |
**notifications_read_all_notifications_deprecated200_response** | [**NotificationsReadAllNotificationsDeprecated200Response**](NotificationsReadAllNotificationsDeprecated200Response.md) |  | [required] |

### Return type

[**models::NotificationsReadAllNotificationsDeprecated200Response**](notifications_read_all_notifications_deprecated_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_notifications_unread_counts_get

> v2_notifications_unread_counts_get()
Get the counts for unread notifications of the user.

The user is determined by the access token used. Lighter weight route instead of getting all unread notifications and count yourself. 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

