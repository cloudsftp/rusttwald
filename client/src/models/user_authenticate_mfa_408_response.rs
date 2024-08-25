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
pub struct UserAuthenticateMfa408Response {
    /// A json object, given further information about the error
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
    /// Some more detailed information about the error
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "name")]
    pub name: Name,
}

impl UserAuthenticateMfa408Response {
    pub fn new(message: String, name: Name) -> UserAuthenticateMfa408Response {
        UserAuthenticateMfa408Response {
            info: None,
            message,
            name,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "FirstAuthenticationFactorExpired")]
    FirstAuthenticationFactorExpired,
}

impl Default for Name {
    fn default() -> Name {
        Self::FirstAuthenticationFactorExpired
    }
}

