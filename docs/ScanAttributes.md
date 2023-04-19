# ScanAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | When the scan was created | 
**deleted_at** | Option<**String**> |  | [optional]
**environment_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Environment ID | [optional]
**error** | Option<**String**> | Error message if the scan failed | 
**finished_at** | Option<**String**> | When the scan finished | [optional]
**kind** | Option<**String**> | Scan kind | 
**options** | Option<[**serde_json::Value**](.md)> |  | [optional]
**organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Organization ID | [optional]
**partial_errors** | Option<**String**> | Errors that didn't fail the scan | [optional]
**revision** | **f32** | Increment for each change to a scan | 
**status** | Option<**String**> | Scan status | 
**updated_at** | Option<**String**> | When the scan was last updated | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


