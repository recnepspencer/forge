use serde::{Deserialize, Serialize};

use super::execution::{InvariantClass, InvariantExecutionPoint, InvariantFailureEffect};
use super::contracts::InvariantPlanContract;
use super::groups::{InvariantCostClass, InvariantGroupSet};
use super::rules::{InvariantRule, RecordKindTag};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantCatalog {
    pub registrations: Vec<InvariantRegistration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantRegistration {
    pub rule: InvariantRule,
    pub execution_point: InvariantExecutionPoint,
    pub failure_effect: InvariantFailureEffect,
    pub groups: InvariantGroupSet,
    pub cost: InvariantCostClass,
}

impl InvariantRegistration {
    pub fn for_rule(
        rule: InvariantRule,
        execution_point: InvariantExecutionPoint,
        failure_effect: InvariantFailureEffect,
    ) -> Self {
        debug_assert!(rule.supports_execution_point(execution_point));
        Self {
            groups: rule.groups(),
            cost: rule.cost_class(),
            rule,
            execution_point,
            failure_effect,
        }
    }

    pub fn block_commit(
        rule: InvariantRule,
        execution_point: InvariantExecutionPoint,
    ) -> Self {
        Self::for_rule(rule, execution_point, InvariantFailureEffect::BlockCommit)
    }

    pub fn block_publication(
        rule: InvariantRule,
        execution_point: InvariantExecutionPoint,
    ) -> Self {
        Self::for_rule(rule, execution_point, InvariantFailureEffect::BlockPublication)
    }

    pub fn audit_only(
        rule: InvariantRule,
        execution_point: InvariantExecutionPoint,
    ) -> Self {
        Self::for_rule(rule, execution_point, InvariantFailureEffect::AuditOnly)
    }

    pub fn class(&self) -> InvariantClass {
        self.execution_point.class()
    }

    pub fn matches_groups(&self, groups: InvariantGroupSet) -> bool {
        self.groups.intersects(groups)
    }

    pub fn applies_to_contract(&self, contract: Option<InvariantPlanContract>) -> bool {
        self.rule.applies_to_contract(contract)
    }
}

impl Default for InvariantCatalog {
    fn default() -> Self {
        Self {
            registrations: vec![
                InvariantRegistration::block_commit(
                    InvariantRule::LiveRecordRequiresSidecar(RecordKindTag::Entity),
                    InvariantExecutionPoint::MutationSensitive,
                ),
                InvariantRegistration::block_commit(
                    InvariantRule::LiveRecordRequiresSidecar(RecordKindTag::Relation),
                    InvariantExecutionPoint::MutationSensitive,
                ),
            ],
        }
    }
}

impl InvariantCatalog {
    pub fn registrations_for_execution_point(
        &self,
        execution_point: InvariantExecutionPoint,
    ) -> Vec<InvariantRegistration> {
        self.registrations
            .iter()
            .filter(|registration| registration.execution_point == execution_point)
            .cloned()
            .collect()
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
