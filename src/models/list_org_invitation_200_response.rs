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
pub struct ListOrgInvitation200Response {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::OrgInvitation>,
    #[serde(rename = "jsonapi")]
    pub jsonapi: Box<crate::models::JsonApi>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::Links>,
}

impl ListOrgInvitation200Response {
    pub fn new(data: Vec<crate::models::OrgInvitation>, jsonapi: crate::models::JsonApi, links: crate::models::Links) -> ListOrgInvitation200Response {
        ListOrgInvitation200Response {
            data,
            jsonapi: Box::new(jsonapi),
            links: Box::new(links),
        }
    }
}


