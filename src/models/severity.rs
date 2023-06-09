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
pub struct Severity {
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// The CVSSv3 value of the vulnerability.
    #[serde(rename = "score", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub score: Option<Option<f32>>,
    /// The source of this severity. The value must be the id of a referenced problem or class, in which case that problem or class is the source of this issue. If source is omitted, this severity is sourced internally in the Snyk application.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The CVSSv3 value of the vulnerability.
    #[serde(rename = "vector", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vector: Option<Option<String>>,
}

impl Severity {
    pub fn new() -> Severity {
        Severity {
            level: None,
            score: None,
            source: None,
            vector: None,
        }
    }
}


