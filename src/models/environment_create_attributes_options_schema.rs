/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EnvironmentCreateAttributesOptionsSchema {
    /// AWS IAM role ARN for Snyk
    #[serde(rename = "role_arn")]
    pub role_arn: String,
    /// Google project ID
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Google service account email
    #[serde(rename = "service_account_email")]
    pub service_account_email: String,
    /// ID of the Azure app registration with permissions to scan
    #[serde(rename = "application_id", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// ID of the Azure subscription to be scanned
    #[serde(rename = "subscription_id", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    /// Azure Tenant (directory) ID
    #[serde(rename = "tenant_id", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

impl EnvironmentCreateAttributesOptionsSchema {
    pub fn new(role_arn: String, service_account_email: String) -> EnvironmentCreateAttributesOptionsSchema {
        EnvironmentCreateAttributesOptionsSchema {
            role_arn,
            project_id: None,
            service_account_email,
            application_id: None,
            subscription_id: None,
            tenant_id: None,
        }
    }
}


