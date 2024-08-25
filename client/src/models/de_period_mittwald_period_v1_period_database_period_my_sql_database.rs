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
pub struct DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlDatabase {
    #[serde(rename = "characterSettings")]
    pub character_settings: Box<models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodCharacterSettings>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "finalizers", skip_serializing_if = "Option::is_none")]
    pub finalizers: Option<Vec<String>>,
    #[serde(rename = "hostname")]
    pub hostname: String,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "isReady")]
    pub is_ready: bool,
    #[serde(rename = "isShared")]
    pub is_shared: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "projectId")]
    pub project_id: uuid::Uuid,
    #[serde(rename = "status")]
    pub status: models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodDatabaseStatus,
    #[serde(rename = "statusSetAt")]
    pub status_set_at: String,
    #[serde(rename = "storageUsageInBytes")]
    pub storage_usage_in_bytes: i32,
    #[serde(rename = "storageUsageInBytesSetAt")]
    pub storage_usage_in_bytes_set_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "version")]
    pub version: String,
}

impl DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlDatabase {
    pub fn new(character_settings: models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodCharacterSettings, created_at: String, description: String, hostname: String, id: uuid::Uuid, is_ready: bool, is_shared: bool, name: String, project_id: uuid::Uuid, status: models::DePeriodMittwaldPeriodV1PeriodDatabasePeriodDatabaseStatus, status_set_at: String, storage_usage_in_bytes: i32, storage_usage_in_bytes_set_at: String, updated_at: String, version: String) -> DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlDatabase {
        DePeriodMittwaldPeriodV1PeriodDatabasePeriodMySqlDatabase {
            character_settings: Box::new(character_settings),
            created_at,
            description,
            finalizers: None,
            hostname,
            id,
            is_ready,
            is_shared,
            name,
            project_id,
            status,
            status_set_at,
            storage_usage_in_bytes,
            storage_usage_in_bytes_set_at,
            updated_at,
            version,
        }
    }
}

