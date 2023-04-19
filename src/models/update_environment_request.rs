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
pub struct UpdateEnvironmentRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::UpdateEnvironmentRequestData>,
}

impl UpdateEnvironmentRequest {
    pub fn new(data: crate::models::UpdateEnvironmentRequestData) -> UpdateEnvironmentRequest {
        UpdateEnvironmentRequest {
            data: Box::new(data),
        }
    }
}

