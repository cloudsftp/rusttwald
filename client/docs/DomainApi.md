# \DomainApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deprecated_domain_get_screenshot_for_domain**](DomainApi.md#deprecated_domain_get_screenshot_for_domain) | **GET** /v2/domains/{domainId}/screenshots/newest | Get File Service Reference for a Screenshot of a domain.
[**dns_create_dns_zone**](DomainApi.md#dns_create_dns_zone) | **POST** /v2/dns-zones | Create a DNSZone.
[**dns_delete_dns_zone**](DomainApi.md#dns_delete_dns_zone) | **DELETE** /v2/dns-zones/{dnsZoneId} | Delete a DNSZone.
[**dns_get_dns_zone**](DomainApi.md#dns_get_dns_zone) | **GET** /v2/dns-zones/{dnsZoneId} | Get a DNSZone.
[**dns_list_dns_zones**](DomainApi.md#dns_list_dns_zones) | **GET** /v2/projects/{projectId}/dns-zones | List DNSZones belonging to a Project.
[**dns_record_a_set_custom_deprecated**](DomainApi.md#dns_record_a_set_custom_deprecated) | **PUT** /v2/dns/zones/{zoneId}/recordset/acombined/custom | updates a-records for a specific zone
[**dns_record_a_set_managed_by_ingress_deprecated**](DomainApi.md#dns_record_a_set_managed_by_ingress_deprecated) | **POST** /v2/dns/zones/{zoneId}/recordset/acombined/managed/ingress | set a-records managed by ingress for a specific zone
[**dns_record_cname_set_deprecated**](DomainApi.md#dns_record_cname_set_deprecated) | **PUT** /v2/dns/zones/{zoneId}/recordset/cname | updates cname-record for a specific zone
[**dns_record_mx_set_custom_deprecated**](DomainApi.md#dns_record_mx_set_custom_deprecated) | **PUT** /v2/dns/zones/{zoneId}/recordset/mx/custom | updates mx-records for a specific zone
[**dns_record_mx_set_managed_deprecated**](DomainApi.md#dns_record_mx_set_managed_deprecated) | **POST** /v2/dns/zones/{zoneId}/recordset/mx/managed | sets mx-records to managed for a specific zone
[**dns_record_srv_set_deprecated**](DomainApi.md#dns_record_srv_set_deprecated) | **PUT** /v2/dns/zones/{zoneId}/recordset/srv | updates srv-records for a specific zone
[**dns_record_txt_set_deprecated**](DomainApi.md#dns_record_txt_set_deprecated) | **PUT** /v2/dns/zones/{zoneId}/recordset/txt | updates txt-records for a specific zone
[**dns_set_record_set_managed**](DomainApi.md#dns_set_record_set_managed) | **POST** /v2/dns-zones/{dnsZoneId}/record-sets/{recordSet}/actions/set-managed | Set a record set on a DNSZone to managed.
[**dns_update_record_set**](DomainApi.md#dns_update_record_set) | **PUT** /v2/dns-zones/{dnsZoneId}/record-sets/{recordSet} | Update a record set on a DNSZone.
[**domain_abort_domain_declaration**](DomainApi.md#domain_abort_domain_declaration) | **DELETE** /v2/domains/{domainId}/declaration | Abort a Domain declaration.
[**domain_change_ownerc_of_domain_v2_deprecated**](DomainApi.md#domain_change_ownerc_of_domain_v2_deprecated) | **PUT** /v2/domains/{domainId}/handles/ownerc | Change the owner contact of a domain.
[**domain_change_project_of_domain_v2_deprecated**](DomainApi.md#domain_change_project_of_domain_v2_deprecated) | **PUT** /v2/domains/{domainId}/projectId | Change the Project relation of a Domain.
[**domain_check_domain_registrability**](DomainApi.md#domain_check_domain_registrability) | **POST** /v2/domain-registrable | Check if a Domain is available to register.
[**domain_check_domain_registrability_v2_deprecated**](DomainApi.md#domain_check_domain_registrability_v2_deprecated) | **POST** /v2/domains | Check if a Domain is available to register.
[**domain_check_domain_transferability**](DomainApi.md#domain_check_domain_transferability) | **POST** /v2/domain-transferable | Check if a Domain is available to transfer.
[**domain_create_domain_auth_code**](DomainApi.md#domain_create_domain_auth_code) | **POST** /v2/domains/{domainId}/actions/auth-code | Create an auth code for a Domains transfer-out process.
[**domain_declare_nameservers_v2_deprecated**](DomainApi.md#domain_declare_nameservers_v2_deprecated) | **PUT** /v2/domains/{domainId}/nameservers | Change all nameservers of a Domain.
[**domain_declare_process_change_authcode_v2_deprecated**](DomainApi.md#domain_declare_process_change_authcode_v2_deprecated) | **PUT** /v2/domains/{domainId}/declarations/authcode | Update an AuthCode.
[**domain_declare_process_change_handles_v2_deprecated**](DomainApi.md#domain_declare_process_change_handles_v2_deprecated) | **PUT** /v2/domains/{domainId}/declarations/handles | Update a Domain's OwnerC handle.
[**domain_delete_domain**](DomainApi.md#domain_delete_domain) | **DELETE** /v2/domains/{domainId} | Delete a Domain.
[**domain_get_domain**](DomainApi.md#domain_get_domain) | **GET** /v2/domains/{domainId} | Get a Domain.
[**domain_get_handle_fields_v2_deprecated**](DomainApi.md#domain_get_handle_fields_v2_deprecated) | **GET** /v2/domains/handle-schema/{domainName} | Get a HandleSchema.
[**domain_get_latest_screenshot**](DomainApi.md#domain_get_latest_screenshot) | **GET** /v2/domains/{domainId}/latest-screenshot | Get the latest screenshot's FileReference belonging to a Domain.
[**domain_list_domains**](DomainApi.md#domain_list_domains) | **GET** /v2/domains | List Domains
[**domain_list_domains_v2_deprecated**](DomainApi.md#domain_list_domains_v2_deprecated) | **GET** /v2/projects/{projectId}/domains | List Domains belonging to a Project.
[**domain_list_tld_contact_schemas**](DomainApi.md#domain_list_tld_contact_schemas) | **GET** /v2/domain-tlds/{tld}/contact-schemas | List the contact schemas for a TLD.
[**domain_list_tlds**](DomainApi.md#domain_list_tlds) | **GET** /v2/domain-tlds | List TLDs.
[**domain_resend_domain_email**](DomainApi.md#domain_resend_domain_email) | **POST** /v2/domains/{domainId}/actions/resend-email | Resend a Domain email.
[**domain_suggest**](DomainApi.md#domain_suggest) | **GET** /v2/domain-suggestions | Suggest a list of domains based on a prompt using AI.
[**domain_update_domain_auth_code**](DomainApi.md#domain_update_domain_auth_code) | **PATCH** /v2/domains/{domainId}/auth-code | Update the auth code of a Domain.
[**domain_update_domain_contact**](DomainApi.md#domain_update_domain_contact) | **PATCH** /v2/domains/{domainId}/contacts/{contact} | Update a contact of a Domain.
[**domain_update_domain_nameservers**](DomainApi.md#domain_update_domain_nameservers) | **PATCH** /v2/domains/{domainId}/nameservers | Update the nameservers of a Domain.
[**domain_update_domain_project_id**](DomainApi.md#domain_update_domain_project_id) | **PATCH** /v2/domains/{domainId}/project-id | Update a Domain's project id.
[**ingress_create_ingress**](DomainApi.md#ingress_create_ingress) | **POST** /v2/ingresses | Create an Ingress.
[**ingress_delete_ingress**](DomainApi.md#ingress_delete_ingress) | **DELETE** /v2/ingresses/{ingressId} | Delete an Ingress.
[**ingress_get_ingress**](DomainApi.md#ingress_get_ingress) | **GET** /v2/ingresses/{ingressId} | Get an Ingress.
[**ingress_ingress_verify_ownership**](DomainApi.md#ingress_ingress_verify_ownership) | **POST** /v2/ingresses/{ingressId}/actions/verify-ownership | Verifiy the ownership of an Ingress.
[**ingress_list_ingresses**](DomainApi.md#ingress_list_ingresses) | **GET** /v2/ingresses | List Ingresses.
[**ingress_list_ingresses_compatible_with_certificate**](DomainApi.md#ingress_list_ingresses_compatible_with_certificate) | **POST** /v2/actions/list-ingresses-compatible-with-certificate | List Ingresses compatible with a certificate.
[**ingress_list_ingresses_v2_deprecated**](DomainApi.md#ingress_list_ingresses_v2_deprecated) | **GET** /v2/projects/{projectId}/ingresses | List Ingresses belonging to a project.
[**ingress_paths_deprecated**](DomainApi.md#ingress_paths_deprecated) | **PUT** /v2/ingresses/{ingressId}/paths | Update an Ingresses paths.
[**ingress_request_ingress_acme_certificate_issuance**](DomainApi.md#ingress_request_ingress_acme_certificate_issuance) | **POST** /v2/ingresses/{ingressId}/actions/request-acme-certificate-issuance | Request the ACME certificate issuance of an Ingress.
[**ingress_tls_deprecated**](DomainApi.md#ingress_tls_deprecated) | **PUT** /v2/ingresses/{ingressId}/tls | Update an Ingresses tls settings.
[**ingress_update_ingress_paths**](DomainApi.md#ingress_update_ingress_paths) | **PATCH** /v2/ingresses/{ingressId}/paths | Update the paths of an Ingress.
[**ingress_update_ingress_tls**](DomainApi.md#ingress_update_ingress_tls) | **PATCH** /v2/ingresses/{ingressId}/tls | Update the tls settings of an Ingress.
[**ssl_check_replace_certificate**](DomainApi.md#ssl_check_replace_certificate) | **POST** /v2/certificates/{certificateId}/actions/check-replace-certificate | Check the replacement of a Certificate.
[**ssl_create_certificate_request**](DomainApi.md#ssl_create_certificate_request) | **POST** /v2/certificate-requests | Create a CertificateRequest.
[**ssl_delete_certificate**](DomainApi.md#ssl_delete_certificate) | **DELETE** /v2/certificate/{certificateId} | Delete a Certificate.
[**ssl_delete_certificate_request**](DomainApi.md#ssl_delete_certificate_request) | **DELETE** /v2/certificate-request/{certificateRequestId} | Delete a CertificateRequest.
[**ssl_get_certificate**](DomainApi.md#ssl_get_certificate) | **GET** /v2/certificates/{certificateId} | Get a Certificate.
[**ssl_get_certificate_request**](DomainApi.md#ssl_get_certificate_request) | **GET** /v2/certificate-requests/{certificateRequestId} | Get a CertificateRequest.
[**ssl_list_certificate_requests**](DomainApi.md#ssl_list_certificate_requests) | **GET** /v2/certificate-requests | List CertificateRequests belonging to a Project or an Ingress.
[**ssl_list_certificates**](DomainApi.md#ssl_list_certificates) | **GET** /v2/certificates | List Certificates belonging to a Project or an Ingress.
[**ssl_replace_certificate**](DomainApi.md#ssl_replace_certificate) | **PUT** /v2/certificates/{certificateId} | Update a Certificate.
[**v2_dns_zones_dns_zone_id_delete**](DomainApi.md#v2_dns_zones_dns_zone_id_delete) | **DELETE** /v2/dns/zones/{dnsZoneId} | Delete a DNSZone.
[**v2_dns_zones_dns_zone_id_get**](DomainApi.md#v2_dns_zones_dns_zone_id_get) | **GET** /v2/dns/zones/{dnsZoneId} | Get a DNSZone.
[**v2_dns_zones_post**](DomainApi.md#v2_dns_zones_post) | **POST** /v2/dns/zones | Create a DNSZone.
[**v2_domains_domain_id_actions_create_authcode_post**](DomainApi.md#v2_domains_domain_id_actions_create_authcode_post) | **POST** /v2/domains/{domainId}/actions/create-authcode | Create an auth code for a Domains transfer-out process.
[**v2_domains_domain_id_authcode_patch**](DomainApi.md#v2_domains_domain_id_authcode_patch) | **PATCH** /v2/domains/{domainId}/authcode | Update the auth code of a Domain.
[**v2_domains_domain_id_declarations_delete**](DomainApi.md#v2_domains_domain_id_declarations_delete) | **DELETE** /v2/domains/{domainId}/declarations | Abort a Domain declaration.
[**v2_domains_supported_tlds_get**](DomainApi.md#v2_domains_supported_tlds_get) | **GET** /v2/domains/supported-tlds | List TLDs.
[**v2_projects_project_id_dns_zones_get**](DomainApi.md#v2_projects_project_id_dns_zones_get) | **GET** /v2/projects/{projectId}/dns/zones | List DNSZones belonging to a Project.



## deprecated_domain_get_screenshot_for_domain

> models::DeprecatedDomainGetScreenshotForDomain200Response deprecated_domain_get_screenshot_for_domain(domain_id, deprecated_domain_get_screenshot_for_domain_request)
Get File Service Reference for a Screenshot of a domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**deprecated_domain_get_screenshot_for_domain_request** | [**DeprecatedDomainGetScreenshotForDomainRequest**](DeprecatedDomainGetScreenshotForDomainRequest.md) |  | [required] |

### Return type

[**models::DeprecatedDomainGetScreenshotForDomain200Response**](deprecated_domain_get_screenshot_for_domain_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_create_dns_zone

> models::AppRequestAppinstallation201Response dns_create_dns_zone(v2_dns_zones_post_request)
Create a DNSZone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v2_dns_zones_post_request** | Option<[**V2DnsZonesPostRequest**](V2DnsZonesPostRequest.md)> |  |  |

### Return type

[**models::AppRequestAppinstallation201Response**](app_request_appinstallation_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_delete_dns_zone

> dns_delete_dns_zone(dns_zone_id)
Delete a DNSZone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone_id** | **uuid::Uuid** | The ID of the DNSZone to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_get_dns_zone

> models::DePeriodMittwaldPeriodV1PeriodDnsPeriodZone dns_get_dns_zone(dns_zone_id)
Get a DNSZone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone_id** | **uuid::Uuid** | The ID of the zone to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodDnsPeriodZone**](de.mittwald.v1.dns.Zone.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_list_dns_zones

> Vec<models::DePeriodMittwaldPeriodV1PeriodDnsPeriodZone> dns_list_dns_zones(project_id)
List DNSZones belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to list DNSZones for. | [required] |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodDnsPeriodZone>**](de.mittwald.v1.dns.Zone.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_record_a_set_custom_deprecated

> dns_record_a_set_custom_deprecated(zone_id, dns_record_a_set_custom_deprecated_request)
updates a-records for a specific zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **uuid::Uuid** | the id of the zone the record belongs to | [required] |
**dns_record_a_set_custom_deprecated_request** | Option<[**DnsRecordASetCustomDeprecatedRequest**](DnsRecordASetCustomDeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_record_a_set_managed_by_ingress_deprecated

> models::DnsRecordASetManagedByIngressDeprecated204Response dns_record_a_set_managed_by_ingress_deprecated(zone_id, body)
set a-records managed by ingress for a specific zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **uuid::Uuid** | the id of the zone you want to set managed | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::DnsRecordASetManagedByIngressDeprecated204Response**](dns_record_a_set_managed_by_ingress_deprecated_204_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_record_cname_set_deprecated

> dns_record_cname_set_deprecated(zone_id, dns_record_cname_set_deprecated_request)
updates cname-record for a specific zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **uuid::Uuid** | the id of the zone the record belongs to | [required] |
**dns_record_cname_set_deprecated_request** | Option<[**DnsRecordCnameSetDeprecatedRequest**](DnsRecordCnameSetDeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_record_mx_set_custom_deprecated

> dns_record_mx_set_custom_deprecated(zone_id, dns_record_mx_set_custom_deprecated_request)
updates mx-records for a specific zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **uuid::Uuid** | the id of the zone the record belongs to | [required] |
**dns_record_mx_set_custom_deprecated_request** | Option<[**DnsRecordMxSetCustomDeprecatedRequest**](DnsRecordMxSetCustomDeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_record_mx_set_managed_deprecated

> dns_record_mx_set_managed_deprecated(zone_id, body)
sets mx-records to managed for a specific zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **uuid::Uuid** | the id of the zone the record belongs to | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_record_srv_set_deprecated

> dns_record_srv_set_deprecated(zone_id, dns_record_srv_set_deprecated_request)
updates srv-records for a specific zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **uuid::Uuid** | the id of the zone the record belongs to | [required] |
**dns_record_srv_set_deprecated_request** | Option<[**DnsRecordSrvSetDeprecatedRequest**](DnsRecordSrvSetDeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_record_txt_set_deprecated

> dns_record_txt_set_deprecated(zone_id, dns_record_txt_set_deprecated_request)
updates txt-records for a specific zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **uuid::Uuid** | the id of the zone the record belongs to | [required] |
**dns_record_txt_set_deprecated_request** | Option<[**DnsRecordTxtSetDeprecatedRequest**](DnsRecordTxtSetDeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_set_record_set_managed

> models::DnsSetRecordSetManaged204Response dns_set_record_set_managed(dns_zone_id, record_set, body)
Set a record set on a DNSZone to managed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone_id** | **uuid::Uuid** | The ID of the DNSZone to set a record set to managed for. | [required] |
**record_set** | **String** | The record set to set to managed. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::DnsSetRecordSetManaged204Response**](dns_set_record_set_managed_204_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_update_record_set

> dns_update_record_set(dns_zone_id, record_set, dns_update_record_set_request)
Update a record set on a DNSZone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone_id** | **uuid::Uuid** | The ID of the DNSZone to update a record set for. | [required] |
**record_set** | **String** | The record set to be updated. | [required] |
**dns_update_record_set_request** | Option<[**DnsUpdateRecordSetRequest**](DnsUpdateRecordSetRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_abort_domain_declaration

> domain_abort_domain_declaration(domain_id)
Abort a Domain declaration.

Abort an incomplete Domain registration/transfer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_change_ownerc_of_domain_v2_deprecated

> models::DomainChangeOwnercOfDomainV2Deprecated200Response domain_change_ownerc_of_domain_v2_deprecated(domain_id, domain_change_ownerc_of_domain_v2_deprecated_request)
Change the owner contact of a domain.

Change the domain owner of a fully registered domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**domain_change_ownerc_of_domain_v2_deprecated_request** | Option<[**DomainChangeOwnercOfDomainV2DeprecatedRequest**](DomainChangeOwnercOfDomainV2DeprecatedRequest.md)> | The new handle fields for the new owner. |  |

### Return type

[**models::DomainChangeOwnercOfDomainV2Deprecated200Response**](domain_change_ownerc_of_domain_v2_deprecated_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_change_project_of_domain_v2_deprecated

> domain_change_project_of_domain_v2_deprecated(domain_id, domain_change_project_of_domain_v2_deprecated_request)
Change the Project relation of a Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**domain_change_project_of_domain_v2_deprecated_request** | Option<[**DomainChangeProjectOfDomainV2DeprecatedRequest**](DomainChangeProjectOfDomainV2DeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_check_domain_registrability

> models::DomainCheckDomainRegistrability200Response domain_check_domain_registrability(domain_check_domain_registrability_request)
Check if a Domain is available to register.

If false, you have to start a transfer with an auth code instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_check_domain_registrability_request** | Option<[**DomainCheckDomainRegistrabilityRequest**](DomainCheckDomainRegistrabilityRequest.md)> |  |  |

### Return type

[**models::DomainCheckDomainRegistrability200Response**](domain_check_domain_registrability_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_check_domain_registrability_v2_deprecated

> models::DomainCheckDomainRegistrabilityV2Deprecated200Response domain_check_domain_registrability_v2_deprecated(domain_check_domain_registrability_request)
Check if a Domain is available to register.

If false, you have to start a transfer with an auth code instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_check_domain_registrability_request** | Option<[**DomainCheckDomainRegistrabilityRequest**](DomainCheckDomainRegistrabilityRequest.md)> |  |  |

### Return type

[**models::DomainCheckDomainRegistrabilityV2Deprecated200Response**](domain_check_domain_registrability_v2_deprecated_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_check_domain_transferability

> models::DomainCheckDomainTransferability200Response domain_check_domain_transferability(domain_check_domain_transferability_request)
Check if a Domain is available to transfer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_check_domain_transferability_request** | Option<[**DomainCheckDomainTransferabilityRequest**](DomainCheckDomainTransferabilityRequest.md)> |  |  |

### Return type

[**models::DomainCheckDomainTransferability200Response**](domain_check_domain_transferability_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_create_domain_auth_code

> models::DomainCreateDomainAuthCode201Response domain_create_domain_auth_code(domain_id)
Create an auth code for a Domains transfer-out process.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DomainCreateDomainAuthCode201Response**](domain_create_domain_auth_code_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_declare_nameservers_v2_deprecated

> domain_declare_nameservers_v2_deprecated(domain_id, domain_declare_nameservers_v2_deprecated_request)
Change all nameservers of a Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**domain_declare_nameservers_v2_deprecated_request** | Option<[**DomainDeclareNameserversV2DeprecatedRequest**](DomainDeclareNameserversV2DeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_declare_process_change_authcode_v2_deprecated

> models::DomainChangeOwnercOfDomainV2Deprecated200Response domain_declare_process_change_authcode_v2_deprecated(domain_id, domain_declare_process_change_authcode_v2_deprecated_request)
Update an AuthCode.

Change a wrong AuthCode of your transfer domain request (declare domain process). This route will also restart the transfer itself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**domain_declare_process_change_authcode_v2_deprecated_request** | Option<[**DomainDeclareProcessChangeAuthcodeV2DeprecatedRequest**](DomainDeclareProcessChangeAuthcodeV2DeprecatedRequest.md)> | The new handle fields for the new owner. |  |

### Return type

[**models::DomainChangeOwnercOfDomainV2Deprecated200Response**](domain_change_ownerc_of_domain_v2_deprecated_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_declare_process_change_handles_v2_deprecated

> models::DomainChangeOwnercOfDomainV2Deprecated200Response domain_declare_process_change_handles_v2_deprecated(domain_id, domain_change_ownerc_of_domain_v2_deprecated_request)
Update a Domain's OwnerC handle.

This route will also restart the transfer or register itself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | The domain id from which the ownerC is to be changed | [required] |
**domain_change_ownerc_of_domain_v2_deprecated_request** | Option<[**DomainChangeOwnercOfDomainV2DeprecatedRequest**](DomainChangeOwnercOfDomainV2DeprecatedRequest.md)> | The new handle fields for the new owner. |  |

### Return type

[**models::DomainChangeOwnercOfDomainV2Deprecated200Response**](domain_change_ownerc_of_domain_v2_deprecated_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_delete_domain

> models::DomainChangeOwnercOfDomainV2Deprecated200Response domain_delete_domain(domain_id, transit)
Delete a Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**transit** | Option<**bool**> |  |  |

### Return type

[**models::DomainChangeOwnercOfDomainV2Deprecated200Response**](domain_change_ownerc_of_domain_v2_deprecated_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_get_domain

> models::DePeriodMittwaldPeriodV1PeriodDomainPeriodDomain domain_get_domain(domain_id)
Get a Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodDomainPeriodDomain**](de.mittwald.v1.domain.Domain.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_get_handle_fields_v2_deprecated

> models::DomainGetHandleFieldsV2Deprecated200Response domain_get_handle_fields_v2_deprecated(domain_name)
Get a HandleSchema.

Get required handle fields of a registrar for your domain process (transfer/registration).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | The whole domain name | [required] |

### Return type

[**models::DomainGetHandleFieldsV2Deprecated200Response**](domain_get_handle_fields_v2_deprecated_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_get_latest_screenshot

> models::DomainGetLatestScreenshot200Response domain_get_latest_screenshot(domain_id, deprecated_domain_get_screenshot_for_domain_request)
Get the latest screenshot's FileReference belonging to a Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**deprecated_domain_get_screenshot_for_domain_request** | [**DeprecatedDomainGetScreenshotForDomainRequest**](DeprecatedDomainGetScreenshotForDomainRequest.md) |  | [required] |

### Return type

[**models::DomainGetLatestScreenshot200Response**](domain_get_latest_screenshot_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_list_domains

> Vec<models::DePeriodMittwaldPeriodV1PeriodDomainPeriodDomain> domain_list_domains(project_id, page, limit, domain_search_name)
List Domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | Option<**uuid::Uuid**> |  |  |
**page** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |
**domain_search_name** | Option<**String**> | Search for domain names in these list. Works only in combination with projectId. Input will be handled like '%YourInput%' |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodDomainPeriodDomain>**](de.mittwald.v1.domain.Domain.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_list_domains_v2_deprecated

> Vec<models::DePeriodMittwaldPeriodV1PeriodDomainPeriodDomain> domain_list_domains_v2_deprecated(project_id, page, limit, domain_search_name)
List Domains belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**page** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |
**domain_search_name** | Option<**String**> | Search for domain names in these list. Input will be handled like '%YourInput%' |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodDomainPeriodDomain>**](de.mittwald.v1.domain.Domain.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_list_tld_contact_schemas

> models::DomainGetHandleFieldsV2Deprecated200Response domain_list_tld_contact_schemas(tld)
List the contact schemas for a TLD.

List the contact schemas describing the fields required to register/transfer a Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tld** | **String** | The top level domain to retrieve the schemas for. | [required] |

### Return type

[**models::DomainGetHandleFieldsV2Deprecated200Response**](domain_get_handle_fields_v2_deprecated_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_list_tlds

> Vec<models::DePeriodMittwaldPeriodV1PeriodDomainPeriodTopLevel> domain_list_tlds()
List TLDs.

List the top level domains currently supported by our API.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodDomainPeriodTopLevel>**](de.mittwald.v1.domain.TopLevel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_resend_domain_email

> domain_resend_domain_email(domain_id)
Resend a Domain email.

Trigger a resend of a confirmation or registrant verification email. Has no effect on .de Domains.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_suggest

> models::DePeriodMittwaldPeriodV1PeriodDomainPeriodSuggestedDomains domain_suggest(prompt, domain_count, tlds)
Suggest a list of domains based on a prompt using AI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt** | **String** |  | [required] |
**domain_count** | Option<**i32**> |  |  |[default to 6]
**tlds** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodDomainPeriodSuggestedDomains**](de.mittwald.v1.domain.SuggestedDomains.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_update_domain_auth_code

> models::DomainChangeOwnercOfDomainV2Deprecated200Response domain_update_domain_auth_code(domain_id, domain_declare_process_change_authcode_v2_deprecated_request)
Update the auth code of a Domain.

Update an incorrect auth code of an ongoing/failed Domain transfer. This route will also restart the transfer itself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**domain_declare_process_change_authcode_v2_deprecated_request** | Option<[**DomainDeclareProcessChangeAuthcodeV2DeprecatedRequest**](DomainDeclareProcessChangeAuthcodeV2DeprecatedRequest.md)> |  |  |

### Return type

[**models::DomainChangeOwnercOfDomainV2Deprecated200Response**](domain_change_ownerc_of_domain_v2_deprecated_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_update_domain_contact

> models::DomainChangeOwnercOfDomainV2Deprecated200Response domain_update_domain_contact(domain_id, contact, domain_update_domain_contact_request)
Update a contact of a Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**contact** | **String** |  | [required] |
**domain_update_domain_contact_request** | Option<[**DomainUpdateDomainContactRequest**](DomainUpdateDomainContactRequest.md)> | The new contact data of the Domain. |  |

### Return type

[**models::DomainChangeOwnercOfDomainV2Deprecated200Response**](domain_change_ownerc_of_domain_v2_deprecated_200_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_update_domain_nameservers

> domain_update_domain_nameservers(domain_id, domain_declare_nameservers_v2_deprecated_request)
Update the nameservers of a Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**domain_declare_nameservers_v2_deprecated_request** | Option<[**DomainDeclareNameserversV2DeprecatedRequest**](DomainDeclareNameserversV2DeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_update_domain_project_id

> domain_update_domain_project_id(domain_id, domain_change_project_of_domain_v2_deprecated_request)
Update a Domain's project id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**domain_change_project_of_domain_v2_deprecated_request** | Option<[**DomainChangeProjectOfDomainV2DeprecatedRequest**](DomainChangeProjectOfDomainV2DeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingress_create_ingress

> models::IngressCreateIngress201Response ingress_create_ingress(ingress_create_ingress_request)
Create an Ingress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ingress_create_ingress_request** | Option<[**IngressCreateIngressRequest**](IngressCreateIngressRequest.md)> |  |  |

### Return type

[**models::IngressCreateIngress201Response**](ingress_create_ingress_201_response.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingress_delete_ingress

> ingress_delete_ingress(ingress_id)
Delete an Ingress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ingress_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingress_get_ingress

> models::DePeriodMittwaldPeriodV1PeriodIngressPeriodIngress ingress_get_ingress(ingress_id)
Get an Ingress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ingress_id** | **uuid::Uuid** | ID of the Ingress to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodIngressPeriodIngress**](de.mittwald.v1.ingress.Ingress.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingress_ingress_verify_ownership

> serde_json::Value ingress_ingress_verify_ownership(ingress_id)
Verifiy the ownership of an Ingress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ingress_id** | **uuid::Uuid** | UUID of the Ingress to verify the ownership for. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingress_list_ingresses

> Vec<models::DePeriodMittwaldPeriodV1PeriodIngressPeriodIngress> ingress_list_ingresses(project_id, certificate_id, limit, skip, page)
List Ingresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | Option<**uuid::Uuid**> | ID of the Project to list Ingresses for. |  |
**certificate_id** | Option<**uuid::Uuid**> | ID of the Certificate to list Ingresses for. |  |
**limit** | Option<**i32**> | The max count of resources to return in the list response. |  |[default to 10000]
**skip** | Option<**i32**> | Number of items to skip. Should be multiple of `limit`. |  |[default to 0]
**page** | Option<**i32**> | Page to display. `skip` and `page` end up doing the same. If both `page` and `skip` are set, skip is used. |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodIngressPeriodIngress>**](de.mittwald.v1.ingress.Ingress.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingress_list_ingresses_compatible_with_certificate

> Vec<models::DePeriodMittwaldPeriodV1PeriodIngressPeriodIngress> ingress_list_ingresses_compatible_with_certificate(ingress_list_ingresses_compatible_with_certificate_request)
List Ingresses compatible with a certificate.

List Ingresses in a Project compatible with a certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ingress_list_ingresses_compatible_with_certificate_request** | Option<[**IngressListIngressesCompatibleWithCertificateRequest**](IngressListIngressesCompatibleWithCertificateRequest.md)> |  |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodIngressPeriodIngress>**](de.mittwald.v1.ingress.Ingress.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingress_list_ingresses_v2_deprecated

> Vec<models::DePeriodMittwaldPeriodV1PeriodIngressPeriodIngressDeprecated> ingress_list_ingresses_v2_deprecated(project_id)
List Ingresses belonging to a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to list Ingresses for | [required] |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodIngressPeriodIngressDeprecated>**](de.mittwald.v1.ingress.IngressDeprecated.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingress_paths_deprecated

> ingress_paths_deprecated(ingress_id, de_period_mittwald_period_v1_period_ingress_period_path)
Update an Ingresses paths.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ingress_id** | **uuid::Uuid** |  | [required] |
**de_period_mittwald_period_v1_period_ingress_period_path** | Option<[**Vec<models::DePeriodMittwaldPeriodV1PeriodIngressPeriodPath>**](de.mittwald.v1.ingress.Path.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingress_request_ingress_acme_certificate_issuance

> ingress_request_ingress_acme_certificate_issuance(ingress_id)
Request the ACME certificate issuance of an Ingress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ingress_id** | **uuid::Uuid** | UUID of the Ingress to request the issuance for. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingress_tls_deprecated

> serde_json::Value ingress_tls_deprecated(ingress_id, ingress_tls_deprecated_request)
Update an Ingresses tls settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ingress_id** | **uuid::Uuid** |  | [required] |
**ingress_tls_deprecated_request** | Option<[**IngressTlsDeprecatedRequest**](IngressTlsDeprecatedRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingress_update_ingress_paths

> ingress_update_ingress_paths(ingress_id, de_period_mittwald_period_v1_period_ingress_period_path)
Update the paths of an Ingress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ingress_id** | **uuid::Uuid** |  | [required] |
**de_period_mittwald_period_v1_period_ingress_period_path** | Option<[**Vec<models::DePeriodMittwaldPeriodV1PeriodIngressPeriodPath>**](de.mittwald.v1.ingress.Path.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingress_update_ingress_tls

> serde_json::Value ingress_update_ingress_tls(ingress_id, ingress_update_ingress_tls_request)
Update the tls settings of an Ingress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ingress_id** | **uuid::Uuid** |  | [required] |
**ingress_update_ingress_tls_request** | Option<[**IngressUpdateIngressTlsRequest**](IngressUpdateIngressTlsRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssl_check_replace_certificate

> models::DePeriodMittwaldPeriodV1PeriodSslPeriodCheckReplaceCertificateResponse ssl_check_replace_certificate(certificate_id, ssl_check_replace_certificate_request)
Check the replacement of a Certificate.

Checks the replacement of a Certificate and shows differences between the current and the new Certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **uuid::Uuid** |  | [required] |
**ssl_check_replace_certificate_request** | Option<[**SslCheckReplaceCertificateRequest**](SslCheckReplaceCertificateRequest.md)> |  |  |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodSslPeriodCheckReplaceCertificateResponse**](de.mittwald.v1.ssl.CheckReplaceCertificateResponse.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssl_create_certificate_request

> models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateRequestCreateResponse ssl_create_certificate_request(ssl_create_certificate_request_request)
Create a CertificateRequest.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssl_create_certificate_request_request** | Option<[**SslCreateCertificateRequestRequest**](SslCreateCertificateRequestRequest.md)> |  |  |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateRequestCreateResponse**](de.mittwald.v1.ssl.CertificateRequestCreateResponse.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssl_delete_certificate

> ssl_delete_certificate(certificate_id)
Delete a Certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssl_delete_certificate_request

> ssl_delete_certificate_request(certificate_request_id)
Delete a CertificateRequest.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_request_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssl_get_certificate

> models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificate ssl_get_certificate(certificate_id)
Get a Certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **uuid::Uuid** | ID of the SSL Certificate to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificate**](de.mittwald.v1.ssl.Certificate.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssl_get_certificate_request

> models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateRequest ssl_get_certificate_request(certificate_request_id)
Get a CertificateRequest.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_request_id** | **uuid::Uuid** | ID of the CertificateRequest to be retrieved. | [required] |

### Return type

[**models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateRequest**](de.mittwald.v1.ssl.CertificateRequest.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssl_list_certificate_requests

> Vec<models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateRequest> ssl_list_certificate_requests(project_id, ingress_id)
List CertificateRequests belonging to a Project or an Ingress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | Option<**uuid::Uuid**> | ID of the Project to list Certificate Requests for. |  |
**ingress_id** | Option<**uuid::Uuid**> | ID of the Ingress to list Certificate Requests for. |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateRequest>**](de.mittwald.v1.ssl.CertificateRequest.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssl_list_certificates

> Vec<models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificate> ssl_list_certificates(project_id, ingress_id)
List Certificates belonging to a Project or an Ingress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | Option<**uuid::Uuid**> | ID of the Project to list Certificates for. |  |
**ingress_id** | Option<**uuid::Uuid**> | ID of the Ingress to list Certificates for. |  |

### Return type

[**Vec<models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificate>**](de.mittwald.v1.ssl.Certificate.md)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssl_replace_certificate

> ssl_replace_certificate(certificate_id, ssl_check_replace_certificate_request)
Update a Certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **uuid::Uuid** |  | [required] |
**ssl_check_replace_certificate_request** | Option<[**SslCheckReplaceCertificateRequest**](SslCheckReplaceCertificateRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_dns_zones_dns_zone_id_delete

> v2_dns_zones_dns_zone_id_delete(dns_zone_id)
Delete a DNSZone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone_id** | **uuid::Uuid** | The ID of the DNSZone to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_dns_zones_dns_zone_id_get

> v2_dns_zones_dns_zone_id_get(dns_zone_id)
Get a DNSZone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone_id** | **uuid::Uuid** | The ID of the zone to be retrieved. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_dns_zones_post

> v2_dns_zones_post(v2_dns_zones_post_request)
Create a DNSZone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v2_dns_zones_post_request** | Option<[**V2DnsZonesPostRequest**](V2DnsZonesPostRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_domains_domain_id_actions_create_authcode_post

> v2_domains_domain_id_actions_create_authcode_post(domain_id)
Create an auth code for a Domains transfer-out process.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_domains_domain_id_authcode_patch

> v2_domains_domain_id_authcode_patch(domain_id, domain_declare_process_change_authcode_v2_deprecated_request)
Update the auth code of a Domain.

Update an incorrect auth code of an ongoing/failed Domain transfer. This route will also restart the transfer itself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**domain_declare_process_change_authcode_v2_deprecated_request** | Option<[**DomainDeclareProcessChangeAuthcodeV2DeprecatedRequest**](DomainDeclareProcessChangeAuthcodeV2DeprecatedRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_domains_domain_id_declarations_delete

> v2_domains_domain_id_declarations_delete(domain_id)
Abort a Domain declaration.

Abort an incomplete Domain registration/transfer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_domains_supported_tlds_get

> v2_domains_supported_tlds_get()
List TLDs.

List the top level domains currently supported by our API.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_projects_project_id_dns_zones_get

> v2_projects_project_id_dns_zones_get(project_id)
List DNSZones belonging to a Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | ID of the Project to list DNSZones for. | [required] |

### Return type

 (empty response body)

### Authorization

[de.mittwald.v1.commons.AccessToken](../README.md#de.mittwald.v1.commons.AccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

