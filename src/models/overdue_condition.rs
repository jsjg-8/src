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
pub struct OverdueCondition {
    #[serde(rename = "timeSinceEarliestUnpaidInvoiceEqualsOrExceeds", skip_serializing_if = "Option::is_none")]
    pub time_since_earliest_unpaid_invoice_equals_or_exceeds: Option<Box<models::Duration>>,
    #[serde(rename = "controlTagInclusion", skip_serializing_if = "Option::is_none")]
    pub control_tag_inclusion: Option<ControlTagInclusion>,
    #[serde(rename = "controlTagExclusion", skip_serializing_if = "Option::is_none")]
    pub control_tag_exclusion: Option<ControlTagExclusion>,
    #[serde(rename = "numberOfUnpaidInvoicesEqualsOrExceeds", skip_serializing_if = "Option::is_none")]
    pub number_of_unpaid_invoices_equals_or_exceeds: Option<i32>,
    #[serde(rename = "responseForLastFailedPayment", skip_serializing_if = "Option::is_none")]
    pub response_for_last_failed_payment: Option<Vec<ResponseForLastFailedPayment>>,
    #[serde(rename = "totalUnpaidInvoiceBalanceEqualsOrExceeds", skip_serializing_if = "Option::is_none")]
    pub total_unpaid_invoice_balance_equals_or_exceeds: Option<f64>,
}

impl OverdueCondition {
    pub fn new() -> OverdueCondition {
        OverdueCondition {
            time_since_earliest_unpaid_invoice_equals_or_exceeds: None,
            control_tag_inclusion: None,
            control_tag_exclusion: None,
            number_of_unpaid_invoices_equals_or_exceeds: None,
            response_for_last_failed_payment: None,
            total_unpaid_invoice_balance_equals_or_exceeds: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ControlTagInclusion {
    #[serde(rename = "AUTO_PAY_OFF")]
    AutoPayOff,
    #[serde(rename = "AUTO_INVOICING_OFF")]
    AutoInvoicingOff,
    #[serde(rename = "OVERDUE_ENFORCEMENT_OFF")]
    OverdueEnforcementOff,
    #[serde(rename = "WRITTEN_OFF")]
    WrittenOff,
    #[serde(rename = "MANUAL_PAY")]
    ManualPay,
    #[serde(rename = "TEST")]
    Test,
    #[serde(rename = "PARTNER")]
    Partner,
    #[serde(rename = "AUTO_INVOICING_DRAFT")]
    AutoInvoicingDraft,
    #[serde(rename = "AUTO_INVOICING_REUSE_DRAFT")]
    AutoInvoicingReuseDraft,
}

impl Default for ControlTagInclusion {
    fn default() -> ControlTagInclusion {
        Self::AutoPayOff
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ControlTagExclusion {
    #[serde(rename = "AUTO_PAY_OFF")]
    AutoPayOff,
    #[serde(rename = "AUTO_INVOICING_OFF")]
    AutoInvoicingOff,
    #[serde(rename = "OVERDUE_ENFORCEMENT_OFF")]
    OverdueEnforcementOff,
    #[serde(rename = "WRITTEN_OFF")]
    WrittenOff,
    #[serde(rename = "MANUAL_PAY")]
    ManualPay,
    #[serde(rename = "TEST")]
    Test,
    #[serde(rename = "PARTNER")]
    Partner,
    #[serde(rename = "AUTO_INVOICING_DRAFT")]
    AutoInvoicingDraft,
    #[serde(rename = "AUTO_INVOICING_REUSE_DRAFT")]
    AutoInvoicingReuseDraft,
}

impl Default for ControlTagExclusion {
    fn default() -> ControlTagExclusion {
        Self::AutoPayOff
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResponseForLastFailedPayment {
    #[serde(rename = "INVALID_CARD")]
    InvalidCard,
    #[serde(rename = "EXPIRED_CARD")]
    ExpiredCard,
    #[serde(rename = "LOST_OR_STOLEN_CARD")]
    LostOrStolenCard,
    #[serde(rename = "DO_NOT_HONOR")]
    DoNotHonor,
    #[serde(rename = "INSUFFICIENT_FUNDS")]
    InsufficientFunds,
    #[serde(rename = "DECLINE")]
    Decline,
    #[serde(rename = "PROCESSING_ERROR")]
    ProcessingError,
    #[serde(rename = "INVALID_AMOUNT")]
    InvalidAmount,
    #[serde(rename = "DUPLICATE_TRANSACTION")]
    DuplicateTransaction,
    #[serde(rename = "OTHER")]
    Other,
}

impl Default for ResponseForLastFailedPayment {
    fn default() -> ResponseForLastFailedPayment {
        Self::InvalidCard
    }
}

