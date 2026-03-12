use crate::logic::runtime::RelationalRuntime;
use crate::validation::data::{InvariantCheckResult, InvariantVerdict};

use super::context::InvariantExecutionContext;
use super::evaluator::evaluate_rule;
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

        let mut results = Vec::new();
        for registration in registrations {
            if !request.includes_registration(registration) {
                continue;
            }
            let applies = registration.applies_to_contract(context.plan_contract);
            let rule = registration.rule.clone();
            let mut violations = Vec::new();
            let verdict = if applies {
                evaluate_rule(
                    &context,
                    registration.execution_point.class(),
                    &rule,
                    &mut violations,
                );
                if violations.is_empty() {
                    InvariantVerdict::Pass
                } else {
                    InvariantVerdict::Fail
                }
            } else {
                InvariantVerdict::NotApplicable
            };
            results.push(InvariantCheckResult {
                execution_point: registration.execution_point,
                failure_effect: registration.failure_effect,
                rule,
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
    use super::super::{InvariantExecutionRequest, InvariantRequestProfile};
    use crate::validation::engine::policy::InvariantExecutionPolicy;
    use crate::facade::{
        InvariantCatalog, InvariantRegistration, InvariantRule,
        PartitionId, RelationId, RelationalRuntimeApi, RelationalSchemaRegistry,
    };
    use crate::transactions::data::{
        DeleteRelationIntent, MergedCommitPlan, MutationIntent, RelationMutationIntent,
        TransactionId,
    };
    use crate::validation::data::{
        InvariantFailureEffect, InvariantGroup, InvariantGroupSet, InvariantVerdict,
    };

    fn runtime_with_invariants(invariant_catalog: InvariantCatalog) -> crate::facade::RelationalRuntime {
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
            InvariantExecutionRequest::from_profile(
                InvariantRequestProfile::CommitBoundary,
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

        assert_eq!(results.results().len(), 1);
        assert_eq!(results.results()[0].verdict, InvariantVerdict::NotApplicable);
        assert_eq!(
            results.results()[0].failure_effect,
            InvariantFailureEffect::BlockCommit
        );
    }
}
