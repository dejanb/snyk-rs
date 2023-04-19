# \GroupsApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user**](GroupsApi.md#delete_user) | **DELETE** /groups/{group_id}/sso_connections/{sso_id}/users/{user_id} | Delete a user from a Group SSO connection
[**get_group**](GroupsApi.md#get_group) | **GET** /groups/{group_id} | Get a group
[**list_group_sso_connection_users**](GroupsApi.md#list_group_sso_connection_users) | **GET** /groups/{group_id}/sso_connections/{sso_id}/users | Get all users using a given SSO connection
[**list_group_sso_connections**](GroupsApi.md#list_group_sso_connections) | **GET** /groups/{group_id}/sso_connections | Get all SSO connections for a group
[**list_groups**](GroupsApi.md#list_groups) | **GET** /groups | Get all groups



## delete_user

> delete_user(group_id, sso_id, user_id, version)
Delete a user from a Group SSO connection

Deletes a user from a Group SSO connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** | The ID of the group | [required] |
**sso_id** | **uuid::Uuid** | The ID of the SSO | [required] |
**user_id** | **uuid::Uuid** | The ID of the User | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group

> crate::models::GetGroup200Response get_group(group_id, version)
Get a group

Returns a group by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** | The ID of the group | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |

### Return type

[**crate::models::GetGroup200Response**](getGroup_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group_sso_connection_users

> crate::models::ListGroupSsoConnectionUsers200Response list_group_sso_connection_users(group_id, sso_id, version, starting_after, ending_before, limit)
Get all users using a given SSO connection

Returns a list of users for a SSO connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** | The ID of the group | [required] |
**sso_id** | **uuid::Uuid** | The ID of the SSO | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::ListGroupSsoConnectionUsers200Response**](listGroupSsoConnectionUsers_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group_sso_connections

> crate::models::ListGroupSsoConnections200Response list_group_sso_connections(group_id, version, starting_after, ending_before, limit)
Get all SSO connections for a group

Returns a list of SSO connections for a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** | The ID of the group | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::ListGroupSsoConnections200Response**](listGroupSsoConnections_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_groups

> crate::models::ListGroups200Response list_groups(version, starting_after, ending_before, limit)
Get all groups

Returns a list of groups which a user is a member of

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::ListGroups200Response**](listGroups_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

