/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Arp : Contains Allocation and Retention Priority information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Arp {
    /// nullable true shall not be used for this attribute. Unsigned integer indicating the ARP Priority Level (see clause 5.7.2.2 of 3GPP TS 23.501, within the range 1 to 15.Values are ordered in decreasing order of priority, i.e. with 1 as the highest priority and 15 as the lowest priority. 
    #[serde(rename = "priorityLevel", deserialize_with = "Option::deserialize")]
    pub priority_level: Option<i32>,
    #[serde(rename = "preemptCap")]
    pub preempt_cap: Box<crate::models::PreemptionCapability>,
    #[serde(rename = "preemptVuln")]
    pub preempt_vuln: Box<crate::models::PreemptionVulnerability>,
}

impl Arp {
    /// Contains Allocation and Retention Priority information.
    pub fn new(priority_level: Option<i32>, preempt_cap: crate::models::PreemptionCapability, preempt_vuln: crate::models::PreemptionVulnerability) -> Arp {
        Arp {
            priority_level,
            preempt_cap: Box::new(preempt_cap),
            preempt_vuln: Box::new(preempt_vuln),
        }
    }
}

