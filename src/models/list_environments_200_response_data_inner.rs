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
pub struct ListEnvironments200ResponseDataInner {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::EnvironmentAttributes>>,
    /// Environment ID
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Environment relationships
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ListEnvironments200ResponseDataInner {
    pub fn new(id: uuid::Uuid, r#type: String) -> ListEnvironments200ResponseDataInner {
        ListEnvironments200ResponseDataInner {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}


