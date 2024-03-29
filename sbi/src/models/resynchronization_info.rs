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
pub struct ResynchronizationInfo {
    #[serde(rename = "rand")]
    pub rand: String,
    #[serde(rename = "auts")]
    pub auts: String,
}

impl ResynchronizationInfo {
    pub fn new(rand: String, auts: String) -> ResynchronizationInfo {
        ResynchronizationInfo {
            rand,
            auts,
        }
    }
}


