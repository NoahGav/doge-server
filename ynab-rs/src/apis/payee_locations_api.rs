/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.ynab.com
 *
 * The version of the OpenAPI document: 1.73.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`get_payee_location_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPayeeLocationByIdError {
    Status404(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_payee_locations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPayeeLocationsError {
    Status404(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_payee_locations_by_payee`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPayeeLocationsByPayeeError {
    Status404(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Returns a single payee location
pub async fn get_payee_location_by_id(configuration: &configuration::Configuration, budget_id: &str, payee_location_id: &str) -> Result<models::PayeeLocationResponse, Error<GetPayeeLocationByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_budget_id = budget_id;
    let p_payee_location_id = payee_location_id;

    let uri_str = format!("{}/budgets/{budget_id}/payee_locations/{payee_location_id}", configuration.base_path, budget_id=crate::apis::urlencode(p_budget_id), payee_location_id=crate::apis::urlencode(p_payee_location_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPayeeLocationByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns all payee locations
pub async fn get_payee_locations(configuration: &configuration::Configuration, budget_id: &str) -> Result<models::PayeeLocationsResponse, Error<GetPayeeLocationsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_budget_id = budget_id;

    let uri_str = format!("{}/budgets/{budget_id}/payee_locations", configuration.base_path, budget_id=crate::apis::urlencode(p_budget_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPayeeLocationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns all payee locations for a specified payee
pub async fn get_payee_locations_by_payee(configuration: &configuration::Configuration, budget_id: &str, payee_id: &str) -> Result<models::PayeeLocationsResponse, Error<GetPayeeLocationsByPayeeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_budget_id = budget_id;
    let p_payee_id = payee_id;

    let uri_str = format!("{}/budgets/{budget_id}/payees/{payee_id}/payee_locations", configuration.base_path, budget_id=crate::apis::urlencode(p_budget_id), payee_id=crate::apis::urlencode(p_payee_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPayeeLocationsByPayeeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

