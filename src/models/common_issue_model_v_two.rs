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
pub struct CommonIssueModelVTwo {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::CommonIssueModelVTwoAttributes>>,
    /// The Snyk ID of the vulnerability.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the REST resource. Always ‘issue’.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl CommonIssueModelVTwo {
    pub fn new() -> CommonIssueModelVTwo {
        CommonIssueModelVTwo {
            attributes: None,
            id: None,
            r#type: None,
        }
    }
}

