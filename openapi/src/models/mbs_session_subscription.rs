/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MbsSessionSubscription : MBS session subscription



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MbsSessionSubscription {
    #[serde(rename = "mbsSessionId", skip_serializing_if = "Option::is_none")]
    pub mbs_session_id: Option<Box<crate::models::MbsSessionId>>,
    /// Integer where the allowed values correspond to the value range of an unsigned 16-bit integer.
    #[serde(rename = "areaSessionId", skip_serializing_if = "Option::is_none")]
    pub area_session_id: Option<i32>,
    #[serde(rename = "eventList")]
    pub event_list: Vec<crate::models::MbsSessionEvent>,
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "notifyUri")]
    pub notify_uri: String,
    #[serde(rename = "notifyCorrelationId", skip_serializing_if = "Option::is_none")]
    pub notify_correlation_id: Option<String>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "expiryTime", skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
    /// String uniquely identifying a NF instance. The format of the NF Instance ID shall be a Universally Unique Identifier (UUID) version 4, as described in IETF RFC 4122. 
    #[serde(rename = "nfcInstanceId", skip_serializing_if = "Option::is_none")]
    pub nfc_instance_id: Option<uuid::Uuid>,
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "mbsSessionSubscUri", skip_serializing_if = "Option::is_none")]
    pub mbs_session_subsc_uri: Option<String>,
}

impl MbsSessionSubscription {
    /// MBS session subscription
    pub fn new(event_list: Vec<crate::models::MbsSessionEvent>, notify_uri: String) -> MbsSessionSubscription {
        MbsSessionSubscription {
            mbs_session_id: None,
            area_session_id: None,
            event_list,
            notify_uri,
            notify_correlation_id: None,
            expiry_time: None,
            nfc_instance_id: None,
            mbs_session_subsc_uri: None,
        }
    }
}

