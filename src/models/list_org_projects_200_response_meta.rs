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
pub struct ListOrgProjects200ResponseMeta {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<f32>,
}

impl ListOrgProjects200ResponseMeta {
    pub fn new() -> ListOrgProjects200ResponseMeta {
        ListOrgProjects200ResponseMeta {
            count: None,
        }
    }
}

