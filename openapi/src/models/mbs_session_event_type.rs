/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MbsSessionEventType : MBS Session Event Type

/// MBS Session Event Type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MbsSessionEventType {
    #[serde(rename = "MBS_REL_TMGI_EXPIRY")]
    MbsRelTmgiExpiry,
    #[serde(rename = "BROADCAST_DELIVERY_STATUS")]
    BroadcastDeliveryStatus,
    #[serde(rename = "INGRESS_TUNNEL_ADD_CHANGE")]
    IngressTunnelAddChange,

}

impl ToString for MbsSessionEventType {
    fn to_string(&self) -> String {
        match self {
            Self::MbsRelTmgiExpiry => String::from("MBS_REL_TMGI_EXPIRY"),
            Self::BroadcastDeliveryStatus => String::from("BROADCAST_DELIVERY_STATUS"),
            Self::IngressTunnelAddChange => String::from("INGRESS_TUNNEL_ADD_CHANGE"),
        }
    }
}

impl Default for MbsSessionEventType {
    fn default() -> MbsSessionEventType {
        Self::MbsRelTmgiExpiry
    }
}



