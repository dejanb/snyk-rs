# ResourceAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | When the resource was first recorded | 
**deleted_at** | Option<**String**> |  | [optional]
**hash** | **String** | Computed hash value for the resource based on its attributes | 
**is_managed** | Option<**bool**> |  | [optional]
**kind** | **String** | Kind of resource: cloud | 
**location** | Option<**String**> | Physical location (AWS region) | [optional]
**name** | Option<**String**> | Human friendly resource name | [optional]
**namespace** | Option<**String**> | Resource namespace (AWS region) | [optional]
**native_id** | Option<**String**> | ID of the physical resource from the cloud provider (AWS ARN, if available) | [optional]
**platform** | **String** | Resource platform: aws | 
**relationships** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**removed_at** | Option<**String**> |  | [optional]
**resource_id** | **String** | Unique ID for the resource | 
**resource_type** | **String** | Terraform resource type | 
**revision** | **i32** | Increment for each change to a resource | 
**schema_version** | Option<**String**> |  | [optional]
**source_location** | Option<[**Vec<::std::collections::HashMap<String, serde_json::Value>>**](map.md)> |  | [optional]
**state** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Terraform state attributes | [optional]
**tags** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Resource tags from the cloud provider | [optional]
**updated_at** | Option<**String**> | When the resource was last updated | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


