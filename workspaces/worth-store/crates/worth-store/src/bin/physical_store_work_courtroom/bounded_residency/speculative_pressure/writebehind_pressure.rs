use worth_store::physical_runtime::{
    PhysicalResidencyCertification, PhysicalSpeculativeWorkKind, PhysicalWorkEffectFate,
    PhysicalWorkSignalFamily, PhysicalWritebackExecution, ServingPhysicalRuntime,
};
use worth_store_buffer_pool::{
    PhysicalResidencyDenial, PhysicalResidencyDimension, PhysicalSpeculativeWorkKind as PoolKind,
};
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
use worth_store_physical_format::RecordFrameCoordinate;

use super::{
    causal_binding, counter_evidence, signal_requests, SpeculativeKindEvidence,
    SpeculativePathEvidence,
};

const FRAME_WRITEBACK_BASIS: &str = "store.physical.record.frame-writeback-basis";

pub(super) fn prove(
    serving: &ServingPhysicalRuntime,
    residency: &PhysicalResidencyCertification,
    coordinates: &[RecordFrameCoordinate],
) -> Result<SpeculativeKindEvidence, String> {
    let [first, second] = coordinates else {
        return Err("bounded write-behind coordinate width drifted".to_owned());
    };
    let kind = PhysicalSpeculativeWorkKind::WriteBehind;
    let counters_before = residency.counters();
    let first_dirty = unchanged_dirty(residency, *first)?;
    let second_dirty = unchanged_dirty(residency, *second)?;
    let first_prepared = residency
        .prepare_writeback(
            first_dirty,
            ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
        )
        .map_err(|failure| format!("bounded first write-behind prepare failed: {failure:?}"))?;
    let denial_signals_before = signal_requests(serving)?;
    let denied = match residency.prepare_writeback(
        second_dirty,
        ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
    ) {
        Ok(_) => return Err("one-past write-behind bypassed its kind ceiling".to_owned()),
        Err(denied) => denied,
    };
    require_writebehind_pressure(denied.cause())?;
    let denial_signal_requests = signal_requests(serving)?.saturating_sub(denial_signals_before);
    let second_dirty = denied.into_dirty();

    let effectful_signals_before = signal_requests(serving)?;
    let first_work = execute_clean(residency, first_prepared)?;
    let second_prepared = residency
        .prepare_writeback(
            second_dirty,
            ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
        )
        .map_err(|failure| format!("bounded second write-behind prepare failed: {failure:?}"))?;
    let second_work = execute_clean(residency, second_prepared)?;
    let effectful_signal_requests =
        signal_requests(serving)?.saturating_sub(effectful_signals_before);
    for work in [first_work, second_work] {
        causal_binding::require_exact(
            serving,
            work,
            PhysicalWorkSignalFamily::ExactWriteback,
            FRAME_WRITEBACK_BASIS,
        )?;
    }
    let evidence = counter_evidence(
        kind,
        counters_before,
        residency.counters(),
        SpeculativePathEvidence {
            hits: 0,
            effectful_misses: 2,
            hit_signal_requests: 0,
            denial_signal_requests,
            effectful_signal_requests,
        },
    );
    if evidence.attempts != 3
        || evidence.admissions != 2
        || evidence.denials != 1
        || evidence.completions != 2
        || evidence.peak_frames != 1
        || evidence.terminal_frames != 0
        || evidence.denial_signal_requests != 0
        || evidence.effectful_signal_requests != 2
    {
        return Err(format!(
            "bounded write-behind evidence did not reconcile: {evidence:?}"
        ));
    }
    Ok(evidence)
}

fn unchanged_dirty(
    residency: &PhysicalResidencyCertification,
    coordinate: RecordFrameCoordinate,
) -> Result<worth_store::physical_runtime::AdmittedDirtyFrame, String> {
    let resident = residency
        .pin_exact(coordinate)
        .map_err(|failure| format!("bounded write-behind pin failed: {failure:?}"))?;
    residency
        .admit_dirty_frame(resident, |source, target| target.copy_from_slice(source))
        .map_err(|failure| format!("bounded unchanged dirty transition failed: {failure:?}"))
}

fn execute_clean(
    residency: &PhysicalResidencyCertification,
    prepared: worth_store::physical_runtime::PreparedPhysicalWriteback,
) -> Result<worth_store::physical_runtime::PhysicalWorkIdentity, String> {
    let ready = residency
        .request_writeback(prepared)
        .map_err(|failure| format!("bounded write-behind readiness failed: {failure:?}"))?;
    let admitted = residency
        .admit_writeback(ready)
        .map_err(|failure| format!("bounded write-behind scheduling failed: {failure:?}"))?;
    match residency
        .execute_writeback(admitted)
        .map_err(|failure| format!("bounded write-behind execution failed: {failure:?}"))?
    {
        PhysicalWritebackExecution::Clean(settlement)
            if settlement.effect_fate() == PhysicalWorkEffectFate::WriteCompleted =>
        {
            Ok(settlement.identity())
        }
        outcome => Err(format!(
            "bounded unchanged write-behind did not settle cleanly: {}",
            outcome_name(&outcome)
        )),
    }
}

fn require_writebehind_pressure(
    cause: worth_store::physical_runtime::PhysicalWritebackFailureCause,
) -> Result<(), String> {
    let worth_store::physical_runtime::PhysicalWritebackFailureCause::Residency(
        PhysicalResidencyDenial::Pressure(pressure),
    ) = cause
    else {
        return Err(format!(
            "bounded write-behind denial had the wrong cause: {cause:?}"
        ));
    };
    if pressure.dimension() != PhysicalResidencyDimension::SpeculativeFrames(PoolKind::WriteBehind)
        || pressure.requested() != 1
        || pressure.current() != 1
        || pressure.limit() != 1
    {
        return Err(format!(
            "bounded write-behind pressure evidence drifted: {pressure:?}"
        ));
    }
    Ok(())
}

fn outcome_name(outcome: &PhysicalWritebackExecution) -> &'static str {
    match outcome {
        PhysicalWritebackExecution::Clean(_) => "clean-with-wrong-fate",
        PhysicalWritebackExecution::Retryable(_) => "retryable",
        PhysicalWritebackExecution::InspectionRequired(_) => "inspection-required",
    }
}
