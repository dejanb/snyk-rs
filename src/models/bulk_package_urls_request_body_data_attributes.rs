/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BulkPackageUrlsRequestBodyDataAttributes {
    /// An array of Package URLs (purl). Supported purl types are npm, maven, cocoapods, composer, gem, nuget, pypi, hex, cargo, and generic. A version for the package is also required.
    #[serde(rename = "purls")]
    pub purls: Vec<String>,
}

impl BulkPackageUrlsRequestBodyDataAttributes {
    pub fn new(purls: Vec<String>) -> BulkPackageUrlsRequestBodyDataAttributes {
        BulkPackageUrlsRequestBodyDataAttributes {
            purls,
        }
    }
}


