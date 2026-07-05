use super::counters::PlanarBooleanLoopReconstructionRequestCounters;
use super::denial::{
    PlanarBooleanLoopReconstructionRequestDenial,
    PlanarBooleanLoopReconstructionRequestDenialKind as Kind,
};
use super::input::PlanarBooleanLoopReconstructionRequestInput;

pub(crate) fn validate_loop_reconstruction_request_input(
    input: &PlanarBooleanLoopReconstructionRequestInput<'_>,
    counters: &mut PlanarBooleanLoopReconstructionRequestCounters,
) -> Result<(), PlanarBooleanLoopReconstructionRequestDenial> {
    let split_consumption = input.split_consumption();
    reject_missing(
        split_consumption.consumption_identity(),
        Kind::MissingLoopSplitConsumption,
        "loop split consumption",
        counters,
        "loop reconstruction request requires the admitted loop split-consumption product",
    )?;
    reject_missing(
        split_consumption.split_ledger_receipt_identity(),
        Kind::MissingSplitLedgerReceipt,
        "split ledger receipt",
        counters,
        "loop reconstruction request requires a real split-ledger receipt lineage",
    )?;
    reject_missing(
        split_consumption.split_request_identity(),
        Kind::MissingSplitRequest,
        "split request",
        counters,
        "loop reconstruction request requires split request lineage",
    )?;
    reject_missing(
        split_consumption.workload_stage_index_identity(),
        Kind::MissingWorkloadStageIndex,
        "workload stage index",
        counters,
        "loop reconstruction request must preserve workload stage-index authority",
    )?;
    reject_missing(
        input.selected_plan_digest(),
        Kind::MissingWorkloadStageIndex,
        "selected plan digest",
        counters,
        "loop reconstruction request must preserve selected-plan readiness authority",
    )?;
    reject_missing(
        input.selected_route_identity_digest(),
        Kind::MissingWorkloadStageIndex,
        "selected route identity",
        counters,
        "loop reconstruction request must preserve selected-route readiness authority",
    )?;
    reject_missing(
        input.selected_family_identity(),
        Kind::MissingWorkloadStageIndex,
        "selected family identity",
        counters,
        "loop reconstruction request must preserve selected-family readiness authority",
    )?;
    reject_missing(
        input.selected_product_identity_digest(),
        Kind::MissingWorkloadStageIndex,
        "selected product identity",
        counters,
        "loop reconstruction request must preserve selected-product readiness authority",
    )?;
    if matches!(
        input.selected_witness_identity_digest(),
        Some(selected_witness_identity_digest) if selected_witness_identity_digest.is_empty()
    ) {
        counters.rejected_missing_authority();
        return Err(PlanarBooleanLoopReconstructionRequestDenial::new(
            Kind::MissingWorkloadStageIndex,
            "selected witness identity",
            *counters,
            "loop reconstruction request must preserve selected-witness readiness authority",
        ));
    }
    reject_missing(
        input.touched_closure_digest(),
        Kind::MissingWorkloadStageIndex,
        "touched closure digest",
        counters,
        "loop reconstruction request must preserve touched-closure readiness authority",
    )?;
    if input.overlap_identity_digests().is_empty() {
        counters.rejected_missing_authority();
        return Err(PlanarBooleanLoopReconstructionRequestDenial::new(
            Kind::MissingWorkloadStageIndex,
            "overlap identity digests",
            *counters,
            "loop reconstruction request must preserve overlap-identity readiness authority",
        ));
    }
    reject_missing(
        input.topology_query_posture_digest(),
        Kind::MissingWorkloadStageIndex,
        "topology Query posture digest",
        counters,
        "loop reconstruction request must preserve topology Query-posture readiness authority",
    )?;
    reject_missing(
        input.spatial_query_posture_digest(),
        Kind::MissingWorkloadStageIndex,
        "spatial Query posture digest",
        counters,
        "loop reconstruction request must preserve spatial Query-posture readiness authority",
    )?;
    reject_missing(
        input.residue_digest(),
        Kind::MissingWorkloadStageIndex,
        "residue digest",
        counters,
        "loop reconstruction request must preserve residue readiness authority",
    )?;
    reject_missing(
        input.source_firewall_digest(),
        Kind::MissingWorkloadStageIndex,
        "source firewall digest",
        counters,
        "loop reconstruction request must preserve source-firewall readiness authority",
    )?;
    reject_missing(
        input.architecture_claim_digest(),
        Kind::MissingWorkloadStageIndex,
        "architecture claim digest",
        counters,
        "loop reconstruction request must preserve architecture-claim readiness authority",
    )
}

fn reject_missing(
    observed: &str,
    kind: Kind,
    rejected_identity: &'static str,
    counters: &mut PlanarBooleanLoopReconstructionRequestCounters,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanLoopReconstructionRequestDenial> {
    if observed.is_empty() {
        counters.rejected_missing_authority();
        return Err(PlanarBooleanLoopReconstructionRequestDenial::new(
            kind,
            rejected_identity,
            *counters,
            human_reason,
        ));
    }
    Ok(())
}
