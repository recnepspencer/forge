use serde::{Deserialize, Serialize};

use crate::schema::data::{
    LoweredPayloadSchemaContract, LoweredRelationIntegrityPlan, PayloadContractRecordKind,
};

use super::descriptor::InvariantRuleDescriptor;
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
    pub descriptor: InvariantRuleDescriptor,
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
        assert!(
            rule.supports_execution_point(execution_point),
            "invariant rule {:?} does not support execution point {:?}",
            rule,
            execution_point
        );
        let descriptor = rule.descriptor_for(failure_effect);
        Self {
            descriptor,
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

    pub fn certification_boundary_blocking(rule: InvariantRule) -> Self {
        Self::block_publication(rule, InvariantExecutionPoint::CertificationBoundary)
    }

    pub fn harness_audit_only(rule: InvariantRule) -> Self {
        Self::audit_only(rule, InvariantExecutionPoint::HarnessAudit)
    }

    pub(crate) fn cost(&self) -> InvariantCostClass {
        self.descriptor.cost_class
    }

    pub(crate) fn groups(&self) -> super::groups::InvariantGroupSet {
        self.descriptor.groups
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

pub(crate) fn relation_integrity_registrations_for_plan(
    plan: &LoweredRelationIntegrityPlan,
) -> Vec<InvariantRegistration> {
    let mut registrations = Vec::with_capacity(plan.contract_count());
    registrations.extend(plan.endpoint_kind_contracts.iter().cloned().map(|contract| {
        InvariantRegistration::commit_boundary_blocking(InvariantRule::EndpointKindContract(
            contract,
        ))
    }));
    registrations.extend(
        plan.cardinality_maximum_contracts
            .iter()
            .cloned()
            .map(|contract| {
                InvariantRegistration::commit_boundary_blocking(
                    InvariantRule::CardinalityMaximumContract(contract),
                )
            }),
    );
    registrations.extend(
        plan.cardinality_minimum_contracts
            .iter()
            .cloned()
            .map(|contract| match contract.minimum_enforcement {
                crate::schema::data::MinimumCardinalityEnforcement::CommitBoundary => {
                    InvariantRegistration::commit_boundary_blocking(
                        InvariantRule::CardinalityMinimumContract(contract),
                    )
                }
                crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary => {
                    InvariantRegistration::certification_boundary_blocking(
                        InvariantRule::CardinalityMinimumContract(contract),
                    )
                }
            }),
    );
    registrations.extend(plan.uniqueness_contracts.iter().cloned().map(|contract| {
        InvariantRegistration::commit_boundary_blocking(InvariantRule::UniquenessContract(
            contract,
        ))
    }));
    registrations.extend(plan.symmetry_contracts.iter().cloned().map(|contract| {
        InvariantRegistration::commit_boundary_blocking(InvariantRule::SymmetryContract(
            contract,
        ))
    }));
    registrations.extend(
        plan.endpoint_deletion_integrity_contracts
            .iter()
            .cloned()
            .map(|contract| {
                InvariantRegistration::commit_boundary_blocking(
                    InvariantRule::EndpointDeletionIntegrityContract(contract),
                )
            }),
    );
    registrations.extend(plan.acyclicity_contracts.iter().cloned().map(|contract| {
        InvariantRegistration::commit_boundary_blocking(InvariantRule::AcyclicityContract(
            contract,
        ))
    }));
    registrations.extend(
        plan.partition_isolation_contracts
            .iter()
            .cloned()
            .map(|contract| {
                InvariantRegistration::commit_boundary_blocking(
                    InvariantRule::PartitionIsolationContract(contract),
                )
            }),
    );
    registrations.extend(
        plan.connectivity_minimum_contracts
            .iter()
            .cloned()
            .map(|contract| {
                InvariantRegistration::snapshot_publication_blocking(
                    InvariantRule::ConnectivityMinimumContract(contract),
                )
            }),
    );
    registrations
}

pub(crate) fn payload_schema_registration(
    contract: LoweredPayloadSchemaContract,
) -> InvariantRegistration {
    InvariantRegistration::commit_boundary_blocking(InvariantRule::PayloadSchemaContract(
        contract,
    ))
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InvariantRegistrationContract {
    DefaultAlwaysOnStructural,
    OptInUserCatalog,
}

#[cfg(test)]
impl InvariantRule {
    pub(crate) const REGISTRATION_EXAMPLE_COUNT: usize = 16;

    pub(crate) fn registration_examples() -> Vec<Self> {
        vec![
            Self::LiveRecordRequiresSidecar(RecordKindTag::Entity),
            Self::LiveRecordRequiresSidecar(RecordKindTag::Relation),
            Self::MaxMergedIntents(1),
            Self::RelationIntegrityScopeBudget(1),
            Self::MaxSnapshotEntities(1),
            Self::UniqueEntityPayloadField("__registration_probe__".to_string()),
            Self::EndpointKindContract(crate::schema::data::LoweredEndpointKindContract {
                contract_id: "__registration_probe__".into(),
                relation_kind_id: crate::identity::data::KindId(999),
                allowed_source_kinds: vec![crate::identity::data::KindId(1)],
                allowed_target_kinds: vec![crate::identity::data::KindId(1)],
                self_edges_allowed: true,
                cross_context_policy: crate::config::data::CrossContextPolicy::AllowExplicit,
                plan_revision: crate::schema::data::RelationIntegrityPlanRevision(1),
            }),
            Self::CardinalityMaximumContract(crate::schema::data::LoweredCardinalityMaximumContract {
                contract_id: "__registration_probe__".into(),
                relation_kind_id: crate::identity::data::KindId(999),
                source_max: Some(1),
                target_max: None,
                pair_max: None,
                plan_revision: crate::schema::data::RelationIntegrityPlanRevision(1),
            }),
            Self::CardinalityMinimumContract(crate::schema::data::LoweredCardinalityMinimumContract {
                contract_id: "__registration_probe__".into(),
                relation_kind_id: crate::identity::data::KindId(999),
                source_min: Some(1),
                target_min: None,
                pair_min: None,
                pair_min_semantics: crate::schema::data::PairMinimumSemantics::ObservedDirectedPairs,
                candidate_source_kinds: vec![crate::identity::data::KindId(1)],
                candidate_target_kinds: vec![crate::identity::data::KindId(1)],
                minimum_enforcement: crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary,
                plan_revision: crate::schema::data::RelationIntegrityPlanRevision(1),
            }),
            Self::UniquenessContract(crate::schema::data::LoweredUniquenessContract {
                contract_id: "__registration_probe__".into(),
                relation_kind_id: crate::identity::data::KindId(999),
                scope: crate::schema::data::UniquenessScope::DirectedSemanticEdge,
                plan_revision: crate::schema::data::RelationIntegrityPlanRevision(1),
            }),
            Self::SymmetryContract(crate::schema::data::LoweredSymmetryContract {
                contract_id: "__registration_probe__".into(),
                relation_kind_id: crate::identity::data::KindId(999),
                mode: crate::schema::data::SymmetryMode::InverseProhibited,
                plan_revision: crate::schema::data::RelationIntegrityPlanRevision(1),
            }),
            Self::EndpointDeletionIntegrityContract(
                crate::schema::data::LoweredEndpointDeletionIntegrityContract {
                    contract_id: "__registration_probe__".into(),
                    relation_kind_id: crate::identity::data::KindId(999),
                    mode: crate::schema::data::EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations,
                    cascade_delete_policy: crate::config::data::CascadeDeletePolicy::CascadeDeleteRelations,
                    plan_revision: crate::schema::data::RelationIntegrityPlanRevision(1),
                },
            ),
            Self::AcyclicityContract(crate::schema::data::LoweredAcyclicityContract {
                contract_id: "__registration_probe__".into(),
                relation_kind_id: crate::identity::data::KindId(999),
                traversal_direction: crate::schema::data::DirectedTraversalKind::SourceToTarget,
                allowed_cycle_class: crate::schema::data::AllowedCycleClass::NoCycles,
                plan_revision: crate::schema::data::RelationIntegrityPlanRevision(1),
            }),
            Self::PayloadSchemaContract(LoweredPayloadSchemaContract {
                contract_id: "__registration_probe__".into(),
                record_kind: PayloadContractRecordKind::Entity,
                kind_id: crate::identity::data::KindId(999),
                allowed_payload_class: crate::payloads::data::PayloadClass::StructuredJson,
                field_constraints: vec![
                    crate::schema::data::PayloadFieldConstraint::Required {
                        field: "name".to_string(),
                    },
                ],
            }),
            Self::PartitionIsolationContract(
                crate::schema::data::LoweredPartitionIsolationContract {
                    contract_id: "__registration_probe__".into(),
                    relation_kind_id: crate::identity::data::KindId(999),
                    isolation_mode: crate::schema::data::PartitionIsolationMode::SamePartitionEndpoints,
                    plan_revision: crate::schema::data::RelationIntegrityPlanRevision(1),
                },
            ),
            Self::ConnectivityMinimumContract(
                crate::schema::data::LoweredConnectivityMinimumContract {
                    contract_id: "__registration_probe__".into(),
                    source_kind_ids: vec![crate::identity::data::KindId(1)],
                    relation_kind_id: crate::identity::data::KindId(999),
                    target_kind_ids: vec![crate::identity::data::KindId(2)],
                    minimum_reachable_targets: 1,
                    enforcement_boundary: crate::schema::data::ConnectivityMinimumEnforcement::SnapshotPublication,
                    plan_revision: crate::schema::data::RelationIntegrityPlanRevision(1),
                },
            ),
        ]
    }

    pub(crate) fn registration_contract(&self) -> InvariantRegistrationContract {
        match self {
            Self::LiveRecordRequiresSidecar(_) => {
                InvariantRegistrationContract::DefaultAlwaysOnStructural
            }
            Self::MaxMergedIntents(_)
            | Self::RelationIntegrityScopeBudget(_)
            | Self::MaxSnapshotEntities(_)
            | Self::UniqueEntityPayloadField(_)
            | Self::EndpointKindContract(_)
            | Self::CardinalityMaximumContract(_)
            | Self::CardinalityMinimumContract(_)
            | Self::UniquenessContract(_)
            | Self::SymmetryContract(_)
            | Self::EndpointDeletionIntegrityContract(_)
            | Self::AcyclicityContract(_)
            | Self::PayloadSchemaContract(_)
            | Self::PartitionIsolationContract(_)
            | Self::ConnectivityMinimumContract(_) => {
                InvariantRegistrationContract::OptInUserCatalog
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InvariantCatalog, InvariantRegistrationContract};
    use crate::validation::data::{InvariantExecutionPoint, InvariantRegistration, InvariantRule};

    #[test]
    fn every_invariant_variant_has_a_registration_contract() {
        let catalog = InvariantCatalog::default();
        assert_eq!(
            InvariantRule::registration_examples().len(),
            InvariantRule::REGISTRATION_EXAMPLE_COUNT
        );

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

    #[test]
    fn every_invariant_variant_supports_at_least_one_execution_point_and_can_register() {
        let execution_points = [
            InvariantExecutionPoint::MutationSensitive,
            InvariantExecutionPoint::CommitBoundary,
            InvariantExecutionPoint::SnapshotPublication,
            InvariantExecutionPoint::CertificationBoundary,
            InvariantExecutionPoint::HarnessAudit,
        ];

        for rule in InvariantRule::registration_examples() {
            let supported_points = execution_points
                .into_iter()
                .filter(|point| rule.supports_execution_point(*point))
                .collect::<Vec<_>>();
            assert!(
                !supported_points.is_empty(),
                "invariant rule {:?} does not support any execution point",
                rule
            );
            for point in supported_points {
                let registration =
                    InvariantRegistration::for_rule(
                        rule.clone(),
                        point,
                        crate::validation::data::InvariantFailureEffect::BlockCommit,
                    );
                assert_eq!(registration.execution_point, point);
                assert_eq!(registration.rule, rule);
            }
        }
    }
}
