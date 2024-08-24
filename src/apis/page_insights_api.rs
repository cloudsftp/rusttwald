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


/// struct for typed errors of method [`pageinsights_get_performance_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PageinsightsGetPerformanceDataError {
    Status400(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodValidationErrors),
    Status403(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`pageinsights_get_strace_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PageinsightsGetStraceDataError {
    Status400(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodValidationErrors),
    Status403(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`pageinsights_list_performance_data_for_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PageinsightsListPerformanceDataForProjectError {
    Status400(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodValidationErrors),
    Status403(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`pageinsights_schedule_strace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PageinsightsScheduleStraceError {
    Status400(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodValidationErrors),
    Status403(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    Status429(models::AppExecuteAction429Response),
    DefaultResponse(models::DePeriodMittwaldPeriodV1PeriodCommonsPeriodError),
    UnknownValue(serde_json::Value),
}


pub async fn pageinsights_get_performance_data(configuration: &configuration::Configuration, domain: &str, path: &str, date: Option<String>) -> Result<models::PageinsightsGetPerformanceData200Response, Error<PageinsightsGetPerformanceDataError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/page-insights", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("domain", &domain.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("path", &path.to_string())]);
    if let Some(ref local_var_str) = date {
        local_var_req_builder = local_var_req_builder.query(&[("date", &local_var_str.to_string())]);
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
        let local_var_entity: Option<PageinsightsGetPerformanceDataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn pageinsights_get_strace_data(configuration: &configuration::Configuration, strace_id: &str, project_id: &str) -> Result<models::PageinsightsGetStraceData200Response, Error<PageinsightsGetStraceDataError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/projects/{projectId}/straces/{straceId}", local_var_configuration.base_path, straceId=crate::apis::urlencode(strace_id), projectId=crate::apis::urlencode(project_id));
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
        let local_var_entity: Option<PageinsightsGetStraceDataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn pageinsights_list_performance_data_for_project(configuration: &configuration::Configuration, project_id: &str, domain: Option<&str>) -> Result<Vec<models::PageinsightsListPerformanceDataForProject200ResponseInner>, Error<PageinsightsListPerformanceDataForProjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/projects/{projectId}/page-insights", local_var_configuration.base_path, projectId=crate::apis::urlencode(project_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = domain {
        local_var_req_builder = local_var_req_builder.query(&[("domain", &local_var_str.to_string())]);
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
        let local_var_entity: Option<PageinsightsListPerformanceDataForProjectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn pageinsights_schedule_strace(configuration: &configuration::Configuration, project_id: &str, pageinsights_schedule_strace_request: Option<models::PageinsightsScheduleStraceRequest>) -> Result<models::AppRequestAppinstallation201Response, Error<PageinsightsScheduleStraceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/projects/{projectId}/straces", local_var_configuration.base_path, projectId=crate::apis::urlencode(project_id));
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
    local_var_req_builder = local_var_req_builder.json(&pageinsights_schedule_strace_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PageinsightsScheduleStraceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

