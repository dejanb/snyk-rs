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
pub struct CreateEnvironment201Response {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::CreateEnvironment201ResponseData>>,
    #[serde(rename = "jsonapi", skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<Box<crate::models::JsonApi>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PaginatedLinks>>,
}

impl CreateEnvironment201Response {
    pub fn new() -> CreateEnvironment201Response {
        CreateEnvironment201Response {
            data: None,
            jsonapi: None,
            links: None,
        }
    }
}


