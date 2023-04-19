/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetIacSettingsForGroup400ResponseErrorsInnerLinks : A link that leads to further details about this particular occurrance of the problem.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetIacSettingsForGroup400ResponseErrorsInnerLinks {
    #[serde(rename = "about", skip_serializing_if = "Option::is_none")]
    pub about: Option<Box<crate::models::GetIacSettingsForGroup400ResponseErrorsInnerLinksAbout>>,
}

impl GetIacSettingsForGroup400ResponseErrorsInnerLinks {
    /// A link that leads to further details about this particular occurrance of the problem.
    pub fn new() -> GetIacSettingsForGroup400ResponseErrorsInnerLinks {
        GetIacSettingsForGroup400ResponseErrorsInnerLinks {
            about: None,
        }
    }
}


