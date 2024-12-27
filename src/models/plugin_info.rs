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
pub struct PluginInfo {
    #[serde(rename = "bundleSymbolicName", skip_serializing_if = "Option::is_none")]
    pub bundle_symbolic_name: Option<String>,
    #[serde(rename = "pluginKey", skip_serializing_if = "Option::is_none")]
    pub plugin_key: Option<String>,
    #[serde(rename = "pluginName", skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "isSelectedForStart", skip_serializing_if = "Option::is_none")]
    pub is_selected_for_start: Option<bool>,
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<models::PluginServiceInfo>>,
}

impl PluginInfo {
    pub fn new() -> PluginInfo {
        PluginInfo {
            bundle_symbolic_name: None,
            plugin_key: None,
            plugin_name: None,
            version: None,
            state: None,
            is_selected_for_start: None,
            services: None,
        }
    }
}

