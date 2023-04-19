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
pub struct HelloWorldAttributes {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "requestSubject")]
    pub request_subject: Box<crate::models::HelloWorldAttributesRequestSubject>,
}

impl HelloWorldAttributes {
    pub fn new(message: String, request_subject: crate::models::HelloWorldAttributesRequestSubject) -> HelloWorldAttributes {
        HelloWorldAttributes {
            message,
            request_subject: Box::new(request_subject),
        }
    }
}

