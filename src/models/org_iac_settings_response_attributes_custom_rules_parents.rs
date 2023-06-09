/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrgIacSettingsResponseAttributesCustomRulesParents : Contains all parents the org can inherit settings from.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrgIacSettingsResponseAttributesCustomRulesParents {
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<crate::models::OrgIacSettingsResponseAttributesCustomRulesParentsGroup>>,
}

impl OrgIacSettingsResponseAttributesCustomRulesParents {
    /// Contains all parents the org can inherit settings from.
    pub fn new() -> OrgIacSettingsResponseAttributesCustomRulesParents {
        OrgIacSettingsResponseAttributesCustomRulesParents {
            group: None,
        }
    }
}


