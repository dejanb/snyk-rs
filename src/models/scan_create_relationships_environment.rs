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
pub struct ScanCreateRelationshipsEnvironment {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::ScanCreateRelationshipsEnvironmentData>>,
}

impl ScanCreateRelationshipsEnvironment {
    pub fn new() -> ScanCreateRelationshipsEnvironment {
        ScanCreateRelationshipsEnvironment {
            data: None,
        }
    }
}

