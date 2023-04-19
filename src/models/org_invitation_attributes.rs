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
pub struct OrgInvitationAttributes {
    /// The email address of the invitee.
    #[serde(rename = "email")]
    pub email: String,
    /// The active status of the invitation. is_active of true indicates that the invitation is still waiting to be accepted. Invitations are considered inactive when accepted or revoked. 
    #[serde(rename = "is_active")]
    pub is_active: bool,
    /// The role assigned to the invitee on acceptance.
    #[serde(rename = "role")]
    pub role: String,
}

impl OrgInvitationAttributes {
    pub fn new(email: String, is_active: bool, role: String) -> OrgInvitationAttributes {
        OrgInvitationAttributes {
            email,
            is_active,
            role,
        }
    }
}

