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
pub struct User {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::UserAttributes>,
    /// The Snyk ID corresponding to this user
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Content type.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl User {
    pub fn new(attributes: crate::models::UserAttributes, id: uuid::Uuid, r#type: String) -> User {
        User {
            attributes: Box::new(attributes),
            id,
            r#type,
        }
    }
}


