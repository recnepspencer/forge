use worth_kernel::workload_composition::{
    PlanarBooleanBlockerEvidenceReceipt, PlanarBooleanOperandPairConstructionReceipt, WorthWorkload,
};
use worth_spatial::certification::workload_evidence::ledger_with_manual_stage_substitution;
use worth_spatial::facade::workload_vocabulary::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedgerError, WorkloadEvidenceStage,
};

use super::PlanarBoolean7_0CloseoutError;

pub(super) fn topology_guard_identity(
    ledger: &CompleteWorkloadEvidenceLedger,
) -> Result<&'static str, PlanarBoolean7_0CloseoutError> {
    let error = ledger_with_manual_stage_substitution(ledger, WorkloadEvidenceStage::Topology)
        .and_then(|ledger| ledger.certify_complete())
        .expect_err("manual topology substitution must fail");
    match error {
        WorkloadEvidenceLedgerError::ManualAuthorityStage(WorkloadEvidenceStage::Topology) => {
            Ok("manual-topology-stage")
        }
        _ => Err(PlanarBoolean7_0CloseoutError::InvalidAntiTheatreGuard(
            "topology",
        )),
    }
}

pub(super) fn blocker_guard_identity(
    workload: &WorthWorkload,
    blocker: &PlanarBooleanBlockerEvidenceReceipt,
) -> Result<&'static str, PlanarBoolean7_0CloseoutError> {
    workload
        .require_boolean_blocker_provenance(blocker)
        .map(|_| "blocker-provenance-stage")
        .map_err(|_| PlanarBoolean7_0CloseoutError::InvalidAntiTheatreGuard("blocker"))
}

pub(super) fn catalog_guard_identity(
    workload: &WorthWorkload,
    pair_construction: &PlanarBooleanOperandPairConstructionReceipt,
) -> Result<&'static str, PlanarBoolean7_0CloseoutError> {
    workload
        .require_boolean_operand_pair_construction(pair_construction)
        .map(|_| "catalog-pair-stage")
        .map_err(|_| PlanarBoolean7_0CloseoutError::InvalidAntiTheatreGuard("catalog"))
}
