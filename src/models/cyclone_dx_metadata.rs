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
pub struct CycloneDxMetadata {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<Box<crate::models::CycloneDxComponent>>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<crate::models::CycloneDxProperty>>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "tools", skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<crate::models::CycloneDxTool>>,
}

impl CycloneDxMetadata {
    pub fn new() -> CycloneDxMetadata {
        CycloneDxMetadata {
            component: None,
            properties: None,
            timestamp: None,
            tools: None,
        }
    }
}


