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

/// DeMittwaldV1SignupProfileMfaDetails : the users mfa details
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeMittwaldV1SignupProfileMfaDetails {
    #[serde(rename = "mfaConfirmed", skip_serializing_if = "Option::is_none")]
    pub mfa_confirmed: Option<bool>,
    #[serde(rename = "mfaInitialized", skip_serializing_if = "Option::is_none")]
    pub mfa_initialized: Option<bool>,
}

impl DeMittwaldV1SignupProfileMfaDetails {
    /// the users mfa details
    pub fn new() -> DeMittwaldV1SignupProfileMfaDetails {
        DeMittwaldV1SignupProfileMfaDetails {
            mfa_confirmed: None,
            mfa_initialized: None,
        }
    }
}

