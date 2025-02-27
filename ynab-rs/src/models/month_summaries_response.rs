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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthSummariesResponse {
    #[serde(rename = "data")]
    pub data: Box<models::MonthSummariesResponseData>,
}

impl MonthSummariesResponse {
    pub fn new(data: models::MonthSummariesResponseData) -> MonthSummariesResponse {
        MonthSummariesResponse {
            data: Box::new(data),
        }
    }
}

