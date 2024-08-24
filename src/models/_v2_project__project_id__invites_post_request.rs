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
pub struct V2ProjectProjectIdInvitesPostRequest {
    /// Mail-address of the person to be invited.
    #[serde(rename = "mailAddress")]
    pub mail_address: String,
    /// Time the resulting ProjectMembership should expire at.
    #[serde(rename = "membershipExpiresAt", skip_serializing_if = "Option::is_none")]
    pub membership_expires_at: Option<String>,
    /// Message contained in the ProjectInvite.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "role")]
    pub role: models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectRoles,
}

impl V2ProjectProjectIdInvitesPostRequest {
    pub fn new(mail_address: String, role: models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodProjectRoles) -> V2ProjectProjectIdInvitesPostRequest {
        V2ProjectProjectIdInvitesPostRequest {
            mail_address,
            membership_expires_at: None,
            message: None,
            role,
        }
    }
}

