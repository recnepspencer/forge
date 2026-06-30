use crate::spatial_compiled_product_family::SpatialCompiledProductLoweredIdentity;
use crate::workload_platform::evidence_ledger::WorkloadEvidenceRow;
use crate::workload_platform::evidence_lookup_query_surface_contract::{
    EvidenceLookupProductQuerySurfaceContractRow, EvidenceLookupQuerySurfaceContract,
};

use super::counters::EvidenceLookupIndexProductCounters;
use super::disposal_posture::EvidenceLookupIndexDisposalPosture;
use super::identity::index_product_digest;
use super::lifecycle_posture::EvidenceLookupIndexLifecyclePosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupIndexProduct {
    index_product_digest: String,
    compiled_product_identity_digest: String,
    equivalence_policy_identity_digest: String,
    reuse_decision_identity_digest: Option<String>,
    selected_plan_digest: String,
    spatial_touch_digest: String,
    stage_receipt_digest: String,
    evidence_ledger_basis_digest: String,
    topology_support_digest: String,
    query_support_digest: String,
    query_surface_contract_rows: Vec<EvidenceLookupProductQuerySurfaceContractRow>,
    lifecycle_posture: EvidenceLookupIndexLifecyclePosture,
    disposal_posture: EvidenceLookupIndexDisposalPosture,
    counters: EvidenceLookupIndexProductCounters,
    rows: Vec<WorkloadEvidenceRow>,
}

impl EvidenceLookupIndexProduct {
    pub(crate) fn new(
        lowered_identity: &SpatialCompiledProductLoweredIdentity,
        selected_plan_digest: String,
        spatial_touch_digest: String,
        stage_receipt_digest: String,
        evidence_ledger_basis_digest: String,
        topology_support_digest: String,
        query_support_digest: String,
        reuse_decision_identity_digest: Option<String>,
        query_surface_contract_rows: Vec<EvidenceLookupProductQuerySurfaceContractRow>,
        lifecycle_posture: EvidenceLookupIndexLifecyclePosture,
        disposal_posture: EvidenceLookupIndexDisposalPosture,
        counters: EvidenceLookupIndexProductCounters,
        rows: Vec<WorkloadEvidenceRow>,
    ) -> Self {
        let index_product_digest = index_product_digest(
            lowered_identity.compiled_product_identity(),
            lowered_identity.equivalence_policy_identity(),
            lifecycle_posture,
            disposal_posture,
            &counters,
        );
        Self {
            index_product_digest,
            compiled_product_identity_digest: lowered_identity
                .compiled_product_identity()
                .identity_digest()
                .to_string(),
            equivalence_policy_identity_digest: lowered_identity
                .equivalence_policy_identity()
                .identity_digest()
                .to_string(),
            reuse_decision_identity_digest,
            selected_plan_digest,
            spatial_touch_digest,
            stage_receipt_digest,
            evidence_ledger_basis_digest,
            topology_support_digest,
            query_support_digest,
            query_surface_contract_rows,
            lifecycle_posture,
            disposal_posture,
            counters,
            rows,
        }
    }

    pub fn index_product_digest(&self) -> &str {
        &self.index_product_digest
    }

    pub fn compiled_product_identity_digest(&self) -> &str {
        &self.compiled_product_identity_digest
    }

    pub fn equivalence_policy_identity_digest(&self) -> &str {
        &self.equivalence_policy_identity_digest
    }

    pub fn reuse_decision_identity_digest(&self) -> Option<&str> {
        self.reuse_decision_identity_digest.as_deref()
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn spatial_touch_digest(&self) -> &str {
        &self.spatial_touch_digest
    }

    pub fn stage_receipt_digest(&self) -> &str {
        &self.stage_receipt_digest
    }

    pub fn evidence_ledger_basis_digest(&self) -> &str {
        &self.evidence_ledger_basis_digest
    }

    pub fn topology_support_digest(&self) -> &str {
        &self.topology_support_digest
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn query_surface_contract_rows(&self) -> &[EvidenceLookupProductQuerySurfaceContractRow] {
        &self.query_surface_contract_rows
    }

    pub fn query_surface_contract_for_family(
        &self,
        family_identity: &str,
    ) -> Option<&EvidenceLookupQuerySurfaceContract> {
        self.query_surface_contract_rows
            .iter()
            .find(|row| row.family_identity() == family_identity)
            .map(EvidenceLookupProductQuerySurfaceContractRow::contract)
    }

    pub const fn lifecycle_posture(&self) -> EvidenceLookupIndexLifecyclePosture {
        self.lifecycle_posture
    }

    pub const fn disposal_posture(&self) -> EvidenceLookupIndexDisposalPosture {
        self.disposal_posture
    }

    pub const fn counters(&self) -> &EvidenceLookupIndexProductCounters {
        &self.counters
    }

    pub const fn claims_lookup_execution(&self) -> bool {
        false
    }

    pub const fn claims_persistent_capability(&self) -> bool {
        self.lifecycle_posture.claims_persistent_capability()
    }

    pub const fn claims_query_descriptor_authority(&self) -> bool {
        false
    }

    pub(crate) fn rows(&self) -> &[WorkloadEvidenceRow] {
        &self.rows
    }
}
