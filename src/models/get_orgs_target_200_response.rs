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
pub struct GetOrgsTarget200Response {
    #[serde(rename = "data")]
    pub data: Box<crate::models::Target>,
    #[serde(rename = "jsonapi")]
    pub jsonapi: Box<crate::models::JsonApi>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::Links>>,
}

impl GetOrgsTarget200Response {
    pub fn new(data: crate::models::Target, jsonapi: crate::models::JsonApi) -> GetOrgsTarget200Response {
        GetOrgsTarget200Response {
            data: Box::new(data),
            jsonapi: Box::new(jsonapi),
            links: None,
        }
    }
}


