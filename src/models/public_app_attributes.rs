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
pub struct PublicAppAttributes {
    /// The oauth2 client id for the app.
    #[serde(rename = "client_id")]
    pub client_id: uuid::Uuid,
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<crate::models::Context>,
    /// New name of the app to display to users during authorization flow.
    #[serde(rename = "name")]
    pub name: String,
    /// The scopes this app is allowed to request during authorization.
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

impl PublicAppAttributes {
    pub fn new(client_id: uuid::Uuid, name: String) -> PublicAppAttributes {
        PublicAppAttributes {
            client_id,
            context: None,
            name,
            scopes: None,
        }
    }
}

