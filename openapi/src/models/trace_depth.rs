/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TraceDepth : The enumeration TraceDepth defines how detailed information should be recorded in the trace. See 3GPP TS 32.422 for further description of the values. It shall comply with the provisions defined in table 5.6.3.1-1 

/// The enumeration TraceDepth defines how detailed information should be recorded in the trace. See 3GPP TS 32.422 for further description of the values. It shall comply with the provisions defined in table 5.6.3.1-1 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TraceDepth {
    #[serde(rename = "MINIMUM")]
    Minimum,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "MAXIMUM")]
    Maximum,
    #[serde(rename = "MINIMUM_WO_VENDOR_EXTENSION")]
    MinimumWoVendorExtension,
    #[serde(rename = "MEDIUM_WO_VENDOR_EXTENSION")]
    MediumWoVendorExtension,
    #[serde(rename = "MAXIMUM_WO_VENDOR_EXTENSION")]
    MaximumWoVendorExtension,

}

impl ToString for TraceDepth {
    fn to_string(&self) -> String {
        match self {
            Self::Minimum => String::from("MINIMUM"),
            Self::Medium => String::from("MEDIUM"),
            Self::Maximum => String::from("MAXIMUM"),
            Self::MinimumWoVendorExtension => String::from("MINIMUM_WO_VENDOR_EXTENSION"),
            Self::MediumWoVendorExtension => String::from("MEDIUM_WO_VENDOR_EXTENSION"),
            Self::MaximumWoVendorExtension => String::from("MAXIMUM_WO_VENDOR_EXTENSION"),
        }
    }
}

impl Default for TraceDepth {
    fn default() -> TraceDepth {
        Self::Minimum
    }
}




