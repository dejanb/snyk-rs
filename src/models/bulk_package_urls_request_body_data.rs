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
pub struct BulkPackageUrlsRequestBodyData {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::BulkPackageUrlsRequestBodyDataAttributes>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl BulkPackageUrlsRequestBodyData {
    pub fn new(attributes: crate::models::BulkPackageUrlsRequestBodyDataAttributes) -> BulkPackageUrlsRequestBodyData {
        BulkPackageUrlsRequestBodyData {
            attributes: Box::new(attributes),
            r#type: None,
        }
    }
}


