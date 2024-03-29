/*
 * AUSF API
 *
 * AUSF UE Authentication Service. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RgAuthCtx : Contains the UE id (i.e. SUPI) and the authentication indication.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RgAuthCtx {
    #[serde(rename = "authResult")]
    pub auth_result: crate::models::AuthResult,
    /// String identifying a Supi that shall contain either an IMSI, a network specific identifier, a Global Cable Identifier (GCI) or a Global Line Identifier (GLI) as specified in clause 2.2A of 3GPP TS 23.003. It shall be formatted as follows  - for an IMSI \"imsi-<imsi>\", where <imsi> shall be formatted according to clause 2.2    of 3GPP TS 23.003 that describes an IMSI.  - for a network specific identifier \"nai-<nai>, where <nai> shall be formatted    according to clause 28.7.2 of 3GPP TS 23.003 that describes an NAI.  - for a GCI \"gci-<gci>\", where <gci> shall be formatted according to clause 28.15.2    of 3GPP TS 23.003.  - for a GLI \"gli-<gli>\", where <gli> shall be formatted according to clause 28.16.2 of    3GPP TS 23.003.To enable that the value is used as part of an URI, the string shall    only contain characters allowed according to the \"lower-with-hyphen\" naming convention    defined in 3GPP TS 29.501. 
    #[serde(rename = "supi", skip_serializing_if = "Option::is_none")]
    pub supi: Option<String>,
    #[serde(rename = "authInd", skip_serializing_if = "Option::is_none")]
    pub auth_ind: Option<bool>,
}

impl RgAuthCtx {
    /// Contains the UE id (i.e. SUPI) and the authentication indication.
    pub fn new(auth_result: crate::models::AuthResult) -> RgAuthCtx {
        RgAuthCtx {
            auth_result,
            supi: None,
            auth_ind: None,
        }
    }
}


