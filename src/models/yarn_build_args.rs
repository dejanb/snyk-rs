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
pub struct YarnBuildArgs {
    #[serde(rename = "root_workspace", skip_serializing_if = "Option::is_none")]
    pub root_workspace: Option<String>,
}

impl YarnBuildArgs {
    pub fn new() -> YarnBuildArgs {
        YarnBuildArgs {
            root_workspace: None,
        }
    }
}


