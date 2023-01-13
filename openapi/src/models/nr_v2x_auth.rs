/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NrV2xAuth : Contains NR V2X services authorized information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NrV2xAuth {
    #[serde(rename = "vehicleUeAuth", skip_serializing_if = "Option::is_none")]
    pub vehicle_ue_auth: Option<crate::models::UeAuth>,
    #[serde(rename = "pedestrianUeAuth", skip_serializing_if = "Option::is_none")]
    pub pedestrian_ue_auth: Option<crate::models::UeAuth>,
}

impl NrV2xAuth {
    /// Contains NR V2X services authorized information.
    pub fn new() -> NrV2xAuth {
        NrV2xAuth {
            vehicle_ue_auth: None,
            pedestrian_ue_auth: None,
        }
    }
}

