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
pub struct PageinsightsGetPerformanceData200Response {
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<models::PageinsightsGetPerformanceData200ResponseMetricsInner>>,
    #[serde(rename = "moreDataAvailable", skip_serializing_if = "Option::is_none")]
    pub more_data_available: Option<Vec<String>>,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "performanceScore")]
    pub performance_score: f64,
    #[serde(rename = "screenshot", skip_serializing_if = "Option::is_none")]
    pub screenshot: Option<Box<models::PageinsightsGetPerformanceData200ResponseScreenshot>>,
}

impl PageinsightsGetPerformanceData200Response {
    pub fn new(domain: String, path: String, performance_score: f64) -> PageinsightsGetPerformanceData200Response {
        PageinsightsGetPerformanceData200Response {
            created_at: None,
            domain,
            metrics: None,
            more_data_available: None,
            path,
            performance_score,
            screenshot: None,
        }
    }
}

