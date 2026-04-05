use crate::logic::runtime::RelationalRuntime;
use crate::validation::execution::{
    evaluate_invariant_packet, plan_invariant_execution, planned_proof_boundary_summary,
};
use crate::validation::reduction::reduce_invariant_execution;
use rayon::prelude::*;
use std::collections::BTreeSet;

use super::request::InvariantExecutionRequest;
use super::result::InvariantExecutionResult;

pub(crate) struct InvariantEngine<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> InvariantEngine<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub(crate) fn execute(
        &self,
        request: InvariantExecutionRequest<'runtime>,
    ) -> InvariantExecutionResult {
        let mut work_plan =
            crate::authority::commit::preparation::planning::work_plan::empty_preparation_work_plan(
            );
        work_plan.invariant_execution = Some(plan_invariant_execution(self.runtime, &request));
        self.record_preparation_plan(&work_plan);
        let planned = work_plan
            .invariant_execution
            .as_ref()
            .expect("validation work plan must include invariant execution");
        let envelopes = match planned.strategy.selected_mode {
            crate::authority::commit::preparation::planning::strategy::PreparationStrategySelection::Serial => {
                planned
                    .packets
                    .iter()
                    .map(|packet| evaluate_invariant_packet(self.runtime, packet))
                    .collect()
            }
            crate::authority::commit::preparation::planning::strategy::PreparationStrategySelection::StagedParallel => {
                planned
                    .packets
                    .par_iter()
                    .map(|packet| evaluate_invariant_packet(self.runtime, packet))
                    .collect()
            }
        };
        let proof_boundary = planned_proof_boundary_summary(planned);
        let (result, _, reducer_conflicts) =
            reduce_invariant_execution(&request, planned.strategy, proof_boundary, envelopes);
        if !reducer_conflicts.is_empty() {
            self.runtime
                .performance_access()
                .count_preparation_reducer_conflicts(reducer_conflicts.len());
        }
        result
    }
}

impl InvariantEngine<'_> {
    fn record_preparation_plan(
        &self,
        work_plan: &crate::authority::commit::preparation::PreparationWorkPlan<'_>,
    ) {
        let Some(planned) = work_plan.invariant_execution.as_ref() else {
            return;
        };
        let performance = self.runtime.performance_access();
        let counters = crate::validation::execution::planned_packet_counters(planned);
        let scope_units = if planned.packets.iter().any(|packet| {
            matches!(
                packet.locality.partition_scope,
                crate::authority::commit::preparation::proofs::locality::PreparationPartitionScope::AllObserved
            )
        }) {
            1
        } else {
            let mut touched = BTreeSet::new();
            for packet in &planned.packets {
                if let crate::authority::commit::preparation::proofs::locality::PreparationPartitionScope::TouchedPartitions(
                    partitions,
                ) = &packet.locality.partition_scope
                {
                    touched.extend(partitions.iter().copied());
                }
            }
            touched.len()
        };
        performance.count_preparation_packet_shape(
            counters.packet_count,
            counters.packet_count,
            usize::from(counters.packet_count > 0),
            scope_units,
        );
        debug_assert!(planned
            .packets
            .iter()
            .all(|packet| packet.planning_context == planned.context));
        match planned.strategy.parallel_legality {
            crate::authority::commit::preparation::planning::strategy::ParallelLegality::ProvenParallel => {
                performance.count_preparation_parallel_legal();
            }
            crate::authority::commit::preparation::planning::strategy::ParallelLegality::RequiresSerial => {}
        }
        match planned.strategy.parallel_profitability {
            crate::authority::commit::preparation::planning::strategy::ParallelProfitability::Profitable => {
                performance.count_preparation_parallel_profitable();
            }
            crate::authority::commit::preparation::planning::strategy::ParallelProfitability::NotProfitable => {}
        }
        match planned.strategy.selected_mode {
            crate::authority::commit::preparation::planning::strategy::PreparationStrategySelection::Serial => {
                performance.count_preparation_serial_strategy();
            }
            crate::authority::commit::preparation::planning::strategy::PreparationStrategySelection::StagedParallel => {
                performance.count_preparation_staged_parallel_strategy();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::super::{InvariantExecutionRequest, InvariantObservation, InvariantRequestProfile};
    use super::InvariantEngine;
    use crate::config::data::RelationIntegrityScopeBudget;
    use crate::config::data::{CascadeDeletePolicy, CrossContextPolicy};
    use crate::facade::identity::{PartitionId, RelationId};
    use crate::facade::runtime::{InvariantCatalog, InvariantRegistration, InvariantRule};
    use crate::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
    use crate::facade::schema::{
        EntityKindRegistration, KindAspectDeclarations, RelationKindRegistration,
        RelationPayloadClass, RelationalSchemaRegistry, SchemaId, SchemaVersionId,
    };
    use crate::facade::transactions::{
        CreateIntent, DeleteRelationIntent, MergedCommitPlan, MutationIntent,
        RelationMutationIntent, TransactionId,
    };
    use crate::identity::data::KindId;
    use crate::payloads::data::RecordPayload;
    use crate::schema::data::{
        AcyclicityContractDeclaration, AllowedCycleClass, ConnectivityMinimumContractDeclaration,
        ConnectivityMinimumEnforcement, DirectedTraversalKind,
        PartitionIsolationContractDeclaration, PartitionIsolationMode,
        PayloadFieldConstraintDeclaration, PayloadSchemaDeclaration, PayloadSchemaValueType,
        RelationIntegrityDeclarations,
    };
    use crate::symbols::data::InternedString;
    use crate::transactions::data::{
        EntitySpec, RelationSpec, TransactionOptions, WorkerIntentBatch,
    };
    use crate::validation::data::{
        CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
        CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
        CustomInvariantRegistration, CustomInvariantRule, CustomInvariantRuleId,
        CustomInvariantScopePlanner, CustomInvariantSemanticIdentity,
        CustomInvariantSemanticVersion, CustomInvariantVerdict, InvariantGroup, InvariantGroupSet,
        InvariantReportedRule, InvariantViolationFields,
    };
    use serde_json::json;

    fn runtime_with_invariants(invariant_catalog: InvariantCatalog) -> RelationalRuntime {
        RelationalRuntimeApi::builder()
            .schema_registry(RelationalSchemaRegistry::new())
            .invariant_catalog(invariant_catalog)
            .build()
    }

    fn runtime_with_payload_schema_and_partition_isolation() -> RelationalRuntime {
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: KindId(1),
                kind_name: "geom.vertex".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::default().with_payload_schema(
                    PayloadSchemaDeclaration {
                        contract_id: "vertex_payload".into(),
                        allowed_payload_class: crate::payloads::data::PayloadClass::StructuredJson,
                        field_constraints: vec![
                            PayloadFieldConstraintDeclaration::Required {
                                field: "name".to_string(),
                            },
                            PayloadFieldConstraintDeclaration::Type {
                                field: "rank".to_string(),
                                expected: PayloadSchemaValueType::Number,
                            },
                        ],
                    },
                ),
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: KindId(2),
                    kind_name: "geom.edge".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::PayloadBearingRelation,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: RelationIntegrityDeclarations::default()
                        .with_partition_isolation_contracts(vec![
                            PartitionIsolationContractDeclaration {
                                contract_id: "same_partition".into(),
                                isolation_mode: PartitionIsolationMode::SamePartitionEndpoints,
                            },
                        ]),
                })
            })
            .unwrap();

        RelationalRuntimeApi::builder()
            .schema_registry(registry)
            .build()
    }

    fn runtime_with_cardinality_minimum() -> RelationalRuntime {
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: KindId(1),
                kind_name: "geom.node".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::default(),
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: KindId(2),
                    kind_name: "geom.edge".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::TopologyOnlyRelation,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: RelationIntegrityDeclarations::new(
                        vec![crate::schema::data::EndpointKindContractDeclaration {
                            contract_id: "node_domains".into(),
                            allowed_source_kinds: vec![KindId(1)],
                            allowed_target_kinds: vec![KindId(1)],
                            self_edges_allowed: false,
                            cross_context_policy: CrossContextPolicy::AllowExplicit,
                        }],
                        vec![crate::schema::data::CardinalityContractDeclaration {
                            contract_id: "min_one".into(),
                            source_max: None,
                            source_min: Some(1),
                            target_max: None,
                            target_min: None,
                            pair_max: None,
                            pair_min: None,
                            pair_min_semantics: crate::schema::data::PairMinimumSemantics::ObservedDirectedPairs,
                            minimum_enforcement:
                                crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary,
                        }],
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

    fn acyclicity_and_connectivity_registry() -> RelationalSchemaRegistry {
        RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: KindId(1),
                kind_name: "geom.node".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::default(),
            })
            .and_then(|registry| {
                registry.register_entity_kind(EntityKindRegistration {
                    kind_id: KindId(3),
                    kind_name: "geom.anchor".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    aspect_declarations: KindAspectDeclarations::default(),
                })
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: KindId(2),
                    kind_name: "geom.constraint".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::TopologyOnlyRelation,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: RelationIntegrityDeclarations::default()
                        .with_acyclicity_contracts(vec![AcyclicityContractDeclaration {
                            contract_id: "no_cycles".into(),
                            traversal_direction: DirectedTraversalKind::SourceToTarget,
                            allowed_cycle_class: AllowedCycleClass::NoCycles,
                        }])
                        .with_connectivity_minimum_contracts(vec![
                            ConnectivityMinimumContractDeclaration {
                                contract_id: "reachable_anchor".into(),
                                source_kind_ids: vec![KindId(1)],
                                target_kind_ids: vec![KindId(3)],
                                minimum_reachable_targets: 1,
                                enforcement_boundary:
                                    ConnectivityMinimumEnforcement::SnapshotPublication,
                            },
                        ]),
                })
            })
            .unwrap()
    }

    fn runtime_with_acyclicity_and_connectivity() -> RelationalRuntime {
        RelationalRuntimeApi::builder()
            .schema_registry(acyclicity_and_connectivity_registry())
            .build()
    }

    fn runtime_with_acyclicity_and_connectivity_budget(
        relation_integrity_scope_budget: RelationIntegrityScopeBudget,
    ) -> RelationalRuntime {
        RelationalRuntimeApi::builder()
            .schema_registry(acyclicity_and_connectivity_registry())
            .relation_integrity_scope_budget(relation_integrity_scope_budget)
            .build()
    }

    fn create_entity_of_kind(
        runtime: &mut RelationalRuntime,
        kind_id: KindId,
        client_key: &str,
    ) -> crate::identity::data::EntityId {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(WorkerIntentBatch::new(format!("entity-{client_key}")).push(
            MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id,
                client_key: InternedString::Raw(client_key.to_string()),
                payload: RecordPayload::StructuredJson(json!({ "name": client_key })),
            })),
        ));
        let outcome = txn.commit().expect("entity creation must succeed");
        outcome
            .changed_records
            .iter()
            .find_map(|record| match record {
                crate::facade::transactions::RecordRef::Entity(entity_id) => Some(*entity_id),
                crate::facade::transactions::RecordRef::Relation(_) => None,
            })
            .expect("created entity id")
    }

    fn create_relation_of_kind(
        runtime: &mut RelationalRuntime,
        kind_id: KindId,
        source: crate::identity::data::EntityId,
        target: crate::identity::data::EntityId,
        client_key: &str,
    ) -> RelationId {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new(format!("relation-{client_key}")).push(MutationIntent::Create(
                CreateIntent::Relation(RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id,
                    client_key: InternedString::Raw(client_key.to_string()),
                    source,
                    target,
                    payload: None,
                }),
            )),
        );
        let outcome = txn.commit().expect("relation creation must succeed");
        outcome
            .changed_records
            .iter()
            .find_map(|record| match record {
                crate::facade::transactions::RecordRef::Relation(relation_id) => Some(*relation_id),
                crate::facade::transactions::RecordRef::Entity(_) => None,
            })
            .expect("created relation id")
    }

    struct AlwaysViolatesCustomRule;
    struct StructuralSurfaceRule;
    struct PanicDuringPrepareRule;
    struct PanicDuringEvaluateRule;

    impl CustomInvariantRule for AlwaysViolatesCustomRule {
        type Scope = ();

        fn descriptor(&self) -> CustomInvariantDescriptor {
            CustomInvariantDescriptor {
                identity: CustomInvariantSemanticIdentity {
                    rule_id: CustomInvariantRuleId::new("test.custom.violation"),
                    semantic_version: CustomInvariantSemanticVersion::new(1, 0),
                },
                display_name: Arc::from("Test Custom Violation"),
                operational: CustomInvariantOperationalMetadata {
                    execution_point:
                        crate::validation::data::InvariantExecutionPoint::CommitBoundary,
                    groups: InvariantGroupSet::of(InvariantGroup::SchemaCompliance),
                    cost_class: crate::validation::data::InvariantCostClass::Touched,
                    failure_effect: crate::validation::data::InvariantFailureEffect::BlockCommit,
                },
            }
        }

        fn prepare_scope(
            &self,
            _planner: &mut CustomInvariantScopePlanner<'_>,
        ) -> Result<Self::Scope, CustomInvariantPreparationError> {
            Ok(())
        }

        fn evaluate(
            &self,
            _context: &CustomInvariantExecutionContext<'_>,
            _scope: &Self::Scope,
        ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError> {
            Ok(CustomInvariantVerdict::Violation)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct StructuralScope {
        visible_entities: usize,
        planned_relations: usize,
        touched_partitions: usize,
    }

    impl CustomInvariantRule for StructuralSurfaceRule {
        type Scope = StructuralScope;

        fn descriptor(&self) -> CustomInvariantDescriptor {
            CustomInvariantDescriptor {
                identity: CustomInvariantSemanticIdentity {
                    rule_id: CustomInvariantRuleId::new("test.custom.structural-surface"),
                    semantic_version: CustomInvariantSemanticVersion::new(1, 0),
                },
                display_name: Arc::from("Structural Surface Rule"),
                operational: CustomInvariantOperationalMetadata {
                    execution_point:
                        crate::validation::data::InvariantExecutionPoint::CommitBoundary,
                    groups: InvariantGroupSet::of(InvariantGroup::SchemaCompliance),
                    cost_class: crate::validation::data::InvariantCostClass::Touched,
                    failure_effect: crate::validation::data::InvariantFailureEffect::BlockCommit,
                },
            }
        }

        fn prepare_scope(
            &self,
            planner: &mut CustomInvariantScopePlanner<'_>,
        ) -> Result<Self::Scope, CustomInvariantPreparationError> {
            let source_entities = planner.touched().visible_entity_ids();
            let traversal = planner.traversal().walk_outgoing_from(source_entities, 1)?;
            assert!(traversal.frontier_exhausted());
            Ok(StructuralScope {
                visible_entities: source_entities.len(),
                planned_relations: planner.touched().planned_relation_creates().len(),
                touched_partitions: planner.touched().touched_partitions().len(),
            })
        }

        fn evaluate(
            &self,
            context: &CustomInvariantExecutionContext<'_>,
            scope: &Self::Scope,
        ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError> {
            let counts = context.counts();
            if counts.visible_entity_count() == scope.visible_entities
                && counts.planned_relation_create_count() == scope.planned_relations
                && counts.touched_partition_count() == scope.touched_partitions
            {
                Ok(CustomInvariantVerdict::Pass)
            } else {
                Ok(CustomInvariantVerdict::Violation)
            }
        }
    }

    impl CustomInvariantRule for PanicDuringPrepareRule {
        type Scope = ();

        fn descriptor(&self) -> CustomInvariantDescriptor {
            CustomInvariantDescriptor {
                identity: CustomInvariantSemanticIdentity {
                    rule_id: CustomInvariantRuleId::new("test.custom.panic-prepare"),
                    semantic_version: CustomInvariantSemanticVersion::new(1, 0),
                },
                display_name: Arc::from("Panic During Prepare"),
                operational: CustomInvariantOperationalMetadata {
                    execution_point:
                        crate::validation::data::InvariantExecutionPoint::CommitBoundary,
                    groups: InvariantGroupSet::of(InvariantGroup::SchemaCompliance),
                    cost_class: crate::validation::data::InvariantCostClass::Touched,
                    failure_effect: crate::validation::data::InvariantFailureEffect::BlockCommit,
                },
            }
        }

        fn prepare_scope(
            &self,
            _planner: &mut CustomInvariantScopePlanner<'_>,
        ) -> Result<Self::Scope, CustomInvariantPreparationError> {
            panic!("prepare panic");
        }

        fn evaluate(
            &self,
            _context: &CustomInvariantExecutionContext<'_>,
            _scope: &Self::Scope,
        ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError> {
            Ok(CustomInvariantVerdict::Pass)
        }
    }

    impl CustomInvariantRule for PanicDuringEvaluateRule {
        type Scope = ();

        fn descriptor(&self) -> CustomInvariantDescriptor {
            CustomInvariantDescriptor {
                identity: CustomInvariantSemanticIdentity {
                    rule_id: CustomInvariantRuleId::new("test.custom.panic-evaluate"),
                    semantic_version: CustomInvariantSemanticVersion::new(1, 0),
                },
                display_name: Arc::from("Panic During Evaluate"),
                operational: CustomInvariantOperationalMetadata {
                    execution_point:
                        crate::validation::data::InvariantExecutionPoint::CommitBoundary,
                    groups: InvariantGroupSet::of(InvariantGroup::SchemaCompliance),
                    cost_class: crate::validation::data::InvariantCostClass::Touched,
                    failure_effect: crate::validation::data::InvariantFailureEffect::BlockCommit,
                },
            }
        }

        fn prepare_scope(
            &self,
            _planner: &mut CustomInvariantScopePlanner<'_>,
        ) -> Result<Self::Scope, CustomInvariantPreparationError> {
            Ok(())
        }

        fn evaluate(
            &self,
            _context: &CustomInvariantExecutionContext<'_>,
            _scope: &Self::Scope,
        ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError> {
            panic!("evaluate panic");
        }
    }

    #[test]
    fn engine_skips_rules_when_request_groups_do_not_intersect() {
        let runtime = runtime_with_invariants(InvariantCatalog {
            registrations: vec![InvariantRegistration::commit_boundary_blocking(
                InvariantRule::UniqueEntityPayloadField("name".to_string()),
            )],
            ..InvariantCatalog::default()
        });
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(1),
            merged_intents: vec![MutationIntent::Relation(RelationMutationIntent::Delete(
                DeleteRelationIntent {
                    relation_id: RelationId::new(PartitionId::main(), 0, 1),
                },
            ))],
        };

        let results = InvariantEngine::new(&runtime).execute(
            InvariantExecutionRequest::from_profile_with_contract(
                InvariantRequestProfile::CommitBoundary,
                &runtime,
                InvariantObservation::committed(runtime.storage_access().current_state()),
                runtime.current_version_id(),
                Some(&plan),
                Some(crate::validation::data::InvariantPlanContract::from_merged_plan(&plan)),
            )
            .with_applicable_groups(InvariantGroupSet::of(InvariantGroup::LineageIntegrity)),
        );

        assert!(results.results().is_empty());
    }

    #[test]
    fn engine_marks_unrelated_commit_boundary_rules_not_applicable() {
        let runtime = runtime_with_invariants(InvariantCatalog {
            registrations: vec![InvariantRegistration::commit_boundary_blocking(
                InvariantRule::UniqueEntityPayloadField("name".to_string()),
            )],
            ..InvariantCatalog::default()
        });
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(2),
            merged_intents: vec![MutationIntent::Relation(RelationMutationIntent::Delete(
                DeleteRelationIntent {
                    relation_id: RelationId::new(PartitionId::main(), 0, 1),
                },
            ))],
        };

        let results = runtime.validation().commit_boundary(&plan);

        assert!(results.results().is_empty());
    }

    #[test]
    fn engine_rejects_entity_payloads_that_violate_payload_schema_contracts() {
        let runtime = runtime_with_payload_schema_and_partition_isolation();
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(10),
            merged_intents: vec![MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw("vertex-a".to_string()),
                payload: RecordPayload::StructuredJson(json!({"name": "vertex-a", "rank": "bad"})),
            }))],
        };

        let results = InvariantEngine::new(&runtime).execute(
            InvariantExecutionRequest::from_profile_with_contract(
                InvariantRequestProfile::CommitBoundary,
                &runtime,
                InvariantObservation::committed(runtime.storage_access().current_state()),
                runtime.current_version_id(),
                Some(&plan),
                Some(crate::validation::data::InvariantPlanContract::from_merged_plan(&plan)),
            ),
        );

        let failure = results
            .results()
            .iter()
            .find_map(|result| match &result.verdict {
                crate::validation::data::InvariantVerdict::Violation(violation) => Some(violation),
                _ => None,
            })
            .expect("payload schema violation");
        match &failure.fields {
            InvariantViolationFields::PayloadSchema {
                contract_id,
                field,
                failure_kind,
                expected_type,
                ..
            } => {
                assert_eq!(contract_id.as_str(), "vertex_payload");
                assert_eq!(field, "rank");
                assert_eq!(failure_kind, "type");
                assert_eq!(expected_type, &Some(PayloadSchemaValueType::Number));
            }
            other => panic!("expected payload schema violation, got {other:?}"),
        }
    }

    #[test]
    fn engine_emits_multiple_payload_schema_witnesses_from_one_packet() {
        let runtime = runtime_with_payload_schema_and_partition_isolation();
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(10),
            merged_intents: vec![MutationIntent::Create(CreateIntent::BulkEntities(
                crate::transactions::data::BulkEntityCreateIntent {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_keys: vec![
                        InternedString::Raw("bad-a".to_string()),
                        InternedString::Raw("bad-b".to_string()),
                    ],
                    payloads: vec![
                        RecordPayload::StructuredJson(json!({"rank":"bad"})),
                        RecordPayload::StructuredJson(json!({"rank":"also-bad"})),
                    ],
                },
            ))],
        };

        let results = InvariantEngine::new(&runtime).execute(
            InvariantExecutionRequest::from_profile_with_contract(
                InvariantRequestProfile::CommitBoundary,
                &runtime,
                InvariantObservation::committed(runtime.storage_access().current_state()),
                runtime.current_version_id(),
                Some(&plan),
                Some(crate::validation::data::InvariantPlanContract::from_merged_plan(&plan)),
            ),
        );

        let payload_failures = results
            .results()
            .iter()
            .filter(|result| {
                matches!(
                    result.verdict,
                    crate::validation::data::InvariantVerdict::Violation(
                        crate::validation::data::InvariantViolation {
                            fields: InvariantViolationFields::PayloadSchema { .. },
                            ..
                        }
                    )
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(payload_failures.len(), 2);
        assert!(payload_failures
            .windows(2)
            .all(|window| window[0].witness().as_str() <= window[1].witness().as_str()));
    }

    #[test]
    fn engine_rejects_cross_partition_relations_under_partition_isolation_contracts() {
        let runtime = runtime_with_payload_schema_and_partition_isolation();
        let source = crate::identity::data::EntityId::new(PartitionId(1), 0, 1);
        let target = crate::identity::data::EntityId::new(PartitionId(2), 0, 1);
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(11),
            merged_intents: vec![MutationIntent::Create(CreateIntent::Relation(
                RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_key: InternedString::Raw("edge-a".to_string()),
                    source,
                    target,
                    payload: Some(RecordPayload::StructuredJson(json!({"kind": "edge"}))),
                },
            ))],
        };

        let results = InvariantEngine::new(&runtime).execute(
            InvariantExecutionRequest::from_profile_with_contract(
                InvariantRequestProfile::CommitBoundary,
                &runtime,
                InvariantObservation::committed(runtime.storage_access().current_state()),
                runtime.current_version_id(),
                Some(&plan),
                Some(crate::validation::data::InvariantPlanContract::from_merged_plan(&plan)),
            ),
        );

        let failure = results
            .results()
            .iter()
            .find_map(|result| match &result.verdict {
                crate::validation::data::InvariantVerdict::Violation(violation) => Some(violation),
                _ => None,
            })
            .expect("partition isolation violation");
        match &failure.fields {
            InvariantViolationFields::PartitionIsolation {
                contract_id,
                source_partition_id,
                target_partition_id,
                ..
            } => {
                assert_eq!(contract_id.as_str(), "same_partition");
                assert_eq!(*source_partition_id, PartitionId(1));
                assert_eq!(*target_partition_id, PartitionId(2));
            }
            other => panic!("expected partition isolation violation, got {other:?}"),
        }
    }

    #[test]
    fn engine_rejects_planned_cycles_under_acyclicity_contracts() {
        let runtime = runtime_with_acyclicity_and_connectivity();
        let a = crate::identity::data::EntityId::new(PartitionId::main(), 0, 1);
        let b = crate::identity::data::EntityId::new(PartitionId::main(), 1, 1);
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(12),
            merged_intents: vec![
                MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_key: InternedString::Raw("edge-ab".to_string()),
                    source: a,
                    target: b,
                    payload: None,
                })),
                MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_key: InternedString::Raw("edge-ba".to_string()),
                    source: b,
                    target: a,
                    payload: None,
                })),
            ],
        };

        let results = InvariantEngine::new(&runtime).execute(
            InvariantExecutionRequest::from_profile_with_contract(
                InvariantRequestProfile::CommitBoundary,
                &runtime,
                InvariantObservation::committed(runtime.storage_access().current_state()),
                runtime.current_version_id(),
                Some(&plan),
                Some(crate::validation::data::InvariantPlanContract::from_merged_plan(&plan)),
            ),
        );

        let failure = results
            .results()
            .iter()
            .find_map(|result| match &result.verdict {
                crate::validation::data::InvariantVerdict::Violation(violation) => Some(violation),
                _ => None,
            })
            .expect("acyclicity violation");
        match &failure.fields {
            InvariantViolationFields::Acyclicity { contract_id, .. } => {
                assert_eq!(contract_id.as_str(), "no_cycles");
            }
            other => panic!("expected acyclicity violation, got {other:?}"),
        }
    }

    #[test]
    fn engine_rejects_acyclicity_checks_that_exceed_traversal_budget() {
        let mut runtime =
            runtime_with_acyclicity_and_connectivity_budget(RelationIntegrityScopeBudget {
                max_relation_kinds: 8,
                max_touched_entities: 16,
                max_deleted_entities: 8,
                max_scanned_relations: 2,
                max_planned_edges: 8,
            });
        let a = create_entity_of_kind(&mut runtime, KindId(3), "a");
        let b = create_entity_of_kind(&mut runtime, KindId(3), "b");
        let c = create_entity_of_kind(&mut runtime, KindId(3), "c");
        let d = create_entity_of_kind(&mut runtime, KindId(3), "d");
        let e = create_entity_of_kind(&mut runtime, KindId(3), "e");
        create_relation_of_kind(&mut runtime, KindId(2), a, b, "edge-ab");
        create_relation_of_kind(&mut runtime, KindId(2), b, c, "edge-bc");
        create_relation_of_kind(&mut runtime, KindId(2), c, d, "edge-cd");
        create_relation_of_kind(&mut runtime, KindId(2), d, e, "edge-de");

        let plan = MergedCommitPlan {
            transaction_id: TransactionId(19),
            merged_intents: vec![MutationIntent::Create(CreateIntent::Relation(
                RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_key: InternedString::Raw("edge-ea".to_string()),
                    source: e,
                    target: a,
                    payload: None,
                },
            ))],
        };

        let results = InvariantEngine::new(&runtime).execute(
            InvariantExecutionRequest::from_profile_with_contract(
                InvariantRequestProfile::CommitBoundary,
                &runtime,
                InvariantObservation::committed(runtime.storage_access().current_state()),
                runtime.current_version_id(),
                Some(&plan),
                Some(crate::validation::data::InvariantPlanContract::from_merged_plan(&plan)),
            ),
        );

        let failure = results
            .blocking_failures()
            .into_iter()
            .next()
            .map(|failure| failure.violation().clone())
            .expect("budget violation");
        match &failure.fields {
            InvariantViolationFields::RelationIntegrityScopeBudgetExceeded {
                limit_name,
                limit,
                observed,
                ..
            } => {
                assert_eq!(limit_name, "max_scanned_relations");
                assert_eq!(*limit, 3);
                assert_eq!(*observed, 4);
            }
            other => panic!("expected traversal budget violation, got {other:?}"),
        }
    }

    #[test]
    fn commit_publication_stage_rejects_sources_without_required_connectivity() {
        let mut runtime = runtime_with_acyclicity_and_connectivity();
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("node-a").push(MutationIntent::Create(CreateIntent::Entity(
                EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: InternedString::Raw("node-a".to_string()),
                    payload: RecordPayload::StructuredJson(json!({ "name": "node-a" })),
                },
            ))),
        );

        let error = txn.commit().expect_err("connectivity publication failure");
        match error {
            crate::facade::transactions::TransactionCommitError::Publication { error, .. } => {
                assert!(error.detail.contains("reachable_anchor"));
                assert!(error.detail.contains("at least 1 reachable target"));
            }
            other => panic!("expected publication error, got {other:?}"),
        }
    }

    #[test]
    fn minimum_cardinality_current_version_scans_only_live_slots() {
        let mut runtime = runtime_with_cardinality_minimum();
        let source = create_entity_of_kind(&mut runtime, KindId(1), "source");
        let target = create_entity_of_kind(&mut runtime, KindId(1), "target");
        let retired_target = create_entity_of_kind(&mut runtime, KindId(1), "retired-target");
        let retired_relation =
            create_relation_of_kind(&mut runtime, KindId(2), source, retired_target, "retired");
        create_relation_of_kind(&mut runtime, KindId(2), source, target, "live");
        let mut delete_txn = runtime.begin_transaction(TransactionOptions::default());
        delete_txn.push_batch(WorkerIntentBatch::new("delete-retired").push(
            MutationIntent::Relation(RelationMutationIntent::Delete(DeleteRelationIntent {
                relation_id: retired_relation,
            })),
        ));
        delete_txn.commit().expect("retire relation");

        runtime.performance_access().reset_counters();
        let _results = InvariantEngine::new(&runtime).execute(
            InvariantExecutionRequest::from_profile_with_contract(
                InvariantRequestProfile::CertificationBoundary,
                &runtime,
                InvariantObservation::committed(runtime.storage_access().current_state()),
                runtime.current_version_id(),
                None,
                None,
            ),
        );

        let counters = runtime.performance_access().counters();
        assert_eq!(
            counters.relation_cardinality_minimum_certification_relation_slot_scans,
            1
        );
        assert_eq!(
            counters.relation_cardinality_minimum_certification_entity_slot_scans,
            3
        );
    }

    #[test]
    fn engine_executes_custom_invariant_packets() {
        let runtime = RelationalRuntimeApi::builder()
            .schema_registry(RelationalSchemaRegistry::new())
            .custom_invariant(CustomInvariantRegistration::new(AlwaysViolatesCustomRule).unwrap())
            .build();

        let results = InvariantEngine::new(&runtime).execute(
            InvariantExecutionRequest::from_profile_with_contract(
                InvariantRequestProfile::CommitBoundary,
                &runtime,
                InvariantObservation::committed(runtime.storage_access().current_state()),
                runtime.current_version_id(),
                None,
                None,
            ),
        );

        assert_eq!(results.results().len(), 1);
        match &results.results()[0].rule {
            InvariantReportedRule::Custom(identity) => {
                assert_eq!(identity.rule_id.as_str(), "test.custom.violation");
            }
            other => panic!("expected custom invariant result, got {other:?}"),
        }
        assert!(matches!(
            results.results()[0].verdict,
            crate::validation::data::InvariantVerdict::Violation(_)
        ));
    }

    #[test]
    fn engine_executes_custom_packets_against_real_structural_surfaces() {
        let runtime = RelationalRuntimeApi::builder()
            .schema_registry(RelationalSchemaRegistry::new())
            .custom_invariant(CustomInvariantRegistration::new(StructuralSurfaceRule).unwrap())
            .build();
        runtime.performance_access().reset_counters();
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(3),
            merged_intents: vec![
                MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: crate::facade::identity::KindId(1),
                    client_key: crate::symbols::data::InternedString::Raw("source".to_string()),
                    payload: crate::payloads::data::RecordPayload::StructuredJson(
                        serde_json::json!({"name": "source"}),
                    ),
                })),
                MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: crate::facade::identity::KindId(2),
                    client_key: crate::symbols::data::InternedString::Raw("edge".to_string()),
                    source: crate::facade::identity::EntityId::new(PartitionId::main(), 10, 1),
                    target: crate::facade::identity::EntityId::new(PartitionId::main(), 11, 1),
                    payload: Some(crate::payloads::data::RecordPayload::StructuredJson(
                        serde_json::json!({"kind": "edge"}),
                    )),
                })),
            ],
        };

        let results = InvariantEngine::new(&runtime).execute(
            InvariantExecutionRequest::from_profile_with_contract(
                InvariantRequestProfile::CommitBoundary,
                &runtime,
                InvariantObservation::committed(runtime.storage_access().current_state()),
                runtime.current_version_id(),
                Some(&plan),
                None,
            ),
        );

        assert_eq!(results.results().len(), 1);
        assert!(matches!(
            results.results()[0].verdict,
            crate::validation::data::InvariantVerdict::Pass
        ));
        let counters = runtime.performance_access().counters();
        assert_eq!(counters.custom_invariant_preparation_count, 1);
        assert_eq!(counters.custom_invariant_execution_count, 1);
        assert!(counters.custom_invariant_traversal_frontier_count >= 2);
    }

    #[test]
    fn engine_captures_custom_prepare_panics_as_typed_failures() {
        let runtime = RelationalRuntimeApi::builder()
            .schema_registry(RelationalSchemaRegistry::new())
            .custom_invariant(CustomInvariantRegistration::new(PanicDuringPrepareRule).unwrap())
            .build();
        runtime.performance_access().reset_counters();

        let results = InvariantEngine::new(&runtime).execute(
            InvariantExecutionRequest::from_profile_with_contract(
                InvariantRequestProfile::CommitBoundary,
                &runtime,
                InvariantObservation::committed(runtime.storage_access().current_state()),
                runtime.current_version_id(),
                None,
                None,
            ),
        );

        assert_eq!(results.results().len(), 1);
        let crate::validation::data::InvariantVerdict::Violation(violation) =
            &results.results()[0].verdict
        else {
            panic!("expected captured prepare panic to produce a violation");
        };
        match &violation.fields {
            crate::validation::data::InvariantViolationFields::CustomInvariantFailure {
                rule_id,
                phase,
                failure_kind,
                ..
            } => {
                assert_eq!(rule_id, "test.custom.panic-prepare");
                assert_eq!(phase, "preparation");
                assert_eq!(failure_kind, "panic");
            }
            other => panic!("expected custom invariant failure fields, got {other:?}"),
        }
        assert_eq!(results.summary().custom_failure_count(), 1);
        assert_eq!(results.summary().custom_panic_count(), 1);
        let counters = runtime.performance_access().counters();
        assert_eq!(counters.custom_invariant_preparation_count, 1);
        assert_eq!(counters.custom_invariant_execution_count, 0);
        assert_eq!(counters.custom_invariant_panic_count, 1);
    }

    #[test]
    fn engine_captures_custom_evaluate_panics_as_typed_failures() {
        let runtime = RelationalRuntimeApi::builder()
            .schema_registry(RelationalSchemaRegistry::new())
            .custom_invariant(CustomInvariantRegistration::new(PanicDuringEvaluateRule).unwrap())
            .build();
        runtime.performance_access().reset_counters();

        let results = InvariantEngine::new(&runtime).execute(
            InvariantExecutionRequest::from_profile_with_contract(
                InvariantRequestProfile::CommitBoundary,
                &runtime,
                InvariantObservation::committed(runtime.storage_access().current_state()),
                runtime.current_version_id(),
                None,
                None,
            ),
        );

        assert_eq!(results.results().len(), 1);
        let crate::validation::data::InvariantVerdict::Violation(violation) =
            &results.results()[0].verdict
        else {
            panic!("expected captured evaluate panic to produce a violation");
        };
        match &violation.fields {
            crate::validation::data::InvariantViolationFields::CustomInvariantFailure {
                rule_id,
                phase,
                failure_kind,
                ..
            } => {
                assert_eq!(rule_id, "test.custom.panic-evaluate");
                assert_eq!(phase, "execution");
                assert_eq!(failure_kind, "panic");
            }
            other => panic!("expected custom invariant failure fields, got {other:?}"),
        }
        assert_eq!(results.summary().custom_failure_count(), 1);
        assert_eq!(results.summary().custom_panic_count(), 1);
        let counters = runtime.performance_access().counters();
        assert_eq!(counters.custom_invariant_preparation_count, 1);
        assert_eq!(counters.custom_invariant_execution_count, 1);
        assert_eq!(counters.custom_invariant_panic_count, 1);
    }
}
