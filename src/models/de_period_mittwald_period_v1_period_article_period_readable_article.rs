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
pub struct DePeriodMittwaldPeriodV1PeriodArticlePeriodReadableArticle {
    #[serde(rename = "addons", skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<models::DePeriodMittwaldPeriodV1PeriodArticlePeriodArticleAddons>>,
    #[serde(rename = "articleId")]
    pub article_id: String,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<models::DePeriodMittwaldPeriodV1PeriodArticlePeriodArticleAttributes>>,
    #[serde(rename = "balanceAddonKey", skip_serializing_if = "Option::is_none")]
    pub balance_addon_key: Option<String>,
    #[serde(rename = "contractDurationInMonth")]
    pub contract_duration_in_month: f64,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "forcedInvoicingPeriodInMonth", skip_serializing_if = "Option::is_none")]
    pub forced_invoicing_period_in_month: Option<f64>,
    #[serde(rename = "hasIndependentContractPeriod", skip_serializing_if = "Option::is_none")]
    pub has_independent_contract_period: Option<bool>,
    #[serde(rename = "hideOnInvoice", skip_serializing_if = "Option::is_none")]
    pub hide_on_invoice: Option<bool>,
    #[serde(rename = "machineType", skip_serializing_if = "Option::is_none")]
    pub machine_type: Option<Box<models::DeMittwaldV1ArticleReadableArticleMachineType>>,
    #[serde(rename = "modifierArticles", skip_serializing_if = "Option::is_none")]
    pub modifier_articles: Option<Vec<models::DePeriodMittwaldPeriodV1PeriodArticlePeriodReadableModifierArticleOptions>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "orderable")]
    pub orderable: Orderable,
    #[serde(rename = "possibleArticleChanges", skip_serializing_if = "Option::is_none")]
    pub possible_article_changes: Option<Vec<models::DePeriodMittwaldPeriodV1PeriodArticlePeriodReadableChangeArticleOptions>>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<models::DePeriodMittwaldPeriodV1PeriodArticlePeriodArticleTag>>,
    #[serde(rename = "template")]
    pub template: Box<models::DePeriodMittwaldPeriodV1PeriodArticlePeriodArticleTemplate>,
}

impl DePeriodMittwaldPeriodV1PeriodArticlePeriodReadableArticle {
    pub fn new(article_id: String, contract_duration_in_month: f64, name: String, orderable: Orderable, template: models::DePeriodMittwaldPeriodV1PeriodArticlePeriodArticleTemplate) -> DePeriodMittwaldPeriodV1PeriodArticlePeriodReadableArticle {
        DePeriodMittwaldPeriodV1PeriodArticlePeriodReadableArticle {
            addons: None,
            article_id,
            attributes: None,
            balance_addon_key: None,
            contract_duration_in_month,
            description: None,
            forced_invoicing_period_in_month: None,
            has_independent_contract_period: None,
            hide_on_invoice: None,
            machine_type: None,
            modifier_articles: None,
            name,
            orderable,
            possible_article_changes: None,
            price: None,
            tags: None,
            template: Box::new(template),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Orderable {
    #[serde(rename = "forbidden")]
    Forbidden,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "beta_testing")]
    BetaTesting,
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "deprecated")]
    Deprecated,
}

impl Default for Orderable {
    fn default() -> Orderable {
        Self::Forbidden
    }
}

