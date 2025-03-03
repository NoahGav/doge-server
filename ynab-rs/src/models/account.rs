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
pub struct Account {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: models::AccountType,
    /// Whether this account is on budget or not
    #[serde(rename = "on_budget")]
    pub on_budget: bool,
    /// Whether this account is closed or not
    #[serde(rename = "closed")]
    pub closed: bool,
    #[serde(rename = "note", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub note: Option<Option<String>>,
    /// The current balance of the account in milliunits format
    #[serde(rename = "balance")]
    pub balance: i64,
    /// The current cleared balance of the account in milliunits format
    #[serde(rename = "cleared_balance")]
    pub cleared_balance: i64,
    /// The current uncleared balance of the account in milliunits format
    #[serde(rename = "uncleared_balance")]
    pub uncleared_balance: i64,
    /// The payee id which should be used when transferring to this account
    #[serde(rename = "transfer_payee_id", deserialize_with = "Option::deserialize")]
    pub transfer_payee_id: Option<uuid::Uuid>,
    /// Whether or not the account is linked to a financial institution for automatic transaction import.
    #[serde(rename = "direct_import_linked", skip_serializing_if = "Option::is_none")]
    pub direct_import_linked: Option<bool>,
    /// If an account linked to a financial institution (direct_import_linked=true) and the linked connection is not in a healthy state, this will be true.
    #[serde(rename = "direct_import_in_error", skip_serializing_if = "Option::is_none")]
    pub direct_import_in_error: Option<bool>,
    /// A date/time specifying when the account was last reconciled.
    #[serde(rename = "last_reconciled_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_reconciled_at: Option<Option<String>>,
    /// The original debt/loan account balance, specified in milliunits format.
    #[serde(rename = "debt_original_balance", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub debt_original_balance: Option<Option<i64>>,
    #[serde(rename = "debt_interest_rates", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub debt_interest_rates: Option<Option<std::collections::HashMap<String, i64>>>,
    #[serde(rename = "debt_minimum_payments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub debt_minimum_payments: Option<Option<std::collections::HashMap<String, i64>>>,
    #[serde(rename = "debt_escrow_amounts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub debt_escrow_amounts: Option<Option<std::collections::HashMap<String, i64>>>,
    /// Whether or not the account has been deleted.  Deleted accounts will only be included in delta requests.
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl Account {
    pub fn new(id: uuid::Uuid, name: String, r#type: models::AccountType, on_budget: bool, closed: bool, balance: i64, cleared_balance: i64, uncleared_balance: i64, transfer_payee_id: Option<uuid::Uuid>, deleted: bool) -> Account {
        Account {
            id,
            name,
            r#type,
            on_budget,
            closed,
            note: None,
            balance,
            cleared_balance,
            uncleared_balance,
            transfer_payee_id,
            direct_import_linked: None,
            direct_import_in_error: None,
            last_reconciled_at: None,
            debt_original_balance: None,
            debt_interest_rates: None,
            debt_minimum_payments: None,
            debt_escrow_amounts: None,
            deleted,
        }
    }
}

