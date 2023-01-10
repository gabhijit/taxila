/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MbsMediaComp : Represents an MBS Media Component.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MbsMediaComp {
    #[serde(rename = "mbsMedCompNum")]
    pub mbs_med_comp_num: i32,
    #[serde(rename = "mbsFlowDescs", skip_serializing_if = "Option::is_none")]
    pub mbs_flow_descs: Option<Vec<String>>,
    #[serde(rename = "mbsSdfResPrio", skip_serializing_if = "Option::is_none")]
    pub mbs_sdf_res_prio: Option<Box<crate::models::ReservPriority>>,
    #[serde(rename = "mbsMediaInfo", skip_serializing_if = "Option::is_none")]
    pub mbs_media_info: Option<Box<crate::models::MbsMediaInfo>>,
    #[serde(rename = "qosRef", skip_serializing_if = "Option::is_none")]
    pub qos_ref: Option<String>,
    #[serde(rename = "mbsQoSReq", skip_serializing_if = "Option::is_none")]
    pub mbs_qo_s_req: Option<Box<crate::models::MbsQoSReq>>,
}

impl MbsMediaComp {
    /// Represents an MBS Media Component.
    pub fn new(mbs_med_comp_num: i32) -> MbsMediaComp {
        MbsMediaComp {
            mbs_med_comp_num,
            mbs_flow_descs: None,
            mbs_sdf_res_prio: None,
            mbs_media_info: None,
            qos_ref: None,
            mbs_qo_s_req: None,
        }
    }
}


