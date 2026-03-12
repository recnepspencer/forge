use crate::logic::runtime::RelationalRuntime;
use crate::validation::data::{InvariantCheckResult, InvariantVerdict};

use super::context::InvariantExecutionContext;
use super::evaluator::evaluate_rule;
use super::request::InvariantExecutionRequest;
use super::result::InvariantExecutionResult;

pub struct InvariantEngine<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> InvariantEngine<'runtime> {
    pub fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn execute(
        &self,
        request: InvariantExecutionRequest<'runtime>,
    ) -> InvariantExecutionResult {
        let context = InvariantExecutionContext::new(
            self.runtime,
            request.state(),
            request.version_id(),
            request.execution_point(),
            request.merged_plan(),
        );
        let registrations = context
            .runtime()
            .config
            .schema
            .invariant_catalog
            .registrations_for_execution_point(context.execution_point);

        let mut results = Vec::with_capacity(registrations.len());
        for registration in registrations {
            if !request.includes_registration(&registration) {
                continue;
            }
            let class = registration.class();
            let mut violations = Vec::new();
            let verdict = if registration.applies_to_contract(context.plan_contract) {
                evaluate_rule(&context, class, &registration.rule, &mut violations);
                if violations.is_empty() {
                    InvariantVerdict::Pass
                } else {
                    InvariantVerdict::Fail
                }
            } else {
                InvariantVerdict::NotApplicable
            };
            results.push(InvariantCheckResult {
                class,
                execution_point: registration.execution_point,
                failure_effect: registration.failure_effect,
                rule: registration.rule,
                groups: registration.groups,
                cost: registration.cost,
                verdict,
                violations,
            });
        }
        InvariantExecutionResult::new(results)
    }
}

#[cfg(test)]
mod tests {
    use super::InvariantEngine;
    use crate::validation::engine::policy::InvariantExecutionPolicy;
    use crate::facade::{
        InvariantCatalog, InvariantExecutionPoint, InvariantRegistration, InvariantRule,
        PartitionId, RelationId, RelationalRuntimeApi, RelationalSchemaRegistry,
    };
    use crate::transactions::data::{
        DeleteRelationIntent, MergedCommitPlan, MutationIntent, RelationMutationIntent,
        TransactionId,
    };
    use crate::validation::data::{
        InvariantFailureEffect, InvariantGroup, InvariantGroupSet, InvariantVerdict,
    };
    use crate::validation::engine::InvariantExecutionRequest;

    fn runtime_with_invariants(invariant_catalog: InvariantCatalog) -> crate::facade::RelationalRuntime {
        RelationalRuntimeApi::builder()
            .schema_registry(RelationalSchemaRegistry::new())
            .invariant_catalog(invariant_catalog)
            .build()
    }

    #[test]
    fn engine_skips_rules_when_request_groups_do_not_intersect() {
        let runtime = runtime_with_invariants(InvariantCatalog {
            registrations: vec![InvariantRegistration::block_commit(
                InvariantRule::UniqueEntityPayloadField("name".to_string()),
                InvariantExecutionPoint::CommitBoundary,
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
            InvariantExecutionRequest::from_profile(
                crate::validation::engine::InvariantRequestProfile::CommitBoundary,
                &runtime.current_state(),
                runtime.current_version_id(),
                Some(&plan),
            )
            .with_groups(InvariantGroupSet::of(InvariantGroup::History))
            .with_policy(InvariantExecutionPolicy::AllowAll),
        );

        assert!(results.results().is_empty());
    }

    #[test]
    fn engine_marks_unrelated_commit_boundary_rules_not_applicable() {
        let runtime = runtime_with_invariants(InvariantCatalog {
            registrations: vec![InvariantRegistration::block_commit(
                InvariantRule::UniqueEntityPayloadField("name".to_string()),
                InvariantExecutionPoint::CommitBoundary,
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

        assert_eq!(results.results().len(), 1);
        assert_eq!(results.results()[0].verdict, InvariantVerdict::NotApplicable);
        assert_eq!(
            results.results()[0].failure_effect,
            InvariantFailureEffect::BlockCommit
        );
    }
}
