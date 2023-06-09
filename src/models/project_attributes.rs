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
pub struct ProjectAttributes {
    #[serde(rename = "build_args", skip_serializing_if = "Option::is_none")]
    pub build_args: Option<Box<crate::models::ProjectAttributesBuildArgs>>,
    #[serde(rename = "business_criticality", skip_serializing_if = "Option::is_none")]
    pub business_criticality: Option<Vec<BusinessCriticality>>,
    /// The date that the project was created on
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<Environment>>,
    #[serde(rename = "lifecycle", skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Vec<Lifecycle>>,
    /// Project name.
    #[serde(rename = "name")]
    pub name: String,
    /// The origin the project was added from.
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "settings")]
    pub settings: Box<crate::models::ProjectSettings>,
    /// Describes if a project is currently monitored or it is de-activated.
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ProjectAttributesTagsInner>>,
    /// Path within the target to identify a specific file/directory/image etc. when scanning just part  of the target, and not the entity.
    #[serde(rename = "target_file")]
    pub target_file: String,
    /// The additional information required to resolve which revision of the resource should be scanned.
    #[serde(rename = "target_reference")]
    pub target_reference: String,
    /// The package manager of the project.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ProjectAttributes {
    pub fn new(created: String, name: String, origin: String, settings: crate::models::ProjectSettings, status: Status, target_file: String, target_reference: String, r#type: String) -> ProjectAttributes {
        ProjectAttributes {
            build_args: None,
            business_criticality: None,
            created,
            environment: None,
            lifecycle: None,
            name,
            origin,
            settings: Box::new(settings),
            status,
            tags: None,
            target_file,
            target_reference,
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BusinessCriticality {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
}

impl Default for BusinessCriticality {
    fn default() -> BusinessCriticality {
        Self::Critical
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Environment {
    #[serde(rename = "frontend")]
    Frontend,
    #[serde(rename = "backend")]
    Backend,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "saas")]
    Saas,
    #[serde(rename = "onprem")]
    Onprem,
    #[serde(rename = "hosted")]
    Hosted,
    #[serde(rename = "distributed")]
    Distributed,
}

impl Default for Environment {
    fn default() -> Environment {
        Self::Frontend
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Lifecycle {
    #[serde(rename = "production")]
    Production,
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "sandbox")]
    Sandbox,
}

impl Default for Lifecycle {
    fn default() -> Lifecycle {
        Self::Production
    }
}
/// Describes if a project is currently monitored or it is de-activated.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}

