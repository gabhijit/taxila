/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NcgiTai : List of NR cell ids, with their pertaining TAIs



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NcgiTai {
    #[serde(rename = "tai")]
    pub tai: Box<crate::models::Tai>,
    /// List of List of NR cell ids
    #[serde(rename = "cellList")]
    pub cell_list: Vec<crate::models::Ncgi>,
}

impl NcgiTai {
    /// List of NR cell ids, with their pertaining TAIs
    pub fn new(tai: crate::models::Tai, cell_list: Vec<crate::models::Ncgi>) -> NcgiTai {
        NcgiTai {
            tai: Box::new(tai),
            cell_list,
        }
    }
}


