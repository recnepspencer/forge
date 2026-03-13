use crate::logic::runtime::RelationalRuntime;
use crate::validation::data::{InvariantCheckResult, InvariantVerdict};
use crate::validation::execution::{evaluate_invariant_packet, plan_invariant_execution};
use crate::validation::reduction::reduce_invariant_execution;
use rayon::prelude::*;
use std::collections::BTreeSet;

use super::context::InvariantExecutionContext;
use super::evaluator::evaluate_rule;
use super::request::InvariantExecutionRequest;
use super::result::{
    InvariantExecutionDisposition, InvariantExecutionMetadata, InvariantExecutionResult,
};

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
        let (result, _, reducer_conflicts) =
            reduce_invariant_execution(&request, planned.strategy, envelopes);
        if !reducer_conflicts.is_empty() {
            self.runtime
                .performance_access()
                .count_preparation_reducer_conflicts(reducer_conflicts.len());
        }
        result
    }

    #[allow(dead_code)]
    pub(crate) fn execute_serial_legacy(
        &self,
        request: InvariantExecutionRequest<'runtime>,
    ) -> InvariantExecutionResult {
        let context = InvariantExecutionContext::new(
            self.runtime,
            request.observation().clone(),
            request.version_id(),
            request.execution_point(),
            request.merged_plan(),
        );
        let registrations = context
            .runtime()
            .config
            .schema
            .invariant_catalog
            .registrations_for_execution_point(context.execution_point());

        let mut results = Vec::new();
        for registration in registrations {
            if !request.includes_registration(registration) {
                continue;
            }
            let rule = registration.rule.clone();
            let verdict = if let Some(violation) =
                evaluate_rule(&context, registration.execution_point.class(), &rule)
            {
                registration.verdict_for_violation(violation)
            } else {
                InvariantVerdict::Pass
            };
            results.push(InvariantCheckResult {
                execution_point: registration.execution_point,
                failure_effect: registration.failure_effect,
                rule,
                verdict,
            });
        }
        InvariantExecutionResult::executed(
            InvariantExecutionMetadata::new(
                request.execution_point(),
                request.observation().kind(),
                request.version_id(),
                request.current_version_id(),
                request.consumed_groups(),
                request.applicable_groups(),
                request.max_cost(),
                InvariantExecutionDisposition::Executed,
                request.plan_contract(),
                request.merged_plan().is_some(),
                self.runtime.config.execution.execution_model,
                None,
                Vec::new(),
            ),
            results,
        )
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
            planned
                .packets
                .iter()
                .flat_map(|packet| match &packet.locality.partition_scope {
                    crate::authority::commit::preparation::proofs::locality::PreparationPartitionScope::AllObserved => Vec::new(),
                    crate::authority::commit::preparation::proofs::locality::PreparationPartitionScope::TouchedPartitions(
                        partitions,
                    ) => partitions.clone(),
                })
                .collect::<BTreeSet<_>>()
                .len()
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
    use super::super::{InvariantExecutionRequest, InvariantObservation, InvariantRequestProfile};
    use super::InvariantEngine;
    use crate::facade::identity::{PartitionId, RelationId};
    use crate::facade::runtime::{InvariantCatalog, InvariantRegistration, InvariantRule};
    use crate::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
    use crate::facade::schema::RelationalSchemaRegistry;
    use crate::facade::transactions::{
        DeleteRelationIntent, MergedCommitPlan, MutationIntent, RelationMutationIntent,
        TransactionId,
    };
    use crate::validation::data::{InvariantGroup, InvariantGroupSet};

    fn runtime_with_invariants(invariant_catalog: InvariantCatalog) -> RelationalRuntime {
        RelationalRuntimeApi::builder()
            .schema_registry(RelationalSchemaRegistry::new())
            .invariant_catalog(invariant_catalog)
            .build()
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
}
