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
pub struct AppPatchRequest {
    /// The access token time to live for your app, in seconds. It only affects the newly generated access tokens, existing access token will  continue to have their previous time to live as expiration.
    #[serde(rename = "access_token_ttl_seconds", skip_serializing_if = "Option::is_none")]
    pub access_token_ttl_seconds: Option<f32>,
    /// New name of the app to display to users during authorization flow.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// List of allowed redirect URIs to call back after authentication.
    #[serde(rename = "redirect_uris", skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
}

impl AppPatchRequest {
    pub fn new() -> AppPatchRequest {
        AppPatchRequest {
            access_token_ttl_seconds: None,
            name: None,
            redirect_uris: None,
        }
    }
}


