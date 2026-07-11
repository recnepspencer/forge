use super::BaselineBTreeExactCounterWitness;
use forge_store_budgets::S8PreExecutionPlanBinding;
use forge_store_physical_format::{
    PersistedPhysicalLayout, PhysicalRecordSlot, PhysicalReference, PlatformPhysicalFacadeDenial,
    PlatformPhysicalRootPublicationReport, SlotGenerationCell,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaselineBTreeExecutionDenial {
    Physical(PlatformPhysicalFacadeDenial),
    InvalidRootNode,
    InvalidLeafNode,
    ProbeMissingFromSelectedLeaf,
}

impl From<PlatformPhysicalFacadeDenial> for BaselineBTreeExecutionDenial {
    fn from(value: PlatformPhysicalFacadeDenial) -> Self {
        Self::Physical(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineBTreeLookupBranch {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineBTreeReadShape {
    PointLookup,
    RangeLookup,
    PrefixLookup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreeLookupExecution {
    plan_binding: S8PreExecutionPlanBinding,
    shape: BaselineBTreeReadShape,
    probe_slot: PhysicalRecordSlot,
    separator_slot: PhysicalRecordSlot,
    branch: BaselineBTreeLookupBranch,
    selected_reference: PhysicalReference,
    exact_counters: BaselineBTreeExactCounterWitness,
}

impl BaselineBTreeLookupExecution {
    pub(super) const fn new(
        plan_binding: S8PreExecutionPlanBinding,
        shape: BaselineBTreeReadShape,
        probe_slot: PhysicalRecordSlot,
        separator_slot: PhysicalRecordSlot,
        branch: BaselineBTreeLookupBranch,
        selected_reference: PhysicalReference,
        exact_counters: BaselineBTreeExactCounterWitness,
    ) -> Self {
        Self {
            plan_binding,
            shape,
            probe_slot,
            separator_slot,
            branch,
            selected_reference,
            exact_counters,
        }
    }

    pub const fn shape(self) -> BaselineBTreeReadShape {
        self.shape
    }
    pub const fn plan_binding(self) -> S8PreExecutionPlanBinding {
        self.plan_binding
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
    pub const fn exact_counters(self) -> BaselineBTreeExactCounterWitness {
        self.exact_counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineBTreeRootPublicationExecution {
    plan_binding: S8PreExecutionPlanBinding,
    publication_report: PlatformPhysicalRootPublicationReport,
    root_reference: PhysicalReference,
    root_payload: Vec<u8>,
    left_child: SlotGenerationCell,
    right_child: SlotGenerationCell,
    exact_counters: BaselineBTreeExactCounterWitness,
}

impl BaselineBTreeRootPublicationExecution {
    pub(super) fn new(
        plan_binding: S8PreExecutionPlanBinding,
        publication_report: PlatformPhysicalRootPublicationReport,
        root_reference: PhysicalReference,
        root_payload: Vec<u8>,
        left_child: SlotGenerationCell,
        right_child: SlotGenerationCell,
        exact_counters: BaselineBTreeExactCounterWitness,
    ) -> Self {
        Self {
            plan_binding,
            publication_report,
            root_reference,
            root_payload,
            left_child,
            right_child,
            exact_counters,
        }
    }

    pub fn published_layout(&self) -> &PersistedPhysicalLayout {
        self.publication_report.persisted_layout()
    }
    pub const fn plan_binding(&self) -> S8PreExecutionPlanBinding {
        self.plan_binding
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
        self.published_layout()
            .root_manifest_candidates()
            .len()
            .try_into()
            .unwrap_or(u16::MAX)
    }
    pub const fn exact_counters(&self) -> BaselineBTreeExactCounterWitness {
        self.exact_counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineBTreeReplayRecoveryExecution {
    plan_binding: S8PreExecutionPlanBinding,
    reopened_layout: PersistedPhysicalLayout,
    published_root_reference: PhysicalReference,
    published_root_manifest: Vec<u8>,
    rebuild_authority_records: u16,
    rebuild_output_records: u16,
    rebuild_source_authoritative: bool,
    exact_counters: BaselineBTreeExactCounterWitness,
}

impl BaselineBTreeReplayRecoveryExecution {
    pub(super) fn new(
        plan_binding: S8PreExecutionPlanBinding,
        reopened_layout: PersistedPhysicalLayout,
        published_root_reference: PhysicalReference,
        published_root_manifest: Vec<u8>,
        rebuild_authority_records: u16,
        rebuild_output_records: u16,
        rebuild_source_authoritative: bool,
        exact_counters: BaselineBTreeExactCounterWitness,
    ) -> Self {
        Self {
            plan_binding,
            reopened_layout,
            published_root_reference,
            published_root_manifest,
            rebuild_authority_records,
            rebuild_output_records,
            rebuild_source_authoritative,
            exact_counters,
        }
    }

    pub fn reopened_layout(&self) -> &PersistedPhysicalLayout {
        &self.reopened_layout
    }
    pub const fn plan_binding(&self) -> S8PreExecutionPlanBinding {
        self.plan_binding
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
    pub const fn exact_counters(&self) -> BaselineBTreeExactCounterWitness {
        self.exact_counters
    }
}
