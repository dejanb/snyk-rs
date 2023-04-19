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
pub struct AppResourceAttributes {
    /// The access token time to live for your app, in seconds. It only affects the newly generated access tokens, existing access token will  continue to have their previous time to live as expiration.
    #[serde(rename = "access_token_ttl_seconds")]
    pub access_token_ttl_seconds: f32,
    /// The oauth2 client id for the app.
    #[serde(rename = "client_id")]
    pub client_id: uuid::Uuid,
    #[serde(rename = "context")]
    pub context: crate::models::Context,
    /// A boolean to indicate if an app is confidential or not as per the OAuth2 RFC.
    #[serde(rename = "is_confidential")]
    pub is_confidential: bool,
    /// A boolean to indicate if an app is publicly available or not.
    #[serde(rename = "is_public")]
    pub is_public: bool,
    /// New name of the app to display to users during authorization flow.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "org_public_id", skip_serializing_if = "Option::is_none")]
    pub org_public_id: Option<uuid::Uuid>,
    /// List of allowed redirect URIs to call back after authentication.
    #[serde(rename = "redirect_uris")]
    pub redirect_uris: Vec<String>,
    /// The scopes this app is allowed to request during authorization.
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
}

impl AppResourceAttributes {
    pub fn new(access_token_ttl_seconds: f32, client_id: uuid::Uuid, context: crate::models::Context, is_confidential: bool, is_public: bool, name: String, redirect_uris: Vec<String>, scopes: Vec<String>) -> AppResourceAttributes {
        AppResourceAttributes {
            access_token_ttl_seconds,
            client_id,
            context,
            is_confidential,
            is_public,
            name,
            org_public_id: None,
            redirect_uris,
            scopes,
        }
    }
}


