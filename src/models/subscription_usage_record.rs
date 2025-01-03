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
pub struct SubscriptionUsageRecord {
    #[serde(rename = "subscriptionId")]
    pub subscription_id: uuid::Uuid,
    #[serde(rename = "trackingId", skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(rename = "unitUsageRecords")]
    pub unit_usage_records: Vec<models::UnitUsageRecord>,
}

impl SubscriptionUsageRecord {
    pub fn new(subscription_id: uuid::Uuid, unit_usage_records: Vec<models::UnitUsageRecord>) -> SubscriptionUsageRecord {
        SubscriptionUsageRecord {
            subscription_id,
            tracking_id: None,
            unit_usage_records,
        }
    }
}

