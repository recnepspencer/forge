use serde::{Deserialize, Serialize};

use super::execution::{InvariantExecutionPoint, InvariantFailureEffect};
use super::groups::InvariantCostClass;
use super::results::{InvariantAdvisory, InvariantViolation};
use super::rules::{InvariantRule, RecordKindTag};
use super::InvariantVerdict;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantCatalog {
    pub registrations: Vec<InvariantRegistration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantRegistration {
    pub rule: InvariantRule,
    pub execution_point: InvariantExecutionPoint,
    pub failure_effect: InvariantFailureEffect,
}

impl InvariantRegistration {
    pub(crate) fn for_rule(
        rule: InvariantRule,
        execution_point: InvariantExecutionPoint,
        failure_effect: InvariantFailureEffect,
    ) -> Self {
        debug_assert!(rule.supports_execution_point(execution_point));
        Self {
            rule,
            execution_point,
            failure_effect,
        }
    }

    pub(crate) fn block_commit(
        rule: InvariantRule,
        execution_point: InvariantExecutionPoint,
    ) -> Self {
        Self::for_rule(rule, execution_point, InvariantFailureEffect::BlockCommit)
    }

    pub(crate) fn block_publication(
        rule: InvariantRule,
        execution_point: InvariantExecutionPoint,
    ) -> Self {
        Self::for_rule(
            rule,
            execution_point,
            InvariantFailureEffect::BlockPublication,
        )
    }

    pub(crate) fn audit_only(
        rule: InvariantRule,
        execution_point: InvariantExecutionPoint,
    ) -> Self {
        Self::for_rule(rule, execution_point, InvariantFailureEffect::AuditOnly)
    }

    pub fn mutation_sensitive_blocking(rule: InvariantRule) -> Self {
        Self::block_commit(rule, InvariantExecutionPoint::MutationSensitive)
    }

    pub fn commit_boundary_blocking(rule: InvariantRule) -> Self {
        Self::block_commit(rule, InvariantExecutionPoint::CommitBoundary)
    }

    pub fn snapshot_publication_blocking(rule: InvariantRule) -> Self {
        Self::block_publication(rule, InvariantExecutionPoint::SnapshotPublication)
    }

    pub fn harness_audit_only(rule: InvariantRule) -> Self {
        Self::audit_only(rule, InvariantExecutionPoint::HarnessAudit)
    }

    pub(crate) fn cost(&self) -> InvariantCostClass {
        self.rule.cost_class()
    }

    pub(crate) fn verdict_for_violation(&self, violation: InvariantViolation) -> InvariantVerdict {
        match self.failure_effect {
            InvariantFailureEffect::AuditOnly => InvariantVerdict::Advisory {
                violation,
                advisory: InvariantAdvisory::AuditOnly,
            },
            InvariantFailureEffect::BlockCommit | InvariantFailureEffect::BlockPublication => {
                InvariantVerdict::Violation(violation)
            }
        }
    }
}

impl Default for InvariantCatalog {
    fn default() -> Self {
        Self {
            registrations: vec![
                InvariantRegistration::mutation_sensitive_blocking(
                    InvariantRule::LiveRecordRequiresSidecar(RecordKindTag::Entity),
                ),
                InvariantRegistration::mutation_sensitive_blocking(
                    InvariantRule::LiveRecordRequiresSidecar(RecordKindTag::Relation),
                ),
            ],
        }
    }
}

impl InvariantCatalog {
    pub(crate) fn registrations_for_execution_point(
        &self,
        execution_point: InvariantExecutionPoint,
    ) -> impl Iterator<Item = &InvariantRegistration> {
        self.registrations
            .iter()
            .filter(move |registration| registration.execution_point == execution_point)
    }

    #[cfg(test)]
    pub(crate) fn contains_registration_kind(&self, rule: &InvariantRule) -> bool {
        self.registrations
            .iter()
            .any(|registration| registration.rule.same_registration_kind(rule))
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InvariantRegistrationContract {
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

    pub(crate) fn registration_contract(&self) -> InvariantRegistrationContract {
        match self {
            Self::LiveRecordRequiresSidecar(_) => {
                InvariantRegistrationContract::DefaultAlwaysOnStructural
            }
            Self::MaxMergedIntents(_)
            | Self::MaxSnapshotEntities(_)
            | Self::UniqueEntityPayloadField(_) => InvariantRegistrationContract::OptInUserCatalog,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InvariantCatalog, InvariantRegistrationContract};
    use crate::validation::data::InvariantRule;

    #[test]
    fn every_invariant_variant_has_a_registration_contract() {
        let catalog = InvariantCatalog::default();

        for rule in InvariantRule::registration_examples() {
            match rule.registration_contract() {
                InvariantRegistrationContract::DefaultAlwaysOnStructural => {
                    assert!(
                        catalog.contains_registration_kind(&rule),
                        "default invariant rule {:?} is not registered in the default catalog",
                        rule
                    );
                }
                InvariantRegistrationContract::OptInUserCatalog => {
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
