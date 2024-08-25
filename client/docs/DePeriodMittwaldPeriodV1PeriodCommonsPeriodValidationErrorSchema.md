# DePeriodMittwaldPeriodV1PeriodCommonsPeriodValidationErrorSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **String** | The standard error message | 
**path** | **String** | The path to the part of the data that was validated. JavaScript property access notation (e.g., \".prop[1].subProp\") is used.  | 
**r#type** | **String** | ajv validation error type (e.g. \"format\", \"required\", \"type\") or own validation error types | 
**context** | Option<**std::collections::HashMap<String, String>**> | The object with the additional information about the error that can be used to create custom error messages. Keys depend on the type that failed validation (e.g. \"missingProperty\" for type \"required\")  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


