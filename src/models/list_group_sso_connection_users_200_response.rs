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
pub struct ListGroupSsoConnectionUsers200Response {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::User>,
    #[serde(rename = "jsonapi")]
    pub jsonapi: Box<crate::models::JsonApi>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::Links>,
}

impl ListGroupSsoConnectionUsers200Response {
    pub fn new(data: Vec<crate::models::User>, jsonapi: crate::models::JsonApi, links: crate::models::Links) -> ListGroupSsoConnectionUsers200Response {
        ListGroupSsoConnectionUsers200Response {
            data,
            jsonapi: Box::new(jsonapi),
            links: Box::new(links),
        }
    }
}


