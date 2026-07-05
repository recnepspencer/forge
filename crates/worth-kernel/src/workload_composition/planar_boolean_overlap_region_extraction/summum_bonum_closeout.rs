use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionSummumBonumCloseout,
    PlanarBooleanOverlapRegionSummumBonumCloseoutDenial,
    PlanarBooleanOverlapRegionSummumBonumCloseoutDenialKind as DenialKind,
    PlanarBooleanOverlapRegionSummumBonumCloseoutInput as SpatialSummumBonumInput,
};

use crate::workload_composition::WorkloadCompositionError;

use super::{
    CompletedPlanarBooleanOverlapRegionExtractionHandoff,
    PlanarBooleanOverlapRegionPublicContractProofRowKind as Kind,
    PlanarBooleanOverlapRegionSummumBonumCloseoutInput,
};

impl CompletedPlanarBooleanOverlapRegionExtractionHandoff {
    pub fn certify_planar_boolean_overlap_region_summum_bonum(
        &self,
        input: PlanarBooleanOverlapRegionSummumBonumCloseoutInput<'_>,
    ) -> Result<PlanarBooleanOverlapRegionSummumBonumCloseout, WorkloadCompositionError> {
        ensure_phase_fifteen_fences(self)?;
        PlanarBooleanOverlapRegionSummumBonumCloseout::certify(SpatialSummumBonumInput::new(
            input.readiness(),
            input.readiness_consumer(),
            input.readiness_binding(),
            self.overlap_ledger_bundle(),
            self.evidence_receipt(),
            self.replay_parity_receipt(),
            self.checkpoint_parity_receipt(),
        ))
        .map_err(WorkloadCompositionError::OverlapRegionSummumBonumCloseout)
    }
}

fn ensure_phase_fifteen_fences(
    handoff: &CompletedPlanarBooleanOverlapRegionExtractionHandoff,
) -> Result<(), WorkloadCompositionError> {
    let required_public_rows = [
        Kind::ReadinessHandoff,
        Kind::ReadinessConsumer,
        Kind::ReadinessBinding,
        Kind::OverlapLedgerReceipt,
        Kind::OverlapEvidenceReceipt,
        Kind::RuntimeRegistrationProof,
        Kind::WorkloadStageIndex,
        Kind::RequestIdentity,
    ];
    for kind in required_public_rows {
        if handoff
            .public_contract_fence_proof()
            .rows()
            .iter()
            .all(|row| row.kind() != kind)
        {
            return Err(WorkloadCompositionError::OverlapRegionSummumBonumCloseout(
                PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
                    DenialKind::MissingPhaseFifteenPublicContractRow,
                    "phase_fifteen_fence",
                    format!("phase-16 closeout requires phase-15 public contract row {kind:?}"),
                ),
            ));
        }
    }
    let required_guards = [
        "synthetic_readiness_rejected",
        "raw_loop_ledger_rejected",
        "copied_overlap_rows_rejected",
        "bypassed_arrangement_or_cell_proof_rejected",
    ];
    for guard in required_guards {
        if handoff
            .anti_theatre_fence_proof()
            .guard_names()
            .iter()
            .all(|name| name != guard)
        {
            return Err(WorkloadCompositionError::OverlapRegionSummumBonumCloseout(
                PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
                    DenialKind::MissingPhaseFifteenAntiTheatreGuard,
                    "phase_fifteen_fence",
                    format!("phase-16 closeout requires anti-theatre guard `{guard}`"),
                ),
            ));
        }
    }
    Ok(())
}
