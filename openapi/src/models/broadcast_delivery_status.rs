/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BroadcastDeliveryStatus : Broadcast MBS Session's Delivery Status

/// Broadcast MBS Session's Delivery Status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BroadcastDeliveryStatus {
    #[serde(rename = "STARTED")]
    Started,
    #[serde(rename = "TERMINATED")]
    Terminated,

}

impl ToString for BroadcastDeliveryStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Started => String::from("STARTED"),
            Self::Terminated => String::from("TERMINATED"),
        }
    }
}

impl Default for BroadcastDeliveryStatus {
    fn default() -> BroadcastDeliveryStatus {
        Self::Started
    }
}



