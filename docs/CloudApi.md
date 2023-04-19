# \CloudApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_environment**](CloudApi.md#create_environment) | **POST** /orgs/{org_id}/cloud/environments | Create New Environment
[**create_scan**](CloudApi.md#create_scan) | **POST** /orgs/{org_id}/cloud/scans | Create Scan
[**delete_environment**](CloudApi.md#delete_environment) | **DELETE** /orgs/{org_id}/cloud/environments/{environment_id} | Delete Environment
[**get_permissions**](CloudApi.md#get_permissions) | **POST** /orgs/{org_id}/cloud/permissions | Generate Cloud Provider Permissions
[**list_environments**](CloudApi.md#list_environments) | **GET** /orgs/{org_id}/cloud/environments | List Environments
[**list_resources**](CloudApi.md#list_resources) | **GET** /orgs/{org_id}/cloud/resources | List Resources
[**list_scan**](CloudApi.md#list_scan) | **GET** /orgs/{org_id}/cloud/scans | List Scans
[**update_environment**](CloudApi.md#update_environment) | **PATCH** /orgs/{org_id}/cloud/environments/{environment_id} | Update Environment



## create_environment

> crate::models::CreateEnvironment201Response create_environment(version, org_id, create_environment_request)
Create New Environment

Create a new environment and run a scan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Organization ID | [required] |
**create_environment_request** | Option<[**CreateEnvironmentRequest**](CreateEnvironmentRequest.md)> |  |  |

### Return type

[**crate::models::CreateEnvironment201Response**](createEnvironment_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_scan

> crate::models::CreateScan201Response create_scan(version, org_id, create_scan_request)
Create Scan

Create and trigger a new scan for an environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Organization ID | [required] |
**create_scan_request** | Option<[**CreateScanRequest**](CreateScanRequest.md)> |  |  |

### Return type

[**crate::models::CreateScan201Response**](createScan_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_environment

> delete_environment(version, org_id, environment_id)
Delete Environment

Delete an environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Organization ID | [required] |
**environment_id** | **uuid::Uuid** | Unique identifier for an environment | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permissions

> crate::models::GetPermissions201Response get_permissions(version, org_id, get_permissions_request)
Generate Cloud Provider Permissions

Generate IAC template for Snyk to access your cloud resources

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Organization ID | [required] |
**get_permissions_request** | Option<[**GetPermissionsRequest**](GetPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::GetPermissions201Response**](getPermissions_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_environments

> crate::models::ListEnvironments200Response list_environments(version, org_id, created_after, created_before, updated_after, updated_before, name, kind, status, id, starting_after, ending_before, limit)
List Environments

List environments for an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Organization ID | [required] |
**created_after** | Option<**String**> | Return environments created after this date |  |
**created_before** | Option<**String**> | Return environments created before this date |  |
**updated_after** | Option<**String**> | Return environments updated after this date |  |
**updated_before** | Option<**String**> | Return environments updated before this date |  |
**name** | Option<**String**> | Filter environments by name (multi-value, comma-separated) |  |
**kind** | Option<**String**> | Filter environments by kind (multi-value, comma-separated): aws |  |
**status** | Option<**String**> | Filter environments by latest scan status (multi-value, comma-separated): queued, in_progress, success, error |  |
**id** | Option<**uuid::Uuid**> | Filter environments by environment ID (multi-value, comma-separated) |  |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::ListEnvironments200Response**](listEnvironments_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_resources

> crate::models::ListResources200Response list_resources(org_id, version, environment_id, resource_type, resource_id, native_id, id, platform, name, kind, location, removed, starting_after, ending_before, limit)
List Resources

List resources for an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Organization ID | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**environment_id** | Option<**String**> | Filter resources by environment ID (multi-value, comma-separated) |  |
**resource_type** | Option<**String**> | Filter resources by resource type (multi-value, comma-separated) |  |
**resource_id** | Option<**String**> | Filter resources by resource ID (multi-value, comma-separated) |  |
**native_id** | Option<**String**> | Filter resources by native ID (multi-value, comma-separated) (AWS ARN) |  |
**id** | Option<**String**> | Filter resources by resource UUID (multi-value, comma-separated) |  |
**platform** | Option<**String**> | Filter resources by platform (multi-value, comma-separated): aws |  |
**name** | Option<**String**> | Filter resources by name (multi-value, comma-separated) |  |
**kind** | Option<**String**> | Filter resources by kind (multi-value, comma-separated): cloud |  |
**location** | Option<**String**> | Filter resources by location (multi-value, comma-separated) (AWS region) |  |
**removed** | Option<**bool**> | Filter resources by whether they have been removed or not. |  |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::ListResources200Response**](listResources_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scan

> crate::models::ListScan200Response list_scan(version, org_id, starting_after, ending_before, limit)
List Scans

List scans for an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Organization ID | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::ListScan200Response**](listScan_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_environment

> crate::models::UpdateEnvironment200Response update_environment(version, org_id, environment_id, update_environment_request)
Update Environment

Update an environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Organization ID | [required] |
**environment_id** | **uuid::Uuid** | Unique identifier for an environment | [required] |
**update_environment_request** | Option<[**UpdateEnvironmentRequest**](UpdateEnvironmentRequest.md)> |  |  |

### Return type

[**crate::models::UpdateEnvironment200Response**](updateEnvironment_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

