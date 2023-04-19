/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EnvironmentAttributes : Environment attributes



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EnvironmentAttributes {
    /// When the environment was created
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "deleted_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<Option<String>>,
    /// Environment kind: aws
    #[serde(rename = "kind")]
    pub kind: String,
    /// Environment name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// Increment for each change to an environment
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    /// When the environment was last updated
    #[serde(rename = "updated_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Option<String>>,
}

impl EnvironmentAttributes {
    /// Environment attributes
    pub fn new(created_at: String, kind: String, name: String) -> EnvironmentAttributes {
        EnvironmentAttributes {
            created_at,
            deleted_at: None,
            kind,
            name,
            options: None,
            properties: None,
            revision: None,
            updated_at: None,
        }
    }
}


