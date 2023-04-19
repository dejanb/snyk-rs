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
pub struct ResourceAttributes {
    /// When the resource was first recorded
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "deleted_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<Option<String>>,
    /// Computed hash value for the resource based on its attributes
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "is_managed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_managed: Option<Option<bool>>,
    /// Kind of resource: cloud
    #[serde(rename = "kind")]
    pub kind: String,
    /// Physical location (AWS region)
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Human friendly resource name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Resource namespace (AWS region)
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// ID of the physical resource from the cloud provider (AWS ARN, if available)
    #[serde(rename = "native_id", skip_serializing_if = "Option::is_none")]
    pub native_id: Option<String>,
    /// Resource platform: aws
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "removed_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub removed_at: Option<Option<String>>,
    /// Unique ID for the resource
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    /// Terraform resource type
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    /// Increment for each change to a resource
    #[serde(rename = "revision")]
    pub revision: i32,
    #[serde(rename = "schema_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<Option<String>>,
    #[serde(rename = "source_location", skip_serializing_if = "Option::is_none")]
    pub source_location: Option<Vec<::std::collections::HashMap<String, serde_json::Value>>>,
    /// Terraform state attributes
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// Resource tags from the cloud provider
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// When the resource was last updated
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
}

impl ResourceAttributes {
    pub fn new(created_at: String, hash: String, kind: String, platform: String, resource_id: String, resource_type: String, revision: i32, updated_at: Option<String>) -> ResourceAttributes {
        ResourceAttributes {
            created_at,
            deleted_at: None,
            hash,
            is_managed: None,
            kind,
            location: None,
            name: None,
            namespace: None,
            native_id: None,
            platform,
            relationships: None,
            removed_at: None,
            resource_id,
            resource_type,
            revision,
            schema_version: None,
            source_location: None,
            state: None,
            tags: None,
            updated_at,
        }
    }
}


