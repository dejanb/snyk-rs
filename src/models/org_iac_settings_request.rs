/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrgIacSettingsRequest : The Infrastructure as Code settings for an org.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrgIacSettingsRequest {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::OrgIacSettingsRequestAttributes>,
    /// Content type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl OrgIacSettingsRequest {
    /// The Infrastructure as Code settings for an org.
    pub fn new(attributes: crate::models::OrgIacSettingsRequestAttributes, r#type: String) -> OrgIacSettingsRequest {
        OrgIacSettingsRequest {
            attributes: Box::new(attributes),
            r#type,
        }
    }
}


