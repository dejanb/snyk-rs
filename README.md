# Rust API client for Snyk REST API

Generated by Openapi Generator https://github.com/openapitools/openapi-generator)

```
openapi-generator generate -g rust \
  -i https://api.snyk.io/rest/openapi/2023-03-30 \
  -o snyk
```


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: REST
- Package version: REST
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.snyk.io/rest*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AppsApi* | [**create_app**](docs/AppsApi.md#create_app) | **POST** /orgs/{org_id}/apps | Create a new app for an organization.
*AppsApi* | [**delete_app**](docs/AppsApi.md#delete_app) | **DELETE** /orgs/{org_id}/apps/{client_id} | Delete an app
*AppsApi* | [**delete_app_bot**](docs/AppsApi.md#delete_app_bot) | **DELETE** /orgs/{org_id}/app_bots/{bot_id} | Revoke app bot authorization
*AppsApi* | [**get_app**](docs/AppsApi.md#get_app) | **GET** /orgs/{org_id}/apps/{client_id} | Get an app by client id
*AppsApi* | [**get_app_bots**](docs/AppsApi.md#get_app_bots) | **GET** /orgs/{org_id}/app_bots | Get a list of app bots authorized to an organization.
*AppsApi* | [**get_apps**](docs/AppsApi.md#get_apps) | **GET** /orgs/{org_id}/apps | Get a list of apps created by an organization.
*AppsApi* | [**get_user_installed_apps**](docs/AppsApi.md#get_user_installed_apps) | **GET** /self/apps | Get a list of apps that can act on your behalf.
*AppsApi* | [**manage_secrets**](docs/AppsApi.md#manage_secrets) | **POST** /orgs/{org_id}/apps/{client_id}/secrets | Manage client secrets for an app.
*AppsApi* | [**revoke_user_installed_app**](docs/AppsApi.md#revoke_user_installed_app) | **DELETE** /self/apps/{app_id} | Revoke an app
*AppsApi* | [**update_app**](docs/AppsApi.md#update_app) | **PATCH** /orgs/{org_id}/apps/{client_id} | Update app attributes that are name, redirect URIs, and access token time to live
*IacSettingsApi* | [**get_iac_settings_for_group**](docs/IacSettingsApi.md#get_iac_settings_for_group) | **GET** /groups/{group_id}/settings/iac | Get the Infrastructure as Code Settings for a group
*IacSettingsApi* | [**get_iac_settings_for_org**](docs/IacSettingsApi.md#get_iac_settings_for_org) | **GET** /orgs/{org_id}/settings/iac | Get the Infrastructure as Code Settings for an org.
*IacSettingsApi* | [**update_iac_settings_for_group**](docs/IacSettingsApi.md#update_iac_settings_for_group) | **PATCH** /groups/{group_id}/settings/iac | Update the Infrastructure as Code Settings for a group
*IacSettingsApi* | [**update_iac_settings_for_org**](docs/IacSettingsApi.md#update_iac_settings_for_org) | **PATCH** /orgs/{org_id}/settings/iac | Update the Infrastructure as Code Settings for an org
*InvitesApi* | [**create_org_invitation**](docs/InvitesApi.md#create_org_invitation) | **POST** /orgs/{org_id}/invites | Invite a user to an organization
*InvitesApi* | [**delete_org_invitation**](docs/InvitesApi.md#delete_org_invitation) | **DELETE** /orgs/{org_id}/invites/{invite_id} | Cancel a pending user invitations to an organization.
*InvitesApi* | [**list_org_invitation**](docs/InvitesApi.md#list_org_invitation) | **GET** /orgs/{org_id}/invites | List pending user invitations to an organization.
*IssuesApi* | [**fetch_issues_per_purl**](docs/IssuesApi.md#fetch_issues_per_purl) | **GET** /orgs/{org_id}/packages/{purl}/issues | List issues for a package
*OpenApiApi* | [**get_api_version**](docs/OpenApiApi.md#get_api_version) | **GET** /openapi/{version} | 
*OpenApiApi* | [**list_api_versions**](docs/OpenApiApi.md#list_api_versions) | **GET** /openapi | 
*ProjectsApi* | [**get_org_project**](docs/ProjectsApi.md#get_org_project) | **GET** /orgs/{org_id}/projects/{project_id} | Get project by project ID.
*ProjectsApi* | [**list_org_projects**](docs/ProjectsApi.md#list_org_projects) | **GET** /orgs/{org_id}/projects | List all Projects for an Org with the given Org ID.
*ProjectsApi* | [**update_org_project**](docs/ProjectsApi.md#update_org_project) | **PATCH** /orgs/{org_id}/projects/{project_id} | Updates project by project ID.
*SbomApi* | [**get_sbom**](docs/SbomApi.md#get_sbom) | **GET** /orgs/{org_id}/projects/{project_id}/sbom | Get a project’s SBOM document


## Documentation For Models

 - [AppBot](docs/AppBot.md)
 - [AppBotRelationships](docs/AppBotRelationships.md)
 - [AppBotRelationshipsApp](docs/AppBotRelationshipsApp.md)
 - [AppData](docs/AppData.md)
 - [AppDataWithSecret](docs/AppDataWithSecret.md)
 - [AppPatchRequest](docs/AppPatchRequest.md)
 - [AppPostRequest](docs/AppPostRequest.md)
 - [AppPostResponse](docs/AppPostResponse.md)
 - [AppResourceAttributes](docs/AppResourceAttributes.md)
 - [AppResourceAttributesWithSecret](docs/AppResourceAttributesWithSecret.md)
 - [AutoDependencyUpgradeSettings](docs/AutoDependencyUpgradeSettings.md)
 - [AutoRemediationPrsSettings](docs/AutoRemediationPrsSettings.md)
 - [CommonIssueModel](docs/CommonIssueModel.md)
 - [CommonIssueModelAttributes](docs/CommonIssueModelAttributes.md)
 - [ContainerBuildArgs](docs/ContainerBuildArgs.md)
 - [Context](docs/Context.md)
 - [Coordinate](docs/Coordinate.md)
 - [CreateOrgInvitation201Response](docs/CreateOrgInvitation201Response.md)
 - [CreateOrgInvitationRequest](docs/CreateOrgInvitationRequest.md)
 - [CycloneDxComponent](docs/CycloneDxComponent.md)
 - [CycloneDxDependency](docs/CycloneDxDependency.md)
 - [CycloneDxDocument](docs/CycloneDxDocument.md)
 - [CycloneDxMetadata](docs/CycloneDxMetadata.md)
 - [CycloneDxProperty](docs/CycloneDxProperty.md)
 - [CycloneDxTool](docs/CycloneDxTool.md)
 - [CycloneDxXmlDocument](docs/CycloneDxXmlDocument.md)
 - [Error](docs/Error.md)
 - [ErrorDocument](docs/ErrorDocument.md)
 - [ErrorLink](docs/ErrorLink.md)
 - [GetApp200Response](docs/GetApp200Response.md)
 - [GetAppBots200Response](docs/GetAppBots200Response.md)
 - [GetApps200Response](docs/GetApps200Response.md)
 - [GetIacSettingsForGroup200Response](docs/GetIacSettingsForGroup200Response.md)
 - [GetIacSettingsForGroup400Response](docs/GetIacSettingsForGroup400Response.md)
 - [GetIacSettingsForGroup400ResponseErrorsInner](docs/GetIacSettingsForGroup400ResponseErrorsInner.md)
 - [GetIacSettingsForGroup400ResponseErrorsInnerLinks](docs/GetIacSettingsForGroup400ResponseErrorsInnerLinks.md)
 - [GetIacSettingsForGroup400ResponseErrorsInnerLinksAbout](docs/GetIacSettingsForGroup400ResponseErrorsInnerLinksAbout.md)
 - [GetIacSettingsForGroup400ResponseErrorsInnerLinksAboutOneOf](docs/GetIacSettingsForGroup400ResponseErrorsInnerLinksAboutOneOf.md)
 - [GetIacSettingsForGroup400ResponseErrorsInnerSource](docs/GetIacSettingsForGroup400ResponseErrorsInnerSource.md)
 - [GetIacSettingsForGroup400ResponseJsonapi](docs/GetIacSettingsForGroup400ResponseJsonapi.md)
 - [GetIacSettingsForOrg200Response](docs/GetIacSettingsForOrg200Response.md)
 - [GetOrgProject200Response](docs/GetOrgProject200Response.md)
 - [GetOrgProject200ResponseData](docs/GetOrgProject200ResponseData.md)
 - [GetUserInstalledApps200Response](docs/GetUserInstalledApps200Response.md)
 - [GroupIacSettingsRequest](docs/GroupIacSettingsRequest.md)
 - [GroupIacSettingsRequestAttributes](docs/GroupIacSettingsRequestAttributes.md)
 - [GroupIacSettingsRequestAttributesCustomRules](docs/GroupIacSettingsRequestAttributesCustomRules.md)
 - [GroupIacSettingsResponse](docs/GroupIacSettingsResponse.md)
 - [GroupIacSettingsResponseAttributes](docs/GroupIacSettingsResponseAttributes.md)
 - [GroupIacSettingsResponseAttributesCustomRules](docs/GroupIacSettingsResponseAttributesCustomRules.md)
 - [HelloWorld](docs/HelloWorld.md)
 - [HelloWorldAttributes](docs/HelloWorldAttributes.md)
 - [HelloWorldAttributesRequestSubject](docs/HelloWorldAttributesRequestSubject.md)
 - [InheritFromParent](docs/InheritFromParent.md)
 - [IssuesMeta](docs/IssuesMeta.md)
 - [IssuesResponse](docs/IssuesResponse.md)
 - [JsonApi](docs/JsonApi.md)
 - [LinkProperty](docs/LinkProperty.md)
 - [Links](docs/Links.md)
 - [ListOrgInvitation200Response](docs/ListOrgInvitation200Response.md)
 - [ListOrgProjects200Response](docs/ListOrgProjects200Response.md)
 - [ListOrgProjects200ResponseDataInner](docs/ListOrgProjects200ResponseDataInner.md)
 - [ListOrgProjects200ResponseDataInnerMeta](docs/ListOrgProjects200ResponseDataInnerMeta.md)
 - [ListOrgProjects200ResponseMeta](docs/ListOrgProjects200ResponseMeta.md)
 - [ManageSecrets200Response](docs/ManageSecrets200Response.md)
 - [ManageSecretsRequest](docs/ManageSecretsRequest.md)
 - [ManualRemediationPrsSettings](docs/ManualRemediationPrsSettings.md)
 - [NugetBuildArgs](docs/NugetBuildArgs.md)
 - [OrgIacSettingsRequest](docs/OrgIacSettingsRequest.md)
 - [OrgIacSettingsRequestAttributes](docs/OrgIacSettingsRequestAttributes.md)
 - [OrgIacSettingsRequestAttributesCustomRules](docs/OrgIacSettingsRequestAttributesCustomRules.md)
 - [OrgIacSettingsResponse](docs/OrgIacSettingsResponse.md)
 - [OrgIacSettingsResponseAttributes](docs/OrgIacSettingsResponseAttributes.md)
 - [OrgIacSettingsResponseAttributesCustomRules](docs/OrgIacSettingsResponseAttributesCustomRules.md)
 - [OrgIacSettingsResponseAttributesCustomRulesParents](docs/OrgIacSettingsResponseAttributesCustomRulesParents.md)
 - [OrgIacSettingsResponseAttributesCustomRulesParentsGroup](docs/OrgIacSettingsResponseAttributesCustomRulesParentsGroup.md)
 - [OrgInvitation](docs/OrgInvitation.md)
 - [OrgInvitationAttributes](docs/OrgInvitationAttributes.md)
 - [OrgInvitationRelationships](docs/OrgInvitationRelationships.md)
 - [PackageMeta](docs/PackageMeta.md)
 - [PaginatedLinks](docs/PaginatedLinks.md)
 - [PatchProjectRequest](docs/PatchProjectRequest.md)
 - [PatchProjectRequestData](docs/PatchProjectRequestData.md)
 - [PatchProjectRequestDataAttributes](docs/PatchProjectRequestDataAttributes.md)
 - [PatchProjectRequestDataRelationships](docs/PatchProjectRequestDataRelationships.md)
 - [PatchProjectRequestDataRelationshipsOwner](docs/PatchProjectRequestDataRelationshipsOwner.md)
 - [PatchProjectRequestDataRelationshipsOwnerData](docs/PatchProjectRequestDataRelationshipsOwnerData.md)
 - [Problem](docs/Problem.md)
 - [ProjectAttributes](docs/ProjectAttributes.md)
 - [ProjectAttributesBuildArgs](docs/ProjectAttributesBuildArgs.md)
 - [ProjectAttributesTagsInner](docs/ProjectAttributesTagsInner.md)
 - [ProjectRelationships](docs/ProjectRelationships.md)
 - [ProjectRelationshipsTarget](docs/ProjectRelationshipsTarget.md)
 - [ProjectSettings](docs/ProjectSettings.md)
 - [PublicApp](docs/PublicApp.md)
 - [PublicAppAttributes](docs/PublicAppAttributes.md)
 - [PullRequestAssignmentSettings](docs/PullRequestAssignmentSettings.md)
 - [PullRequestsSettings](docs/PullRequestsSettings.md)
 - [RecurringTestsSettings](docs/RecurringTestsSettings.md)
 - [RelatedLink](docs/RelatedLink.md)
 - [Relationship](docs/Relationship.md)
 - [RelationshipData](docs/RelationshipData.md)
 - [Remedy](docs/Remedy.md)
 - [RemedyDetails](docs/RemedyDetails.md)
 - [SbomResource](docs/SbomResource.md)
 - [SbomResponse](docs/SbomResponse.md)
 - [SelfLink](docs/SelfLink.md)
 - [Severity](docs/Severity.md)
 - [Slots](docs/Slots.md)
 - [SlotsReferencesInner](docs/SlotsReferencesInner.md)
 - [Target](docs/Target.md)
 - [TargetData](docs/TargetData.md)
 - [TargetDataAttributes](docs/TargetDataAttributes.md)
 - [UpdateIacSettingsForGroupRequest](docs/UpdateIacSettingsForGroupRequest.md)
 - [UpdateIacSettingsForOrgRequest](docs/UpdateIacSettingsForOrgRequest.md)
 - [UpdateOrgProject200Response](docs/UpdateOrgProject200Response.md)
 - [UpdateOrgProject200ResponseData](docs/UpdateOrgProject200ResponseData.md)
 - [YarnBuildArgs](docs/YarnBuildArgs.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



