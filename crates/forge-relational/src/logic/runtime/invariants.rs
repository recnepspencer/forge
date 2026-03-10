use std::collections::BTreeSet;

use crate::data::diagnostics::DiagnosticCode;
use crate::data::payload::RecordPayload;
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

        for (class, failure_effect, rules) in groups {
            let mut violations = Vec::new();
            for rule in rules {
                match rule {
                    InvariantRule::LiveEntityRequiresKind => {
                        self.complexity_counters
                            .borrow_mut()
                            .invariant_entity_slot_scans += state.entity_arena.generations.len();
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
                        self.complexity_counters
                            .borrow_mut()
                            .invariant_relation_slot_scans +=
                            state.relation_arena.generations.len();
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
                        self.complexity_counters
                            .borrow_mut()
                            .invariant_entity_slot_scans += state.entity_arena.generations.len();
                        let visible_entities = (0..state.entity_arena.generations.len())
                            .filter(|slot| entity_visible_at_version(state, *slot, version_id))
                            .count();
                        if visible_entities > *limit {
                            violations.push(InvariantViolation {
                                class,
                                code: DiagnosticCode::InvariantViolation,
                                detail: format!(
                                    "snapshot at version {} has {} entities, limit is {}",
                                    version_id.0, visible_entities, limit
                                ),
                            });
                        }
                    }
                    InvariantRule::UniqueEntityPayloadField(field) => {
                        let mut seen = BTreeSet::new();
                        self.complexity_counters
                            .borrow_mut()
                            .invariant_entity_slot_scans += state.entity_arena.generations.len();
                        for slot in 0..state.entity_arena.generations.len() {
                            if !entity_visible_at_version(state, slot, version_id) {
                                continue;
                            }
                            let Some(payload) = visible_payload(
                                &state.entity_arena.payload_history[slot],
                                version_id,
                            ) else {
                                continue;
                            };
                            if let Some(value) = payload
                                .as_json()
                                .and_then(|value| value.get(field))
                                .and_then(|value| value.as_str())
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

fn visible_payload(
    history: &[super::state::VersionedValue],
    version_id: crate::data::identity::VersionId,
) -> Option<&RecordPayload> {
    history
        .iter()
        .find(|entry| {
            entry.effective_at <= version_id
                && entry.retired_at.is_none_or(|retired| version_id < retired)
        })
        .map(|entry| &entry.value)
}

fn entity_visible_at_version(
    state: &WorkingState,
    slot: usize,
    version_id: crate::data::identity::VersionId,
) -> bool {
    state.entity_arena.lifecycle[slot] != RecordLifecycleState::Reusable
        && state.entity_arena.created_at[slot] <= version_id
        && state.entity_arena.retired_at[slot].is_none_or(|retired| version_id < retired)
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
