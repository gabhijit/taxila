/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinksValueSchema : A list of mutually exclusive alternatives of 1 or more links.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinksValueSchema {
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

impl LinksValueSchema {
    /// A list of mutually exclusive alternatives of 1 or more links.
    pub fn new() -> LinksValueSchema {
        LinksValueSchema {
            href: None,
        }
    }
}

