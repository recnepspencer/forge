#[path = "baseline_btree_counter_support.rs"]
mod baseline_btree_counter_support;
#[path = "baseline_btree_execution_witness.rs"]
mod baseline_btree_execution_witness;

use self::baseline_btree_counter_support::readiness;
pub use self::baseline_btree_execution_witness::{
    execute_baseline_btree_lookup, execute_baseline_btree_replay_recovery,
    execute_baseline_btree_root_publication, execute_baseline_btree_transcript,
};
use crate::{
    PersistedPhysicalLayout, PhysicalRecordSlot, PhysicalReference, PlatformPhysicalFacade,
    PlatformPhysicalOpenRequest, PlatformPhysicalRootPublicationReport, SlotGenerationCell,
};

#[cfg(test)]
#[path = "baseline_btree_counter_observation_tests.rs"]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BaselineBTreeCounterObservation {
    point_lookups: u16,
    range_lookups: u16,
    publications: u16,
    maintenance_reads: u16,
}

impl BaselineBTreeCounterObservation {
    const fn new(
        point_lookups: u16,
        range_lookups: u16,
        publications: u16,
        maintenance_reads: u16,
    ) -> Self {
        Self {
            point_lookups,
            range_lookups,
            publications,
            maintenance_reads,
        }
    }

    pub const fn point_lookups(self) -> u16 {
        self.point_lookups
    }

    pub const fn range_lookups(self) -> u16 {
        self.range_lookups
    }

    pub const fn publications(self) -> u16 {
        self.publications
    }

    pub const fn maintenance_reads(self) -> u16 {
        self.maintenance_reads
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineBTreeLookupBranch {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreeLookupExecution {
    probe_slot: PhysicalRecordSlot,
    separator_slot: PhysicalRecordSlot,
    branch: BaselineBTreeLookupBranch,
    selected_reference: PhysicalReference,
    counters: BaselineBTreeCounterObservation,
}

impl BaselineBTreeLookupExecution {
    const fn new(
        probe_slot: PhysicalRecordSlot,
        separator_slot: PhysicalRecordSlot,
        branch: BaselineBTreeLookupBranch,
        selected_reference: PhysicalReference,
        counters: BaselineBTreeCounterObservation,
    ) -> Self {
        Self {
            probe_slot,
            separator_slot,
            branch,
            selected_reference,
            counters,
        }
    }

    pub const fn probe_slot(self) -> PhysicalRecordSlot {
        self.probe_slot
    }

    pub const fn separator_slot(self) -> PhysicalRecordSlot {
        self.separator_slot
    }

    pub const fn branch(self) -> BaselineBTreeLookupBranch {
        self.branch
    }

    pub const fn selected_reference(self) -> PhysicalReference {
        self.selected_reference
    }

    pub const fn counters(self) -> BaselineBTreeCounterObservation {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineBTreeRootPublicationExecution {
    publication_report: PlatformPhysicalRootPublicationReport,
    root_reference: PhysicalReference,
    root_payload: Vec<u8>,
    left_child: SlotGenerationCell,
    right_child: SlotGenerationCell,
}

impl BaselineBTreeRootPublicationExecution {
    fn new(
        publication_report: PlatformPhysicalRootPublicationReport,
        root_reference: PhysicalReference,
        root_payload: Vec<u8>,
        left_child: SlotGenerationCell,
        right_child: SlotGenerationCell,
    ) -> Self {
        Self {
            publication_report,
            root_reference,
            root_payload,
            left_child,
            right_child,
        }
    }

    pub fn published_layout(&self) -> &PersistedPhysicalLayout {
        self.publication_report.persisted_layout()
    }

    pub const fn root_reference(&self) -> PhysicalReference {
        self.root_reference
    }

    pub fn root_generation_advanced(&self) -> bool {
        self.root_reference.generation().get()
            > self
                .left_child
                .generation()
                .get()
                .max(self.right_child.generation().get())
    }

    pub fn checksum_scope_matches(&self) -> bool {
        self.published_layout().root_manifest_candidates().len() == 1
            && !self.published_layout().root_manifest_candidates()[0].is_empty()
            && self.published_layout().root_manifest_candidates()[0] != self.root_payload
    }

    pub fn root_manifest_candidate_count(&self) -> u16 {
        self.published_layout().root_manifest_candidates().len() as u16
    }

    pub const fn counters(&self) -> BaselineBTreeCounterObservation {
        BaselineBTreeCounterObservation::new(0, 0, 1, 0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineBTreeReplayRecoveryExecution {
    reopened_layout: PersistedPhysicalLayout,
    published_root_reference: PhysicalReference,
    published_root_manifest: Vec<u8>,
    rebuild_authority_records: u16,
    rebuild_output_records: u16,
    rebuild_source_authoritative: bool,
}

impl BaselineBTreeReplayRecoveryExecution {
    fn new(
        reopened_layout: PersistedPhysicalLayout,
        published_root_reference: PhysicalReference,
        published_root_manifest: Vec<u8>,
        rebuild_authority_records: u16,
        rebuild_output_records: u16,
        rebuild_source_authoritative: bool,
    ) -> Self {
        Self {
            reopened_layout,
            published_root_reference,
            published_root_manifest,
            rebuild_authority_records,
            rebuild_output_records,
            rebuild_source_authoritative,
        }
    }

    pub fn reopened_layout(&self) -> &PersistedPhysicalLayout {
        &self.reopened_layout
    }

    pub fn replay_generation_monotonic(&self) -> bool {
        self.published_root_reference.generation().get() > 0 && self.manifest_advanced()
    }

    pub fn manifest_advanced(&self) -> bool {
        self.reopened_layout.root_manifest_candidates().len() == 1
            && self.reopened_layout.root_manifest_candidates()[0] == self.published_root_manifest
    }

    pub const fn rebuild_authority_records(&self) -> u16 {
        self.rebuild_authority_records
    }

    pub const fn rebuild_output_records(&self) -> u16 {
        self.rebuild_output_records
    }

    pub const fn rebuild_source_authoritative(&self) -> bool {
        self.rebuild_source_authoritative
    }

    pub const fn counters(&self) -> BaselineBTreeCounterObservation {
        BaselineBTreeCounterObservation::new(0, 0, 0, 1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineBTreeExecutionTranscript {
    lookup: BaselineBTreeLookupExecution,
    publication: BaselineBTreeRootPublicationExecution,
    recovery: BaselineBTreeReplayRecoveryExecution,
}

impl BaselineBTreeExecutionTranscript {
    fn new(
        lookup: BaselineBTreeLookupExecution,
        publication: BaselineBTreeRootPublicationExecution,
        recovery: BaselineBTreeReplayRecoveryExecution,
    ) -> Self {
        Self {
            lookup,
            publication,
            recovery,
        }
    }

    pub const fn lookup(&self) -> BaselineBTreeLookupExecution {
        self.lookup
    }

    pub fn publication(&self) -> &BaselineBTreeRootPublicationExecution {
        &self.publication
    }

    pub fn recovery(&self) -> &BaselineBTreeReplayRecoveryExecution {
        &self.recovery
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineBTreeExecutionWitness {
    pub(crate) root_reference: PhysicalReference,
    pub(crate) published_layout: PersistedPhysicalLayout,
}

fn reopen_facade(layout: PersistedPhysicalLayout) -> PlatformPhysicalFacade {
    PlatformPhysicalFacade::reopen_s1(
        readiness(),
        PlatformPhysicalOpenRequest::s1_canonical(),
        layout,
    )
    .expect("reopen S.1 physical facade")
}
