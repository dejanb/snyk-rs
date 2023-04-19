# \IssuesApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_issues_per_purl**](IssuesApi.md#fetch_issues_per_purl) | **GET** /orgs/{org_id}/packages/{purl}/issues | List issues for a package



## fetch_issues_per_purl

> crate::models::IssuesResponse fetch_issues_per_purl(version, purl, org_id, offset, limit)
List issues for a package

Query issues for a specific package version identified by Package URL (purl). Snyk returns only direct vulnerabilities. Transitive vulnerabilities (from dependencies) are not returned because they can vary depending on context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**purl** | **String** | A URI-encoded  Package URL (purl) . Supported purl types are npm, maven, cocoapods, composer, gem, nuget, pypi, hex, cargo, and generic. A version for the package is also required. | [required] |
**org_id** | **uuid::Uuid** | Unique identifier for an organization | [required] |
**offset** | Option<**f32**> | Specify the number of results to skip before returning results. Must be greater than or equal to 0. Default is 0. |  |
**limit** | Option<**f32**> | Specify the number of results to return. Must be greater than 0 and less than 1000. Default is 1000. |  |

### Return type

[**crate::models::IssuesResponse**](IssuesResponse.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

