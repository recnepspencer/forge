use std::collections::BTreeSet;

use crate::data::diagnostics::DiagnosticCode;
use crate::data::schema::SchemaRegistryError;
use crate::data::transaction::{CommitConflict, MergedCommitPlan};
use crate::logic::runtime::{
    InvariantCheckResult, InvariantClass, InvariantExecutionPoint, InvariantFailureEffect,
    InvariantRule, InvariantViolation, RecordLifecycleState, RelationalRuntime,
};

use super::state::WorkingState;

impl RelationalRuntime {
    pub(super) fn run_invariants_for_state(
        &self,
        state: &WorkingState,
        version_id: crate::data::identity::VersionId,
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

        let entity_records = self.live_entities_from_state(state);
        let relation_records = self.live_relations_from_state(state);

        for (class, failure_effect, rules) in groups {
            let mut violations = Vec::new();
            for rule in rules {
                match rule {
                    InvariantRule::LiveEntityRequiresKind => {
                        for slot in 0..state.entity_arena.generations.len() {
                            if state.entity_arena.lifecycle[slot] == RecordLifecycleState::Live
                                && state.entity_arena.kind_ids[slot].is_none()
                            {
                                violations.push(InvariantViolation {
                                    class,
                                    code: DiagnosticCode::SidecarConsistencyFailure,
                                    detail: format!("live entity slot {} missing kind id", slot),
                                });
                            }
                        }
                    }
                    InvariantRule::LiveRelationRequiresEndpoints => {
                        for slot in 0..state.relation_arena.generations.len() {
                            if state.relation_arena.lifecycle[slot] == RecordLifecycleState::Live
                                && state.relation_arena.endpoints[slot].is_none()
                            {
                                violations.push(InvariantViolation {
                                    class,
                                    code: DiagnosticCode::SidecarConsistencyFailure,
                                    detail: format!(
                                        "live relation slot {} missing endpoints",
                                        slot
                                    ),
                                });
                            }
                        }
                    }
                    InvariantRule::MaxMergedIntents(limit) => {
                        let merged_len = merged_plan
                            .map(|plan| plan.merged_intents.len())
                            .unwrap_or(0);
                        if merged_len > *limit {
                            violations.push(InvariantViolation {
                                class,
                                code: DiagnosticCode::InvariantViolation,
                                detail: format!(
                                    "merged commit plan has {} intents, limit is {}",
                                    merged_len, limit
                                ),
                            });
                        }
                    }
                    InvariantRule::MaxSnapshotEntities(limit) => {
                        if entity_records.len() > *limit {
                            violations.push(InvariantViolation {
                                class,
                                code: DiagnosticCode::InvariantViolation,
                                detail: format!(
                                    "snapshot at version {} has {} entities, limit is {}",
                                    version_id.0,
                                    entity_records.len(),
                                    limit
                                ),
                            });
                        }
                    }
                    InvariantRule::UniqueEntityPayloadField(field) => {
                        let mut seen = BTreeSet::new();
                        for entity in &entity_records {
                            if let Some(value) =
                                entity.payload.get(field).and_then(|value| value.as_str())
                            {
                                if !seen.insert(value.to_string()) {
                                    violations.push(InvariantViolation {
                                        class,
                                        code: DiagnosticCode::InvariantViolation,
                                        detail: format!(
                                            "duplicate entity payload field {}={}",
                                            field, value
                                        ),
                                    });
                                }
                            }
                        }
                    }
                }
            }
            if !relation_records.is_empty() || !entity_records.is_empty() || !rules.is_empty() {
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

pub(super) fn first_blocking_invariant_error(
    results: &[InvariantCheckResult],
) -> Option<CommitConflict> {
    results
        .iter()
        .find(|result| {
            result.failure_effect == InvariantFailureEffect::BlockCommit
                && !result.violations.is_empty()
        })
        .and_then(|result| result.violations.first())
        .map(|violation| CommitConflict {
            code: violation.code,
            detail: violation.detail.clone(),
        })
}

pub(super) fn first_publication_invariant_error(
    results: &[InvariantCheckResult],
) -> Option<CommitConflict> {
    results
        .iter()
        .find(|result| {
            result.failure_effect == InvariantFailureEffect::BlockPublication
                && !result.violations.is_empty()
        })
        .and_then(|result| result.violations.first())
        .map(|violation| CommitConflict {
            code: violation.code,
            detail: violation.detail.clone(),
        })
}

pub(super) fn schema_error_to_commit_conflict(error: SchemaRegistryError) -> CommitConflict {
    CommitConflict {
        code: DiagnosticCode::InvariantViolation,
        detail: format!("{error:?}"),
    }
}
