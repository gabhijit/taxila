/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AcsInfoRm : This data type is defined in the same way as the 'AcsInfo' data type, but with the OpenAPI 'nullable: true' property. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AcsInfoRm {
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "acsUrl", skip_serializing_if = "Option::is_none")]
    pub acs_url: Option<String>,
    /// String identifying a IPv4 address formatted in the 'dotted decimal' notation as defined in RFC 1166. 
    #[serde(rename = "acsIpv4Addr", skip_serializing_if = "Option::is_none")]
    pub acs_ipv4_addr: Option<String>,
    #[serde(rename = "acsIpv6Addr", skip_serializing_if = "Option::is_none")]
    pub acs_ipv6_addr: Option<Box<crate::models::Ipv6Addr>>,
}

impl AcsInfoRm {
    /// This data type is defined in the same way as the 'AcsInfo' data type, but with the OpenAPI 'nullable: true' property. 
    pub fn new() -> AcsInfoRm {
        AcsInfoRm {
            acs_url: None,
            acs_ipv4_addr: None,
            acs_ipv6_addr: None,
        }
    }
}

