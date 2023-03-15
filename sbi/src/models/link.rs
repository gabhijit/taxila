/*
 * AUSF API
 *
 * AUSF UE Authentication Service. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Link : It contains the URI of the linked resource.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Link {
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

impl Link {
    /// It contains the URI of the linked resource.
    pub fn new() -> Link {
        Link {
            href: None,
        }
    }
}


