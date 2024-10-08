/*
 * Mittwald API
 *
 * ## Introduction  This OpenAPI spec documents the mittwald API. It follows the [OpenAPI 3.0.0 specification](https://spec.openapis.org/oas/v3.0.0.html).  ## Authentication  You will need an API token to access the API. You can obtain one by logging into the [mStudio](https://studio.mittwald.de) and navigating to the [\"API Tokens\" section in the user menu](https://studio.mittwald.de/app/profile/api-tokens).  When making requests to the API, you can authenticate by passing your API token in the `X-Access-Token` header or as a bearer token.  ## Rate Limiting  Please note that usage of the API is rate-limited to prevent abuse. You can inspect the rate limiting for your current user by observing the `X-Ratelimit-*` headers included in each response.  ## mStudio  A main consumer of the mittwald API is the management interface for our customers, the [mStudio](https://studio.mittwald.de).  ## Contact and support  For support, please use the [mStudio support area](https://studio.mittwald.de/app/support/conversations) or drop us an email at [support@mittwald.de](mailto:support@mittwald.de).  For security issues, please report to [security@mittwald.de](mailto:security@mittwald.de). 
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateRequest {
    #[serde(rename = "certificateData")]
    pub certificate_data: Box<models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateData>,
    #[serde(rename = "certificateType")]
    pub certificate_type: models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateType,
    #[serde(rename = "commonName", skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    #[serde(rename = "contact", skip_serializing_if = "Option::is_none")]
    pub contact: Option<Box<models::DePeriodMittwaldPeriodV1PeriodSslPeriodContact>>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "dnsNames", skip_serializing_if = "Option::is_none")]
    pub dns_names: Option<Vec<String>>,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "isCompleted")]
    pub is_completed: bool,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: uuid::Uuid,
    #[serde(rename = "validFrom", skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo", skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
}

impl DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateRequest {
    pub fn new(certificate_data: models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateData, certificate_type: models::DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateType, created_at: String, id: uuid::Uuid, is_completed: bool, project_id: uuid::Uuid) -> DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateRequest {
        DePeriodMittwaldPeriodV1PeriodSslPeriodCertificateRequest {
            certificate_data: Box::new(certificate_data),
            certificate_type,
            common_name: None,
            contact: None,
            created_at,
            dns_names: None,
            id,
            is_completed,
            issuer: None,
            project_id,
            valid_from: None,
            valid_to: None,
        }
    }
}

