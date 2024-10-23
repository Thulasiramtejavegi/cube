/*
 * Cube.js
 *
 * Cube.js Swagger Schema
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1CubeMetaDimensionGranularity {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

impl V1CubeMetaDimensionGranularity {
    pub fn new(name: String, title: String, interval: String) -> V1CubeMetaDimensionGranularity {
        V1CubeMetaDimensionGranularity {
            name,
            title,
            interval,
            offset: None,
            origin: None,
        }
    }
}