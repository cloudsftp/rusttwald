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
pub struct DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite {
    /// Reference to the Project's avatar.
    #[serde(rename = "avatarRefId", skip_serializing_if = "Option::is_none")]
    pub avatar_ref_id: Option<uuid::Uuid>,
    /// ID of the Customer the invite is for.
    #[serde(rename = "customerId")]
    pub customer_id: uuid::Uuid,
    /// Name of the Customer the user is invited to.
    #[serde(rename = "customerName")]
    pub customer_name: String,
    /// ID of the CustomerInvite.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "information")]
    pub information: Box<models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodInviteInformation>,
    /// Mail-address of the user the invite is for.
    #[serde(rename = "mailAddress")]
    pub mail_address: String,
    /// Time the CustomerMembership should expire at.
    #[serde(rename = "membershipExpiresAt", skip_serializing_if = "Option::is_none")]
    pub membership_expires_at: Option<String>,
    /// Message contained in the CustomerInvite.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "role")]
    pub role: models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerRoles,
}

impl DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite {
    pub fn new(customer_id: uuid::Uuid, customer_name: String, id: uuid::Uuid, information: models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodInviteInformation, mail_address: String, role: models::DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerRoles) -> DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite {
        DePeriodMittwaldPeriodV1PeriodMembershipPeriodCustomerInvite {
            avatar_ref_id: None,
            customer_id,
            customer_name,
            id,
            information: Box::new(information),
            mail_address,
            membership_expires_at: None,
            message: None,
            role,
        }
    }
}

