/*
 * AUSF API
 *
 * AUSF UE Authentication Service. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UeAuthenticationCtx5gAuthData {
    #[serde(rename = "rand")]
    pub rand: String,
    /// Contains the HXRES*.
    #[serde(rename = "hxresStar")]
    pub hxres_star: String,
    #[serde(rename = "autn")]
    pub autn: String,
}

impl UeAuthenticationCtx5gAuthData {
    pub fn new(rand: String, hxres_star: String, autn: String) -> UeAuthenticationCtx5gAuthData {
        UeAuthenticationCtx5gAuthData {
            rand,
            hxres_star,
            autn,
        }
    }
}


