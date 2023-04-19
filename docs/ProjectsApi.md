# \ProjectsApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_org_project**](ProjectsApi.md#get_org_project) | **GET** /orgs/{org_id}/projects/{project_id} | Get project by project ID.
[**list_org_projects**](ProjectsApi.md#list_org_projects) | **GET** /orgs/{org_id}/projects | List all Projects for an Org with the given Org ID.
[**update_org_project**](ProjectsApi.md#update_org_project) | **PATCH** /orgs/{org_id}/projects/{project_id} | Updates project by project ID.



## get_org_project

> crate::models::GetOrgProject200Response get_org_project(org_id, project_id, version, expand)
Get project by project ID.

Get one project of the organization by project ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | The ID of the org to which the project belongs to. | [required] |
**project_id** | **uuid::Uuid** | The ID of the project. | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Expand relationships. |  |

### Return type

[**crate::models::GetOrgProject200Response**](getOrgProject_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_org_projects

> crate::models::ListOrgProjects200Response list_org_projects(org_id, version, meta_count, ids, names, origins, types, expand, cli_monitored_before, cli_monitored_after, starting_after, ending_before, limit)
List all Projects for an Org with the given Org ID.

List all Projects for an Org.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | The ID of the org that the projects belong to. | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**meta_count** | Option<**String**> | The collection count. |  |
**ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Return projects that match the provided IDs. |  |
**names** | Option<[**Vec<String>**](String.md)> | Return projects that match the provided names. |  |
**origins** | Option<[**Vec<String>**](String.md)> | Return projects that match the provided origins. |  |
**types** | Option<[**Vec<String>**](String.md)> | Return projects that match the provided types. |  |
**expand** | Option<[**Vec<String>**](String.md)> | Expand relationships. |  |
**cli_monitored_before** | Option<**String**> | Filter projects uploaded and monitored before this date (encoded value) |  |
**cli_monitored_after** | Option<**String**> | Filter projects uploaded and monitored after this date (encoded value) |  |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::ListOrgProjects200Response**](listOrgProjects_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org_project

> crate::models::UpdateOrgProject200Response update_org_project(version, org_id, project_id, user_id, expand, patch_project_request)
Updates project by project ID.

Updates one project of the organization by project ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | The ID of the Org the project belongs to. | [required] |
**project_id** | **uuid::Uuid** | The ID of the project to patch. | [required] |
**user_id** | **uuid::Uuid** | The ID of user initiating the action. | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Expand relationships. |  |
**patch_project_request** | Option<[**PatchProjectRequest**](PatchProjectRequest.md)> | The project attributes to be updated. |  |

### Return type

[**crate::models::UpdateOrgProject200Response**](updateOrgProject_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

