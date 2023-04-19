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
pub struct ListGroups400ResponseJsonapi {
    /// Version of the JSON API specification this server supports.
    #[serde(rename = "version")]
    pub version: String,
}

impl ListGroups400ResponseJsonapi {
    pub fn new(version: String) -> ListGroups400ResponseJsonapi {
        ListGroups400ResponseJsonapi {
            version,
        }
    }
}

