/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GeographicArea : Geographic area specified by different shape.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GeographicArea {
    #[serde(rename = "shape")]
    pub shape: Box<crate::models::SupportedGadShapes>,
    #[serde(rename = "point")]
    pub point: Box<crate::models::GeographicalCoordinates>,
    /// Indicates value of uncertainty.
    #[serde(rename = "uncertainty")]
    pub uncertainty: f32,
    #[serde(rename = "uncertaintyEllipse")]
    pub uncertainty_ellipse: Box<crate::models::UncertaintyEllipse>,
    /// Indicates value of confidence.
    #[serde(rename = "confidence")]
    pub confidence: i32,
    /// List of points.
    #[serde(rename = "pointList")]
    pub point_list: Vec<crate::models::GeographicalCoordinates>,
    /// Indicates value of altitude.
    #[serde(rename = "altitude")]
    pub altitude: f64,
    /// Indicates value of uncertainty.
    #[serde(rename = "uncertaintyAltitude")]
    pub uncertainty_altitude: f32,
    /// Indicates value of the inner radius.
    #[serde(rename = "innerRadius")]
    pub inner_radius: i32,
    /// Indicates value of uncertainty.
    #[serde(rename = "uncertaintyRadius")]
    pub uncertainty_radius: f32,
    /// Indicates value of angle.
    #[serde(rename = "offsetAngle")]
    pub offset_angle: i32,
    /// Indicates value of angle.
    #[serde(rename = "includedAngle")]
    pub included_angle: i32,
}

impl GeographicArea {
    /// Geographic area specified by different shape.
    pub fn new(shape: crate::models::SupportedGadShapes, point: crate::models::GeographicalCoordinates, uncertainty: f32, uncertainty_ellipse: crate::models::UncertaintyEllipse, confidence: i32, point_list: Vec<crate::models::GeographicalCoordinates>, altitude: f64, uncertainty_altitude: f32, inner_radius: i32, uncertainty_radius: f32, offset_angle: i32, included_angle: i32) -> GeographicArea {
        GeographicArea {
            shape: Box::new(shape),
            point: Box::new(point),
            uncertainty,
            uncertainty_ellipse: Box::new(uncertainty_ellipse),
            confidence,
            point_list,
            altitude,
            uncertainty_altitude,
            inner_radius,
            uncertainty_radius,
            offset_angle,
            included_angle,
        }
    }
}


