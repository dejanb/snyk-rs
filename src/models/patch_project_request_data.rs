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
pub struct PatchProjectRequestData {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::PatchProjectRequestDataAttributes>,
    /// The Resource ID.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "relationships")]
    pub relationships: Box<crate::models::PatchProjectRequestDataRelationships>,
    /// The Resource type.
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl PatchProjectRequestData {
    pub fn new(attributes: crate::models::PatchProjectRequestDataAttributes, id: uuid::Uuid, relationships: crate::models::PatchProjectRequestDataRelationships, r#type: RHashType) -> PatchProjectRequestData {
        PatchProjectRequestData {
            attributes: Box::new(attributes),
            id,
            relationships: Box::new(relationships),
            r#type,
        }
    }
}

/// The Resource type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "project")]
    Project,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Project
    }
}

