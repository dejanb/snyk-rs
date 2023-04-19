# AppResourceAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token_ttl_seconds** | **f32** | The access token time to live for your app, in seconds. It only affects the newly generated access tokens, existing access token will  continue to have their previous time to live as expiration. | 
**client_id** | [**uuid::Uuid**](uuid::Uuid.md) | The oauth2 client id for the app. | 
**context** | [**crate::models::Context**](Context.md) |  | 
**is_confidential** | **bool** | A boolean to indicate if an app is confidential or not as per the OAuth2 RFC. | 
**is_public** | **bool** | A boolean to indicate if an app is publicly available or not. | 
**name** | **String** | New name of the app to display to users during authorization flow. | 
**org_public_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**redirect_uris** | **Vec<String>** | List of allowed redirect URIs to call back after authentication. | 
**scopes** | **Vec<String>** | The scopes this app is allowed to request during authorization. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


