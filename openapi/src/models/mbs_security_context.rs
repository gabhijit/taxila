/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MbsSecurityContext {
    /// A map (list of key-value pairs) where a (unique) valid JSON string serves as key of MbsSecurityContext
    #[serde(rename = "keyList")]
    pub key_list: ::std::collections::HashMap<String, crate::models::MbsKeyInfo>,
}

impl MbsSecurityContext {
    pub fn new(key_list: ::std::collections::HashMap<String, crate::models::MbsKeyInfo>) -> MbsSecurityContext {
        MbsSecurityContext {
            key_list,
        }
    }
}


