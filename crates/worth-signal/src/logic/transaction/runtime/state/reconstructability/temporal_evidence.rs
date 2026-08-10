use serde::{Deserialize, Serialize};

use super::super::super::transaction::TemporalTransactionEvidence;
use super::super::merge::canonical_digest;
use super::super::temporal::TemporalRuntimeState;
use crate::data::temporal::{RuntimeClockBasis, TemporalWakeSummary};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalReconstructabilityArtifact {
    pub clock_basis: RuntimeClockBasis,
    pub wake_summary: TemporalWakeSummary,
    pub eligibility_fact_count: u64,
    pub scheduled_wake_count: u64,
    pub ready_wake_count: u64,
    pub retired_wake_count: u64,
    pub rescheduled_wake_count: u64,
    pub reused_wake_count: u64,
    pub interval_regeneration_count: u64,
    pub previous_value_reference_count: u64,
    pub clock_checkpoint_digest: String,
    pub scheduled_wake_digest: String,
    pub ready_wake_digest: String,
    pub retired_wake_digest: String,
    pub rescheduled_wake_digest: String,
    pub reused_wake_digest: String,
    pub interval_regeneration_digest: String,
    pub temporal_eligibility_digest: String,
    pub previous_value_reference_digest: String,
    pub certification_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TemporalReplayMismatchClass {
    ClockCheckpointDigestMismatch,
    ScheduledWakeDigestMismatch,
    ReadyWakeDigestMismatch,
    RetiredWakeDigestMismatch,
    RescheduledWakeDigestMismatch,
    ReusedWakeDigestMismatch,
    IntervalRegenerationDigestMismatch,
    TemporalEligibilityDigestMismatch,
    PreviousValueReferenceDigestMismatch,
    CertificationDigestMismatch,
    WakeSummaryMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemporalReplayParityReport {
    pub proof_schema_version: String,
    pub expected: TemporalReconstructabilityArtifact,
    pub replayed: TemporalReconstructabilityArtifact,
    pub parity: bool,
    pub mismatch_classes: Vec<TemporalReplayMismatchClass>,
}

pub const TEMPORAL_REPLAY_PARITY_SCHEMA_VERSION: &str = "worth-signal-temporal-replay-parity-v1";

impl Default for TemporalReconstructabilityArtifact {
    fn default() -> Self {
        Self::from_evidence(
            TemporalWakeSummary::default(),
            &TemporalTransactionEvidence::default(),
        )
    }
}

pub fn temporal_replay_parity_report(
    expected: &TemporalReconstructabilityArtifact,
    replayed: &TemporalReconstructabilityArtifact,
) -> TemporalReplayParityReport {
    let mut mismatch_classes = Vec::new();
    if expected.clock_checkpoint_digest != replayed.clock_checkpoint_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::ClockCheckpointDigestMismatch);
    }
    if expected.scheduled_wake_digest != replayed.scheduled_wake_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::ScheduledWakeDigestMismatch);
    }
    if expected.ready_wake_digest != replayed.ready_wake_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::ReadyWakeDigestMismatch);
    }
    if expected.retired_wake_digest != replayed.retired_wake_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::RetiredWakeDigestMismatch);
    }
    if expected.rescheduled_wake_digest != replayed.rescheduled_wake_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::RescheduledWakeDigestMismatch);
    }
    if expected.reused_wake_digest != replayed.reused_wake_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::ReusedWakeDigestMismatch);
    }
    if expected.interval_regeneration_digest != replayed.interval_regeneration_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::IntervalRegenerationDigestMismatch);
    }
    if expected.temporal_eligibility_digest != replayed.temporal_eligibility_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::TemporalEligibilityDigestMismatch);
    }
    if expected.previous_value_reference_digest != replayed.previous_value_reference_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::PreviousValueReferenceDigestMismatch);
    }
    if expected.certification_digest != replayed.certification_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::CertificationDigestMismatch);
    }
    if expected.wake_summary != replayed.wake_summary {
        mismatch_classes.push(TemporalReplayMismatchClass::WakeSummaryMismatch);
    }
    TemporalReplayParityReport {
        proof_schema_version: TEMPORAL_REPLAY_PARITY_SCHEMA_VERSION.to_owned(),
        expected: expected.clone(),
        replayed: replayed.clone(),
        parity: mismatch_classes.is_empty(),
        mismatch_classes,
    }
}

impl TemporalReconstructabilityArtifact {
    pub fn from_evidence(
        wake_summary: TemporalWakeSummary,
        evidence: &TemporalTransactionEvidence,
    ) -> Self {
        let clock_checkpoint_digest = canonical_digest(&evidence.clock_basis);
        let scheduled_wake_digest = canonical_digest(&evidence.scheduled_wakes);
        let ready_wake_digest = canonical_digest(&evidence.ready_wakes);
        let retired_wake_digest = canonical_digest(&evidence.retired_wakes);
        let rescheduled_wake_digest = canonical_digest(&evidence.rescheduled_wakes);
        let reused_wake_digest = canonical_digest(&evidence.reused_wakes);
        let interval_regeneration_digest = canonical_digest(&evidence.interval_regenerations);
        let temporal_eligibility_digest = canonical_digest(&evidence.eligibility_facts);
        let previous_value_reference_digest = canonical_digest(&evidence.previous_value_references);
        let certification_digest = canonical_digest(&TemporalCertificationDigestBasis {
            clock_checkpoint_digest: &clock_checkpoint_digest,
            scheduled_wake_digest: &scheduled_wake_digest,
            ready_wake_digest: &ready_wake_digest,
            retired_wake_digest: &retired_wake_digest,
            rescheduled_wake_digest: &rescheduled_wake_digest,
            reused_wake_digest: &reused_wake_digest,
            interval_regeneration_digest: &interval_regeneration_digest,
            temporal_eligibility_digest: &temporal_eligibility_digest,
            previous_value_reference_digest: &previous_value_reference_digest,
        });
        Self {
            clock_basis: evidence.clock_basis,
            wake_summary,
            eligibility_fact_count: evidence.eligibility_facts.len() as u64,
            scheduled_wake_count: evidence.scheduled_wakes.len() as u64,
            ready_wake_count: evidence.ready_wakes.len() as u64,
            retired_wake_count: evidence.retired_wakes.len() as u64,
            rescheduled_wake_count: evidence.rescheduled_wakes.len() as u64,
            reused_wake_count: evidence.reused_wakes.len() as u64,
            interval_regeneration_count: evidence.interval_regenerations.len() as u64,
            previous_value_reference_count: evidence.previous_value_references.len() as u64,
            clock_checkpoint_digest,
            scheduled_wake_digest,
            ready_wake_digest,
            retired_wake_digest,
            rescheduled_wake_digest,
            reused_wake_digest,
            interval_regeneration_digest,
            temporal_eligibility_digest,
            previous_value_reference_digest,
            certification_digest,
        }
    }

    pub(in crate::logic::transaction::runtime) fn from_temporal_state(
        temporal: &TemporalRuntimeState,
    ) -> Self {
        let evidence = TemporalTransactionEvidence {
            clock_basis: temporal.clock_basis(),
            eligibility_facts: Vec::new(),
            scheduled_wakes: temporal.scheduled_wake_evidence(),
            ready_wakes: temporal.ready_wake_evidence(),
            retired_wakes: temporal.retired_wake_evidence(),
            rescheduled_wakes: Vec::new(),
            reused_wakes: Vec::new(),
            interval_regenerations: Vec::new(),
            previous_value_references: Vec::new(),
        };
        Self::from_evidence(temporal.wake_summary(), &evidence)
    }
}

#[derive(Debug, Serialize)]
struct TemporalCertificationDigestBasis<'a> {
    clock_checkpoint_digest: &'a str,
    scheduled_wake_digest: &'a str,
    ready_wake_digest: &'a str,
    retired_wake_digest: &'a str,
    rescheduled_wake_digest: &'a str,
    reused_wake_digest: &'a str,
    interval_regeneration_digest: &'a str,
    temporal_eligibility_digest: &'a str,
    previous_value_reference_digest: &'a str,
}
