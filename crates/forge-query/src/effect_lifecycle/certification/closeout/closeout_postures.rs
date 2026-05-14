use crate::effect_lifecycle::EffectLifecycleCounters;
use crate::identity::hash_parts;

use super::closeout_artifacts::{
    EffectExecutionCertificationLane, EffectExecutionCertificationRow,
};
use super::closeout_audits::EffectExecutionCloseoutAudits;
use super::{
    EffectLifecyclePhase4CertificationBundle, EffectLifecyclePhase4LaneOutcome,
    EffectLifecycleSeededCertificationBundle, EffectLifecycleSeededOutcomeClass,
};

pub(super) struct CloseoutPostureRows {
    pub(super) advisory: EffectExecutionCertificationRow,
    pub(super) deferred: EffectExecutionCertificationRow,
    pub(super) denied: EffectExecutionCertificationRow,
    pub(super) mismatch: EffectExecutionCertificationRow,
}

impl CloseoutPostureRows {
    pub(super) fn failure_digest(&self) -> String {
        hash_parts(&[
            self.deferred.failure_digest().unwrap_or("none").to_string(),
            self.denied.failure_digest().unwrap_or("none").to_string(),
            self.mismatch.failure_digest().unwrap_or("none").to_string(),
        ])
    }
}

pub(super) fn build_closeout_posture_rows(
    seeded: &EffectLifecycleSeededCertificationBundle,
    phase4: &EffectLifecyclePhase4CertificationBundle,
    audits: &EffectExecutionCloseoutAudits,
) -> CloseoutPostureRows {
    let advisory_seeded = seeded
        .rows()
        .iter()
        .filter(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::Advisory)
        .collect::<Vec<_>>();
    let deferred_seeded = seeded
        .rows()
        .iter()
        .filter(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::Deferred)
        .collect::<Vec<_>>();
    let denied_seeded = seeded
        .rows()
        .iter()
        .filter(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::Denied)
        .collect::<Vec<_>>();
    let deferred_phase4 = phase4
        .rows()
        .iter()
        .filter(|row| row.outcome() == EffectLifecyclePhase4LaneOutcome::Deferred)
        .collect::<Vec<_>>();
    let denied_phase4 = phase4
        .rows()
        .iter()
        .filter(|row| row.outcome() == EffectLifecyclePhase4LaneOutcome::Denied)
        .collect::<Vec<_>>();

    CloseoutPostureRows {
        advisory: advisory_row(&advisory_seeded),
        deferred: deferred_row(&deferred_seeded, &deferred_phase4),
        denied: denied_row(&denied_seeded, &denied_phase4),
        mismatch: mismatch_row(seeded, phase4, audits),
    }
}

fn advisory_row(
    seeded_rows: &[&super::EffectLifecycleSeededCertificationRow],
) -> EffectExecutionCertificationRow {
    let evidence_digest = hash_parts(
        &seeded_rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    let detail = seeded_rows
        .iter()
        .map(|row| format!("{}:{}", row.scenario_name(), row.effect_family().as_str()))
        .collect::<Vec<_>>()
        .join("|");
    EffectExecutionCertificationRow::new(
        EffectExecutionCertificationLane::AdvisorySurface,
        evidence_digest,
        detail,
        &combine_seeded_counters(seeded_rows),
        None,
    )
}

fn deferred_row(
    seeded_rows: &[&super::EffectLifecycleSeededCertificationRow],
    phase4_rows: &[&super::EffectLifecyclePhase4CertificationRow],
) -> EffectExecutionCertificationRow {
    let failure_digest = hash_parts(
        &seeded_rows
            .iter()
            .filter_map(|row| row.failure_digest().map(ToString::to_string))
            .chain(
                phase4_rows
                    .iter()
                    .map(|row| row.evidence_digest().to_string()),
            )
            .collect::<Vec<_>>(),
    );
    let evidence_digest = hash_parts(
        &seeded_rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .chain(phase4_rows.iter().map(|row| row.row_digest().to_string()))
            .collect::<Vec<_>>(),
    );
    let detail = seeded_rows
        .iter()
        .map(|row| format!("seeded:{}", row.scenario_name()))
        .chain(
            phase4_rows
                .iter()
                .map(|row| format!("phase4:{}", row.lane_kind().as_str())),
        )
        .collect::<Vec<_>>()
        .join("|");
    EffectExecutionCertificationRow::new(
        EffectExecutionCertificationLane::DeferredSurface,
        evidence_digest,
        detail,
        &combine_seeded_counters(seeded_rows).combine(&combine_phase4_counters(phase4_rows)),
        Some(failure_digest),
    )
}

fn denied_row(
    seeded_rows: &[&super::EffectLifecycleSeededCertificationRow],
    phase4_rows: &[&super::EffectLifecyclePhase4CertificationRow],
) -> EffectExecutionCertificationRow {
    let failure_digest = hash_parts(
        &seeded_rows
            .iter()
            .filter_map(|row| row.failure_digest().map(ToString::to_string))
            .chain(
                phase4_rows
                    .iter()
                    .map(|row| row.evidence_digest().to_string()),
            )
            .collect::<Vec<_>>(),
    );
    let evidence_digest = hash_parts(
        &seeded_rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .chain(phase4_rows.iter().map(|row| row.row_digest().to_string()))
            .collect::<Vec<_>>(),
    );
    let detail = seeded_rows
        .iter()
        .map(|row| format!("seeded:{}", row.scenario_name()))
        .chain(
            phase4_rows
                .iter()
                .map(|row| format!("phase4:{}", row.lane_kind().as_str())),
        )
        .collect::<Vec<_>>()
        .join("|");
    EffectExecutionCertificationRow::new(
        EffectExecutionCertificationLane::DeniedSurface,
        evidence_digest,
        detail,
        &combine_seeded_counters(seeded_rows).combine(&combine_phase4_counters(phase4_rows)),
        Some(failure_digest),
    )
}

fn mismatch_row(
    seeded: &EffectLifecycleSeededCertificationBundle,
    phase4: &EffectLifecyclePhase4CertificationBundle,
    audits: &EffectExecutionCloseoutAudits,
) -> EffectExecutionCertificationRow {
    let evidence_digest = hash_parts(&[
        format!("seeded_bundle:{}", seeded.certification_bundle_digest()),
        format!("phase4_bundle:{}", phase4.phase4_bundle_digest()),
        format!("proof_shape:{}", audits.proof_shape_digest()),
        format!("phase_progression:{}", audits.phase_progression_digest()),
        format!(
            "compile_fail_boundary:{}",
            audits.compile_fail_boundary_digest()
        ),
        format!("replay_parity:{}", seeded.seed_replay_digest()),
    ]);
    let failure_digest = hash_parts(&[
        audits.proof_shape_digest().to_string(),
        audits.phase_progression_digest().to_string(),
        audits.compile_fail_boundary_digest().to_string(),
        seeded.seed_replay_digest().to_string(),
    ]);
    EffectExecutionCertificationRow::new(
        EffectExecutionCertificationLane::MismatchDetectionSurface,
        evidence_digest,
        "proof_shape|phase_progression|compile_fail_boundary|replay_parity".to_string(),
        &EffectLifecycleCounters::default(),
        Some(failure_digest),
    )
}

fn combine_seeded_counters(
    rows: &[&super::EffectLifecycleSeededCertificationRow],
) -> EffectLifecycleCounters {
    rows.iter()
        .fold(EffectLifecycleCounters::default(), |acc, row| {
            acc.combine(row.counters())
        })
}

fn combine_phase4_counters(
    rows: &[&super::EffectLifecyclePhase4CertificationRow],
) -> EffectLifecycleCounters {
    rows.iter()
        .fold(EffectLifecycleCounters::default(), |acc, row| {
            acc.combine(row.counters())
        })
}
