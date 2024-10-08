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
pub struct DePeriodMittwaldPeriodV1PeriodSignupPeriodApiToken {
    #[serde(rename = "apiTokenId")]
    pub api_token_id: uuid::Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "roles")]
    pub roles: Vec<Roles>,
}

impl DePeriodMittwaldPeriodV1PeriodSignupPeriodApiToken {
    pub fn new(api_token_id: uuid::Uuid, created_at: String, description: String, roles: Vec<Roles>) -> DePeriodMittwaldPeriodV1PeriodSignupPeriodApiToken {
        DePeriodMittwaldPeriodV1PeriodSignupPeriodApiToken {
            api_token_id,
            created_at,
            description,
            expires_at: None,
            roles,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Roles {
    #[serde(rename = "api_read")]
    Read,
    #[serde(rename = "api_write")]
    Write,
}

impl Default for Roles {
    fn default() -> Roles {
        Self::Read
    }
}

