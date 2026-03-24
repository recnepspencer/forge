use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::execution::{InvariantExecutionPoint, InvariantFailureEffect};
use super::groups::{InvariantCostClass, InvariantGroupSet};
use super::rule_id::{CustomInvariantSemanticIdentity, InvariantRuleId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantSemanticsClass {
    NativeAlwaysOn,
    NativeSchemaLowered,
    CustomStructural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportedExecutionPoints {
    mask: u8,
}

impl SupportedExecutionPoints {
    pub const fn empty() -> Self {
        Self { mask: 0 }
    }

    pub const fn only(point: InvariantExecutionPoint) -> Self {
        Self {
            mask: 1 << (point as u8),
        }
    }

    pub const fn union(self, other: Self) -> Self {
        Self {
            mask: self.mask | other.mask,
        }
    }

    pub const fn supports(self, point: InvariantExecutionPoint) -> bool {
        (self.mask & (1 << (point as u8))) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomInvariantOperationalMetadata {
    pub execution_point: InvariantExecutionPoint,
    pub groups: InvariantGroupSet,
    pub cost_class: InvariantCostClass,
    pub failure_effect: InvariantFailureEffect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantRuleDescriptor {
    pub id: InvariantRuleId,
    pub execution_points: SupportedExecutionPoints,
    pub groups: InvariantGroupSet,
    pub cost_class: InvariantCostClass,
    pub failure_effect: InvariantFailureEffect,
    pub semantics: InvariantSemanticsClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomInvariantDescriptor {
    pub identity: CustomInvariantSemanticIdentity,
    pub display_name: Arc<str>,
    pub operational: CustomInvariantOperationalMetadata,
}

