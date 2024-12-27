/*
 * Kill Bill
 *
 * Kill Bill is an open-source billing and payments platform
 *
 * The version of the OpenAPI document: 0.24.10
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogValidationError {
    #[serde(rename = "errorDescription", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}

impl CatalogValidationError {
    pub fn new() -> CatalogValidationError {
        CatalogValidationError {
            error_description: None,
        }
    }
}
