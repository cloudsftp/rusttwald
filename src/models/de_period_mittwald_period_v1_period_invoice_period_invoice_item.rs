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
pub struct DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoiceItem {
    #[serde(rename = "additionalDescription", skip_serializing_if = "Option::is_none")]
    pub additional_description: Option<String>,
    #[serde(rename = "contractItemId")]
    pub contract_item_id: uuid::Uuid,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "itemCancelledOrCorrectedBy", skip_serializing_if = "Option::is_none")]
    pub item_cancelled_or_corrected_by: Option<Vec<models::DeMittwaldV1InvoiceInvoiceItemItemCancelledOrCorrectedByInner>>,
    #[serde(rename = "itemId")]
    pub item_id: uuid::Uuid,
    #[serde(rename = "price")]
    pub price: Box<models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodPrice>,
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<Box<models::DeMittwaldV1InvoiceInvoiceItemReference>>,
    #[serde(rename = "serviceDate", skip_serializing_if = "Option::is_none")]
    pub service_date: Option<String>,
    #[serde(rename = "servicePeriod", skip_serializing_if = "Option::is_none")]
    pub service_period: Option<Box<models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodDatePeriod>>,
    #[serde(rename = "vatRate")]
    pub vat_rate: f64,
}

impl DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoiceItem {
    pub fn new(contract_item_id: uuid::Uuid, description: String, item_id: uuid::Uuid, price: models::DePeriodMittwaldPeriodV1PeriodInvoicePeriodPrice, vat_rate: f64) -> DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoiceItem {
        DePeriodMittwaldPeriodV1PeriodInvoicePeriodInvoiceItem {
            additional_description: None,
            contract_item_id,
            description,
            item_cancelled_or_corrected_by: None,
            item_id,
            price: Box::new(price),
            reference: None,
            service_date: None,
            service_period: None,
            vat_rate,
        }
    }
}

