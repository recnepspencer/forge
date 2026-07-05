use forge_store_budgets::CounterEvidenceStrength;

use crate::queue_execution::QueueExecutionViolationCause;
use crate::{
    QueueBackpressureCause, QueueExecutionCounterSnapshot, QueueExecutionOutcome,
    QueueExecutionReplayIdentity,
};

use super::requirements::{
    require_any_attribution, require_claim_rows, require_violation_attribution,
};
use super::{
    InterferenceAttribution, InterferenceCounterDenial, InterferenceCounterName,
    InterferenceCounterRow, InterferenceReplayScope, LatencyEnvelopeClaim,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LatencyEnvelopeAssessmentStatus {
    Held,
    ExecutionViolated,
    BackendContradictedWitness,
    EnvelopeExceeded,
    PolicyDebtIncurred,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LatencyEnvelopeAssessment {
    status: LatencyEnvelopeAssessmentStatus,
    replay_identity: QueueExecutionReplayIdentity,
    replay_scope: InterferenceReplayScope,
    counter_rows: Vec<InterferenceCounterRow>,
}

pub fn assess_queue_latency_envelope(
    claim: &LatencyEnvelopeClaim,
    outcome: &QueueExecutionOutcome,
) -> Result<LatencyEnvelopeAssessment, InterferenceCounterDenial> {
    let (replay_identity, counters) = queue_replay_and_counters(outcome);
    if replay_identity != claim.replay_identity() {
        return Err(InterferenceCounterDenial::LaneMismatch);
    }
    let envelope_exceeded =
        queue_interference_events(counters) > claim.max_interference_events().unwrap_or(u64::MAX);
    let status = queue_assessment_status(outcome, envelope_exceeded);
    let mut rows = queue_counter_rows(claim, counters, outcome, status);
    require_claim_rows(claim, &rows)?;
    if status != LatencyEnvelopeAssessmentStatus::Held {
        require_violation_attribution(&rows)?;
    }
    if claim.requires_attribution() {
        require_any_attribution(&rows)?;
    }
    rows.shrink_to_fit();
    Ok(LatencyEnvelopeAssessment {
        status,
        replay_identity,
        replay_scope: InterferenceReplayScope::deterministic_policy_counter_and_proof_scope(),
        counter_rows: rows,
    })
}

impl LatencyEnvelopeAssessment {
    pub const fn status(&self) -> LatencyEnvelopeAssessmentStatus {
        self.status
    }

    pub const fn replay_identity(&self) -> QueueExecutionReplayIdentity {
        self.replay_identity
    }

    pub const fn replay_scope(&self) -> InterferenceReplayScope {
        self.replay_scope
    }

    pub fn counter_rows(&self) -> &[InterferenceCounterRow] {
        &self.counter_rows
    }
}

fn queue_replay_and_counters(
    outcome: &QueueExecutionOutcome,
) -> (QueueExecutionReplayIdentity, QueueExecutionCounterSnapshot) {
    match outcome {
        QueueExecutionOutcome::Executed(evidence) => {
            (evidence.plan().replay_identity(), evidence.counters())
        }
        QueueExecutionOutcome::Backpressured(evidence) => {
            (evidence.plan().replay_identity(), evidence.counters())
        }
        QueueExecutionOutcome::Denied(evidence) => {
            (evidence.plan().replay_identity(), evidence.counters())
        }
        QueueExecutionOutcome::Violation(evidence) => {
            (evidence.plan().replay_identity(), evidence.counters())
        }
    }
}

fn queue_counter_rows(
    claim: &LatencyEnvelopeClaim,
    counters: QueueExecutionCounterSnapshot,
    outcome: &QueueExecutionOutcome,
    status: LatencyEnvelopeAssessmentStatus,
) -> Vec<InterferenceCounterRow> {
    let lane = claim.lane();
    let profile = claim.profile_scope();
    let backpressure = counters
        .backpressure_cause()
        .map(InterferenceAttribution::Backpressure);
    let violation = if matches!(outcome, QueueExecutionOutcome::Violation(_)) {
        Some(InterferenceAttribution::ExecutionViolation)
    } else {
        None
    };
    let flush_delay = if counters.backpressure_cause() == Some(QueueBackpressureCause::FlushDelayed)
    {
        counters.backpressure_events()
    } else {
        0
    };
    let backend_contradiction =
        u64::from(status == LatencyEnvelopeAssessmentStatus::BackendContradictedWitness);
    let envelope_exceeded = u64::from(status == LatencyEnvelopeAssessmentStatus::EnvelopeExceeded);
    vec![
        row(
            InterferenceCounterName::QueueSubmittedUnits,
            counters.submitted_units(),
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            None,
        ),
        row(
            InterferenceCounterName::QueueAdmittedUnits,
            counters.admitted_units(),
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            None,
        ),
        row(
            InterferenceCounterName::QueueDeniedUnits,
            counters.denied_units(),
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            backpressure,
        ),
        row(
            InterferenceCounterName::QueuePeakDepth,
            u64::from(counters.peak_queue_depth()),
            CounterEvidenceStrength::Sampled,
            profile,
            lane,
            Some(InterferenceAttribution::Queueing),
        ),
        row(
            InterferenceCounterName::QueueGroupedWrites,
            u64::from(counters.grouped_writes()),
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            None,
        ),
        row(
            InterferenceCounterName::QueueReadAheadUnits,
            counters.read_ahead_units(),
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            None,
        ),
        row(
            InterferenceCounterName::QueueWriteBackUnits,
            counters.write_back_units(),
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            None,
        ),
        row(
            InterferenceCounterName::QueueBackpressureEvents,
            counters.backpressure_events(),
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            backpressure,
        ),
        row(
            InterferenceCounterName::QueueForegroundWaitEvents,
            counters.foreground_wait_events(),
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            Some(InterferenceAttribution::ForegroundWait),
        ),
        row(
            InterferenceCounterName::QueueMechanicalRetries,
            counters.mechanical_retries(),
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            None,
        ),
        row(
            InterferenceCounterName::QueuePartialReadEvents,
            counters.partial_read_events(),
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            None,
        ),
        row(
            InterferenceCounterName::QueueShortWriteEvents,
            counters.short_write_events(),
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            None,
        ),
        row(
            InterferenceCounterName::QueueViolationEvents,
            counters.violation_events(),
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            violation,
        ),
        row(
            InterferenceCounterName::FlushDelayEvents,
            flush_delay,
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            nonzero(flush_delay, InterferenceAttribution::FlushDelay),
        ),
        row(
            InterferenceCounterName::SyncDebtUnits,
            claim.replay_identity().requested_budget().sync_debt(),
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            nonzero(
                claim.replay_identity().requested_budget().sync_debt(),
                InterferenceAttribution::SyncDebt,
            ),
        ),
        row(
            InterferenceCounterName::PageCacheWaitEvents,
            0,
            CounterEvidenceStrength::Unavailable,
            profile,
            lane,
            None,
        ),
        row(
            InterferenceCounterName::WorkerHandoffWaitEvents,
            0,
            CounterEvidenceStrength::Unavailable,
            profile,
            lane,
            None,
        ),
        row(
            InterferenceCounterName::BackendContradictionEvents,
            backend_contradiction,
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            nonzero(
                backend_contradiction,
                InterferenceAttribution::BackendContradictedWitness,
            ),
        ),
        row(
            InterferenceCounterName::EnvelopeExceededEvents,
            envelope_exceeded,
            CounterEvidenceStrength::Exact,
            profile,
            lane,
            nonzero(envelope_exceeded, InterferenceAttribution::EnvelopeExceeded),
        ),
        row(
            InterferenceCounterName::PolicyDebtEvents,
            0,
            CounterEvidenceStrength::Unavailable,
            profile,
            lane,
            None,
        ),
    ]
}

fn queue_assessment_status(
    outcome: &QueueExecutionOutcome,
    envelope_exceeded: bool,
) -> LatencyEnvelopeAssessmentStatus {
    if let QueueExecutionOutcome::Violation(violation) = outcome {
        return match violation.cause() {
            QueueExecutionViolationCause::BackendContradictedWitness => {
                LatencyEnvelopeAssessmentStatus::BackendContradictedWitness
            }
            QueueExecutionViolationCause::ExecutionReclassifiedWork => {
                LatencyEnvelopeAssessmentStatus::ExecutionViolated
            }
        };
    }
    if envelope_exceeded {
        LatencyEnvelopeAssessmentStatus::EnvelopeExceeded
    } else {
        LatencyEnvelopeAssessmentStatus::Held
    }
}

fn row(
    name: InterferenceCounterName,
    value: u64,
    strength: CounterEvidenceStrength,
    profile: &'static str,
    lane: crate::QueueWorkClass,
    attribution: Option<InterferenceAttribution>,
) -> InterferenceCounterRow {
    InterferenceCounterRow::new(name, value, strength, profile, lane, attribution)
}

fn nonzero(value: u64, attribution: InterferenceAttribution) -> Option<InterferenceAttribution> {
    if value > 0 {
        Some(attribution)
    } else {
        None
    }
}

fn queue_interference_events(counters: QueueExecutionCounterSnapshot) -> u64 {
    counters
        .foreground_wait_events()
        .saturating_add(counters.backpressure_events())
        .saturating_add(counters.violation_events())
}
