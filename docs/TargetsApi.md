# \TargetsApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_orgs_target**](TargetsApi.md#delete_orgs_target) | **DELETE** /orgs/{org_id}/targets/{target_id} | Delete target by target ID
[**get_orgs_target**](TargetsApi.md#get_orgs_target) | **GET** /orgs/{org_id}/targets/{target_id} | Get target by org ID
[**get_orgs_targets**](TargetsApi.md#get_orgs_targets) | **GET** /orgs/{org_id}/targets | Get targets by org ID



## delete_orgs_target

> delete_orgs_target(version, org_id, target_id)
Delete target by target ID

Delete the specified target. Will fail if the target has any projects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | The id of the org to return the target from | [required] |
**target_id** | **uuid::Uuid** | The id of the target to return | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgs_target

> crate::models::GetOrgsTarget200Response get_orgs_target(version, org_id, target_id)
Get target by org ID

Get a specified target for an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | The id of the org to return the target from | [required] |
**target_id** | **uuid::Uuid** | The id of the target to return | [required] |

### Return type

[**crate::models::GetOrgsTarget200Response**](getOrgsTarget_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgs_targets

> crate::models::GetOrgsTargets200Response get_orgs_targets(version, org_id, starting_after, ending_before, limit, is_private, remote_url, origin, exclude_empty, display_name)
Get targets by org ID

Get a list of an organization's targets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | The id of the org to return a list of targets | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]
**is_private** | Option<**bool**> | Return targets that match the provided value of isPrivate |  |
**remote_url** | Option<**String**> | Return targets that match the provided remoteUrl. Currently null for all projects except those imported from the CLI |  |
**origin** | Option<**String**> | Return targets that match the provided origin |  |
**exclude_empty** | Option<**bool**> | Whether to exclude targets from the response that have no associated projects |  |[default to true]
**display_name** | Option<**String**> | Return targets with display names starting with the provided string |  |

### Return type

[**crate::models::GetOrgsTargets200Response**](getOrgsTargets_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

