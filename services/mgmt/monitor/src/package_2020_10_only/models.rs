#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroup {
    #[serde(rename = "actionGroupId")]
    pub action_group_id: String,
    #[serde(rename = "webhookProperties", default, skip_serializing_if = "Option::is_none")]
    pub webhook_properties: Option<serde_json::Value>,
}
impl ActionGroup {
    pub fn new(action_group_id: String) -> Self {
        Self {
            action_group_id,
            webhook_properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionList {
    #[serde(rename = "actionGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub action_groups: Vec<ActionGroup>,
}
impl ActionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityLogAlertResource {
    #[serde(flatten)]
    pub azure_resource: AzureResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertRuleProperties>,
}
impl ActivityLogAlertResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleAllOfCondition {
    #[serde(rename = "allOf")]
    pub all_of: Vec<AlertRuleAnyOfOrLeafCondition>,
}
impl AlertRuleAllOfCondition {
    pub fn new(all_of: Vec<AlertRuleAnyOfOrLeafCondition>) -> Self {
        Self { all_of }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleAnyOfOrLeafCondition {
    #[serde(flatten)]
    pub alert_rule_leaf_condition: AlertRuleLeafCondition,
    #[serde(rename = "anyOf", default, skip_serializing_if = "Vec::is_empty")]
    pub any_of: Vec<AlertRuleLeafCondition>,
}
impl AlertRuleAnyOfOrLeafCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleLeafCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,
    #[serde(rename = "containsAny", default, skip_serializing_if = "Vec::is_empty")]
    pub contains_any: Vec<String>,
}
impl AlertRuleLeafCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ActivityLogAlertResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AlertRuleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRulePatchObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertRulePatchProperties>,
}
impl AlertRulePatchObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRulePatchProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl AlertRulePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleProperties {
    pub scopes: Vec<String>,
    pub condition: AlertRuleAllOfCondition,
    pub actions: ActionList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AlertRuleProperties {
    pub fn new(scopes: Vec<String>, condition: AlertRuleAllOfCondition, actions: ActionList) -> Self {
        Self {
            scopes,
            condition,
            actions,
            enabled: None,
            description: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AzureResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
