/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`get_org_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrgProjectError {
    Status400(crate::models::GetIacSettingsForGroup400Response),
    Status401(crate::models::GetIacSettingsForGroup400Response),
    Status403(crate::models::GetIacSettingsForGroup400Response),
    Status404(crate::models::GetIacSettingsForGroup400Response),
    Status500(crate::models::GetIacSettingsForGroup400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_org_projects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOrgProjectsError {
    Status400(crate::models::GetIacSettingsForGroup400Response),
    Status401(crate::models::GetIacSettingsForGroup400Response),
    Status403(crate::models::GetIacSettingsForGroup400Response),
    Status404(crate::models::GetIacSettingsForGroup400Response),
    Status500(crate::models::GetIacSettingsForGroup400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_org_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOrgProjectError {
    Status400(crate::models::GetIacSettingsForGroup400Response),
    Status401(crate::models::GetIacSettingsForGroup400Response),
    Status403(crate::models::GetIacSettingsForGroup400Response),
    Status404(crate::models::GetIacSettingsForGroup400Response),
    Status500(crate::models::GetIacSettingsForGroup400Response),
    UnknownValue(serde_json::Value),
}


/// Get one project of the organization by project ID.
pub async fn get_org_project(configuration: &configuration::Configuration, org_id: &str, project_id: &str, version: &str, expand: Option<Vec<String>>) -> Result<crate::models::GetOrgProject200Response, Error<GetOrgProjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/projects/{project_id}", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id), project_id=crate::apis::urlencode(project_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = expand {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("expand".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("expand", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    local_var_req_builder = local_var_req_builder.query(&[("version", &version.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetOrgProjectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all Projects for an Org.
pub async fn list_org_projects(configuration: &configuration::Configuration, org_id: &str, version: &str, meta_count: Option<&str>, ids: Option<Vec<uuid::Uuid>>, names: Option<Vec<String>>, origins: Option<Vec<String>>, types: Option<Vec<String>>, expand: Option<Vec<String>>, cli_monitored_before: Option<String>, cli_monitored_after: Option<String>, starting_after: Option<&str>, ending_before: Option<&str>, limit: Option<i32>) -> Result<crate::models::ListOrgProjects200Response, Error<ListOrgProjectsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/projects", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = meta_count {
        local_var_req_builder = local_var_req_builder.query(&[("meta_count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = ids {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("ids".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("ids", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = names {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("names".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("names", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = origins {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("origins".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("origins", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = types {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("types".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("types", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = expand {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("expand".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("expand", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = cli_monitored_before {
        local_var_req_builder = local_var_req_builder.query(&[("cli_monitored_before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cli_monitored_after {
        local_var_req_builder = local_var_req_builder.query(&[("cli_monitored_after", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("version", &version.to_string())]);
    if let Some(ref local_var_str) = starting_after {
        local_var_req_builder = local_var_req_builder.query(&[("starting_after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = ending_before {
        local_var_req_builder = local_var_req_builder.query(&[("ending_before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
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
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListOrgProjectsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates one project of the organization by project ID.
pub async fn update_org_project(configuration: &configuration::Configuration, version: &str, org_id: &str, project_id: &str, user_id: &str, expand: Option<Vec<String>>, patch_project_request: Option<crate::models::PatchProjectRequest>) -> Result<crate::models::UpdateOrgProject200Response, Error<UpdateOrgProjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/projects/{project_id}", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id), project_id=crate::apis::urlencode(project_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("version", &version.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("user_id", &user_id.to_string())]);
    if let Some(ref local_var_str) = expand {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("expand".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("expand", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
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
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&patch_project_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateOrgProjectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

