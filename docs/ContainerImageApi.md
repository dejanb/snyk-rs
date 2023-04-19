# \ContainerImageApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_container_image**](ContainerImageApi.md#create_container_image) | **POST** /orgs/{org_id}/container_images | Create a new container image
[**get_container_image**](ContainerImageApi.md#get_container_image) | **GET** /orgs/{org_id}/container_images/{image_id} | Get instance of container image
[**list_container_image**](ContainerImageApi.md#list_container_image) | **GET** /orgs/{org_id}/container_images | List instances of container image
[**update_container_image**](ContainerImageApi.md#update_container_image) | **PATCH** /orgs/{org_id}/container_images/{image_id} | Update an instance of container image



## create_container_image

> crate::models::CreateContainerImage201Response create_container_image(version, org_id, create_container_image_request)
Create a new container image

Create a new container image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Organization ID | [required] |
**create_container_image_request** | Option<[**CreateContainerImageRequest**](CreateContainerImageRequest.md)> |  |  |

### Return type

[**crate::models::CreateContainerImage201Response**](createContainerImage_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_container_image

> crate::models::CreateContainerImage201Response get_container_image(version, org_id, image_id)
Get instance of container image

Get instance of container image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Organization ID | [required] |
**image_id** | **String** | Image ID | [required] |

### Return type

[**crate::models::CreateContainerImage201Response**](createContainerImage_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_container_image

> crate::models::ListContainerImage200Response list_container_image(org_id, version, image_ids, platform, names, limit, starting_after, ending_before)
List instances of container image

List instances of container image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Organization ID | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**image_ids** | Option<[**Vec<String>**](String.md)> | A comma-separated list of Image IDs |  |
**platform** | Option<[**Platform**](.md)> | The image Operating System and processor architecture |  |
**names** | Option<[**Vec<String>**](String.md)> | The container registry names |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |

### Return type

[**crate::models::ListContainerImage200Response**](listContainerImage_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_container_image

> update_container_image(version, org_id, image_id, update_container_image_request)
Update an instance of container image

Update an instance of container image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Organization ID | [required] |
**image_id** | **String** | Image ID | [required] |
**update_container_image_request** | Option<[**UpdateContainerImageRequest**](UpdateContainerImageRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

