/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MoExpDataCounter : Contain the MO Exception Data Counter.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MoExpDataCounter {
    /// Unsigned integer identifying the MO Exception Data Counter, as specified in clause 5.31.14.3 of 3GPP TS 23.501. 
    #[serde(rename = "counter")]
    pub counter: i32,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "timeStamp", skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl MoExpDataCounter {
    /// Contain the MO Exception Data Counter.
    pub fn new(counter: i32) -> MoExpDataCounter {
        MoExpDataCounter {
            counter,
            time_stamp: None,
        }
    }
}


