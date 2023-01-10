/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LoggingIntervalNrMdt : The enumeration LoggingIntervalNrMdt defines Logging Interval in NR for MDT in the trace. See 3GPP TS 32.422 for further description of the values. It shall comply with the provisions defined in table 5.6.3.18-1. 

/// The enumeration LoggingIntervalNrMdt defines Logging Interval in NR for MDT in the trace. See 3GPP TS 32.422 for further description of the values. It shall comply with the provisions defined in table 5.6.3.18-1. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LoggingIntervalNrMdt {
    #[serde(rename = "128")]
    Variant128,
    #[serde(rename = "256")]
    Variant256,
    #[serde(rename = "512")]
    Variant512,
    #[serde(rename = "1024")]
    Variant1024,
    #[serde(rename = "2048")]
    Variant2048,
    #[serde(rename = "3072")]
    Variant3072,
    #[serde(rename = "4096")]
    Variant4096,
    #[serde(rename = "6144")]
    Variant6144,
    #[serde(rename = "320")]
    Variant320,
    #[serde(rename = "640")]
    Variant640,
    #[serde(rename = "infinity")]
    Infinity,

}

impl ToString for LoggingIntervalNrMdt {
    fn to_string(&self) -> String {
        match self {
            Self::Variant128 => String::from("128"),
            Self::Variant256 => String::from("256"),
            Self::Variant512 => String::from("512"),
            Self::Variant1024 => String::from("1024"),
            Self::Variant2048 => String::from("2048"),
            Self::Variant3072 => String::from("3072"),
            Self::Variant4096 => String::from("4096"),
            Self::Variant6144 => String::from("6144"),
            Self::Variant320 => String::from("320"),
            Self::Variant640 => String::from("640"),
            Self::Infinity => String::from("infinity"),
        }
    }
}

impl Default for LoggingIntervalNrMdt {
    fn default() -> LoggingIntervalNrMdt {
        Self::Variant128
    }
}




