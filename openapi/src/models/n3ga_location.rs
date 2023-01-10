/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// N3gaLocation : Contains the Non-3GPP access user location.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct N3gaLocation {
    #[serde(rename = "n3gppTai", skip_serializing_if = "Option::is_none")]
    pub n3gpp_tai: Option<Box<crate::models::Tai>>,
    /// This IE shall contain the N3IWF identifier received over NGAP and shall be encoded as a string of hexadecimal characters. Each character in the string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\" and shall represent 4 bits. The most significant character representing the 4 most significant bits of the N3IWF ID shall appear first in the string, and the character representing the 4 least significant bit of the N3IWF ID shall appear last in the string. 
    #[serde(rename = "n3IwfId", skip_serializing_if = "Option::is_none")]
    pub n3_iwf_id: Option<String>,
    /// String identifying a IPv4 address formatted in the 'dotted decimal' notation as defined in RFC 1166. 
    #[serde(rename = "ueIpv4Addr", skip_serializing_if = "Option::is_none")]
    pub ue_ipv4_addr: Option<String>,
    #[serde(rename = "ueIpv6Addr", skip_serializing_if = "Option::is_none")]
    pub ue_ipv6_addr: Option<Box<crate::models::Ipv6Addr>>,
    /// Unsigned Integer, i.e. only value 0 and integers above 0 are permissible.
    #[serde(rename = "portNumber", skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i32>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<crate::models::TransportProtocol>,
    #[serde(rename = "tnapId", skip_serializing_if = "Option::is_none")]
    pub tnap_id: Option<Box<crate::models::TnapId>>,
    #[serde(rename = "twapId", skip_serializing_if = "Option::is_none")]
    pub twap_id: Option<Box<crate::models::TwapId>>,
    #[serde(rename = "hfcNodeId", skip_serializing_if = "Option::is_none")]
    pub hfc_node_id: Option<Box<crate::models::HfcNodeId>>,
    /// string with format 'bytes' as defined in OpenAPI
    #[serde(rename = "gli", skip_serializing_if = "Option::is_none")]
    pub gli: Option<String>,
    #[serde(rename = "w5gbanLineType", skip_serializing_if = "Option::is_none")]
    pub w5gban_line_type: Option<crate::models::LineType>,
    /// Global Cable Identifier uniquely identifying the connection between the 5G-CRG or FN-CRG to the 5GS. See clause 28.15.4 of 3GPP TS 23.003. This shall be encoded as a string per clause 28.15.4 of 3GPP TS 23.003, and compliant with the syntax specified  in clause 2.2 of IETF RFC 7542 for the username part of a NAI. The GCI value is specified in CableLabs WR-TR-5WWC-ARCH. 
    #[serde(rename = "gci", skip_serializing_if = "Option::is_none")]
    pub gci: Option<String>,
}

impl N3gaLocation {
    /// Contains the Non-3GPP access user location.
    pub fn new() -> N3gaLocation {
        N3gaLocation {
            n3gpp_tai: None,
            n3_iwf_id: None,
            ue_ipv4_addr: None,
            ue_ipv6_addr: None,
            port_number: None,
            protocol: None,
            tnap_id: None,
            twap_id: None,
            hfc_node_id: None,
            gli: None,
            w5gban_line_type: None,
            gci: None,
        }
    }
}


