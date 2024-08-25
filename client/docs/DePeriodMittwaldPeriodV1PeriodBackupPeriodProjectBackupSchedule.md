# DePeriodMittwaldPeriodV1PeriodBackupPeriodProjectBackupSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional]
**description** | Option<**String**> | Description of this ProjectBackupSchedule. | [optional]
**id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of this ProjectBackupSchedule. | 
**is_system_backup** | **bool** | True if this ProjectBackupSchedule was created and is managed by mittwald. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the Project this ProjectBackupSchedule belongs to. | 
**schedule** | **String** | Execution schedule in crontab notation. | 
**ttl** | Option<**String**> | TTL of the ProjectBackupSchedule as time string. | [optional]
**updated_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


