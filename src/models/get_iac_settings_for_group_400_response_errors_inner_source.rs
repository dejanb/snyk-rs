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
pub struct GetIacSettingsForGroup400ResponseErrorsInnerSource {
    /// A string indicating which URI query parameter caused the error.
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    /// A JSON Pointer [RFC6901] to the associated entity in the request document.
    #[serde(rename = "pointer", skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

impl GetIacSettingsForGroup400ResponseErrorsInnerSource {
    pub fn new() -> GetIacSettingsForGroup400ResponseErrorsInnerSource {
        GetIacSettingsForGroup400ResponseErrorsInnerSource {
            parameter: None,
            pointer: None,
        }
    }
}

