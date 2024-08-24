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
pub struct DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjobExecution {
    #[serde(rename = "abortedBy", skip_serializing_if = "Option::is_none")]
    pub aborted_by: Option<Box<models::DeMittwaldV1CronjobCronjobExecutionAbortedBy>>,
    #[serde(rename = "durationInMilliseconds", skip_serializing_if = "Option::is_none")]
    pub duration_in_milliseconds: Option<i64>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "logPath", skip_serializing_if = "Option::is_none")]
    pub log_path: Option<String>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "successful")]
    pub successful: bool,
    #[serde(rename = "triggeredBy", skip_serializing_if = "Option::is_none")]
    pub triggered_by: Option<Box<models::DeMittwaldV1CronjobCronjobExecutionAbortedBy>>,
}

impl DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjobExecution {
    pub fn new(id: String, status: Status, successful: bool) -> DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjobExecution {
        DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjobExecution {
            aborted_by: None,
            duration_in_milliseconds: None,
            end: None,
            id,
            log_path: None,
            start: None,
            status,
            successful,
            triggered_by: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Complete")]
    Complete,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "AbortedBySystem")]
    AbortedBySystem,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "AbortedByUser")]
    AbortedByUser,
    #[serde(rename = "TimedOut")]
    TimedOut,
}

impl Default for Status {
    fn default() -> Status {
        Self::Complete
    }
}

