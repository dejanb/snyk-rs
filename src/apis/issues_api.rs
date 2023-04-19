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


/// struct for typed errors of method [`fetch_issues_per_purl`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchIssuesPerPurlError {
    Status400(crate::models::GetIacSettingsForGroup400Response),
    Status401(crate::models::GetIacSettingsForGroup400Response),
    Status403(crate::models::GetIacSettingsForGroup400Response),
    Status404(crate::models::GetIacSettingsForGroup400Response),
    Status409(crate::models::GetIacSettingsForGroup400Response),
    Status500(crate::models::GetIacSettingsForGroup400Response),
    UnknownValue(serde_json::Value),
}


/// Query issues for a specific package version identified by Package URL (purl). Snyk returns only direct vulnerabilities. Transitive vulnerabilities (from dependencies) are not returned because they can vary depending on context.
pub async fn fetch_issues_per_purl(configuration: &configuration::Configuration, version: &str, purl: &str, org_id: &str, offset: Option<f32>, limit: Option<f32>) -> Result<crate::models::IssuesResponse, Error<FetchIssuesPerPurlError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/packages/{purl}/issues", local_var_configuration.base_path, purl=crate::apis::urlencode(purl), org_id=crate::apis::urlencode(org_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("version", &version.to_string())]);
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
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
        let local_var_entity: Option<FetchIssuesPerPurlError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}