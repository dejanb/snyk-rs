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
pub struct UserAttributes {
    /// Whether the user status is enabled or not
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The email of the user.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "membership", skip_serializing_if = "Option::is_none")]
    pub membership: Option<Box<crate::models::UserAttributesMembership>>,
    /// The name of the user.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The username of the user.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl UserAttributes {
    pub fn new() -> UserAttributes {
        UserAttributes {
            active: None,
            email: None,
            membership: None,
            name: None,
            username: None,
        }
    }
}


