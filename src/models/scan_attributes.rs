/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScanAttributes : Scan attributes



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScanAttributes {
    /// When the scan was created
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "deleted_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<Option<String>>,
    /// Environment ID
    #[serde(rename = "environment_id", skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<uuid::Uuid>,
    /// Error message if the scan failed
    #[serde(rename = "error", deserialize_with = "Option::deserialize")]
    pub error: Option<String>,
    /// When the scan finished
    #[serde(rename = "finished_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Option<String>>,
    /// Scan kind
    #[serde(rename = "kind", deserialize_with = "Option::deserialize")]
    pub kind: Option<Kind>,
    #[serde(rename = "options", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub options: Option<Option<serde_json::Value>>,
    /// Organization ID
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<uuid::Uuid>,
    /// Errors that didn't fail the scan
    #[serde(rename = "partial_errors", skip_serializing_if = "Option::is_none")]
    pub partial_errors: Option<String>,
    /// Increment for each change to a scan
    #[serde(rename = "revision")]
    pub revision: f32,
    /// Scan status
    #[serde(rename = "status", deserialize_with = "Option::deserialize")]
    pub status: Option<Status>,
    /// When the scan was last updated
    #[serde(rename = "updated_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Option<String>>,
}

impl ScanAttributes {
    /// Scan attributes
    pub fn new(created_at: String, error: Option<String>, kind: Option<Kind>, revision: f32, status: Option<Status>) -> ScanAttributes {
        ScanAttributes {
            created_at,
            deleted_at: None,
            environment_id: None,
            error,
            finished_at: None,
            kind,
            options: None,
            organization_id: None,
            partial_errors: None,
            revision,
            status,
            updated_at: None,
        }
    }
}

/// Scan kind
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "user_initiated")]
    UserInitiated,
    #[serde(rename = "event_driven")]
    EventDriven,
    #[serde(rename = "null")]
    Null,
}

impl Default for Kind {
    fn default() -> Kind {
        Self::Scheduled
    }
}
/// Scan status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "null")]
    Null,
}

impl Default for Status {
    fn default() -> Status {
        Self::Queued
    }
}
