use super::BaselineBTreeExactCounterWitness;
use crate::keyspace::AdmittedPhysicalAccessIdentity;
use crate::planning::AccessPlanIdentity;
use forge_store_physical_format::{
    PersistedPhysicalLayout, PhysicalRecordSlot, PhysicalReference, PlatformPhysicalFacadeDenial,
};
use forge_store_physical_isolation::{
    CompactionProtectedReferenceSet, PhysicalReadPlanAdmissionDenial, StablePhysicalReadReceipt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaselineBTreeExecutionDenial {
    Physical(PlatformPhysicalFacadeDenial),
    InvalidRootNode,
    InvalidLeafNode,
    InvalidPhysicalReferenceForBTree,
    WrongSelectedOperation,
    StableReadPlan(PhysicalReadPlanAdmissionDenial),
    Recovery(forge_store_recovery_physics::BTreeReplaySourceDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineBTreeLookupAbsence {
    plan_binding: AccessPlanIdentity,
    request_identity: AdmittedPhysicalAccessIdentity,
    probe_slot: PhysicalRecordSlot,
    selected_leaf: PhysicalReference,
    current_materialization: crate::CurrentLayoutMaterialization,
}

impl BaselineBTreeLookupAbsence {
    pub(super) fn issue(
        admission: &super::BaselineBTreeLookupAdmission,
        probe_slot: PhysicalRecordSlot,
        selected_leaf: PhysicalReference,
    ) -> Self {
        Self {
            plan_binding: admission.plan_binding().clone(),
            request_identity: admission.request_identity(),
            probe_slot,
            selected_leaf,
            current_materialization: admission.current_materialization().clone(),
        }
    }

    pub const fn plan_binding(&self) -> &AccessPlanIdentity {
        &self.plan_binding
    }
    pub const fn request_identity(&self) -> AdmittedPhysicalAccessIdentity {
        self.request_identity
    }
    pub const fn probe_slot(&self) -> PhysicalRecordSlot {
        self.probe_slot
    }
    pub const fn selected_leaf(&self) -> PhysicalReference {
        self.selected_leaf
    }
    pub const fn current_materialization(&self) -> &crate::CurrentLayoutMaterialization {
        &self.current_materialization
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum BaselineBTreeLookupObservation {
    Found(BaselineBTreeLookupExecution),
    Absent(BaselineBTreeLookupAbsence),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BTreeLookupExecutionCaseId(&'static str);

impl BTreeLookupExecutionCaseId {
    pub const fn name(self) -> &'static str {
        self.0
    }
}

pub fn btree_lookup_execution_cases() -> impl Iterator<Item = BTreeLookupExecutionCaseId> {
    [
        BTreeLookupExecutionCaseId("layout.btree_lookup.execution.found"),
        BTreeLookupExecutionCaseId("layout.btree_lookup.execution.absent"),
    ]
    .into_iter()
}

impl BaselineBTreeLookupObservation {
    const fn case_id(&self) -> BTreeLookupExecutionCaseId {
        match self {
            Self::Found(_) => BTreeLookupExecutionCaseId("layout.btree_lookup.execution.found"),
            Self::Absent(_) => BTreeLookupExecutionCaseId("layout.btree_lookup.execution.absent"),
        }
    }
}

impl From<forge_store_recovery_physics::BTreeReplaySourceDenial> for BaselineBTreeExecutionDenial {
    fn from(value: forge_store_recovery_physics::BTreeReplaySourceDenial) -> Self {
        Self::Recovery(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StableBTreeLookupExecution {
    observation: BaselineBTreeLookupObservation,
    stable_read: StablePhysicalReadReceipt,
    protected: CompactionProtectedReferenceSet,
    current_materialization: crate::CurrentLayoutMaterialization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BTreeLookupExecutionView<'a> {
    Found(&'a BaselineBTreeLookupExecution),
    Absent(&'a BaselineBTreeLookupAbsence),
}

impl StableBTreeLookupExecution {
    pub(super) const fn new(
        observation: BaselineBTreeLookupObservation,
        stable_read: StablePhysicalReadReceipt,
        protected: CompactionProtectedReferenceSet,
        current_materialization: crate::CurrentLayoutMaterialization,
    ) -> Self {
        Self {
            observation,
            stable_read,
            protected,
            current_materialization,
        }
    }

    pub const fn view(&self) -> BTreeLookupExecutionView<'_> {
        match &self.observation {
            BaselineBTreeLookupObservation::Found(lookup) => {
                BTreeLookupExecutionView::Found(lookup)
            }
            BaselineBTreeLookupObservation::Absent(absence) => {
                BTreeLookupExecutionView::Absent(absence)
            }
        }
    }

    pub const fn case_id(&self) -> BTreeLookupExecutionCaseId {
        self.observation.case_id()
    }

    pub const fn stable_read(&self) -> &StablePhysicalReadReceipt {
        &self.stable_read
    }

    pub const fn protected(&self) -> &CompactionProtectedReferenceSet {
        &self.protected
    }

    pub const fn current_materialization(&self) -> &crate::CurrentLayoutMaterialization {
        &self.current_materialization
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineBTreeLookupExecution {
    plan_binding: AccessPlanIdentity,
    request_identity: AdmittedPhysicalAccessIdentity,
    shape: BaselineBTreeReadShape,
    probe_slot: PhysicalRecordSlot,
    separator_slot: PhysicalRecordSlot,
    branch: BaselineBTreeLookupBranch,
    selected_reference: PhysicalReference,
    exact_counters: BaselineBTreeExactCounterWitness,
}

impl BaselineBTreeLookupExecution {
    pub(super) const fn new(
        plan_binding: AccessPlanIdentity,
        request_identity: AdmittedPhysicalAccessIdentity,
        shape: BaselineBTreeReadShape,
        probe_slot: PhysicalRecordSlot,
        separator_slot: PhysicalRecordSlot,
        branch: BaselineBTreeLookupBranch,
        selected_reference: PhysicalReference,
        exact_counters: BaselineBTreeExactCounterWitness,
    ) -> Self {
        Self {
            plan_binding,
            request_identity,
            shape,
            probe_slot,
            separator_slot,
            branch,
            selected_reference,
            exact_counters,
        }
    }

    pub const fn shape(&self) -> BaselineBTreeReadShape {
        self.shape
    }
    pub const fn plan_binding(&self) -> &AccessPlanIdentity {
        &self.plan_binding
    }
    pub const fn request_identity(&self) -> AdmittedPhysicalAccessIdentity {
        self.request_identity
    }
    pub const fn probe_slot(&self) -> PhysicalRecordSlot {
        self.probe_slot
    }
    pub const fn separator_slot(&self) -> PhysicalRecordSlot {
        self.separator_slot
    }
    pub const fn branch(&self) -> BaselineBTreeLookupBranch {
        self.branch
    }
    pub const fn selected_reference(&self) -> PhysicalReference {
        self.selected_reference
    }
    pub const fn exact_counters(&self) -> BaselineBTreeExactCounterWitness {
        self.exact_counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineBTreeReplayRecoveryExecution {
    plan_binding: AccessPlanIdentity,
    request_identity: AdmittedPhysicalAccessIdentity,
    reopened_layout: PersistedPhysicalLayout,
    published_root_reference: PhysicalReference,
    published_root_manifest: Vec<u8>,
    rebuild_authority_records: u16,
    rebuild_output_records: u16,
    rebuild_source_authoritative: bool,
    exact_counters: BaselineBTreeExactCounterWitness,
    recovery_source_digest: String,
    current_materialization: crate::CurrentLayoutMaterialization,
}

impl BaselineBTreeReplayRecoveryExecution {
    pub(super) fn new(
        plan_binding: AccessPlanIdentity,
        request_identity: AdmittedPhysicalAccessIdentity,
        reopened_layout: PersistedPhysicalLayout,
        published_root_reference: PhysicalReference,
        published_root_manifest: Vec<u8>,
        rebuild_authority_records: u16,
        rebuild_output_records: u16,
        rebuild_source_authoritative: bool,
        exact_counters: BaselineBTreeExactCounterWitness,
        recovery_source_digest: String,
        current_materialization: crate::CurrentLayoutMaterialization,
    ) -> Self {
        Self {
            plan_binding,
            request_identity,
            reopened_layout,
            published_root_reference,
            published_root_manifest,
            rebuild_authority_records,
            rebuild_output_records,
            rebuild_source_authoritative,
            exact_counters,
            recovery_source_digest,
            current_materialization,
        }
    }

    pub fn reopened_layout(&self) -> &PersistedPhysicalLayout {
        &self.reopened_layout
    }
    pub const fn plan_binding(&self) -> &AccessPlanIdentity {
        &self.plan_binding
    }
    pub const fn request_identity(&self) -> AdmittedPhysicalAccessIdentity {
        self.request_identity
    }
    pub const fn current_materialization(&self) -> &crate::CurrentLayoutMaterialization {
        &self.current_materialization
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
    pub fn recovery_source_digest(&self) -> &str {
        &self.recovery_source_digest
    }
}
