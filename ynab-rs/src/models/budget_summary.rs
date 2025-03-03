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
pub struct BudgetSummary {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// The last time any changes were made to the budget from either a web or mobile client
    #[serde(rename = "last_modified_on", skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<String>,
    /// The earliest budget month
    #[serde(rename = "first_month", skip_serializing_if = "Option::is_none")]
    pub first_month: Option<String>,
    /// The latest budget month
    #[serde(rename = "last_month", skip_serializing_if = "Option::is_none")]
    pub last_month: Option<String>,
    #[serde(rename = "date_format", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_format: Option<Option<Box<models::DateFormat>>>,
    #[serde(rename = "currency_format", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub currency_format: Option<Option<Box<models::CurrencyFormat>>>,
    /// The budget accounts (only included if `include_accounts=true` specified as query parameter)
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<models::Account>>,
}

impl BudgetSummary {
    pub fn new(id: uuid::Uuid, name: String) -> BudgetSummary {
        BudgetSummary {
            id,
            name,
            last_modified_on: None,
            first_month: None,
            last_month: None,
            date_format: None,
            currency_format: None,
            accounts: None,
        }
    }
}

