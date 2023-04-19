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
pub struct DeprecatedRelationship {
    #[serde(rename = "data")]
    pub data: Box<crate::models::DeprecatedRelationshipData>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::Links>>,
    /// Free-form object that may contain non-standard information.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl DeprecatedRelationship {
    pub fn new(data: crate::models::DeprecatedRelationshipData) -> DeprecatedRelationship {
        DeprecatedRelationship {
            data: Box::new(data),
            links: None,
            meta: None,
        }
    }
}


