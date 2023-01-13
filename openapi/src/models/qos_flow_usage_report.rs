/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// QosFlowUsageReport : Contains QoS flows usage data information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct QosFlowUsageReport {
    /// Unsigned integer identifying a QoS flow, within the range 0 to 63.
    #[serde(rename = "qfi")]
    pub qfi: i32,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "startTimeStamp")]
    pub start_time_stamp: String,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "endTimeStamp")]
    pub end_time_stamp: String,
    /// string with format 'int64' as defined in OpenAPI.
    #[serde(rename = "downlinkVolume")]
    pub downlink_volume: i64,
    /// string with format 'int64' as defined in OpenAPI.
    #[serde(rename = "uplinkVolume")]
    pub uplink_volume: i64,
}

impl QosFlowUsageReport {
    /// Contains QoS flows usage data information.
    pub fn new(qfi: i32, start_time_stamp: String, end_time_stamp: String, downlink_volume: i64, uplink_volume: i64) -> QosFlowUsageReport {
        QosFlowUsageReport {
            qfi,
            start_time_stamp,
            end_time_stamp,
            downlink_volume,
            uplink_volume,
        }
    }
}

