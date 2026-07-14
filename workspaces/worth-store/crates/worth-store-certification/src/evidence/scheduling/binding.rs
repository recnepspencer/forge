use worth_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};
use worth_store_security::StoreSecurityScopeIdentity;

use crate::{
    IoPressureHarnessCloseoutEvidence, S6AccessPolicyEvidenceRow,
    S6BackendCapabilityAdmissionCertificationEvidence, S6BackendQualificationMatrixCertification,
    S6BackgroundPacingCertificationEvidence, S6CertificationMaterializationDenial,
    S6CertifiedQueueExecutionEvidence, S6FlushDurabilityEvidenceRow,
    S6ForegroundReservationCertificationEvidence, S6PostAdmissionViolationEvidenceRow,
};

use super::binding_identity::{evidence_class_tag, mix, post_admission_violation_tag, profile_tag};
use super::S6MaterializedCounterStrength;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum S6StoreEvidenceLane {
    BackendAdmission,
    ForegroundReservation,
    BackgroundPacing,
    QueueExecution,
    FlushDurability,
    SecurityScopePreservation,
    AccessPolicy,
    PostAdmissionViolation,
    HarnessReplay,
    QualificationMatrix,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct S6StoreCounterStrengthWitness {
    foreground_reservation: S6MaterializedCounterStrength,
    background_pacing: S6MaterializedCounterStrength,
    queue_execution: S6MaterializedCounterStrength,
    post_admission_violation: S6MaterializedCounterStrength,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct S6StoreExecutionEvidenceBinding {
    execution_identity_tag: u64,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    security_scope_identity: StoreSecurityScopeIdentity,
    required_lane_mask: u16,
    readmission_boundaries: usize,
    counter_strengths: S6StoreCounterStrengthWitness,
}

impl S6StoreCounterStrengthWitness {
    fn from_source_counters(
        post_admission_violations: &[S6PostAdmissionViolationEvidenceRow],
    ) -> Self {
        Self {
            foreground_reservation: S6MaterializedCounterStrength::CertificationOnly,
            background_pacing: S6MaterializedCounterStrength::CertificationOnly,
            queue_execution: S6MaterializedCounterStrength::CertificationOnly,
            post_admission_violation: weakest_post_admission_strength(post_admission_violations),
        }
    }

    pub(crate) const fn foreground_reservation(self) -> S6MaterializedCounterStrength {
        self.foreground_reservation
    }

    pub(crate) const fn background_pacing(self) -> S6MaterializedCounterStrength {
        self.background_pacing
    }

    pub(crate) const fn queue_execution(self) -> S6MaterializedCounterStrength {
        self.queue_execution
    }

    pub(crate) const fn post_admission_violation(self) -> S6MaterializedCounterStrength {
        self.post_admission_violation
    }
}

impl S6StoreExecutionEvidenceBinding {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_materialized_lanes(
        backend_admission: &S6BackendCapabilityAdmissionCertificationEvidence,
        foreground_reservation: &S6ForegroundReservationCertificationEvidence,
        background_pacing: &S6BackgroundPacingCertificationEvidence,
        queue_execution: &S6CertifiedQueueExecutionEvidence,
        secure_io_preservation: worth_store_io_scheduler::SecureIoPreservationReceipt,
        access_policy_rows: &[S6AccessPolicyEvidenceRow],
        post_admission_violations: &[S6PostAdmissionViolationEvidenceRow],
        flush_durability: &[S6FlushDurabilityEvidenceRow],
        harness_closeout: &IoPressureHarnessCloseoutEvidence,
        qualification_matrix: &S6BackendQualificationMatrixCertification,
    ) -> Result<Self, S6CertificationMaterializationDenial> {
        let required_lane_mask = observed_lane_mask(
            foreground_reservation,
            background_pacing,
            queue_execution,
            secure_io_preservation,
            access_policy_rows,
            post_admission_violations,
            flush_durability,
            harness_closeout,
            qualification_matrix,
        );
        if required_lane_mask != all_required_lane_mask() {
            return Err(
                S6CertificationMaterializationDenial::StoreEvidenceReadmissionBindingMismatch,
            );
        }
        Ok(Self {
            execution_identity_tag: execution_identity_tag(
                backend_admission,
                foreground_reservation,
                background_pacing,
                secure_io_preservation,
                access_policy_rows,
                post_admission_violations,
                flush_durability,
                qualification_matrix,
                queue_execution,
                required_lane_mask,
            ),
            backend_profile: backend_admission.profile(),
            backend_evidence_class: backend_admission.evidence_class(),
            security_scope_identity: secure_io_preservation.identity(),
            required_lane_mask,
            readmission_boundaries: 0,
            counter_strengths: S6StoreCounterStrengthWitness::from_source_counters(
                post_admission_violations,
            ),
        })
    }

    pub(crate) const fn execution_identity_tag(self) -> u64 {
        self.execution_identity_tag
    }

    pub(crate) const fn backend_profile_tag(self) -> u64 {
        profile_tag(self.backend_profile)
    }

    pub(crate) const fn backend_evidence_class_tag(self) -> u64 {
        evidence_class_tag(self.backend_evidence_class)
    }

    pub(crate) const fn required_lane_mask(self) -> u16 {
        self.required_lane_mask
    }

    pub(crate) const fn counter_strengths(self) -> S6StoreCounterStrengthWitness {
        self.counter_strengths
    }
}

fn observed_lane_mask(
    foreground_reservation: &S6ForegroundReservationCertificationEvidence,
    background_pacing: &S6BackgroundPacingCertificationEvidence,
    queue_execution: &S6CertifiedQueueExecutionEvidence,
    secure_io_preservation: worth_store_io_scheduler::SecureIoPreservationReceipt,
    access_policy_rows: &[S6AccessPolicyEvidenceRow],
    post_admission_violations: &[S6PostAdmissionViolationEvidenceRow],
    flush_durability: &[S6FlushDurabilityEvidenceRow],
    harness_closeout: &IoPressureHarnessCloseoutEvidence,
    qualification_matrix: &S6BackendQualificationMatrixCertification,
) -> u16 {
    lane_bit(S6StoreEvidenceLane::BackendAdmission)
        | present_lane(
            foreground_counter_rows(foreground_reservation) > 0,
            S6StoreEvidenceLane::ForegroundReservation,
        )
        | present_lane(
            background_counter_rows(background_pacing) > 0,
            S6StoreEvidenceLane::BackgroundPacing,
        )
        | present_lane(
            queue_execution.counters().submitted_units() > 0,
            S6StoreEvidenceLane::QueueExecution,
        )
        | present_lane(
            secure_io_preservation.counters().scope_checks() > 0,
            S6StoreEvidenceLane::SecurityScopePreservation,
        )
        | present_lane(
            !flush_durability.is_empty(),
            S6StoreEvidenceLane::FlushDurability,
        )
        | present_lane(
            !access_policy_rows.is_empty(),
            S6StoreEvidenceLane::AccessPolicy,
        )
        | present_lane(
            !post_admission_violations.is_empty(),
            S6StoreEvidenceLane::PostAdmissionViolation,
        )
        | present_lane(
            !harness_closeout
                .harness_evidence()
                .executed_replay_coverage_rows()
                .rows()
                .is_empty(),
            S6StoreEvidenceLane::HarnessReplay,
        )
        | present_lane(
            qualification_matrix.row_count() > 0,
            S6StoreEvidenceLane::QualificationMatrix,
        )
}

#[allow(clippy::too_many_arguments)]
fn execution_identity_tag(
    backend_admission: &S6BackendCapabilityAdmissionCertificationEvidence,
    foreground_reservation: &S6ForegroundReservationCertificationEvidence,
    background_pacing: &S6BackgroundPacingCertificationEvidence,
    secure_io_preservation: worth_store_io_scheduler::SecureIoPreservationReceipt,
    access_policy_rows: &[S6AccessPolicyEvidenceRow],
    post_admission_violations: &[S6PostAdmissionViolationEvidenceRow],
    flush_durability: &[S6FlushDurabilityEvidenceRow],
    qualification_matrix: &S6BackendQualificationMatrixCertification,
    queue_execution: &S6CertifiedQueueExecutionEvidence,
    lane_mask: u16,
) -> u64 {
    let mut tag = 17_u64;
    tag = mix(tag, profile_tag(backend_admission.profile()));
    tag = mix(tag, evidence_class_tag(backend_admission.evidence_class()));
    tag = mix(tag, u64::from(lane_mask));
    tag = mix(tag, foreground_counter_rows(foreground_reservation) as u64);
    tag = mix(tag, background_counter_rows(background_pacing) as u64);
    tag = mix(
        tag,
        foreground_reservation
            .counters()
            .admitted_budget()
            .queue_slots(),
    );
    tag = mix(tag, background_pacing.counters().violation_events());
    tag = mix(tag, queue_execution.counters().submitted_units());
    tag = mix(tag, queue_execution.counters().violation_events());
    tag = mix(tag, flush_durability.len() as u64);
    tag = mix(tag, qualification_matrix.row_count() as u64);
    tag = mix(tag, access_policy_rows.len() as u64);
    for row in post_admission_violations {
        tag = mix(tag, post_admission_violation_tag(*row));
        tag = mix(tag, row.observed_violations());
    }
    mix(tag, secure_io_preservation.counters().scope_checks())
}

const fn all_required_lane_mask() -> u16 {
    lane_bit(S6StoreEvidenceLane::BackendAdmission)
        | lane_bit(S6StoreEvidenceLane::ForegroundReservation)
        | lane_bit(S6StoreEvidenceLane::BackgroundPacing)
        | lane_bit(S6StoreEvidenceLane::QueueExecution)
        | lane_bit(S6StoreEvidenceLane::FlushDurability)
        | lane_bit(S6StoreEvidenceLane::SecurityScopePreservation)
        | lane_bit(S6StoreEvidenceLane::AccessPolicy)
        | lane_bit(S6StoreEvidenceLane::PostAdmissionViolation)
        | lane_bit(S6StoreEvidenceLane::HarnessReplay)
        | lane_bit(S6StoreEvidenceLane::QualificationMatrix)
}

fn foreground_counter_rows(evidence: &S6ForegroundReservationCertificationEvidence) -> usize {
    [
        evidence.counters().admitted_budget().queue_slots(),
        evidence.counters().admitted_budget().bandwidth_tokens(),
        evidence.counters().admitted_budget().worker_permits(),
        evidence.counters().denied_capacity_events(),
        evidence.counters().stable_read_wait_count(),
        evidence.counters().stable_read_retry_count(),
    ]
    .into_iter()
    .filter(|value| *value > 0)
    .count()
}

fn background_counter_rows(evidence: &S6BackgroundPacingCertificationEvidence) -> usize {
    [
        evidence.counters().yield_events(),
        evidence.counters().deferred_events(),
        evidence.counters().denied_events(),
        evidence.counters().revoke_events(),
        evidence.counters().throttle_events(),
        evidence.counters().admitted_with_debt_events(),
        evidence.counters().violation_events(),
        evidence.counters().foreground_pressure_events(),
    ]
    .into_iter()
    .filter(|value| *value > 0)
    .count()
}

fn weakest_post_admission_strength(
    rows: &[S6PostAdmissionViolationEvidenceRow],
) -> S6MaterializedCounterStrength {
    rows.iter()
        .fold(S6MaterializedCounterStrength::Exact, |strength, row| {
            weakest_strength(strength, row.counter_strength())
        })
}

fn weakest_strength(
    first: S6MaterializedCounterStrength,
    second: S6MaterializedCounterStrength,
) -> S6MaterializedCounterStrength {
    use S6MaterializedCounterStrength::{
        Bounded, CertificationOnly, Derived, Exact, Sampled, Unavailable,
    };
    match (first, second) {
        (Unavailable, _) | (_, Unavailable) => Unavailable,
        (CertificationOnly, _) | (_, CertificationOnly) => CertificationOnly,
        (Sampled, _) | (_, Sampled) => Sampled,
        (Derived, _) | (_, Derived) => Derived,
        (Bounded, _) | (_, Bounded) => Bounded,
        (Exact, Exact) => Exact,
    }
}

const fn present_lane(present: bool, lane: S6StoreEvidenceLane) -> u16 {
    if present {
        lane_bit(lane)
    } else {
        0
    }
}

const fn lane_bit(lane: S6StoreEvidenceLane) -> u16 {
    match lane {
        S6StoreEvidenceLane::BackendAdmission => 1 << 0,
        S6StoreEvidenceLane::ForegroundReservation => 1 << 1,
        S6StoreEvidenceLane::BackgroundPacing => 1 << 2,
        S6StoreEvidenceLane::QueueExecution => 1 << 3,
        S6StoreEvidenceLane::FlushDurability => 1 << 4,
        S6StoreEvidenceLane::SecurityScopePreservation => 1 << 5,
        S6StoreEvidenceLane::AccessPolicy => 1 << 6,
        S6StoreEvidenceLane::PostAdmissionViolation => 1 << 7,
        S6StoreEvidenceLane::HarnessReplay => 1 << 8,
        S6StoreEvidenceLane::QualificationMatrix => 1 << 9,
    }
}
