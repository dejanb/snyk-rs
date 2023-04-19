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
pub struct Package {
    /// The package’s name
    #[serde(rename = "name")]
    pub name: String,
    /// A name prefix, such as a maven group id or docker image owner
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// The package type or protocol
    #[serde(rename = "type")]
    pub r#type: String,
    /// The purl of the package
    #[serde(rename = "url")]
    pub url: String,
    /// The version of the package
    #[serde(rename = "version")]
    pub version: String,
}

impl Package {
    pub fn new(name: String, r#type: String, url: String, version: String) -> Package {
        Package {
            name,
            namespace: None,
            r#type,
            url,
            version,
        }
    }
}

