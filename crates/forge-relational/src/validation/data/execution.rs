use serde::{Deserialize, Serialize};

use super::groups::{InvariantCostClass, InvariantGroupSet};
use super::results::InvariantViolation;
use super::rules::InvariantRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantClass {
    AlwaysOnStructural,
    CommitBoundary,
    SnapshotAudit,
    HarnessHeavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantExecutionPoint {
    MutationSensitive,
    CommitBoundary,
    SnapshotPublication,
    HarnessAudit,
}

impl InvariantExecutionPoint {
    pub const fn class(self) -> InvariantClass {
        match self {
            Self::MutationSensitive => InvariantClass::AlwaysOnStructural,
            Self::CommitBoundary => InvariantClass::CommitBoundary,
            Self::SnapshotPublication => InvariantClass::SnapshotAudit,
            Self::HarnessAudit => InvariantClass::HarnessHeavy,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantFailureEffect {
    BlockCommit,
    BlockPublication,
    AuditOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantVerdict {
    Pass,
    Fail,
    NotApplicable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantCheckResult {
    pub class: InvariantClass,
    pub execution_point: InvariantExecutionPoint,
    pub failure_effect: InvariantFailureEffect,
    pub rule: InvariantRule,
    pub groups: InvariantGroupSet,
    pub cost: InvariantCostClass,
    pub verdict: InvariantVerdict,
    pub violations: Vec<InvariantViolation>,
}
