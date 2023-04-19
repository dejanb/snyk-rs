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
pub struct UpdateUserRequestDataAttributesMembership {
    /// Role name
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl UpdateUserRequestDataAttributesMembership {
    pub fn new() -> UpdateUserRequestDataAttributesMembership {
        UpdateUserRequestDataAttributesMembership {
            role: None,
        }
    }
}

