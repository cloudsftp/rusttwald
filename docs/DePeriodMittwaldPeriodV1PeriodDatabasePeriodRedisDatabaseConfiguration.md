# DePeriodMittwaldPeriodV1PeriodDatabasePeriodRedisDatabaseConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_flags** | Option<**Vec<String>**> | Additional flags passed to the database. | [optional]
**max_memory** | Option<**String**> | The database's maximum memory. This may be a number, optionally suffixed by one of the IEC binary suffixes `Ki`, `Mi` or `Gi`. | [optional]
**max_memory_policy** | Option<**String**> | The database's key eviction policy. See the [Redis documentation on key evictions](https://redis.io/docs/reference/eviction/) for more information. | [optional]
**persistent** | Option<**bool**> | Persistent status of the database. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


