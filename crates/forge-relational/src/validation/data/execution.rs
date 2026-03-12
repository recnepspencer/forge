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

    pub const fn diagnostic_label(self) -> &'static str {
        match self {
            Self::MutationSensitive => "mutation_sensitive",
            Self::CommitBoundary => "commit_boundary",
            Self::SnapshotPublication => "snapshot_publication",
            Self::HarnessAudit => "harness_audit",
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
    pub execution_point: InvariantExecutionPoint,
    pub failure_effect: InvariantFailureEffect,
    pub rule: InvariantRule,
    pub verdict: InvariantVerdict,
    pub violations: Vec<InvariantViolation>,
}

impl InvariantCheckResult {
    pub fn class(&self) -> InvariantClass {
        self.execution_point.class()
    }

    pub fn groups(&self) -> InvariantGroupSet {
        self.rule.groups()
    }

    pub fn cost(&self) -> InvariantCostClass {
        self.rule.cost_class()
    }
}
