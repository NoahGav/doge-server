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
pub struct CategoryResponseData {
    #[serde(rename = "category")]
    pub category: Box<models::Category>,
}

impl CategoryResponseData {
    pub fn new(category: models::Category) -> CategoryResponseData {
        CategoryResponseData {
            category: Box::new(category),
        }
    }
}

