/*
 * AUSF API
 *
 * AUSF UE Authentication Service. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServerAddressingInfo : Contains addressing information (IP addresses and/or FQDNs) of a server.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerAddressingInfo {
    #[serde(rename = "ipv4Addresses", skip_serializing_if = "Option::is_none")]
    pub ipv4_addresses: Option<Vec<String>>,
    #[serde(rename = "ipv6Addresses", skip_serializing_if = "Option::is_none")]
    pub ipv6_addresses: Option<Vec<String>>,
    #[serde(rename = "fqdnList", skip_serializing_if = "Option::is_none")]
    pub fqdn_list: Option<Vec<String>>,
}

impl ServerAddressingInfo {
    /// Contains addressing information (IP addresses and/or FQDNs) of a server.
    pub fn new() -> ServerAddressingInfo {
        ServerAddressingInfo {
            ipv4_addresses: None,
            ipv6_addresses: None,
            fqdn_list: None,
        }
    }
}


