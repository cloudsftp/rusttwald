/*
 * Mittwald API
 *
 * ## Introduction  This OpenAPI spec documents the mittwald API. It follows the [OpenAPI 3.0.0 specification](https://spec.openapis.org/oas/v3.0.0.html).  ## Authentication  You will need an API token to access the API. You can obtain one by logging into the [mStudio](https://studio.mittwald.de) and navigating to the [\"API Tokens\" section in the user menu](https://studio.mittwald.de/app/profile/api-tokens).  When making requests to the API, you can authenticate by passing your API token in the `X-Access-Token` header or as a bearer token.  ## Rate Limiting  Please note that usage of the API is rate-limited to prevent abuse. You can inspect the rate limiting for your current user by observing the `X-Ratelimit-*` headers included in each response.  ## mStudio  A main consumer of the mittwald API is the management interface for our customers, the [mStudio](https://studio.mittwald.de).  ## Contact and support  For support, please use the [mStudio support area](https://studio.mittwald.de/app/support/conversations) or drop us an email at [support@mittwald.de](mailto:support@mittwald.de).  For security issues, please report to [security@mittwald.de](mailto:security@mittwald.de). 
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`cronjob_abort_execution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronjobAbortExecutionError {
    Status404(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cronjob_create_cronjob`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronjobCreateCronjobError {
    Status400(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodValidationErrors),
    Status412(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cronjob_create_execution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronjobCreateExecutionError {
    Status404(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status412(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cronjob_delete_cronjob`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronjobDeleteCronjobError {
    Status400(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodValidationErrors),
    Status412(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cronjob_get_cronjob`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronjobGetCronjobError {
    Status404(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cronjob_get_execution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronjobGetExecutionError {
    Status404(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cronjob_list_cronjobs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronjobListCronjobsError {
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cronjob_list_executions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronjobListExecutionsError {
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cronjob_update_cronjob`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronjobUpdateCronjobError {
    Status400(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodValidationErrors),
    Status404(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status412(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cronjob_update_cronjob_app_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronjobUpdateCronjobAppIdError {
    Status400(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodValidationErrors),
    Status412(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}


pub async fn cronjob_abort_execution(configuration: &configuration::Configuration, cronjob_id: &str, execution_id: &str, body: Option<serde_json::Value>) -> Result<(), Error<CronjobAbortExecutionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/cronjobs/{cronjobId}/executions/{executionId}/actions/abort", local_var_configuration.base_path, cronjobId=crate::apis::urlencode(cronjob_id), executionId=crate::apis::urlencode(execution_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CronjobAbortExecutionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cronjob_create_cronjob(configuration: &configuration::Configuration, project_id: &str, de_period_mittwald_period_v1_period_cronjob_period_cronjob_request: Option<models::DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjobRequest>) -> Result<models::AppRequestAppinstallation201Response, Error<CronjobCreateCronjobError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/projects/{projectId}/cronjobs", local_var_configuration.base_path, projectId=crate::apis::urlencode(project_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&de_period_mittwald_period_v1_period_cronjob_period_cronjob_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CronjobCreateCronjobError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cronjob_create_execution(configuration: &configuration::Configuration, cronjob_id: &str, body: Option<serde_json::Value>) -> Result<models::CronjobCreateExecution201Response, Error<CronjobCreateExecutionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/cronjobs/{cronjobId}/executions", local_var_configuration.base_path, cronjobId=crate::apis::urlencode(cronjob_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CronjobCreateExecutionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cronjob_delete_cronjob(configuration: &configuration::Configuration, cronjob_id: &str) -> Result<(), Error<CronjobDeleteCronjobError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/cronjobs/{cronjobId}", local_var_configuration.base_path, cronjobId=crate::apis::urlencode(cronjob_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CronjobDeleteCronjobError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cronjob_get_cronjob(configuration: &configuration::Configuration, cronjob_id: &str) -> Result<models::DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjob, Error<CronjobGetCronjobError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/cronjobs/{cronjobId}", local_var_configuration.base_path, cronjobId=crate::apis::urlencode(cronjob_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CronjobGetCronjobError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cronjob_get_execution(configuration: &configuration::Configuration, execution_id: &str, cronjob_id: &str) -> Result<models::DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjobExecution, Error<CronjobGetExecutionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/cronjobs/{cronjobId}/executions/{executionId}", local_var_configuration.base_path, executionId=crate::apis::urlencode(execution_id), cronjobId=crate::apis::urlencode(cronjob_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CronjobGetExecutionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cronjob_list_cronjobs(configuration: &configuration::Configuration, project_id: &str, limit: Option<i32>, skip: Option<i32>, page: Option<i32>) -> Result<Vec<models::DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjob>, Error<CronjobListCronjobsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/projects/{projectId}/cronjobs", local_var_configuration.base_path, projectId=crate::apis::urlencode(project_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = skip {
        local_var_req_builder = local_var_req_builder.query(&[("skip", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CronjobListCronjobsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cronjob_list_executions(configuration: &configuration::Configuration, cronjob_id: &str, limit: Option<i32>, skip: Option<i32>, page: Option<i32>, since: Option<String>, until: Option<String>, status: Option<&str>) -> Result<Vec<models::DePeriodMittwaldPeriodV1PeriodCronjobPeriodCronjobExecution>, Error<CronjobListExecutionsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/cronjobs/{cronjobId}/executions", local_var_configuration.base_path, cronjobId=crate::apis::urlencode(cronjob_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = skip {
        local_var_req_builder = local_var_req_builder.query(&[("skip", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = since {
        local_var_req_builder = local_var_req_builder.query(&[("since", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = until {
        local_var_req_builder = local_var_req_builder.query(&[("until", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CronjobListExecutionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cronjob_update_cronjob(configuration: &configuration::Configuration, cronjob_id: &str, cronjob_update_cronjob_request: Option<models::CronjobUpdateCronjobRequest>) -> Result<(), Error<CronjobUpdateCronjobError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/cronjobs/{cronjobId}", local_var_configuration.base_path, cronjobId=crate::apis::urlencode(cronjob_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&cronjob_update_cronjob_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CronjobUpdateCronjobError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cronjob_update_cronjob_app_id(configuration: &configuration::Configuration, cronjob_id: &str, cronjob_update_cronjob_app_id_request: Option<models::CronjobUpdateCronjobAppIdRequest>) -> Result<(), Error<CronjobUpdateCronjobAppIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/cronjobs/{cronjobId}/app-id", local_var_configuration.base_path, cronjobId=crate::apis::urlencode(cronjob_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&cronjob_update_cronjob_app_id_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CronjobUpdateCronjobAppIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

