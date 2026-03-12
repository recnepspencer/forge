use crate::diagnostics::data::DiagnosticCode;
use crate::history::data::CommitId;
use crate::publication::data::diff::PatchStreamPosition;
use crate::publication::data::PublicationStage;
use crate::snapshots::data::SnapshotId;
use crate::validation::data::InvariantExecutionPoint;
use crate::validation::engine::InvariantExecutionResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommitPhase {
    DraftPreparation,
    InvariantPreCheck,
    AuthoritativeMutation,
    HistoryResolution,
    InvariantPostCheck,
    ArtifactAssembly,
    DurableAppend,
    Publication,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommitTraceEvent {
    PhaseStarted(CommitPhase),
    PhaseCompleted(CommitPhase),
    StructuralSummary {
        invariant_group_mask: u32,
        commit_topology_mask: u32,
        touched_partition_count: usize,
        bulk_entity_slots_reserved: usize,
        bulk_relation_slots_reserved: usize,
    },
    ChangedRecordsPrepared {
        changed_record_count: usize,
        adjacency_delta_count: usize,
    },
    InvariantEvaluated {
        execution_point: InvariantExecutionPoint,
        result_count: usize,
        advisory_count: usize,
        violation_count: usize,
        blocking_violation: bool,
        publication_violation: bool,
    },
    MergeParentsResolved {
        target_branch: String,
        requested_merge_parent_count: usize,
        effective_merge_parent_count: usize,
    },
    HistoryResolved {
        branch_id: String,
        parent_count: usize,
        merge_base_count: usize,
        had_previous_branch_head: bool,
    },
    PublicationArtifactsPrepared {
        patch_record_count: usize,
        diagnostics_entry_count: usize,
        lineage_event_count: usize,
    },
    PatchBudgetEvaluated {
        patch_record_count: usize,
        max_patch_records_per_commit: usize,
    },
    DurableAppendPrepared {
        commit_id: CommitId,
        branch_id: String,
        patch_position: PatchStreamPosition,
    },
    CommitPublished {
        commit_id: CommitId,
        branch_id: String,
        snapshot_id: SnapshotId,
        patch_position: PatchStreamPosition,
        changed_record_count: usize,
        merge_parent_count: usize,
    },
    CommitRejected {
        phase: CommitPhase,
        diagnostic_code: Option<DiagnosticCode>,
        publication_stage: Option<PublicationStage>,
        detail: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CommitSummary {
    pub phase_count: usize,
    pub invariant_group_mask: u32,
    pub commit_topology_mask: u32,
    pub touched_partition_count: usize,
    pub bulk_entity_slots_reserved: usize,
    pub bulk_relation_slots_reserved: usize,
    pub changed_record_count: usize,
    pub adjacency_delta_count: usize,
    pub invariant_result_count: usize,
    pub invariant_advisory_count: usize,
    pub invariant_violation_count: usize,
    pub parent_count: usize,
    pub merge_base_count: usize,
    pub requested_merge_parent_count: usize,
    pub effective_merge_parent_count: usize,
    pub patch_record_count: usize,
    pub diagnostics_entry_count: usize,
    pub lineage_event_count: usize,
    pub max_patch_records_per_commit: usize,
    pub final_snapshot_id: Option<SnapshotId>,
    pub patch_position: Option<PatchStreamPosition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CommitLog {
    events: Vec<CommitTraceEvent>,
    running_summary: CommitSummary,
}

impl CommitLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn events(&self) -> &[CommitTraceEvent] {
        &self.events
    }

    pub fn summary(&self) -> &CommitSummary {
        &self.running_summary
    }

    pub fn begin_phase(&mut self, phase: CommitPhase) {
        self.events.push(CommitTraceEvent::PhaseStarted(phase));
        self.running_summary.phase_count += 1;
    }

    pub fn complete_phase(&mut self, phase: CommitPhase) {
        self.events.push(CommitTraceEvent::PhaseCompleted(phase));
    }

    pub fn record_structural_summary(
        &mut self,
        invariant_group_mask: u32,
        commit_topology_mask: u32,
        touched_partition_count: usize,
        bulk_entity_slots_reserved: usize,
        bulk_relation_slots_reserved: usize,
    ) {
        self.running_summary.invariant_group_mask = invariant_group_mask;
        self.running_summary.commit_topology_mask = commit_topology_mask;
        self.running_summary.touched_partition_count = touched_partition_count;
        self.running_summary.bulk_entity_slots_reserved = bulk_entity_slots_reserved;
        self.running_summary.bulk_relation_slots_reserved = bulk_relation_slots_reserved;
        self.events.push(CommitTraceEvent::StructuralSummary {
            invariant_group_mask,
            commit_topology_mask,
            touched_partition_count,
            bulk_entity_slots_reserved,
            bulk_relation_slots_reserved,
        });
    }

    pub fn record_changed_records(
        &mut self,
        changed_record_count: usize,
        adjacency_delta_count: usize,
    ) {
        self.running_summary.changed_record_count = changed_record_count;
        self.running_summary.adjacency_delta_count = adjacency_delta_count;
        self.events.push(CommitTraceEvent::ChangedRecordsPrepared {
            changed_record_count,
            adjacency_delta_count,
        });
    }

    pub fn record_invariant_outcomes(
        &mut self,
        execution_point: InvariantExecutionPoint,
        result: &InvariantExecutionResult,
    ) {
        let result_count = result.results().len();
        let mut advisory_count = 0;
        let mut violation_count = 0;
        let mut blocking_violation = false;
        let mut publication_violation = false;

        for check in result.results() {
            match &check.verdict {
                crate::validation::data::InvariantVerdict::Pass => {}
                crate::validation::data::InvariantVerdict::Advisory { .. } => {
                    advisory_count += 1;
                }
                crate::validation::data::InvariantVerdict::Violation(_) => {
                    violation_count += 1;
                    match check.failure_effect {
                        crate::validation::data::InvariantFailureEffect::BlockCommit => {
                            blocking_violation = true;
                        }
                        crate::validation::data::InvariantFailureEffect::BlockPublication => {
                            publication_violation = true;
                        }
                        crate::validation::data::InvariantFailureEffect::AuditOnly => {}
                    }
                }
            }
        }

        self.running_summary.invariant_result_count += result_count;
        self.running_summary.invariant_advisory_count += advisory_count;
        self.running_summary.invariant_violation_count += violation_count;
        self.events.push(CommitTraceEvent::InvariantEvaluated {
            execution_point,
            result_count,
            advisory_count,
            violation_count,
            blocking_violation,
            publication_violation,
        });
    }

    pub fn record_merge_parents_resolved(
        &mut self,
        target_branch: &str,
        requested_merge_parent_count: usize,
        effective_merge_parent_count: usize,
    ) {
        self.running_summary.requested_merge_parent_count = requested_merge_parent_count;
        self.running_summary.effective_merge_parent_count = effective_merge_parent_count;
        self.events.push(CommitTraceEvent::MergeParentsResolved {
            target_branch: target_branch.to_string(),
            requested_merge_parent_count,
            effective_merge_parent_count,
        });
    }

    pub fn record_history_resolution(
        &mut self,
        branch_id: &str,
        parent_count: usize,
        merge_base_count: usize,
        had_previous_branch_head: bool,
    ) {
        self.running_summary.parent_count = parent_count;
        self.running_summary.merge_base_count = merge_base_count;
        self.events.push(CommitTraceEvent::HistoryResolved {
            branch_id: branch_id.to_string(),
            parent_count,
            merge_base_count,
            had_previous_branch_head,
        });
    }

    pub fn record_publication_artifacts(
        &mut self,
        patch_record_count: usize,
        diagnostics_entry_count: usize,
        lineage_event_count: usize,
    ) {
        self.running_summary.patch_record_count = patch_record_count;
        self.running_summary.diagnostics_entry_count = diagnostics_entry_count;
        self.running_summary.lineage_event_count = lineage_event_count;
        self.events.push(CommitTraceEvent::PublicationArtifactsPrepared {
            patch_record_count,
            diagnostics_entry_count,
            lineage_event_count,
        });
    }

    pub fn record_patch_budget(
        &mut self,
        patch_record_count: usize,
        max_patch_records_per_commit: usize,
    ) {
        self.running_summary.max_patch_records_per_commit = max_patch_records_per_commit;
        self.events.push(CommitTraceEvent::PatchBudgetEvaluated {
            patch_record_count,
            max_patch_records_per_commit,
        });
    }

    pub fn record_durable_append_prepared(
        &mut self,
        commit_id: CommitId,
        branch_id: &str,
        patch_position: PatchStreamPosition,
    ) {
        self.running_summary.patch_position = Some(patch_position);
        self.events.push(CommitTraceEvent::DurableAppendPrepared {
            commit_id,
            branch_id: branch_id.to_string(),
            patch_position,
        });
    }

    pub fn record_commit_published(
        &mut self,
        commit_id: CommitId,
        branch_id: &str,
        snapshot_id: SnapshotId,
        patch_position: PatchStreamPosition,
        changed_record_count: usize,
        merge_parent_count: usize,
    ) {
        self.running_summary.final_snapshot_id = Some(snapshot_id);
        self.running_summary.patch_position = Some(patch_position);
        self.events.push(CommitTraceEvent::CommitPublished {
            commit_id,
            branch_id: branch_id.to_string(),
            snapshot_id,
            patch_position,
            changed_record_count,
            merge_parent_count,
        });
    }

    pub fn record_rejection(
        &mut self,
        phase: CommitPhase,
        diagnostic_code: Option<DiagnosticCode>,
        publication_stage: Option<PublicationStage>,
        detail: impl Into<String>,
    ) {
        self.events.push(CommitTraceEvent::CommitRejected {
            phase,
            diagnostic_code,
            publication_stage,
            detail: detail.into(),
        });
    }
}
