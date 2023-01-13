/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TrafficProfile : Possible values are: - SINGLE_TRANS_UL: Uplink single packet transmission. - SINGLE_TRANS_DL: Downlink single packet transmission. - DUAL_TRANS_UL_FIRST: Dual packet transmission, firstly uplink packet transmission   with subsequent downlink packet transmission. - DUAL_TRANS_DL_FIRST: Dual packet transmission, firstly downlink packet transmission   with subsequent uplink packet transmission. 

/// Possible values are: - SINGLE_TRANS_UL: Uplink single packet transmission. - SINGLE_TRANS_DL: Downlink single packet transmission. - DUAL_TRANS_UL_FIRST: Dual packet transmission, firstly uplink packet transmission   with subsequent downlink packet transmission. - DUAL_TRANS_DL_FIRST: Dual packet transmission, firstly downlink packet transmission   with subsequent uplink packet transmission. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrafficProfile {
    #[serde(rename = "SINGLE_TRANS_UL")]
    SingleTransUl,
    #[serde(rename = "SINGLE_TRANS_DL")]
    SingleTransDl,
    #[serde(rename = "DUAL_TRANS_UL_FIRST")]
    DualTransUlFirst,
    #[serde(rename = "DUAL_TRANS_DL_FIRST")]
    DualTransDlFirst,
    #[serde(rename = "MULTI_TRANS")]
    MultiTrans,

}

impl ToString for TrafficProfile {
    fn to_string(&self) -> String {
        match self {
            Self::SingleTransUl => String::from("SINGLE_TRANS_UL"),
            Self::SingleTransDl => String::from("SINGLE_TRANS_DL"),
            Self::DualTransUlFirst => String::from("DUAL_TRANS_UL_FIRST"),
            Self::DualTransDlFirst => String::from("DUAL_TRANS_DL_FIRST"),
            Self::MultiTrans => String::from("MULTI_TRANS"),
        }
    }
}

impl Default for TrafficProfile {
    fn default() -> TrafficProfile {
        Self::SingleTransUl
    }
}



