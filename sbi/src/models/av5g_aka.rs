/*
 * AUSF API
 *
 * AUSF UE Authentication Service. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Av5gAka : Contains Authentication Vector for method 5G AKA.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Av5gAka {
    #[serde(rename = "rand")]
    pub rand: String,
    /// Contains the HXRES*.
    #[serde(rename = "hxresStar")]
    pub hxres_star: String,
    #[serde(rename = "autn")]
    pub autn: String,
}

impl Av5gAka {
    /// Contains Authentication Vector for method 5G AKA.
    pub fn new(rand: String, hxres_star: String, autn: String) -> Av5gAka {
        Av5gAka {
            rand,
            hxres_star,
            autn,
        }
    }
}


