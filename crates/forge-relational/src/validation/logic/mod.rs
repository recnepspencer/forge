mod helpers;
mod rules;

pub(crate) use helpers::{
    entity_payload_for_state, first_blocking_invariant_error, first_publication_invariant_error,
    schema_error_to_commit_conflict,
};

use crate::logic::runtime::{PartitionAccess, RelationalRuntime};
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{
    InvariantCheckResult, InvariantClass, InvariantExecutionPoint, InvariantFailureEffect,
};
use rules::evaluate_rule;

impl RelationalRuntime {
    pub(crate) fn run_invariants_for_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
        execution_point: InvariantExecutionPoint,
        include_harness_heavy: bool,
        merged_plan: Option<&MergedCommitPlan>,
    ) -> Vec<InvariantCheckResult> {
        let mut results = Vec::new();
        let groups = match execution_point {
            InvariantExecutionPoint::MutationSensitive => vec![(
                InvariantClass::AlwaysOnStructural,
                InvariantFailureEffect::BlockCommit,
                &self.config.invariant_catalog.always_on_structural,
            )],
            InvariantExecutionPoint::CommitBoundary => vec![(
                InvariantClass::CommitBoundary,
                InvariantFailureEffect::BlockCommit,
                &self.config.invariant_catalog.commit_boundary,
            )],
            InvariantExecutionPoint::SnapshotPublication => vec![(
                InvariantClass::SnapshotAudit,
                InvariantFailureEffect::BlockPublication,
                &self.config.invariant_catalog.snapshot_audit,
            )],
            InvariantExecutionPoint::HarnessAudit => {
                if include_harness_heavy {
                    vec![(
                        InvariantClass::HarnessHeavy,
                        InvariantFailureEffect::AuditOnly,
                        &self.config.invariant_catalog.harness_heavy,
                    )]
                } else {
                    Vec::new()
                }
            }
        };

        for (class, failure_effect, rules) in groups {
            let mut violations = Vec::new();
            for rule in rules {
                evaluate_rule(self, state, version_id, class, rule, merged_plan, &mut violations);
            }
            if !violations.is_empty() || !rules.is_empty() {
                results.push(InvariantCheckResult {
                    class,
                    execution_point,
                    failure_effect,
                    violations,
                });
            }
        }

        results
    }
}
