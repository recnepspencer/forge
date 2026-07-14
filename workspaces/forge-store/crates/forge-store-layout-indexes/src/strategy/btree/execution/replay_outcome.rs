use super::BaselineBTreeExactCounterWitness;
use crate::{keyspace::AdmittedPhysicalAccessIdentity, planning::AccessPlanIdentity};
use forge_store_physical_format::{PersistedPhysicalLayout, PhysicalReference};

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
    #[allow(clippy::too_many_arguments)]
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
