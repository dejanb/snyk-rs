/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PullRequestAssignmentSettings : Automatically assign pull requests created by Snyk (limited to private repos). If not specified, settings will be inherited from the Project's integration.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PullRequestAssignmentSettings {
    /// Manually specify users to assign (and all will be assigned).
    #[serde(rename = "assignees", skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,
    /// Automatically assign pull requests created by Snyk.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// Automatically assign the last user to change the manifest file (\"auto\"), or manually specify a list of users (\"manual\").
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
}

impl PullRequestAssignmentSettings {
    /// Automatically assign pull requests created by Snyk (limited to private repos). If not specified, settings will be inherited from the Project's integration.
    pub fn new() -> PullRequestAssignmentSettings {
        PullRequestAssignmentSettings {
            assignees: None,
            is_enabled: None,
            r#type: None,
        }
    }
}

/// Automatically assign the last user to change the manifest file (\"auto\"), or manually specify a list of users (\"manual\").
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "manual")]
    Manual,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Auto
    }
}

