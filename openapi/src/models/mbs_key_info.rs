/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MbsKeyInfo : MBS Security Key Data Structure



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MbsKeyInfo {
    /// string with format 'bytes' as defined in OpenAPI
    #[serde(rename = "keyDomainId")]
    pub key_domain_id: String,
    /// string with format 'bytes' as defined in OpenAPI
    #[serde(rename = "mskId")]
    pub msk_id: String,
    /// string with format 'bytes' as defined in OpenAPI
    #[serde(rename = "msk", skip_serializing_if = "Option::is_none")]
    pub msk: Option<String>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "mskLifetime", skip_serializing_if = "Option::is_none")]
    pub msk_lifetime: Option<String>,
    /// string with format 'bytes' as defined in OpenAPI
    #[serde(rename = "mtkId", skip_serializing_if = "Option::is_none")]
    pub mtk_id: Option<String>,
    /// string with format 'bytes' as defined in OpenAPI
    #[serde(rename = "mtk", skip_serializing_if = "Option::is_none")]
    pub mtk: Option<String>,
}

impl MbsKeyInfo {
    /// MBS Security Key Data Structure
    pub fn new(key_domain_id: String, msk_id: String) -> MbsKeyInfo {
        MbsKeyInfo {
            key_domain_id,
            msk_id,
            msk: None,
            msk_lifetime: None,
            mtk_id: None,
            mtk: None,
        }
    }
}


