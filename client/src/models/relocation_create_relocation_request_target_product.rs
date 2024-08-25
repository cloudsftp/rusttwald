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

/// RelocationCreateRelocationRequestTargetProduct : Help our customer service finding your target account
/// Help our customer service finding your target account
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RelocationCreateRelocationRequestTargetProduct {
    #[serde(rename = "Space-Server")]
    SpaceServer,
    #[serde(rename = "proSpace")]
    ProSpace,
    #[serde(rename = "Agentur-Server")]
    AgenturServer,
    #[serde(rename = "CMS-Hosting")]
    CMSHosting,
    #[serde(rename = "Shop-Hosting")]
    ShopHosting,
    #[serde(untagged)]
    Other(String),
}

impl Default for RelocationCreateRelocationRequestTargetProduct {
    fn default() -> Self {
        Self::Other(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ser_other() {
        let variant = RelocationCreateRelocationRequestTargetProduct::Other("other".to_string());
        assert_eq!(
            serde_json::to_string(&variant).expect("could not serialize"),
            r#""other""#
        );
    }

    #[test]
    fn test_de_other() {
        let raw = r#""other""#;
        assert_eq!(
            RelocationCreateRelocationRequestTargetProduct::Other("other".to_string()),
            serde_json::from_str(raw).expect("could not decode"),
        );
    }

    #[test]
    fn test_ser_space_server() {
        let variant = RelocationCreateRelocationRequestTargetProduct::SpaceServer;
        assert_eq!(
            serde_json::to_string(&variant).expect("could not serialize"),
            r#""Space-Server""#
        );
    }

    #[test]
    fn test_de_space_server() {
        let raw = r#""Space-Server""#;
        assert_eq!(
            RelocationCreateRelocationRequestTargetProduct::SpaceServer,
            serde_json::from_str(raw).expect("could not decode"),
        );
    }
}
