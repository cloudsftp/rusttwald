# AppReplaceDatabaseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**database_user_ids** | Option<**std::collections::HashMap<String, String>**> | The IDs of the users of the database you want the old database to be replaced with. | [optional]
**new_database_id** | [**uuid::Uuid**](uuid::Uuid.md) | The ID of the database you want the old database to be replaced with. | 
**old_database_id** | [**uuid::Uuid**](uuid::Uuid.md) | The ID of the database you want to be replaced. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


