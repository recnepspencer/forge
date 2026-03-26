use serde::{Deserialize, Serialize};

use super::custom_rule::CustomInvariantProvenance;
use super::groups::{InvariantCostClass, InvariantGroupSet};
use super::results::{InvariantAdvisory, InvariantViolation};
use super::rule_id::CustomInvariantSemanticIdentity;
use super::rules::InvariantRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InvariantClass {
    AlwaysOnStructural,
    CommitBoundary,
    SnapshotAudit,
    HarnessHeavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InvariantExecutionPoint {
    MutationSensitive,
    CommitBoundary,
    SnapshotPublication,
    CertificationBoundary,
    HarnessAudit,
}

impl InvariantExecutionPoint {
    pub const COUNT: usize = 5;

    pub const fn class(self) -> InvariantClass {
        match self {
            Self::MutationSensitive => InvariantClass::AlwaysOnStructural,
            Self::CommitBoundary => InvariantClass::CommitBoundary,
            Self::SnapshotPublication => InvariantClass::SnapshotAudit,
            Self::CertificationBoundary => InvariantClass::SnapshotAudit,
            Self::HarnessAudit => InvariantClass::HarnessHeavy,
        }
    }

    pub const fn diagnostic_label(self) -> &'static str {
        match self {
            Self::MutationSensitive => "mutation_sensitive",
            Self::CommitBoundary => "commit_boundary",
            Self::SnapshotPublication => "snapshot_publication",
            Self::CertificationBoundary => "certification_boundary",
            Self::HarnessAudit => "harness_audit",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InvariantFailureEffect {
    BlockCommit,
    BlockPublication,
    AuditOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InvariantVerdict {
    Pass,
    Advisory {
        violation: InvariantViolation,
        advisory: InvariantAdvisory,
    },
    Violation(InvariantViolation),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum InvariantReportedRule {
    Native(InvariantRule),
    Custom(CustomInvariantSemanticIdentity),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct InvariantWitnessKey(String);

impl InvariantWitnessKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn pass() -> Self {
        Self("pass".to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantCheckResult {
    pub execution_point: InvariantExecutionPoint,
    pub failure_effect: InvariantFailureEffect,
    pub rule: InvariantReportedRule,
    pub witness: InvariantWitnessKey,
    pub groups: InvariantGroupSet,
    pub cost: InvariantCostClass,
    pub custom_provenance: Option<CustomInvariantProvenance>,
    pub verdict: InvariantVerdict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantDecisionKind {
    Passed,
    Advisory,
    Violated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantDecisionRecord {
    pub execution_point: InvariantExecutionPoint,
    pub failure_effect: InvariantFailureEffect,
    pub rule: InvariantReportedRule,
    pub witness: InvariantWitnessKey,
    pub decision: InvariantDecisionKind,
    pub groups: InvariantGroupSet,
    pub cost: InvariantCostClass,
    pub custom_provenance_present: bool,
}

impl InvariantCheckResult {
    pub fn class(&self) -> InvariantClass {
        self.execution_point.class()
    }

    pub fn groups(&self) -> InvariantGroupSet {
        self.groups
    }

    pub fn cost(&self) -> InvariantCostClass {
        self.cost
    }

    pub fn custom_provenance(&self) -> Option<&CustomInvariantProvenance> {
        self.custom_provenance.as_ref()
    }

    pub fn witness(&self) -> &InvariantWitnessKey {
        &self.witness
    }

    pub fn decision_record(&self) -> InvariantDecisionRecord {
        InvariantDecisionRecord {
            execution_point: self.execution_point,
            failure_effect: self.failure_effect,
            rule: self.rule.clone(),
            witness: self.witness.clone(),
            decision: match self.verdict {
                InvariantVerdict::Pass => InvariantDecisionKind::Passed,
                InvariantVerdict::Advisory { .. } => InvariantDecisionKind::Advisory,
                InvariantVerdict::Violation(_) => InvariantDecisionKind::Violated,
            },
            groups: self.groups,
            cost: self.cost,
            custom_provenance_present: self.custom_provenance.is_some(),
        }
    }
}
