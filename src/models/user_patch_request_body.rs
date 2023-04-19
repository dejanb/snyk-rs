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
pub struct UserPatchRequestBody {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::UpdateUserRequestDataAttributes>,
    /// The Snyk ID corresponding to this user
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Content type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl UserPatchRequestBody {
    pub fn new(attributes: crate::models::UpdateUserRequestDataAttributes, id: uuid::Uuid, r#type: String) -> UserPatchRequestBody {
        UserPatchRequestBody {
            attributes: Box::new(attributes),
            id,
            r#type,
        }
    }
}

