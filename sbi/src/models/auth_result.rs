/*
 * AUSF API
 *
 * AUSF UE Authentication Service. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AuthResult : Indicates the result of the authentication.

/// Indicates the result of the authentication.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthResult {
    #[serde(rename = "AUTHENTICATION_SUCCESS")]
    Success,
    #[serde(rename = "AUTHENTICATION_FAILURE")]
    Failure,
    #[serde(rename = "AUTHENTICATION_ONGOING")]
    Ongoing,

}

impl ToString for AuthResult {
    fn to_string(&self) -> String {
        match self {
            Self::Success => String::from("AUTHENTICATION_SUCCESS"),
            Self::Failure => String::from("AUTHENTICATION_FAILURE"),
            Self::Ongoing => String::from("AUTHENTICATION_ONGOING"),
        }
    }
}

impl Default for AuthResult {
    fn default() -> AuthResult {
        Self::Success
    }
}




