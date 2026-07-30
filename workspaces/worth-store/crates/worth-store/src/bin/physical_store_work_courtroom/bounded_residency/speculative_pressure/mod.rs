use worth_store::physical_runtime::{
    PhysicalRecordId, PhysicalSpeculativeWorkKind, ServingPhysicalRuntime,
};

use super::configuration::BoundedResidencyConfiguration;
use super::schedule::{BoundedResidencySchedulePlan, ExecutedPrefetchSchedule};

mod causal_binding;
mod frame_coordinates;
mod read_pressure;
mod writebehind_pressure;

#[derive(Debug, Clone, Copy)]
pub(super) struct SpeculativeKindEvidence {
    pub(super) attempts: u64,
    pub(super) admissions: u64,
    pub(super) denials: u64,
    pub(super) completions: u64,
    pub(super) peak_frames: u32,
    pub(super) terminal_frames: u32,
    pub(super) hits: u64,
    pub(super) effectful_misses: u64,
    pub(super) hit_signal_requests: u64,
    pub(super) denial_signal_requests: u64,
    pub(super) effectful_signal_requests: u64,
    pub(super) signal_family_exact: bool,
    pub(super) foundational_basis_exact: bool,
}

pub(super) struct BoundedSpeculativePressureEvidence {
    pub(super) prefetch: SpeculativeKindEvidence,
    pub(super) read_ahead: SpeculativeKindEvidence,
    pub(super) write_behind: SpeculativeKindEvidence,
}

pub(super) struct BoundedSpeculativePressureProof {
    pub(super) evidence: BoundedSpeculativePressureEvidence,
    pub(super) schedule: ExecutedPrefetchSchedule,
}

#[derive(Debug, Clone, Copy)]
struct SpeculativePathEvidence {
    hits: u64,
    effectful_misses: u64,
    hit_signal_requests: u64,
    denial_signal_requests: u64,
    effectful_signal_requests: u64,
}

pub(super) fn prove(
    serving: &ServingPhysicalRuntime,
    records: &[PhysicalRecordId],
    configuration: BoundedResidencyConfiguration,
    schedule: BoundedResidencySchedulePlan,
) -> Result<BoundedSpeculativePressureProof, String> {
    let coordinates = frame_coordinates::discover(serving, records, configuration)?;
    let residency = serving.certification_physical_residency();
    residency.drain_unpinned_clean_frames();
    let prefetch = read_pressure::prove_prefetch(
        serving,
        &residency,
        &coordinates,
        schedule.worker_start_order(),
        schedule.ready_work_selection(),
    )?;
    let read_ahead = read_pressure::prove_read_ahead(serving, &residency, &coordinates)?;
    finish(serving, &residency, &coordinates, prefetch, read_ahead)
}

fn finish(
    serving: &ServingPhysicalRuntime,
    residency: &worth_store::physical_runtime::PhysicalResidencyCertification,
    coordinates: &[worth_store_physical_format::RecordFrameCoordinate; 8],
    prefetch: read_pressure::PrefetchProof,
    read_ahead: SpeculativeKindEvidence,
) -> Result<BoundedSpeculativePressureProof, String> {
    let write_behind = writebehind_pressure::prove(serving, residency, &coordinates[4..6])?;
    Ok(BoundedSpeculativePressureProof {
        evidence: BoundedSpeculativePressureEvidence {
            prefetch: prefetch.evidence,
            read_ahead,
            write_behind,
        },
        schedule: prefetch.schedule,
    })
}

fn counter_evidence(
    kind: PhysicalSpeculativeWorkKind,
    before: worth_store_buffer_pool::PhysicalResidencyCounters,
    after: worth_store_buffer_pool::PhysicalResidencyCounters,
    path: SpeculativePathEvidence,
) -> SpeculativeKindEvidence {
    SpeculativeKindEvidence {
        attempts: after
            .speculative_attempts(kind)
            .saturating_sub(before.speculative_attempts(kind)),
        admissions: after
            .speculative_admissions(kind)
            .saturating_sub(before.speculative_admissions(kind)),
        denials: after
            .speculative_denials(kind)
            .saturating_sub(before.speculative_denials(kind)),
        completions: after
            .speculative_completions(kind)
            .saturating_sub(before.speculative_completions(kind)),
        peak_frames: after.peak_speculative_frames(kind),
        terminal_frames: after.active_speculative_frames(kind),
        hits: path.hits,
        effectful_misses: path.effectful_misses,
        hit_signal_requests: path.hit_signal_requests,
        denial_signal_requests: path.denial_signal_requests,
        effectful_signal_requests: path.effectful_signal_requests,
        signal_family_exact: true,
        foundational_basis_exact: true,
    }
}

fn signal_requests(serving: &ServingPhysicalRuntime) -> Result<u64, String> {
    serving
        .physical_signal_observation()
        .map(|observation| observation.request_admission_count())
        .map_err(|failure| format!("bounded-residency Signal observation failed: {failure:?}"))
}
