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
pub struct IssuesResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::CommonIssueModel>>,
    #[serde(rename = "jsonapi", skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<Box<crate::models::JsonApi>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PaginatedLinks>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::IssuesMeta>>,
}

impl IssuesResponse {
    pub fn new() -> IssuesResponse {
        IssuesResponse {
            data: None,
            jsonapi: None,
            links: None,
            meta: None,
        }
    }
}


