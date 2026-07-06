use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceReceiptSealed, BooleanEvidenceRowAuthority,
    BooleanEvidenceStageKind, WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

use super::evidence::PlanarBooleanOverlapRegionEvidenceReceipt;

impl PlanarBooleanOverlapRegionEvidenceReceipt {
    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }
    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }
    pub fn readiness_handoff_identity(&self) -> &str {
        &self.readiness_handoff_identity
    }
    pub fn readiness_consumer_identity(&self) -> &str {
        &self.readiness_consumer_identity
    }
    pub fn readiness_binding_identity(&self) -> &str {
        &self.readiness_binding_identity
    }
    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }
    pub fn selected_family_identity(&self) -> &str {
        &self.selected_family_identity
    }
    pub fn selected_product_identity_digest(&self) -> &str {
        &self.selected_product_identity_digest
    }
    pub fn selected_witness_identity_digest(&self) -> Option<&str> {
        self.selected_witness_identity_digest.as_deref()
    }
    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }
    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }
    pub fn topology_query_posture_digest(&self) -> &str {
        &self.topology_query_posture_digest
    }
    pub fn spatial_query_posture_digest(&self) -> &str {
        &self.spatial_query_posture_digest
    }
    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }
    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }
    pub fn architecture_claim_digest(&self) -> &str {
        &self.architecture_claim_digest
    }
    pub fn loop_ledger_receipt_identity(&self) -> &str {
        &self.loop_ledger_receipt_identity
    }
    pub fn overlap_ledger_receipt_identity(&self) -> &str {
        &self.overlap_ledger_receipt_identity
    }
    pub fn overlap_decision_log_identity(&self) -> &str {
        &self.overlap_decision_log_identity
    }
    pub fn overlap_ledger_identity(&self) -> &str {
        &self.overlap_ledger_identity
    }
    pub fn overlap_region_identity_map_identity(&self) -> &str {
        &self.overlap_region_identity_map_identity
    }
    pub fn persistent_name_propagation_map_identity(&self) -> &str {
        &self.persistent_name_propagation_map_identity
    }
    pub fn subshape_signature_map_identity(&self) -> &str {
        &self.subshape_signature_map_identity
    }
    pub fn replay_checkpoint_identity(&self) -> &str {
        &self.replay_checkpoint_identity
    }
    pub fn replay_evidence_identity(&self) -> &str {
        &self.replay_evidence_identity
    }
}

impl BooleanEvidenceReceipt for PlanarBooleanOverlapRegionEvidenceReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::Assemble
    }

    fn evidence_identity(&self) -> &str {
        self.receipt_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_assemble()
    }
}

impl BooleanEvidenceReceiptSealed for PlanarBooleanOverlapRegionEvidenceReceipt {}

impl BooleanEvidenceRowAuthority for PlanarBooleanOverlapRegionEvidenceReceipt {}
