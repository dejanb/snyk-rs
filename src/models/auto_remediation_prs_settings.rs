/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AutoRemediationPrsSettings : Automatically raise pull requests on recurring tests to fix new and existing vulnerabilities. If not specified, settings will be inherited from the Project's integration.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AutoRemediationPrsSettings {
    /// Automatically create pull requests on scheduled tests for known (backlog) vulnerabilities.
    #[serde(rename = "is_backlog_prs_enabled", skip_serializing_if = "Option::is_none")]
    pub is_backlog_prs_enabled: Option<bool>,
    /// Automatically create pull requests on scheduled tests for new vulnerabilities.
    #[serde(rename = "is_fresh_prs_enabled", skip_serializing_if = "Option::is_none")]
    pub is_fresh_prs_enabled: Option<bool>,
    /// Include vulnerability patches in automatic pull requests.
    #[serde(rename = "is_patch_remediation_enabled", skip_serializing_if = "Option::is_none")]
    pub is_patch_remediation_enabled: Option<bool>,
}

impl AutoRemediationPrsSettings {
    /// Automatically raise pull requests on recurring tests to fix new and existing vulnerabilities. If not specified, settings will be inherited from the Project's integration.
    pub fn new() -> AutoRemediationPrsSettings {
        AutoRemediationPrsSettings {
            is_backlog_prs_enabled: None,
            is_fresh_prs_enabled: None,
            is_patch_remediation_enabled: None,
        }
    }
}


