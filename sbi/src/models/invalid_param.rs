/*
 * AUSF API
 *
 * AUSF UE Authentication Service. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvalidParam : It contains an invalid parameter and a related description.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvalidParam {
    /// If the invalid parameter is an attribute in a JSON body, this IE shall contain the attribute's name and shall be encoded as a JSON Pointer. If the invalid parameter is an HTTP header, this IE shall be formatted as the concatenation of the string \"header \" plus the name of such header. If the invalid parameter is a query parameter, this IE shall be formatted as the concatenation of the string \"query \" plus the name of such query parameter. If the invalid parameter is a variable part in the path of a resource URI, this IE shall contain the name of the variable, including the symbols \"{\" and \"}\" used in OpenAPI specification as the notation to represent variable path segments. 
    #[serde(rename = "param")]
    pub param: String,
    /// A human-readable reason, e.g. \"must be a positive integer\". In cases involving failed operations in a PATCH request, the reason string should identify the operation that failed using the operation's array index to assist in correlation of the invalid parameter with the failed operation, e.g.\" Replacement value invalid for attribute (failed operation index= 4)\" 
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl InvalidParam {
    /// It contains an invalid parameter and a related description.
    pub fn new(param: String) -> InvalidParam {
        InvalidParam {
            param,
            reason: None,
        }
    }
}


