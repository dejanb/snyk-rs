/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Context : Allow installing the app to a org/group or to a user, default tenant.

/// Allow installing the app to a org/group or to a user, default tenant.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Context {
    #[serde(rename = "tenant")]
    Tenant,
    #[serde(rename = "user")]
    User,

}

impl ToString for Context {
    fn to_string(&self) -> String {
        match self {
            Self::Tenant => String::from("tenant"),
            Self::User => String::from("user"),
        }
    }
}

impl Default for Context {
    fn default() -> Context {
        Self::Tenant
    }
}




