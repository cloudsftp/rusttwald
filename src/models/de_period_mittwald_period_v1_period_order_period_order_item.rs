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
pub struct DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderItem {
    #[serde(rename = "addons", skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodAddons>>,
    #[serde(rename = "articleId")]
    pub article_id: String,
    #[serde(rename = "articleName", skip_serializing_if = "Option::is_none")]
    pub article_name: Option<String>,
    #[serde(rename = "articleTemplateName", skip_serializing_if = "Option::is_none")]
    pub article_template_name: Option<String>,
    #[serde(rename = "attributeConfiguration", skip_serializing_if = "Option::is_none")]
    pub attribute_configuration: Option<Vec<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodAttributeConfiguration>>,
    #[serde(rename = "isInclusive")]
    pub is_inclusive: bool,
    #[serde(rename = "orderItemId")]
    pub order_item_id: uuid::Uuid,
    #[serde(rename = "predefinedDomainAggregateId", skip_serializing_if = "Option::is_none")]
    pub predefined_domain_aggregate_id: Option<String>,
    #[serde(rename = "price")]
    pub price: f64,
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<Box<models::DePeriodMittwaldPeriodV1PeriodOrderPeriodReference>>,
}

impl DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderItem {
    pub fn new(article_id: String, is_inclusive: bool, order_item_id: uuid::Uuid, price: f64) -> DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderItem {
        DePeriodMittwaldPeriodV1PeriodOrderPeriodOrderItem {
            addons: None,
            article_id,
            article_name: None,
            article_template_name: None,
            attribute_configuration: None,
            is_inclusive,
            order_item_id,
            predefined_domain_aggregate_id: None,
            price,
            reference: None,
        }
    }
}

