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
pub struct ScanCreateRelationshipsEnvironmentData {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ScanCreateRelationshipsEnvironmentData {
    pub fn new(id: String, r#type: String) -> ScanCreateRelationshipsEnvironmentData {
        ScanCreateRelationshipsEnvironmentData {
            id,
            r#type,
        }
    }
}

