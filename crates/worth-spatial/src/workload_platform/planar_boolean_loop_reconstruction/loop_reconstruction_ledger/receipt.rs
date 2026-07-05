use super::counters::PlanarBooleanLoopReconstructionLedgerCounters;
use super::identity::{downstream_consumption_identity, receipt_identity};
use super::ledger::PlanarBooleanLoopReconstructionLedger;
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceReceiptSealed, BooleanEvidenceRowAuthority,
    BooleanEvidenceStageKind, WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionLedgerReceipt {
    receipt_identity: String,
    ledger_identity: String,
    downstream_consumption_identity: String,
    request_identity: String,
    selected_plan_digest: String,
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    selected_witness_identity_digest: Option<String>,
    touched_closure_digest: String,
    overlap_identity_digests: Vec<String>,
    topology_query_posture_digest: String,
    spatial_query_posture_digest: String,
    residue_digest: String,
    source_firewall_digest: String,
    architecture_claim_digest: String,
    decision_log_identity: String,
    loop_identity_map_identity: String,
    persistent_name_map_identity: String,
    subshape_signature_map_identity: String,
    ledger_row_identities: Vec<String>,
    counters: PlanarBooleanLoopReconstructionLedgerCounters,
}

impl PlanarBooleanLoopReconstructionLedgerReceipt {
    pub(crate) fn from_ledger(ledger: &PlanarBooleanLoopReconstructionLedger) -> Self {
        let ledger_row_identities = ledger
            .rows()
            .iter()
            .map(|row| row.ledger_row_identity().to_string())
            .collect::<Vec<_>>();
        let consumed_identities = vec![
            ledger.request_identity().to_string(),
            ledger.decision_log_identity().to_string(),
            ledger.loop_identity_map_identity().to_string(),
            ledger.persistent_name_map_identity().to_string(),
            ledger.subshape_signature_map_identity().to_string(),
        ];
        let receipt_identity = receipt_identity(ledger.ledger_identity(), &consumed_identities);
        let mut counters = ledger.counters();
        counters.emitted_downstream_identity();
        Self {
            downstream_consumption_identity: downstream_consumption_identity(&receipt_identity),
            receipt_identity,
            ledger_identity: ledger.ledger_identity().to_string(),
            request_identity: ledger.request_identity().to_string(),
            selected_plan_digest: ledger.selected_plan_digest().to_string(),
            selected_route_identity_digest: ledger.selected_route_identity_digest().to_string(),
            selected_family_identity: ledger.selected_family_identity().to_string(),
            selected_product_identity_digest: ledger.selected_product_identity_digest().to_string(),
            selected_witness_identity_digest: ledger
                .selected_witness_identity_digest()
                .map(str::to_string),
            touched_closure_digest: ledger.touched_closure_digest().to_string(),
            overlap_identity_digests: ledger.overlap_identity_digests().to_vec(),
            topology_query_posture_digest: ledger.topology_query_posture_digest().to_string(),
            spatial_query_posture_digest: ledger.spatial_query_posture_digest().to_string(),
            residue_digest: ledger.residue_digest().to_string(),
            source_firewall_digest: ledger.source_firewall_digest().to_string(),
            architecture_claim_digest: ledger.architecture_claim_digest().to_string(),
            decision_log_identity: ledger.decision_log_identity().to_string(),
            loop_identity_map_identity: ledger.loop_identity_map_identity().to_string(),
            persistent_name_map_identity: ledger.persistent_name_map_identity().to_string(),
            subshape_signature_map_identity: ledger.subshape_signature_map_identity().to_string(),
            ledger_row_identities,
            counters,
        }
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }

    pub fn ledger_identity(&self) -> &str {
        &self.ledger_identity
    }

    pub fn downstream_consumption_identity(&self) -> &str {
        &self.downstream_consumption_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
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

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn overlap_identity_digests(&self) -> &[String] {
        &self.overlap_identity_digests
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

    pub fn decision_log_identity(&self) -> &str {
        &self.decision_log_identity
    }

    pub fn loop_identity_map_identity(&self) -> &str {
        &self.loop_identity_map_identity
    }

    pub fn persistent_name_map_identity(&self) -> &str {
        &self.persistent_name_map_identity
    }

    pub fn subshape_signature_map_identity(&self) -> &str {
        &self.subshape_signature_map_identity
    }

    pub fn ledger_row_identities(&self) -> &[String] {
        &self.ledger_row_identities
    }

    pub fn counters(&self) -> PlanarBooleanLoopReconstructionLedgerCounters {
        self.counters
    }

    #[cfg(test)]
    pub(crate) fn with_receipt_identity_for_tests(
        &self,
        receipt_identity: impl Into<String>,
    ) -> Self {
        let mut cloned = self.clone();
        cloned.receipt_identity = receipt_identity.into();
        cloned
    }

    #[cfg(test)]
    pub(crate) fn with_selected_plan_digest_for_tests(
        &self,
        selected_plan_digest: impl Into<String>,
    ) -> Self {
        let mut cloned = self.clone();
        cloned.selected_plan_digest = selected_plan_digest.into();
        cloned
    }

    #[cfg(test)]
    pub(crate) fn with_selected_route_identity_digest_for_tests(
        &self,
        selected_route_identity_digest: impl Into<String>,
    ) -> Self {
        let mut cloned = self.clone();
        cloned.selected_route_identity_digest = selected_route_identity_digest.into();
        cloned
    }
}

impl BooleanEvidenceReceipt for PlanarBooleanLoopReconstructionLedgerReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::LoopReconstruction
    }

    fn evidence_identity(&self) -> &str {
        self.receipt_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_loop_reconstruction()
    }
}

impl BooleanEvidenceReceiptSealed for PlanarBooleanLoopReconstructionLedgerReceipt {}

impl BooleanEvidenceRowAuthority for PlanarBooleanLoopReconstructionLedgerReceipt {}
