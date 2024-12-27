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
pub struct AdminPayment {
    #[serde(rename = "lastSuccessPaymentState", skip_serializing_if = "Option::is_none")]
    pub last_success_payment_state: Option<String>,
    #[serde(rename = "currentPaymentStateName", skip_serializing_if = "Option::is_none")]
    pub current_payment_state_name: Option<String>,
    #[serde(rename = "transactionStatus", skip_serializing_if = "Option::is_none")]
    pub transaction_status: Option<String>,
}

impl AdminPayment {
    pub fn new() -> AdminPayment {
        AdminPayment {
            last_success_payment_state: None,
            current_payment_state_name: None,
            transaction_status: None,
        }
    }
}
