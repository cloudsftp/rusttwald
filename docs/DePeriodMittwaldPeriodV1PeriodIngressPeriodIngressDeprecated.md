# DePeriodMittwaldPeriodV1PeriodIngressPeriodIngressDeprecated

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dns_validation_errors** | **Vec<String>** | A list of errors that occurred while validating the ingress's dns before requesting a certificate. | 
**hostname** | **String** |  | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**ips** | [**models::DeMittwaldV1IngressIngressIps**](de_mittwald_v1_ingress_Ingress_ips.md) |  | 
**is_default** | **bool** | Whether this ingress is the default ingress or not. A default ingress is automatically created, it cannot be deleted. There can be only one default ingress per project. | 
**is_domain** | Option<**bool**> |  | [optional]
**is_enabled** | **bool** |  | 
**ownership** | [**models::DePeriodMittwaldPeriodV1PeriodIngressPeriodOwnership**](de.mittwald.v1.ingress.Ownership.md) |  | 
**paths** | [**Vec<models::DePeriodMittwaldPeriodV1PeriodIngressPeriodPath>**](de.mittwald.v1.ingress.Path.md) | A list of paths. The default path `/` is always present and cannot be removed. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tls** | [**models::DeMittwaldV1IngressIngressDeprecatedTls**](de_mittwald_v1_ingress_IngressDeprecated_tls.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


