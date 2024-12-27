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
pub struct AccountTimeline {
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<Box<models::Account>>,
    #[serde(rename = "bundles", skip_serializing_if = "Option::is_none")]
    pub bundles: Option<Vec<models::Bundle>>,
    #[serde(rename = "invoices", skip_serializing_if = "Option::is_none")]
    pub invoices: Option<Vec<models::Invoice>>,
    #[serde(rename = "payments", skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<models::InvoicePayment>>,
}

impl AccountTimeline {
    pub fn new() -> AccountTimeline {
        AccountTimeline {
            account: None,
            bundles: None,
            invoices: None,
            payments: None,
        }
    }
}

