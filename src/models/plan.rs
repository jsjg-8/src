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
pub struct Plan {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "prettyName", skip_serializing_if = "Option::is_none")]
    pub pretty_name: Option<String>,
    #[serde(rename = "recurringBillingMode", skip_serializing_if = "Option::is_none")]
    pub recurring_billing_mode: Option<RecurringBillingMode>,
    #[serde(rename = "billingPeriod", skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<BillingPeriod>,
    #[serde(rename = "phases", skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<models::Phase>>,
}

impl Plan {
    pub fn new() -> Plan {
        Plan {
            name: None,
            pretty_name: None,
            recurring_billing_mode: None,
            billing_period: None,
            phases: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecurringBillingMode {
    #[serde(rename = "IN_ADVANCE")]
    Advance,
    #[serde(rename = "IN_ARREAR")]
    Arrear,
}

impl Default for RecurringBillingMode {
    fn default() -> RecurringBillingMode {
        Self::Advance
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BillingPeriod {
    #[serde(rename = "DAILY")]
    Daily,
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "BIWEEKLY")]
    Biweekly,
    #[serde(rename = "THIRTY_DAYS")]
    ThirtyDays,
    #[serde(rename = "THIRTY_ONE_DAYS")]
    ThirtyOneDays,
    #[serde(rename = "SIXTY_DAYS")]
    SixtyDays,
    #[serde(rename = "NINETY_DAYS")]
    NinetyDays,
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "BIMESTRIAL")]
    Bimestrial,
    #[serde(rename = "QUARTERLY")]
    Quarterly,
    #[serde(rename = "TRIANNUAL")]
    Triannual,
    #[serde(rename = "BIANNUAL")]
    Biannual,
    #[serde(rename = "ANNUAL")]
    Annual,
    #[serde(rename = "SESQUIENNIAL")]
    Sesquiennial,
    #[serde(rename = "BIENNIAL")]
    Biennial,
    #[serde(rename = "TRIENNIAL")]
    Triennial,
    #[serde(rename = "NO_BILLING_PERIOD")]
    NoBillingPeriod,
}

impl Default for BillingPeriod {
    fn default() -> BillingPeriod {
        Self::Daily
    }
}

