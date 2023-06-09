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
pub struct SelfLink {
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<Box<crate::models::LinkProperty>>,
}

impl SelfLink {
    pub fn new() -> SelfLink {
        SelfLink {
            param_self: None,
        }
    }
}


