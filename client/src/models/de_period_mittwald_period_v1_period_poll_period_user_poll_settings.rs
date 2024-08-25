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
pub struct DePeriodMittwaldPeriodV1PeriodPollPeriodUserPollSettings {
    #[serde(rename = "completedAt", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "dontShowUntil", skip_serializing_if = "Option::is_none")]
    pub dont_show_until: Option<String>,
    #[serde(rename = "ignoredAt", skip_serializing_if = "Option::is_none")]
    pub ignored_at: Option<String>,
    #[serde(rename = "shouldShow")]
    pub should_show: bool,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl DePeriodMittwaldPeriodV1PeriodPollPeriodUserPollSettings {
    pub fn new(should_show: bool, status: Status, user_id: String) -> DePeriodMittwaldPeriodV1PeriodPollPeriodUserPollSettings {
        DePeriodMittwaldPeriodV1PeriodPollPeriodUserPollSettings {
            completed_at: None,
            dont_show_until: None,
            ignored_at: None,
            should_show,
            status,
            user_id,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "muted")]
    Muted,
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "new")]
    New,
}

impl Default for Status {
    fn default() -> Status {
        Self::Completed
    }
}

