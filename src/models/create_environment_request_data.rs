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
pub struct CreateEnvironmentRequestData {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::EnvironmentCreateAttributes>>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CreateEnvironmentRequestData {
    pub fn new(r#type: String) -> CreateEnvironmentRequestData {
        CreateEnvironmentRequestData {
            attributes: None,
            r#type,
        }
    }
}


