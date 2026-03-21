use crate::authority::commit::preparation::diagnostics::counters::ValidationPreparationCounters;
use crate::authority::commit::preparation::facade::PreparedInvariantExecution;
use crate::authority::commit::preparation::planning::context::PreparationPlanningContext;
use crate::authority::commit::preparation::planning::strategy::{
    packet_width_is_profitable, ParallelLegality, ParallelProfitability, PreparationFallbackReason,
    PreparationStrategy, PreparationStrategySelection, MIN_PARALLEL_PACKET_WIDTH,
};
use crate::authority::commit::preparation::proofs::kinds::PreparationProofKind;
use crate::authority::commit::preparation::proofs::locality::{
    PreparationLocalityProof, PreparationPartitionScope, PreparationReadSetApproximation,
    PreparationRecordDomain, PreparationWriteExclusionClass,
};
use crate::authority::commit::preparation::proofs::validity::PreparationProofValidity;
use crate::authority::commit::preparation::reduction::keys::ValidationReductionKey;
use crate::logic::planning::RelationalExecutionModel;
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::InvariantRegistration;
use crate::validation::engine::{
    InvariantPlanScopeClass, InvariantProofBoundarySummary, InvariantScopeWideningCause,
};
use crate::validation::engine::InvariantExecutionRequest;
use std::collections::BTreeSet;

pub(crate) type PlannedInvariantExecution<'runtime> = PreparedInvariantExecution<'runtime>;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TestPreparationFault {
    PlanningProofInsufficient,
    PublicationIsolationViolation,
    ReductionIdentityConflict,
    WorkerEvaluationFailure,
}

#[cfg(test)]
static TEST_PREPARATION_FAULT: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

#[cfg(test)]
pub(crate) fn current_test_preparation_fault() -> Option<TestPreparationFault> {
    match TEST_PREPARATION_FAULT.load(std::sync::atomic::Ordering::SeqCst) {
        1 => Some(TestPreparationFault::PlanningProofInsufficient),
        2 => Some(TestPreparationFault::PublicationIsolationViolation),
        3 => Some(TestPreparationFault::ReductionIdentityConflict),
        4 => Some(TestPreparationFault::WorkerEvaluationFailure),
        _ => None,
    }
}

#[cfg(test)]
pub(crate) fn with_test_preparation_fault<T>(
    fault: TestPreparationFault,
    run: impl FnOnce() -> T,
) -> T {
    struct ResetGuard<'a> {
        fault: &'a std::sync::atomic::AtomicU8,
        _lock: std::sync::MutexGuard<'a, ()>,
    }

    impl Drop for ResetGuard<'_> {
        fn drop(&mut self) {
            self.fault.store(0, std::sync::atomic::Ordering::SeqCst);
        }
    }

    let guard = crate::testing::fault_injection_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let _reset = ResetGuard {
        fault: &TEST_PREPARATION_FAULT,
        _lock: guard,
    };
    TEST_PREPARATION_FAULT.store(
        match fault {
            TestPreparationFault::PlanningProofInsufficient => 1,
            TestPreparationFault::PublicationIsolationViolation => 2,
            TestPreparationFault::ReductionIdentityConflict => 3,
            TestPreparationFault::WorkerEvaluationFailure => 4,
        },
        std::sync::atomic::Ordering::SeqCst,
    );
    run()
}

#[cfg(test)]
pub(crate) fn has_test_preparation_fault() -> bool {
    current_test_preparation_fault().is_some()
}

pub(crate) fn plan_invariant_execution<'runtime>(
    runtime: &'runtime RelationalRuntime,
    request: &InvariantExecutionRequest<'runtime>,
) -> PlannedInvariantExecution<'runtime> {
    let registrations = eligible_registrations(runtime, &request);
    let context = planning_context(runtime, &request);
    let proof_kind = if matches!(
        runtime.config.execution.execution_model,
        RelationalExecutionModel::StagedParallelPreparation
    ) {
        PreparationProofKind::ReadOnlyShared
    } else {
        PreparationProofKind::RequiresSerial
    };
    let packet_count = registrations.len();
    let strategy = if !matches!(
        runtime.config.execution.execution_model,
        RelationalExecutionModel::StagedParallelPreparation
    ) {
        PreparationStrategy::serial(PreparationFallbackReason::ExecutionModelSerial)
    } else if !packet_width_is_profitable(packet_count, MIN_PARALLEL_PACKET_WIDTH) {
        PreparationStrategy {
            parallel_legality: ParallelLegality::ProvenParallel,
            parallel_profitability: ParallelProfitability::NotProfitable,
            selected_mode: PreparationStrategySelection::Serial,
            fallback_reason: Some(PreparationFallbackReason::InsufficientPacketBreadth),
        }
    } else if proof_kind == PreparationProofKind::RequiresSerial {
        PreparationStrategy::serial(PreparationFallbackReason::ProofRequiresSerial)
    } else {
        PreparationStrategy {
            parallel_legality: ParallelLegality::ProvenParallel,
            parallel_profitability: ParallelProfitability::Profitable,
            selected_mode: PreparationStrategySelection::StagedParallel,
            fallback_reason: None,
        }
    };

    let packets = registrations
        .into_iter()
        .enumerate()
        .map(|(packet_index, registration)| {
            let partition_scope = packet_partition_scope(request.merged_plan());
            let invariant_group_scope = registration.rule.groups();
            let record_domain = if request.merged_plan().is_some() {
                PreparationRecordDomain::Mixed
            } else {
                PreparationRecordDomain::None
            };
            let locality = PreparationLocalityProof {
                observation_scope: request.observation().kind(),
                record_domain,
                partition_scope: if partition_scope.is_empty() {
                    PreparationPartitionScope::AllObserved
                } else {
                    PreparationPartitionScope::TouchedPartitions(partition_scope.clone())
                },
                invariant_group_scope,
                read_set_approximation: PreparationReadSetApproximation::SharedCommittedRead,
                write_exclusion: match proof_kind {
                    PreparationProofKind::RequiresSerial => {
                        PreparationWriteExclusionClass::RequiresSerialAuthority
                    }
                    _ => PreparationWriteExclusionClass::ReadOnly,
                },
            };
            let mut packet = crate::authority::commit::preparation::InvariantWorkPacket {
                packet_index,
                registration,
                reduction_key: ValidationReductionKey::new(
                    request.execution_point(),
                    request.observation().kind(),
                    partition_scope,
                    invariant_group_scope,
                    packet_index,
                ),
                proof_kind,
                locality,
                validity: PreparationProofValidity {
                    context: context.clone(),
                },
                planning_context: context.clone(),
                observation: request.observation().clone(),
                version_id: request.version_id(),
                merged_plan: request.merged_plan(),
            };
            #[cfg(test)]
            match current_test_preparation_fault() {
                Some(TestPreparationFault::PlanningProofInsufficient) => {
                    packet.validity.context.invariant_registration_count += 1;
                }
                Some(TestPreparationFault::PublicationIsolationViolation) => {
                    packet.locality.write_exclusion =
                        PreparationWriteExclusionClass::RequiresSerialAuthority;
                }
                Some(TestPreparationFault::ReductionIdentityConflict) => {}
                Some(TestPreparationFault::WorkerEvaluationFailure) => {}
                None => {}
            }
            packet
        })
        .collect();

    PreparedInvariantExecution {
        context,
        strategy,
        packets,
    }
}

pub(crate) fn planned_packet_counters(
    planned: &PreparedInvariantExecution<'_>,
) -> ValidationPreparationCounters {
    ValidationPreparationCounters {
        packet_count: planned.packets.len(),
        worker_result_count: 0,
        reducer_input_count: 0,
        reducer_conflict_count: 0,
        failure_count: 0,
    }
}

fn planning_context(
    runtime: &RelationalRuntime,
    request: &InvariantExecutionRequest<'_>,
) -> PreparationPlanningContext {
    PreparationPlanningContext {
        transaction_id: request.merged_plan().map(|plan| plan.transaction_id),
        execution_point: request.execution_point(),
        observation_kind: request.observation().kind(),
        version_id: request.version_id(),
        current_version_id: request.current_version_id(),
        structural_summary: None,
        plan_contract: request.plan_contract(),
        schema_registry_entry_count: runtime.config.schema.registry.entity_kinds.len()
            + runtime.config.schema.registry.relation_kinds.len(),
        invariant_registration_count: runtime.config.schema.invariant_catalog.registrations.len()
            + runtime.aspect_semantics.relation_integrity_registrations.len(),
        planning_contract: runtime.config.execution.planning.clone(),
    }
}

pub(crate) fn planned_proof_boundary_summary(
    planned: &PreparedInvariantExecution<'_>,
) -> InvariantProofBoundarySummary {
    let mut widened_causes = Vec::new();
    let mut touched_partitions = BTreeSet::new();
    let mut saw_touched_only = false;

    for packet in &planned.packets {
        match &packet.locality.partition_scope {
            PreparationPartitionScope::AllObserved => {
                if !widened_causes.contains(&InvariantScopeWideningCause::AllObservedPartitionScope)
                {
                    widened_causes
                        .push(InvariantScopeWideningCause::AllObservedPartitionScope);
                }
            }
            PreparationPartitionScope::TouchedPartitions(partitions) => {
                touched_partitions.extend(partitions.iter().copied());
            }
        }
        match packet.locality.read_set_approximation {
            PreparationReadSetApproximation::FullObservedScan => {
                if !widened_causes.contains(&InvariantScopeWideningCause::FullObservedReadSet) {
                    widened_causes.push(InvariantScopeWideningCause::FullObservedReadSet);
                }
            }
            PreparationReadSetApproximation::TouchedOnly => {
                saw_touched_only = true;
            }
            PreparationReadSetApproximation::SharedCommittedRead => {}
        }
    }

    let scope_class = if !widened_causes.is_empty() {
        InvariantPlanScopeClass::BroaderScope
    } else if saw_touched_only {
        InvariantPlanScopeClass::TouchedScope
    } else {
        InvariantPlanScopeClass::PartitionScope
    };

    InvariantProofBoundarySummary::new(
        scope_class,
        widened_causes,
        planned.packets.len(),
        touched_partitions.len(),
    )
}

fn eligible_registrations(
    runtime: &RelationalRuntime,
    request: &InvariantExecutionRequest<'_>,
) -> Vec<InvariantRegistration> {
    runtime
        .config
        .schema
        .invariant_catalog
        .registrations_for_execution_point(request.execution_point())
        .chain(
            runtime
                .aspect_semantics
                .relation_integrity_registrations
                .iter()
                .filter(move |registration| registration.execution_point == request.execution_point()),
        )
        .filter(|registration| request.includes_registration(registration))
        .cloned()
        .collect()
}

fn packet_partition_scope(
    merged_plan: Option<&MergedCommitPlan>,
) -> Vec<crate::identity::data::PartitionId> {
    let mut touched = BTreeSet::new();
    if let Some(plan) = merged_plan {
        for intent in &plan.merged_intents {
            intent.seed_touched_partitions(&mut touched);
        }
    }
    touched.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::{plan_invariant_execution, planned_proof_boundary_summary};
    use crate::config::data::{CascadeDeletePolicy, CrossContextPolicy};
    use crate::facade::{
        runtime::RelationalRuntimeApi,
        schema::{
            EntityKindRegistration, KindAspectDeclarations, RelationKindRegistration,
            RelationalSchemaRegistry, SchemaId, SchemaVersionId,
        },
    };
    use crate::identity::data::KindId;
    use crate::schema::data::{
        EndpointKindContractDeclaration, RelationIntegrityDeclarations, RelationPayloadClass,
    };
    use crate::identity::data::PartitionId;
    use crate::payloads::data::RecordPayload;
    use crate::transactions::data::{
        CreateIntent, EntitySpec, MergedCommitPlan, MutationIntent, RelationSpec,
        TransactionOptions, WorkerIntentBatch,
    };
    use crate::validation::data::InvariantPlanContract;
    use crate::validation::engine::{InvariantExecutionRequest, InvariantObservation, InvariantRequestProfile};
    use crate::validation::engine::{InvariantPlanScopeClass, InvariantScopeWideningCause};
    use serde_json::json;

    fn relation_runtime() -> crate::logic::runtime::RelationalRuntime {
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: KindId(1),
                kind_name: "test.entity".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::default(),
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: KindId(2),
                    kind_name: "test.edge.a".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::PayloadBearingRelation,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: RelationIntegrityDeclarations::new(
                        vec![EndpointKindContractDeclaration {
                            contract_id: "kind2".to_string(),
                            allowed_source_kinds: vec![KindId(1)],
                            allowed_target_kinds: vec![KindId(1)],
                            self_edges_allowed: false,
                            cross_context_policy: CrossContextPolicy::AllowExplicit,
                        }],
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                    ),
                })
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: KindId(3),
                    kind_name: "test.edge.b".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::PayloadBearingRelation,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: RelationIntegrityDeclarations::new(
                        vec![EndpointKindContractDeclaration {
                            contract_id: "kind3".to_string(),
                            allowed_source_kinds: vec![KindId(1)],
                            allowed_target_kinds: vec![KindId(1)],
                            self_edges_allowed: false,
                            cross_context_policy: CrossContextPolicy::AllowExplicit,
                        }],
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                    ),
                })
            })
            .unwrap();
        RelationalRuntimeApi::builder()
            .schema_registry(registry)
            .build()
    }

    fn request_for_plan<'runtime>(
        runtime: &'runtime crate::logic::runtime::RelationalRuntime,
        plan: &'runtime MergedCommitPlan,
    ) -> InvariantExecutionRequest<'runtime> {
        InvariantExecutionRequest::from_profile_with_contract(
            InvariantRequestProfile::CommitBoundary,
            runtime,
            InvariantObservation::committed(runtime.storage_access().current_state()),
            runtime.current_version_id(),
            Some(plan),
            Some(InvariantPlanContract::from_merged_plan(plan)),
        )
    }

    fn create_entity(
        runtime: &mut crate::logic::runtime::RelationalRuntime,
        name: &str,
    ) -> crate::identity::data::EntityId {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new(format!("entity-{name}")).push(MutationIntent::Create(
                CreateIntent::Entity(EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: crate::symbols::data::InternedString::Raw(name.to_string()),
                    payload: RecordPayload::StructuredJson(json!({"name": name})),
                }),
            )),
        );
        let outcome = txn.commit().unwrap();
        outcome
            .changed_records
            .iter()
            .find_map(|record| match record {
                crate::facade::transactions::RecordRef::Entity(entity_id) => Some(*entity_id),
                crate::facade::transactions::RecordRef::Relation(_) => None,
            })
            .expect("created entity")
    }

    #[test]
    fn planner_packets_only_include_relation_integrity_registrations_authorized_by_plan_scope() {
        let mut runtime = relation_runtime();
        let source = create_entity(&mut runtime, "source");
        let target = create_entity(&mut runtime, "target");
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("planned").push(MutationIntent::Create(CreateIntent::Relation(
                RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_key: crate::symbols::data::InternedString::Raw("planned".to_string()),
                    source,
                    target,
                    payload: Some(RecordPayload::StructuredJson(json!({"label":"planned"}))),
                },
            ))),
        );
        let plan = txn.merged_plan().unwrap().clone();

        let prepared = plan_invariant_execution(&runtime, &request_for_plan(&runtime, &plan));
        let packet_relation_kinds = prepared
            .packets
            .iter()
            .filter_map(|packet| match &packet.registration.rule {
                crate::validation::data::InvariantRule::EndpointKindContract(contract) => {
                    Some(contract.relation_kind_id)
                }
                crate::validation::data::InvariantRule::CardinalityContract(contract) => {
                    Some(contract.relation_kind_id)
                }
                crate::validation::data::InvariantRule::UniquenessContract(contract) => {
                    Some(contract.relation_kind_id)
                }
                crate::validation::data::InvariantRule::SymmetryContract(contract) => {
                    Some(contract.relation_kind_id)
                }
                crate::validation::data::InvariantRule::EndpointDeletionIntegrityContract(contract) => {
                    Some(contract.relation_kind_id)
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(packet_relation_kinds, vec![KindId(2)]);
        assert_eq!(prepared.packets.len(), 1);
    }

    #[test]
    fn planner_proof_boundary_reports_partition_scoped_relation_integrity_packets() {
        let mut runtime = relation_runtime();
        let source = create_entity(&mut runtime, "source");
        let target = create_entity(&mut runtime, "target");
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("planned").push(MutationIntent::Create(CreateIntent::Relation(
                RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_key: crate::symbols::data::InternedString::Raw("planned".to_string()),
                    source,
                    target,
                    payload: Some(RecordPayload::StructuredJson(json!({"label":"planned"}))),
                },
            ))),
        );
        let plan = txn.merged_plan().unwrap().clone();

        let prepared = plan_invariant_execution(&runtime, &request_for_plan(&runtime, &plan));
        let summary = planned_proof_boundary_summary(&prepared);

        assert_eq!(summary.scope_class(), InvariantPlanScopeClass::PartitionScope);
        assert!(summary.widened_causes().is_empty());
        assert_eq!(summary.packet_count(), 1);
        assert_eq!(summary.touched_partition_count(), 1);
    }

    #[test]
    fn planner_proof_boundary_reports_broader_scope_when_no_merged_plan_is_available() {
        let runtime = relation_runtime();
        let request = InvariantExecutionRequest::from_profile_with_contract(
            InvariantRequestProfile::CommitBoundary,
            &runtime,
            InvariantObservation::committed(runtime.storage_access().current_state()),
            runtime.current_version_id(),
            None,
            None,
        );

        let prepared = plan_invariant_execution(&runtime, &request);
        let summary = planned_proof_boundary_summary(&prepared);

        assert_eq!(summary.scope_class(), InvariantPlanScopeClass::BroaderScope);
        assert_eq!(
            summary.widened_causes(),
            &[InvariantScopeWideningCause::AllObservedPartitionScope]
        );
    }
}
