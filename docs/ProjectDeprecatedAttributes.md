# ProjectDeprecatedAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_criticality** | Option<**Vec<String>**> |  | [optional]
**created** | **String** | The date that the project was created on | 
**environment** | Option<**Vec<String>**> |  | [optional]
**lifecycle** | Option<**Vec<String>**> |  | [optional]
**name** | **String** |  | 
**origin** | **String** | The origin the project was added from | 
**status** | **String** | Describes if a project is currently monitored or it is de-activated | 
**tags** | Option<[**Vec<crate::models::Tag>**](Tag.md)> |  | [optional]
**target_reference** | Option<**String**> | The identifier for which revision of the resource is scanned by Snyk. For example this may be a branch for SCM project, or a tag for a container image | [optional]
**r#type** | **String** | The package manager of the project | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


