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
    use crate::facade::identity::{PartitionId, RelationId};
    use crate::facade::runtime::{InvariantCatalog, InvariantRegistration, InvariantRule};
    use crate::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
    use crate::facade::schema::RelationalSchemaRegistry;
    use crate::facade::transactions::{
        CreateIntent, DeleteRelationIntent, MergedCommitPlan, MutationIntent,
        RelationMutationIntent, TransactionId,
    };
    use crate::transactions::data::{EntitySpec, RelationSpec};
    use crate::validation::data::{
        CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
        CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
        CustomInvariantRegistration, CustomInvariantRule, CustomInvariantRuleId,
        CustomInvariantScopePlanner, CustomInvariantSemanticIdentity,
        CustomInvariantSemanticVersion, CustomInvariantVerdict, InvariantGroup,
        InvariantGroupSet, InvariantReportedRule,
    };

    fn runtime_with_invariants(invariant_catalog: InvariantCatalog) -> RelationalRuntime {
        RelationalRuntimeApi::builder()
            .schema_registry(RelationalSchemaRegistry::new())
            .invariant_catalog(invariant_catalog)
            .build()
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
                    execution_point: crate::validation::data::InvariantExecutionPoint::CommitBoundary,
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
                    execution_point: crate::validation::data::InvariantExecutionPoint::CommitBoundary,
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
                    execution_point: crate::validation::data::InvariantExecutionPoint::CommitBoundary,
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
                    execution_point: crate::validation::data::InvariantExecutionPoint::CommitBoundary,
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

        let results = runtime.invariant_access().commit_boundary(&plan);

        assert!(results.results().is_empty());
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
