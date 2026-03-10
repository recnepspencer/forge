use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantFailureEffect {
    BlockCommit,
    BlockPublication,
    AuditOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantViolation {
    pub class: InvariantClass,
    pub code: crate::diagnostics::data::DiagnosticCode,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageInvariantReport {
    pub violations: Vec<InvariantViolation>,
}

impl StorageInvariantReport {
    pub fn is_clean(&self) -> bool {
        self.violations.is_empty()
    }

    pub fn for_class(&self, class: InvariantClass) -> Vec<&InvariantViolation> {
        self.violations
            .iter()
            .filter(|violation| violation.class == class)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantCheckResult {
    pub class: InvariantClass,
    pub execution_point: InvariantExecutionPoint,
    pub failure_effect: InvariantFailureEffect,
    pub violations: Vec<InvariantViolation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantRule {
    LiveEntityRequiresKind,
    LiveRelationRequiresEndpoints,
    MaxMergedIntents(usize),
    MaxSnapshotEntities(usize),
    UniqueEntityPayloadField(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantCatalog {
    pub always_on_structural: Vec<InvariantRule>,
    pub commit_boundary: Vec<InvariantRule>,
    pub snapshot_audit: Vec<InvariantRule>,
    pub harness_heavy: Vec<InvariantRule>,
}

impl Default for InvariantCatalog {
    fn default() -> Self {
        Self {
            always_on_structural: vec![
                InvariantRule::LiveEntityRequiresKind,
                InvariantRule::LiveRelationRequiresEndpoints,
            ],
            commit_boundary: Vec::new(),
            snapshot_audit: Vec::new(),
            harness_heavy: Vec::new(),
        }
    }
}
