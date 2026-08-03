use worth_store::physical_runtime::{PhysicalResidencyDimension, PhysicalSpeculativeWorkKind};

use super::SpeculativeKindEvidence;

mod prefetch;
mod read_ahead;

pub(super) use prefetch::{prove as prove_prefetch, PrefetchProof};
pub(super) use read_ahead::prove as prove_read_ahead;

const FRAME_READ_BASIS: &str = "store.physical.record.frame-read-basis";

#[derive(Clone, Copy)]
struct ExpectedReadEvidence {
    attempts: u64,
    admissions: u64,
    completions: u64,
    peak: u32,
    hits: u64,
    misses: u64,
}

fn require_pressure(
    pressure: Option<worth_store::physical_runtime::PhysicalRecordPressureEvidence>,
    kind: PhysicalSpeculativeWorkKind,
    requested: u64,
    admitted: u64,
    limit: u64,
    label: &str,
) -> Result<(), String> {
    let pressure = pressure.ok_or_else(|| format!("{label} denial omitted pressure evidence"))?;
    if pressure.dimension() != PhysicalResidencyDimension::SpeculativeFrames(kind)
        || pressure.requested() != requested
        || pressure.admitted() != admitted
        || pressure.limit() != limit
        || pressure.effect_may_have_started()
    {
        return Err(format!("{label} pressure evidence drifted: {pressure:?}"));
    }
    Ok(())
}

fn await_arrivals(
    gate: &worth_store::physical_runtime::certification::CertificationPhysicalExecutionPauseGate,
    expected: usize,
) -> Result<(), String> {
    if !gate.await_arrivals(expected) || gate.arrival_count() != expected {
        gate.release();
        return Err(format!(
            "bounded speculation reached {} of {expected} executor arrivals",
            gate.arrival_count()
        ));
    }
    Ok(())
}

fn require_evidence(
    evidence: SpeculativeKindEvidence,
    expected: ExpectedReadEvidence,
    label: &str,
) -> Result<SpeculativeKindEvidence, String> {
    if evidence.attempts != expected.attempts
        || evidence.admissions != expected.admissions
        || evidence.denials != 1
        || evidence.completions != expected.completions
        || evidence.peak_frames != expected.peak
        || evidence.terminal_frames != 0
        || evidence.hits != expected.hits
        || evidence.effectful_misses != expected.misses
        || evidence.hit_signal_requests != 0
        || evidence.denial_signal_requests != 0
        || evidence.effectful_signal_requests != expected.misses
    {
        return Err(format!("{label} evidence did not reconcile: {evidence:?}"));
    }
    Ok(evidence)
}
