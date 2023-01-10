/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SacInfo : Represents threshold(s) to control the triggering of network slice reporting notifications or the information contained in the network slice reporting notification. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SacInfo {
    #[serde(rename = "numericValNumUes", skip_serializing_if = "Option::is_none")]
    pub numeric_val_num_ues: Option<i32>,
    #[serde(rename = "numericValNumPduSess", skip_serializing_if = "Option::is_none")]
    pub numeric_val_num_pdu_sess: Option<i32>,
    #[serde(rename = "percValueNumUes", skip_serializing_if = "Option::is_none")]
    pub perc_value_num_ues: Option<i32>,
    #[serde(rename = "percValueNumPduSess", skip_serializing_if = "Option::is_none")]
    pub perc_value_num_pdu_sess: Option<i32>,
}

impl SacInfo {
    /// Represents threshold(s) to control the triggering of network slice reporting notifications or the information contained in the network slice reporting notification. 
    pub fn new() -> SacInfo {
        SacInfo {
            numeric_val_num_ues: None,
            numeric_val_num_pdu_sess: None,
            perc_value_num_ues: None,
            perc_value_num_pdu_sess: None,
        }
    }
}


