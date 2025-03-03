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


/// struct for typed errors of method [`get_payee_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPayeeByIdError {
    Status404(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_payees`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPayeesError {
    Status404(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_payee`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePayeeError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Returns a single payee
pub async fn get_payee_by_id(configuration: &configuration::Configuration, budget_id: &str, payee_id: &str) -> Result<models::PayeeResponse, Error<GetPayeeByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_budget_id = budget_id;
    let p_payee_id = payee_id;

    let uri_str = format!("{}/budgets/{budget_id}/payees/{payee_id}", configuration.base_path, budget_id=crate::apis::urlencode(p_budget_id), payee_id=crate::apis::urlencode(p_payee_id));
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
        let entity: Option<GetPayeeByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns all payees
pub async fn get_payees(configuration: &configuration::Configuration, budget_id: &str, last_knowledge_of_server: Option<i64>) -> Result<models::PayeesResponse, Error<GetPayeesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_budget_id = budget_id;
    let p_last_knowledge_of_server = last_knowledge_of_server;

    let uri_str = format!("{}/budgets/{budget_id}/payees", configuration.base_path, budget_id=crate::apis::urlencode(p_budget_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_last_knowledge_of_server {
        req_builder = req_builder.query(&[("last_knowledge_of_server", &param_value.to_string())]);
    }
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
        let entity: Option<GetPayeesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update a payee
pub async fn update_payee(configuration: &configuration::Configuration, budget_id: &str, payee_id: &str, data: models::PatchPayeeWrapper) -> Result<models::SavePayeeResponse, Error<UpdatePayeeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_budget_id = budget_id;
    let p_payee_id = payee_id;
    let p_data = data;

    let uri_str = format!("{}/budgets/{budget_id}/payees/{payee_id}", configuration.base_path, budget_id=crate::apis::urlencode(p_budget_id), payee_id=crate::apis::urlencode(p_payee_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_data);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdatePayeeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

