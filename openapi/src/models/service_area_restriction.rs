/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServiceAreaRestriction : Provides information about allowed or not allowed areas.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceAreaRestriction {
    #[serde(rename = "restrictionType", skip_serializing_if = "Option::is_none")]
    pub restriction_type: Option<crate::models::RestrictionType>,
    #[serde(rename = "areas", skip_serializing_if = "Option::is_none")]
    pub areas: Option<Vec<crate::models::Area>>,
    /// Unsigned Integer, i.e. only value 0 and integers above 0 are permissible.
    #[serde(rename = "maxNumOfTAs", skip_serializing_if = "Option::is_none")]
    pub max_num_of_tas: Option<i32>,
    /// Unsigned Integer, i.e. only value 0 and integers above 0 are permissible.
    #[serde(rename = "maxNumOfTAsForNotAllowedAreas", skip_serializing_if = "Option::is_none")]
    pub max_num_of_tas_for_not_allowed_areas: Option<i32>,
}

impl ServiceAreaRestriction {
    /// Provides information about allowed or not allowed areas.
    pub fn new() -> ServiceAreaRestriction {
        ServiceAreaRestriction {
            restriction_type: None,
            areas: None,
            max_num_of_tas: None,
            max_num_of_tas_for_not_allowed_areas: None,
        }
    }
}

