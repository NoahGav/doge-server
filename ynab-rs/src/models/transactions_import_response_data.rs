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
pub struct TransactionsImportResponseData {
    /// The list of transaction ids that were imported.
    #[serde(rename = "transaction_ids")]
    pub transaction_ids: Vec<String>,
}

impl TransactionsImportResponseData {
    pub fn new(transaction_ids: Vec<String>) -> TransactionsImportResponseData {
        TransactionsImportResponseData {
            transaction_ids,
        }
    }
}

