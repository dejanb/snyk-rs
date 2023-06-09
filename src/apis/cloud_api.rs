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

/// struct for passing parameters to the method [`create_environment`]
#[derive(Clone, Debug, Default)]
pub struct CreateEnvironmentParams {
    /// The requested version of the endpoint to process the request
    pub version: String,
    /// Organization ID
    pub org_id: String,
    pub create_environment_request: Option<crate::models::CreateEnvironmentRequest>
}

/// struct for passing parameters to the method [`create_scan`]
#[derive(Clone, Debug, Default)]
pub struct CreateScanParams {
    /// The requested version of the endpoint to process the request
    pub version: String,
    /// Organization ID
    pub org_id: String,
    pub create_scan_request: Option<crate::models::CreateScanRequest>
}

/// struct for passing parameters to the method [`delete_environment`]
#[derive(Clone, Debug, Default)]
pub struct DeleteEnvironmentParams {
    /// The requested version of the endpoint to process the request
    pub version: String,
    /// Organization ID
    pub org_id: String,
    /// Unique identifier for an environment
    pub environment_id: String
}

/// struct for passing parameters to the method [`get_permissions`]
#[derive(Clone, Debug, Default)]
pub struct GetPermissionsParams {
    /// The requested version of the endpoint to process the request
    pub version: String,
    /// Organization ID
    pub org_id: String,
    pub get_permissions_request: Option<crate::models::GetPermissionsRequest>
}

/// struct for passing parameters to the method [`list_environments`]
#[derive(Clone, Debug, Default)]
pub struct ListEnvironmentsParams {
    /// The requested version of the endpoint to process the request
    pub version: String,
    /// Organization ID
    pub org_id: String,
    /// Return environments created after this date
    pub created_after: Option<String>,
    /// Return environments created before this date
    pub created_before: Option<String>,
    /// Return environments updated after this date
    pub updated_after: Option<String>,
    /// Return environments updated before this date
    pub updated_before: Option<String>,
    /// Filter environments by name (multi-value, comma-separated)
    pub name: Option<String>,
    /// Filter environments by kind (multi-value, comma-separated): aws
    pub kind: Option<String>,
    /// Filter environments by latest scan status (multi-value, comma-separated): queued, in_progress, success, error
    pub status: Option<String>,
    /// Filter environments by environment ID (multi-value, comma-separated)
    pub id: Option<String>,
    /// Return the page of results immediately after this cursor
    pub starting_after: Option<String>,
    /// Return the page of results immediately before this cursor
    pub ending_before: Option<String>,
    /// Number of results to return per page
    pub limit: Option<i32>
}

/// struct for passing parameters to the method [`list_resources`]
#[derive(Clone, Debug, Default)]
pub struct ListResourcesParams {
    /// Organization ID
    pub org_id: String,
    /// The requested version of the endpoint to process the request
    pub version: String,
    /// Filter resources by environment ID (multi-value, comma-separated)
    pub environment_id: Option<String>,
    /// Filter resources by resource type (multi-value, comma-separated)
    pub resource_type: Option<String>,
    /// Filter resources by resource ID (multi-value, comma-separated)
    pub resource_id: Option<String>,
    /// Filter resources by native ID (multi-value, comma-separated) (AWS ARN)
    pub native_id: Option<String>,
    /// Filter resources by resource UUID (multi-value, comma-separated)
    pub id: Option<String>,
    /// Filter resources by platform (multi-value, comma-separated): aws
    pub platform: Option<String>,
    /// Filter resources by name (multi-value, comma-separated)
    pub name: Option<String>,
    /// Filter resources by kind (multi-value, comma-separated): cloud
    pub kind: Option<String>,
    /// Filter resources by location (multi-value, comma-separated) (AWS region)
    pub location: Option<String>,
    /// Filter resources by whether they have been removed or not.
    pub removed: Option<bool>,
    /// Return the page of results immediately after this cursor
    pub starting_after: Option<String>,
    /// Return the page of results immediately before this cursor
    pub ending_before: Option<String>,
    /// Number of results to return per page
    pub limit: Option<i32>
}

/// struct for passing parameters to the method [`list_scan`]
#[derive(Clone, Debug, Default)]
pub struct ListScanParams {
    /// The requested version of the endpoint to process the request
    pub version: String,
    /// Organization ID
    pub org_id: String,
    /// Return the page of results immediately after this cursor
    pub starting_after: Option<String>,
    /// Return the page of results immediately before this cursor
    pub ending_before: Option<String>,
    /// Number of results to return per page
    pub limit: Option<i32>
}

/// struct for passing parameters to the method [`update_environment`]
#[derive(Clone, Debug, Default)]
pub struct UpdateEnvironmentParams {
    /// The requested version of the endpoint to process the request
    pub version: String,
    /// Organization ID
    pub org_id: String,
    /// Unique identifier for an environment
    pub environment_id: String,
    pub update_environment_request: Option<crate::models::UpdateEnvironmentRequest>
}


/// struct for typed errors of method [`create_environment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEnvironmentError {
    Status400(crate::models::ListGroups400Response),
    Status401(crate::models::ListGroups400Response),
    Status403(crate::models::ListGroups400Response),
    Status404(crate::models::ListGroups400Response),
    Status409(crate::models::ListGroups400Response),
    Status500(crate::models::ListGroups400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_scan`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateScanError {
    Status400(crate::models::ListGroups400Response),
    Status401(crate::models::ListGroups400Response),
    Status403(crate::models::ListGroups400Response),
    Status404(crate::models::ListGroups400Response),
    Status409(crate::models::ListGroups400Response),
    Status500(crate::models::ListGroups400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_environment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteEnvironmentError {
    Status400(crate::models::ListGroups400Response),
    Status401(crate::models::ListGroups400Response),
    Status403(crate::models::ListGroups400Response),
    Status404(crate::models::ListGroups400Response),
    Status409(crate::models::ListGroups400Response),
    Status500(crate::models::ListGroups400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPermissionsError {
    Status400(crate::models::ListGroups400Response),
    Status401(crate::models::ListGroups400Response),
    Status403(crate::models::ListGroups400Response),
    Status404(crate::models::ListGroups400Response),
    Status409(crate::models::ListGroups400Response),
    Status500(crate::models::ListGroups400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_environments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEnvironmentsError {
    Status400(crate::models::ListGroups400Response),
    Status401(crate::models::ListGroups400Response),
    Status403(crate::models::ListGroups400Response),
    Status404(crate::models::ListGroups400Response),
    Status409(crate::models::ListGroups400Response),
    Status500(crate::models::ListGroups400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_resources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListResourcesError {
    Status400(crate::models::ListGroups400Response),
    Status401(crate::models::ListGroups400Response),
    Status403(crate::models::ListGroups400Response),
    Status404(crate::models::ListGroups400Response),
    Status409(crate::models::ListGroups400Response),
    Status500(crate::models::ListGroups400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_scan`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListScanError {
    Status400(crate::models::ListGroups400Response),
    Status401(crate::models::ListGroups400Response),
    Status403(crate::models::ListGroups400Response),
    Status404(crate::models::ListGroups400Response),
    Status409(crate::models::ListGroups400Response),
    Status500(crate::models::ListGroups400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_environment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateEnvironmentError {
    Status400(crate::models::ListGroups400Response),
    Status401(crate::models::ListGroups400Response),
    Status403(crate::models::ListGroups400Response),
    Status404(crate::models::ListGroups400Response),
    Status409(crate::models::ListGroups400Response),
    Status500(crate::models::ListGroups400Response),
    UnknownValue(serde_json::Value),
}


/// Create a new environment and run a scan
pub async fn create_environment(configuration: &configuration::Configuration, params: CreateEnvironmentParams) -> Result<crate::models::CreateEnvironment201Response, Error<CreateEnvironmentError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let version = params.version;
    let org_id = params.org_id;
    let create_environment_request = params.create_environment_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/cloud/environments", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&create_environment_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateEnvironmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create and trigger a new scan for an environment
pub async fn create_scan(configuration: &configuration::Configuration, params: CreateScanParams) -> Result<crate::models::CreateScan201Response, Error<CreateScanError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let version = params.version;
    let org_id = params.org_id;
    let create_scan_request = params.create_scan_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/cloud/scans", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&create_scan_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateScanError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an environment
pub async fn delete_environment(configuration: &configuration::Configuration, params: DeleteEnvironmentParams) -> Result<(), Error<DeleteEnvironmentError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let version = params.version;
    let org_id = params.org_id;
    let environment_id = params.environment_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/cloud/environments/{environment_id}", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id), environment_id=crate::apis::urlencode(environment_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        Ok(())
    } else {
        let local_var_entity: Option<DeleteEnvironmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Generate IAC template for Snyk to access your cloud resources
pub async fn get_permissions(configuration: &configuration::Configuration, params: GetPermissionsParams) -> Result<crate::models::GetPermissions201Response, Error<GetPermissionsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let version = params.version;
    let org_id = params.org_id;
    let get_permissions_request = params.get_permissions_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/cloud/permissions", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&get_permissions_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetPermissionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List environments for an organization
pub async fn list_environments(configuration: &configuration::Configuration, params: ListEnvironmentsParams) -> Result<crate::models::ListEnvironments200Response, Error<ListEnvironmentsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let version = params.version;
    let org_id = params.org_id;
    let created_after = params.created_after;
    let created_before = params.created_before;
    let updated_after = params.updated_after;
    let updated_before = params.updated_before;
    let name = params.name;
    let kind = params.kind;
    let status = params.status;
    let id = params.id;
    let starting_after = params.starting_after;
    let ending_before = params.ending_before;
    let limit = params.limit;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/cloud/environments", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("version", &version.to_string())]);
    if let Some(ref local_var_str) = created_after {
        local_var_req_builder = local_var_req_builder.query(&[("created_after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_before {
        local_var_req_builder = local_var_req_builder.query(&[("created_before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = updated_after {
        local_var_req_builder = local_var_req_builder.query(&[("updated_after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = updated_before {
        local_var_req_builder = local_var_req_builder.query(&[("updated_before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = kind {
        local_var_req_builder = local_var_req_builder.query(&[("kind", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = id {
        local_var_req_builder = local_var_req_builder.query(&[("id", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<ListEnvironmentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List resources for an organization
pub async fn list_resources(configuration: &configuration::Configuration, params: ListResourcesParams) -> Result<crate::models::ListResources200Response, Error<ListResourcesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let org_id = params.org_id;
    let version = params.version;
    let environment_id = params.environment_id;
    let resource_type = params.resource_type;
    let resource_id = params.resource_id;
    let native_id = params.native_id;
    let id = params.id;
    let platform = params.platform;
    let name = params.name;
    let kind = params.kind;
    let location = params.location;
    let removed = params.removed;
    let starting_after = params.starting_after;
    let ending_before = params.ending_before;
    let limit = params.limit;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/cloud/resources", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = environment_id {
        local_var_req_builder = local_var_req_builder.query(&[("environment_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = resource_type {
        local_var_req_builder = local_var_req_builder.query(&[("resource_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = resource_id {
        local_var_req_builder = local_var_req_builder.query(&[("resource_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = native_id {
        local_var_req_builder = local_var_req_builder.query(&[("native_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = id {
        local_var_req_builder = local_var_req_builder.query(&[("id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = platform {
        local_var_req_builder = local_var_req_builder.query(&[("platform", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = kind {
        local_var_req_builder = local_var_req_builder.query(&[("kind", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = location {
        local_var_req_builder = local_var_req_builder.query(&[("location", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = removed {
        local_var_req_builder = local_var_req_builder.query(&[("removed", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListResourcesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List scans for an organization
pub async fn list_scan(configuration: &configuration::Configuration, params: ListScanParams) -> Result<crate::models::ListScan200Response, Error<ListScanError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let version = params.version;
    let org_id = params.org_id;
    let starting_after = params.starting_after;
    let ending_before = params.ending_before;
    let limit = params.limit;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/cloud/scans", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<ListScanError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update an environment
pub async fn update_environment(configuration: &configuration::Configuration, params: UpdateEnvironmentParams) -> Result<crate::models::UpdateEnvironment200Response, Error<UpdateEnvironmentError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let version = params.version;
    let org_id = params.org_id;
    let environment_id = params.environment_id;
    let update_environment_request = params.update_environment_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/cloud/environments/{environment_id}", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id), environment_id=crate::apis::urlencode(environment_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&update_environment_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateEnvironmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

