# \AppsApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_app**](AppsApi.md#create_app) | **POST** /orgs/{org_id}/apps | Create a new app for an organization.
[**delete_app**](AppsApi.md#delete_app) | **DELETE** /orgs/{org_id}/apps/{client_id} | Delete an app
[**delete_app_bot**](AppsApi.md#delete_app_bot) | **DELETE** /orgs/{org_id}/app_bots/{bot_id} | Revoke app bot authorization
[**get_app**](AppsApi.md#get_app) | **GET** /orgs/{org_id}/apps/{client_id} | Get an app by client id
[**get_app_bots**](AppsApi.md#get_app_bots) | **GET** /orgs/{org_id}/app_bots | Get a list of app bots authorized to an organization.
[**get_apps**](AppsApi.md#get_apps) | **GET** /orgs/{org_id}/apps | Get a list of apps created by an organization.
[**get_user_installed_apps**](AppsApi.md#get_user_installed_apps) | **GET** /self/apps | Get a list of apps that can act on your behalf.
[**manage_secrets**](AppsApi.md#manage_secrets) | **POST** /orgs/{org_id}/apps/{client_id}/secrets | Manage client secrets for an app.
[**revoke_user_installed_app**](AppsApi.md#revoke_user_installed_app) | **DELETE** /self/apps/{app_id} | Revoke an app
[**update_app**](AppsApi.md#update_app) | **PATCH** /orgs/{org_id}/apps/{client_id} | Update app attributes that are name, redirect URIs, and access token time to live



## create_app

> crate::models::AppPostResponse create_app(version, org_id, app_post_request)
Create a new app for an organization.

Create a new app for an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Unique identifier for an organization | [required] |
**app_post_request** | Option<[**AppPostRequest**](AppPostRequest.md)> | app to be created |  |

### Return type

[**crate::models::AppPostResponse**](AppPostResponse.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_app

> delete_app(version, org_id, client_id)
Delete an app

Delete an app by app id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Unique identifier for an organization | [required] |
**client_id** | **uuid::Uuid** | Client ID | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_app_bot

> delete_app_bot(bot_id, version, org_id)
Revoke app bot authorization

Revoke app bot authorization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | **uuid::Uuid** | The ID of the app bot | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Unique identifier for an organization | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app

> crate::models::GetApp200Response get_app(org_id, client_id, version)
Get an app by client id

Get an App by client id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Unique identifier for an organization | [required] |
**client_id** | **uuid::Uuid** | Client ID | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |

### Return type

[**crate::models::GetApp200Response**](getApp_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app_bots

> crate::models::GetAppBots200Response get_app_bots(org_id, version, expand, starting_after, ending_before, limit)
Get a list of app bots authorized to an organization.

Get a list of app bots authorized to an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Unique identifier for an organization | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Expand relationships. |  |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetAppBots200Response**](getAppBots_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_apps

> crate::models::GetApps200Response get_apps(org_id, version, starting_after, ending_before, limit)
Get a list of apps created by an organization.

Get a list of apps created by an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Unique identifier for an organization | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetApps200Response**](getApps_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_installed_apps

> crate::models::GetUserInstalledApps200Response get_user_installed_apps(version, starting_after, ending_before, limit)
Get a list of apps that can act on your behalf.

Get a list of apps that can act on your behalf.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetUserInstalledApps200Response**](getUserInstalledApps_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_secrets

> crate::models::ManageSecrets200Response manage_secrets(version, org_id, client_id, manage_secrets_request)
Manage client secrets for an app.

Manage client secrets for an app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Unique identifier for an organization | [required] |
**client_id** | **uuid::Uuid** | Client ID | [required] |
**manage_secrets_request** | Option<[**ManageSecretsRequest**](ManageSecretsRequest.md)> |  |  |

### Return type

[**crate::models::ManageSecrets200Response**](manageSecrets_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_user_installed_app

> revoke_user_installed_app(version, app_id)
Revoke an app

Revoke access for an app by app id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**app_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_app

> crate::models::GetApp200Response update_app(version, org_id, client_id, app_patch_request)
Update app attributes that are name, redirect URIs, and access token time to live

Update app attributes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Unique identifier for an organization | [required] |
**client_id** | **uuid::Uuid** | Client ID | [required] |
**app_patch_request** | Option<[**AppPatchRequest**](AppPatchRequest.md)> |  |  |

### Return type

[**crate::models::GetApp200Response**](getApp_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

