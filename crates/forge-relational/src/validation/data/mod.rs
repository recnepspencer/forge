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
pub enum RecordKindTag {
    Entity,
    Relation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantRule {
    LiveRecordRequiresSidecar(RecordKindTag),
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
                InvariantRule::LiveRecordRequiresSidecar(RecordKindTag::Entity),
                InvariantRule::LiveRecordRequiresSidecar(RecordKindTag::Relation),
            ],
            commit_boundary: Vec::new(),
            snapshot_audit: Vec::new(),
            harness_heavy: Vec::new(),
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InvariantRegistration {
    DefaultAlwaysOnStructural,
    OptInUserCatalog,
}

#[cfg(test)]
impl InvariantRule {
    pub(crate) fn registration_examples() -> Vec<Self> {
        vec![
            Self::LiveRecordRequiresSidecar(RecordKindTag::Entity),
            Self::LiveRecordRequiresSidecar(RecordKindTag::Relation),
            Self::MaxMergedIntents(1),
            Self::MaxSnapshotEntities(1),
            Self::UniqueEntityPayloadField("__registration_probe__".to_string()),
        ]
    }

    pub(crate) fn registration_contract(&self) -> InvariantRegistration {
        match self {
            Self::LiveRecordRequiresSidecar(_) => InvariantRegistration::DefaultAlwaysOnStructural,
            Self::MaxMergedIntents(_)
            | Self::MaxSnapshotEntities(_)
            | Self::UniqueEntityPayloadField(_) => InvariantRegistration::OptInUserCatalog,
        }
    }

    pub(crate) fn same_registration_kind(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::LiveRecordRequiresSidecar(left),
                Self::LiveRecordRequiresSidecar(right),
            ) => left == right,
            (Self::MaxMergedIntents(_), Self::MaxMergedIntents(_))
            | (Self::MaxSnapshotEntities(_), Self::MaxSnapshotEntities(_))
            | (
                Self::UniqueEntityPayloadField(_),
                Self::UniqueEntityPayloadField(_),
            ) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
impl InvariantCatalog {
    pub(crate) fn contains_registration_kind(&self, rule: &InvariantRule) -> bool {
        self.always_on_structural
            .iter()
            .chain(self.commit_boundary.iter())
            .chain(self.snapshot_audit.iter())
            .chain(self.harness_heavy.iter())
            .any(|registered| registered.same_registration_kind(rule))
    }
}

#[cfg(test)]
mod tests {
    use super::{InvariantCatalog, InvariantRegistration, InvariantRule};

    #[test]
    fn every_invariant_variant_has_a_registration_contract() {
        let catalog = InvariantCatalog::default();

        for rule in InvariantRule::registration_examples() {
            match rule.registration_contract() {
                InvariantRegistration::DefaultAlwaysOnStructural => {
                    assert!(
                        catalog.contains_registration_kind(&rule),
                        "default invariant rule {:?} is not registered in the default catalog",
                        rule
                    );
                }
                InvariantRegistration::OptInUserCatalog => {
                    assert!(
                        !catalog.contains_registration_kind(&rule),
                        "opt-in invariant rule {:?} should not be silently pre-registered",
                        rule
                    );
                }
            }
        }
    }
}
