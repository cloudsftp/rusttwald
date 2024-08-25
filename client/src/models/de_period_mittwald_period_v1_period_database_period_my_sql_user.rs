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
pub struct DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlUser {
    #[serde(rename = "accessIpMask", skip_serializing_if = "Option::is_none")]
    pub access_ip_mask: Option<String>,
    #[serde(rename = "accessLevel")]
    pub access_level: AccessLevel,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "databaseId")]
    pub database_id: uuid::Uuid,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "disabled")]
    pub disabled: bool,
    #[serde(rename = "externalAccess")]
    pub external_access: bool,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "mainUser")]
    pub main_user: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "passwordUpdatedAt")]
    pub password_updated_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlUser {
    pub fn new(access_level: AccessLevel, created_at: String, database_id: uuid::Uuid, disabled: bool, external_access: bool, id: uuid::Uuid, main_user: bool, name: String, password_updated_at: String, updated_at: String) -> DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlUser {
        DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlUser {
            access_ip_mask: None,
            access_level,
            created_at,
            database_id,
            description: None,
            disabled,
            external_access,
            id,
            main_user,
            name,
            password_updated_at,
            updated_at,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccessLevel {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "readonly")]
    Readonly,
}

impl Default for AccessLevel {
    fn default() -> AccessLevel {
        Self::Full
    }
}

