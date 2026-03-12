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
    pub parent_count: usize,
    pub merge_base_count: usize,
    pub patch_record_count: usize,
    pub diagnostics_entry_count: usize,
    pub lineage_event_count: usize,
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
}
