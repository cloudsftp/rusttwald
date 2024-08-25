# \DatabaseApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**database_create_mysql_database**](DatabaseApi.md#database_create_mysql_database) | **POST** /v2/projects/{projectId}/mysql-databases | Create a MySQLDatabase with a MySQLUser.
[**database_create_mysql_user**](DatabaseApi.md#database_create_mysql_user) | **POST** /v2/mysql-databases/{mysqlDatabaseId}/users | Create a MySQLUser.
[**database_create_redis_database**](DatabaseApi.md#database_create_redis_database) | **POST** /v2/projects/{projectId}/redis-databases | Create a RedisDatabase.
[**database_delete_mysql_database**](DatabaseApi.md#database_delete_mysql_database) | **DELETE** /v2/mysql-databases/{mysqlDatabaseId} | Delete a MySQLDatabase.
[**database_delete_mysql_user**](DatabaseApi.md#database_delete_mysql_user) | **DELETE** /v2/mysql-users/{mysqlUserId} | Delete a MySQLUser.
[**database_delete_redis_database**](DatabaseApi.md#database_delete_redis_database) | **DELETE** /v2/redis-databases/{redisDatabaseId} | Delete a RedisDatabase.
[**database_disable_mysql_user**](DatabaseApi.md#database_disable_mysql_user) | **POST** /v2/mysql-users/{mysqlUserId}/actions/disable | Disable a MySQLUser.
[**database_enable_mysql_user**](DatabaseApi.md#database_enable_mysql_user) | **POST** /v2/mysql-users/{mysqlUserId}/actions/enable | Enable a MySQLUser.
[**database_get_mysql_database**](DatabaseApi.md#database_get_mysql_database) | **GET** /v2/mysql-databases/{mysqlDatabaseId} | Get a MySQLDatabase.
[**database_get_mysql_user**](DatabaseApi.md#database_get_mysql_user) | **GET** /v2/mysql-users/{mysqlUserId} | Get a MySQLUser.
[**database_get_mysql_user_php_my_admin_url**](DatabaseApi.md#database_get_mysql_user_php_my_admin_url) | **GET** /v2/mysql-users/{mysqlUserId}/php-my-admin-url | Get a MySQLUser's PhpMyAdmin-URL.
[**database_get_redis_database**](DatabaseApi.md#database_get_redis_database) | **GET** /v2/redis-databases/{redisDatabaseId} | Get a RedisDatabase.
[**database_list_mysql_charsets**](DatabaseApi.md#database_list_mysql_charsets) | **GET** /v2/mysql-charsets | List available MySQL character sets and collations, optionally filtered by a MySQLVersion.
[**database_list_mysql_databases**](DatabaseApi.md#database_list_mysql_databases) | **GET** /v2/projects/{projectId}/mysql-databases | List MySQLDatabases belonging to a Project.
[**database_list_mysql_users**](DatabaseApi.md#database_list_mysql_users) | **GET** /v2/mysql-databases/{mysqlDatabaseId}/users | List MySQLUsers belonging to a Database.
[**database_list_mysql_versions**](DatabaseApi.md#database_list_mysql_versions) | **GET** /v2/mysql-versions | List MySQLVersions.
[**database_list_redis_databases**](DatabaseApi.md#database_list_redis_databases) | **GET** /v2/projects/{projectId}/redis-databases | List RedisDatabases belonging to a Project.
[**database_list_redis_versions**](DatabaseApi.md#database_list_redis_versions) | **GET** /v2/redis-versions | List RedisVersions.
[**database_update_mysql_database_default_charset**](DatabaseApi.md#database_update_mysql_database_default_charset) | **PATCH** /v2/mysql-databases/{mysqlDatabaseId}/default-charset | Update a MySQLDatabase's default character settings.
[**database_update_mysql_database_description**](DatabaseApi.md#database_update_mysql_database_description) | **PATCH** /v2/mysql-databases/{mysqlDatabaseId}/description | Update a MySQLDatabase's description.
[**database_update_mysql_user**](DatabaseApi.md#database_update_mysql_user) | **PUT** /v2/mysql-users/{mysqlUserId} | Update a MySQLUser.
[**database_update_mysql_user_password**](DatabaseApi.md#database_update_mysql_user_password) | **PATCH** /v2/mysql-users/{mysqlUserId}/password | Update a MySQLUser's password.
[**database_update_redis_database_configuration**](DatabaseApi.md#database_update_redis_database_configuration) | **PATCH** /v2/redis-databases/{redisDatabaseId}/configuration | Update a RedisDatabase's configuration.
[**database_update_redis_database_description**](DatabaseApi.md#database_update_redis_database_description) | **PATCH** /v2/redis-databases/{redisDatabaseId}/description | Update a RedisDatabase's description.



## database_create_mysql_database

> models::DatabaseCreateMysqlDatabase201Response database_create_mysql_database(project_id, database_create_mysql_database_request)
Create a MySQLDatabase with a MySQLUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to create a MySQLDatabase for. | [required] |
**database_create_mysql_database_request** | Option<[**DatabaseCreateMysqlDatabaseRequest**](DatabaseCreateMysqlDatabaseRequest.md)> |  |  |

### Return type

[**models::DatabaseCreateMysqlDatabase201Response**](database_create_mysql_database_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_create_mysql_user

> models::AppRequestAppinstallation201Response database_create_mysql_user(mysql_database_id, de_period_mittwald_period_v1_period_database_period_create_my_sql_user)
Create a MySQLUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mysql_database_id** | **uuid::Uuid** | ID of the MySQLDatabase to create a MySQLUser for. | [required] |
**de_period_mittwald_period_v1_period_database_period_create_my_sql_user** | Option<[**DePeriodMittwaldPeriodV1PeriodDatabasePeriodCreateMySqlUser**](DePeriodMittwaldPeriodV1PeriodDatabasePeriodCreateMySqlUser.md)> |  |  |

### Return type

[**models::AppRequestAppinstallation201Response**](app_request_appinstallation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_create_redis_database

> models::AppRequestAppinstallation201Response database_create_redis_database(project_id, database_create_redis_database_request)
Create a RedisDatabase.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to create a RedisDatabase for. | [required] |
**database_create_redis_database_request** | Option<[**DatabaseCreateRedisDatabaseRequest**](DatabaseCreateRedisDatabaseRequest.md)> |  |  |

### Return type

[**models::AppRequestAppinstallation201Response**](app_request_appinstallation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_delete_mysql_database

> database_delete_mysql_database(mysql_database_id)
Delete a MySQLDatabase.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mysql_database_id** | **uuid::Uuid** | ID of the MySQLDatabase to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_delete_mysql_user

> database_delete_mysql_user(mysql_user_id)
Delete a MySQLUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mysql_user_id** | **uuid::Uuid** | ID of the MySQLUser to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_delete_redis_database

> database_delete_redis_database(redis_database_id)
Delete a RedisDatabase.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**redis_database_id** | **uuid::Uuid** | ID of the RedisDatabase to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_disable_mysql_user

> database_disable_mysql_user(mysql_user_id, body)
Disable a MySQLUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mysql_user_id** | **uuid::Uuid** | ID of the MySQLUser to disable. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_enable_mysql_user

> database_enable_mysql_user(mysql_user_id, body)
Enable a MySQLUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mysql_user_id** | **uuid::Uuid** | ID of the MySQLUser to enable. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_get_mysql_database

> models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlDatabase database_get_mysql_database(mysql_database_id)
Get a MySQLDatabase.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mysql_database_id** | **uuid::Uuid** | ID of the MySQLDatabase to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlDatabase**](de.mittwald.v1.database.MySqlDatabase.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_get_mysql_user

> models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlUser database_get_mysql_user(mysql_user_id)
Get a MySQLUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mysql_user_id** | **uuid::Uuid** | ID of the MySQLUser to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlUser**](de.mittwald.v1.database.MySqlUser.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_get_mysql_user_php_my_admin_url

> models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodPhpMyAdminUrl database_get_mysql_user_php_my_admin_url(mysql_user_id)
Get a MySQLUser's PhpMyAdmin-URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mysql_user_id** | **uuid::Uuid** | ID of the MySQLUser for which to get the URL for. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodPhpMyAdminUrl**](de.mittwald.v1.database.PhpMyAdminURL.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_get_redis_database

> models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodRedisDatabase database_get_redis_database(redis_database_id)
Get a RedisDatabase.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**redis_database_id** | **uuid::Uuid** | ID of the RedisDatabase to retrieve. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodRedisDatabase**](de.mittwald.v1.database.RedisDatabase.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_list_mysql_charsets

> Vec<models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlCharacterSettings> database_list_mysql_charsets(version)
List available MySQL character sets and collations, optionally filtered by a MySQLVersion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | Option<**String**> | ID of the MySQLVersion for which to list the available character sets in the format `mysqlXY`. |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlCharacterSettings>**](de.mittwald.v1.database.MySqlCharacterSettings.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_list_mysql_databases

> Vec<models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlDatabase> database_list_mysql_databases(project_id)
List MySQLDatabases belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to list MySQLDatabases for. | [required] |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlDatabase>**](de.mittwald.v1.database.MySqlDatabase.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_list_mysql_users

> Vec<models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlUser> database_list_mysql_users(mysql_database_id)
List MySQLUsers belonging to a Database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mysql_database_id** | **uuid::Uuid** | ID of the MySQLDatabase to list Users for. | [required] |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlUser>**](de.mittwald.v1.database.MySqlUser.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_list_mysql_versions

> Vec<models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlVersion> database_list_mysql_versions(project_id)
List MySQLVersions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | Option<**String**> | ID of the Project to list available MySQLVersions for. |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlVersion>**](de.mittwald.v1.database.MySqlVersion.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_list_redis_databases

> Vec<models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodRedisDatabase> database_list_redis_databases(project_id)
List RedisDatabases belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to list RedisDatabases for. | [required] |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodRedisDatabase>**](de.mittwald.v1.database.RedisDatabase.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_list_redis_versions

> Vec<models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodRedisVersion> database_list_redis_versions(project_id)
List RedisVersions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | Option<**String**> | ID of the Project to list available RedisVersions for. |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodRedisVersion>**](de.mittwald.v1.database.RedisVersion.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_update_mysql_database_default_charset

> database_update_mysql_database_default_charset(mysql_database_id, database_update_mysql_database_default_charset_request)
Update a MySQLDatabase's default character settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mysql_database_id** | **uuid::Uuid** | ID of the MySQLDatabase to update. | [required] |
**database_update_mysql_database_default_charset_request** | Option<[**DatabaseUpdateMysqlDatabaseDefaultCharsetRequest**](DatabaseUpdateMysqlDatabaseDefaultCharsetRequest.md)> | The default character settings to be used for the MySQLDatabase. |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_update_mysql_database_description

> database_update_mysql_database_description(mysql_database_id, database_update_mysql_database_description_request)
Update a MySQLDatabase's description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mysql_database_id** | **uuid::Uuid** | ID of the MySQLDatabase to update. | [required] |
**database_update_mysql_database_description_request** | Option<[**DatabaseUpdateMysqlDatabaseDescriptionRequest**](DatabaseUpdateMysqlDatabaseDescriptionRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_update_mysql_user

> database_update_mysql_user(mysql_user_id, database_update_mysql_user_request)
Update a MySQLUser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mysql_user_id** | **uuid::Uuid** | ID of the MySQLUser to update. | [required] |
**database_update_mysql_user_request** | Option<[**DatabaseUpdateMysqlUserRequest**](DatabaseUpdateMysqlUserRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_update_mysql_user_password

> database_update_mysql_user_password(mysql_user_id, database_update_mysql_user_password_request)
Update a MySQLUser's password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mysql_user_id** | **uuid::Uuid** | ID of the MySQLUser for which to update the password for. | [required] |
**database_update_mysql_user_password_request** | Option<[**DatabaseUpdateMysqlUserPasswordRequest**](DatabaseUpdateMysqlUserPasswordRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_update_redis_database_configuration

> database_update_redis_database_configuration(redis_database_id, database_update_redis_database_configuration_request)
Update a RedisDatabase's configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**redis_database_id** | **uuid::Uuid** | ID of the RedisDatabase to update. | [required] |
**database_update_redis_database_configuration_request** | Option<[**DatabaseUpdateRedisDatabaseConfigurationRequest**](DatabaseUpdateRedisDatabaseConfigurationRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## database_update_redis_database_description

> database_update_redis_database_description(redis_database_id, database_update_mysql_database_description_request)
Update a RedisDatabase's description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**redis_database_id** | **uuid::Uuid** | ID of the RedisDatabase to update. | [required] |
**database_update_mysql_database_description_request** | Option<[**DatabaseUpdateMysqlDatabaseDescriptionRequest**](DatabaseUpdateMysqlDatabaseDescriptionRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

