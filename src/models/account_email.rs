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
pub struct AccountEmail {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<uuid::Uuid>,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "auditLogs", skip_serializing_if = "Option::is_none")]
    pub audit_logs: Option<Vec<models::AuditLog>>,
}

impl AccountEmail {
    pub fn new(email: String) -> AccountEmail {
        AccountEmail {
            account_id: None,
            email,
            audit_logs: None,
        }
    }
}

