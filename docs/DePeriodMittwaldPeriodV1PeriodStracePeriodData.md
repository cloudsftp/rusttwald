# DePeriodMittwaldPeriodV1PeriodStracePeriodData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actual_url** | **String** |  | 
**db_queries** | [**Vec<models::DeMittwaldV1StraceDataDbQueriesInner>**](de_mittwald_v1_strace_Data_dbQueries_inner.md) |  | 
**db_stats** | [**models::DePeriodMittwaldPeriodV1PeriodStracePeriodStatistics**](de.mittwald.v1.strace.Statistics.md) |  | 
**file_ops** | [**Vec<models::DeMittwaldV1StraceDataFileOpsInner>**](de_mittwald_v1_strace_Data_fileOps_inner.md) |  | 
**file_ops_stats** | [**models::DePeriodMittwaldPeriodV1PeriodStracePeriodStatistics**](de.mittwald.v1.strace.Statistics.md) |  | 
**misc_stats** | [**models::DePeriodMittwaldPeriodV1PeriodStracePeriodStatistics**](de.mittwald.v1.strace.Statistics.md) |  | 
**networking_ops** | [**Vec<models::DeMittwaldV1StraceDataNetworkingOpsInner>**](de_mittwald_v1_strace_Data_networkingOps_inner.md) |  | 
**networking_stats** | [**models::DePeriodMittwaldPeriodV1PeriodStracePeriodStatistics**](de.mittwald.v1.strace.Statistics.md) |  | 
**slowdown_factor** | **f64** | Shows how much slower the websites TTFB was when it got traced (1.0 = 100% = same TTFB). | 
**ttfb_ms** | **i32** | Time to first byte in milliseconds while tracing the website. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


