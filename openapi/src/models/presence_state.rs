/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PresenceState : Possible values are: -IN_AREA: Indicates that the UE is inside or enters the presence reporting area. -OUT_OF_AREA: Indicates that the UE is outside or leaves the presence reporting area -UNKNOW: Indicates it is unknown whether the UE is in the presence reporting area or not -INACTIVE: Indicates that the presence reporting area is inactive in the serving node. 

/// Possible values are: -IN_AREA: Indicates that the UE is inside or enters the presence reporting area. -OUT_OF_AREA: Indicates that the UE is outside or leaves the presence reporting area -UNKNOW: Indicates it is unknown whether the UE is in the presence reporting area or not -INACTIVE: Indicates that the presence reporting area is inactive in the serving node. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PresenceState {
    #[serde(rename = "IN_AREA")]
    InArea,
    #[serde(rename = "OUT_OF_AREA")]
    OutOfArea,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "INACTIVE")]
    Inactive,

}

impl ToString for PresenceState {
    fn to_string(&self) -> String {
        match self {
            Self::InArea => String::from("IN_AREA"),
            Self::OutOfArea => String::from("OUT_OF_AREA"),
            Self::Unknown => String::from("UNKNOWN"),
            Self::Inactive => String::from("INACTIVE"),
        }
    }
}

impl Default for PresenceState {
    fn default() -> PresenceState {
        Self::InArea
    }
}



