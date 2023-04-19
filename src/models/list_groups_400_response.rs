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
pub struct ListGroups400Response {
    #[serde(rename = "errors")]
    pub errors: Vec<crate::models::ListGroups400ResponseErrorsInner>,
    #[serde(rename = "jsonapi")]
    pub jsonapi: Box<crate::models::ListGroups400ResponseJsonapi>,
}

impl ListGroups400Response {
    pub fn new(errors: Vec<crate::models::ListGroups400ResponseErrorsInner>, jsonapi: crate::models::ListGroups400ResponseJsonapi) -> ListGroups400Response {
        ListGroups400Response {
            errors,
            jsonapi: Box::new(jsonapi),
        }
    }
}


