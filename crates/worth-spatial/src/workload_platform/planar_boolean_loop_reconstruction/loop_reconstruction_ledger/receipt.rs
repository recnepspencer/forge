use super::counters::PlanarBooleanLoopReconstructionLedgerCounters;
use super::identity::{downstream_consumption_identity, receipt_identity};
use super::ledger::PlanarBooleanLoopReconstructionLedger;
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, BooleanEvidenceStageKind,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionLedgerReceipt {
    receipt_identity: String,
    ledger_identity: String,
    downstream_consumption_identity: String,
    request_identity: String,
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

impl BooleanEvidenceRowAuthority for PlanarBooleanLoopReconstructionLedgerReceipt {}
