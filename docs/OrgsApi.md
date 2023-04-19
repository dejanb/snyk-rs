# \OrgsApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_org**](OrgsApi.md#get_org) | **GET** /orgs/{org_id} | Get an org



## get_org

> crate::models::GetOrg200Response get_org(org_id, version)
Get an org

Returns an org by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | The ID of the org | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |

### Return type

[**crate::models::GetOrg200Response**](getOrg_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

