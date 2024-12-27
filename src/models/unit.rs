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
pub struct Unit {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "prettyName", skip_serializing_if = "Option::is_none")]
    pub pretty_name: Option<String>,
}

impl Unit {
    pub fn new() -> Unit {
        Unit {
            name: None,
            pretty_name: None,
        }
    }
}
