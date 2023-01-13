/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchItem : it contains information on data to be changed.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PatchItem {
    #[serde(rename = "op")]
    pub op: crate::models::PatchOperation,
    /// contains a JSON pointer value (as defined in IETF RFC 6901) that references a location of a resource on which the patch operation shall be performed. 
    #[serde(rename = "path")]
    pub path: String,
    /// indicates the path of the source JSON element (according to JSON Pointer syntax) being moved or copied to the location indicated by the \"path\" attribute. 
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<serde_json::Value>>,
}

impl PatchItem {
    /// it contains information on data to be changed.
    pub fn new(op: crate::models::PatchOperation, path: String) -> PatchItem {
        PatchItem {
            op,
            path,
            from: None,
            value: None,
        }
    }
}

