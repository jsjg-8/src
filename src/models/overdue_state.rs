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
pub struct OverdueState {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "externalMessage", skip_serializing_if = "Option::is_none")]
    pub external_message: Option<String>,
    #[serde(rename = "isDisableEntitlementAndChangesBlocked", skip_serializing_if = "Option::is_none")]
    pub is_disable_entitlement_and_changes_blocked: Option<bool>,
    #[serde(rename = "isBlockChanges", skip_serializing_if = "Option::is_none")]
    pub is_block_changes: Option<bool>,
    #[serde(rename = "isClearState", skip_serializing_if = "Option::is_none")]
    pub is_clear_state: Option<bool>,
    #[serde(rename = "reevaluationIntervalDays", skip_serializing_if = "Option::is_none")]
    pub reevaluation_interval_days: Option<i32>,
}

impl OverdueState {
    pub fn new() -> OverdueState {
        OverdueState {
            name: None,
            external_message: None,
            is_disable_entitlement_and_changes_blocked: None,
            is_block_changes: None,
            is_clear_state: None,
            reevaluation_interval_days: None,
        }
    }
}

