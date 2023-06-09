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
pub struct GetOrgProject200Response {
    #[serde(rename = "data")]
    pub data: Box<crate::models::GetOrgProject200ResponseData>,
    #[serde(rename = "jsonapi")]
    pub jsonapi: Box<crate::models::JsonApi>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::Links>,
}

impl GetOrgProject200Response {
    pub fn new(data: crate::models::GetOrgProject200ResponseData, jsonapi: crate::models::JsonApi, links: crate::models::Links) -> GetOrgProject200Response {
        GetOrgProject200Response {
            data: Box::new(data),
            jsonapi: Box::new(jsonapi),
            links: Box::new(links),
        }
    }
}


