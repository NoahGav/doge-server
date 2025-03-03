/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.ynab.com
 *
 * The version of the OpenAPI document: 1.73.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ScheduledTransactionFrequency : The scheduled transaction frequency
/// The scheduled transaction frequency
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScheduledTransactionFrequency {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "everyOtherWeek")]
    EveryOtherWeek,
    #[serde(rename = "twiceAMonth")]
    TwiceAMonth,
    #[serde(rename = "every4Weeks")]
    Every4Weeks,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "everyOtherMonth")]
    EveryOtherMonth,
    #[serde(rename = "every3Months")]
    Every3Months,
    #[serde(rename = "every4Months")]
    Every4Months,
    #[serde(rename = "twiceAYear")]
    TwiceAYear,
    #[serde(rename = "yearly")]
    Yearly,
    #[serde(rename = "everyOtherYear")]
    EveryOtherYear,

}

impl std::fmt::Display for ScheduledTransactionFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Never => write!(f, "never"),
            Self::Daily => write!(f, "daily"),
            Self::Weekly => write!(f, "weekly"),
            Self::EveryOtherWeek => write!(f, "everyOtherWeek"),
            Self::TwiceAMonth => write!(f, "twiceAMonth"),
            Self::Every4Weeks => write!(f, "every4Weeks"),
            Self::Monthly => write!(f, "monthly"),
            Self::EveryOtherMonth => write!(f, "everyOtherMonth"),
            Self::Every3Months => write!(f, "every3Months"),
            Self::Every4Months => write!(f, "every4Months"),
            Self::TwiceAYear => write!(f, "twiceAYear"),
            Self::Yearly => write!(f, "yearly"),
            Self::EveryOtherYear => write!(f, "everyOtherYear"),
        }
    }
}

impl Default for ScheduledTransactionFrequency {
    fn default() -> ScheduledTransactionFrequency {
        Self::Never
    }
}

