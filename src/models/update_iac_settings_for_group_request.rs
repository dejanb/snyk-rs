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
pub struct UpdateIacSettingsForGroupRequest {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::GroupIacSettingsRequest>>,
}

impl UpdateIacSettingsForGroupRequest {
    pub fn new() -> UpdateIacSettingsForGroupRequest {
        UpdateIacSettingsForGroupRequest {
            data: None,
        }
    }
}


