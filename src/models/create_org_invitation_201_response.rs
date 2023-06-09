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
pub struct CreateOrgInvitation201Response {
    #[serde(rename = "data")]
    pub data: Box<crate::models::OrgInvitation>,
    #[serde(rename = "jsonapi")]
    pub jsonapi: Box<crate::models::JsonApi>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::Links>>,
}

impl CreateOrgInvitation201Response {
    pub fn new(data: crate::models::OrgInvitation, jsonapi: crate::models::JsonApi) -> CreateOrgInvitation201Response {
        CreateOrgInvitation201Response {
            data: Box::new(data),
            jsonapi: Box::new(jsonapi),
            links: None,
        }
    }
}


