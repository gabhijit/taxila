/*
 * AUSF API
 *
 * AUSF UE Authentication Service. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved.
 *
 * The version of the OpenAPI document: 1.2.2
 *
 * Generated by: https://openapi-generator.tech
 */

/// AuthType : Indicates the authentication method used.

/// Indicates the authentication method used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthType {
    #[serde(rename = "5G_AKA")]
    Variant5GAka,
    #[serde(rename = "EAP_AKA_PRIME")]
    EapAkaPrime,
    #[serde(rename = "EAP_TLS")]
    EapTls,
    #[serde(rename = "EAP_TTLS")]
    EapTtls,
}

impl ToString for AuthType {
    fn to_string(&self) -> String {
        match self {
            Self::Variant5GAka => String::from("5G_AKA"),
            Self::EapAkaPrime => String::from("EAP_AKA_PRIME"),
            Self::EapTls => String::from("EAP_TLS"),
            Self::EapTtls => String::from("EAP_TTLS"),
        }
    }
}

impl Default for AuthType {
    fn default() -> AuthType {
        Self::Variant5GAka
    }
}