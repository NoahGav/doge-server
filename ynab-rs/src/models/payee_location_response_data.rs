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
pub struct PayeeLocationResponseData {
    #[serde(rename = "payee_location")]
    pub payee_location: Box<models::PayeeLocation>,
}

impl PayeeLocationResponseData {
    pub fn new(payee_location: models::PayeeLocation) -> PayeeLocationResponseData {
        PayeeLocationResponseData {
            payee_location: Box::new(payee_location),
        }
    }
}

