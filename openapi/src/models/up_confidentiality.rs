/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpConfidentiality : indicates whether UP confidentiality protection is required, preferred or not needed for all the traffic on the PDU Session. It shall comply with the provisions defined in table 5.4.3.5-1. 

/// indicates whether UP confidentiality protection is required, preferred or not needed for all the traffic on the PDU Session. It shall comply with the provisions defined in table 5.4.3.5-1. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UpConfidentiality {
    #[serde(rename = "REQUIRED")]
    Required,
    #[serde(rename = "PREFERRED")]
    Preferred,
    #[serde(rename = "NOT_NEEDED")]
    NotNeeded,

}

impl ToString for UpConfidentiality {
    fn to_string(&self) -> String {
        match self {
            Self::Required => String::from("REQUIRED"),
            Self::Preferred => String::from("PREFERRED"),
            Self::NotNeeded => String::from("NOT_NEEDED"),
        }
    }
}

impl Default for UpConfidentiality {
    fn default() -> UpConfidentiality {
        Self::Required
    }
}




