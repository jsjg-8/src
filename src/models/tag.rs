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
pub struct Tag {
    #[serde(rename = "tagId", skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<uuid::Uuid>,
    #[serde(rename = "objectType", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectType>,
    #[serde(rename = "objectId", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<uuid::Uuid>,
    #[serde(rename = "tagDefinitionId", skip_serializing_if = "Option::is_none")]
    pub tag_definition_id: Option<uuid::Uuid>,
    #[serde(rename = "tagDefinitionName", skip_serializing_if = "Option::is_none")]
    pub tag_definition_name: Option<String>,
    #[serde(rename = "auditLogs", skip_serializing_if = "Option::is_none")]
    pub audit_logs: Option<Vec<models::AuditLog>>,
}

impl Tag {
    pub fn new() -> Tag {
        Tag {
            tag_id: None,
            object_type: None,
            object_id: None,
            tag_definition_id: None,
            tag_definition_name: None,
            audit_logs: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ObjectType {
    #[serde(rename = "ACCOUNT")]
    Account,
    #[serde(rename = "ACCOUNT_EMAIL")]
    AccountEmail,
    #[serde(rename = "BLOCKING_STATES")]
    BlockingStates,
    #[serde(rename = "BUNDLE")]
    Bundle,
    #[serde(rename = "CUSTOM_FIELD")]
    CustomField,
    #[serde(rename = "INVOICE")]
    Invoice,
    #[serde(rename = "PAYMENT")]
    Payment,
    #[serde(rename = "TRANSACTION")]
    Transaction,
    #[serde(rename = "INVOICE_ITEM")]
    InvoiceItem,
    #[serde(rename = "INVOICE_PAYMENT")]
    InvoicePayment,
    #[serde(rename = "SUBSCRIPTION")]
    Subscription,
    #[serde(rename = "SUBSCRIPTION_EVENT")]
    SubscriptionEvent,
    #[serde(rename = "SERVICE_BROADCAST")]
    ServiceBroadcast,
    #[serde(rename = "PAYMENT_ATTEMPT")]
    PaymentAttempt,
    #[serde(rename = "PAYMENT_METHOD")]
    PaymentMethod,
    #[serde(rename = "TAG")]
    Tag,
    #[serde(rename = "TAG_DEFINITION")]
    TagDefinition,
    #[serde(rename = "TENANT")]
    Tenant,
    #[serde(rename = "TENANT_KVS")]
    TenantKvs,
}

impl Default for ObjectType {
    fn default() -> ObjectType {
        Self::Account
    }
}

