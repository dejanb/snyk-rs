/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GroupIacSettingsRequest : The Infrastructure as Code settings for a group.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GroupIacSettingsRequest {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::GroupIacSettingsRequestAttributes>,
    /// Content type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl GroupIacSettingsRequest {
    /// The Infrastructure as Code settings for a group.
    pub fn new(attributes: crate::models::GroupIacSettingsRequestAttributes, r#type: String) -> GroupIacSettingsRequest {
        GroupIacSettingsRequest {
            attributes: Box::new(attributes),
            r#type,
        }
    }
}

