use worth_store_io_scheduler::{
    foreground_reservation::ForegroundReservationReceipt,
    queue_execution::QueueExecutionViolationCause, BackgroundDebtKind, BackgroundPacingOutcome,
    QueueExecutionOutcome, SecureIoPreservationReceipt,
};
use worth_store_physical_backend::{AccessPolicyViolationKind, AdmittedBackendCapabilityWitness};

use crate::{
    certify_s6_backend_capability_admission, certify_s6_background_pacing,
    certify_s6_foreground_reservation, S6AccessPolicyEvidenceOutcomeKind,
    S6AccessPolicyEvidenceRow, S6BackendCapabilityAdmissionCertificationEvidence,
    S6BackendQualificationMatrixCertification, S6BackgroundPacingCertificationEvidence,
    S6CertificationMaterializationDenial, S6CertifiedQueueExecutionEvidence,
    S6FlushDurabilityEvidenceRow, S6ForegroundReservationCertificationEvidence,
    S6IoPressureHarnessCloseoutEvidence, S6LatencyInterferenceEvidence,
    S6LaterReadinessHandoffCertification, S6MaterializedCounterStrength,
};

use super::binding::S6StoreExecutionEvidenceBinding;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6PostAdmissionViolationFamily {
    QueueExecution,
    BackgroundPacing,
    AccessPolicy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6PostAdmissionViolationCause {
    QueueExecution(QueueExecutionViolationCause),
    BackgroundPacing(BackgroundDebtKind),
    AccessPolicy(AccessPolicyViolationKind),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6PostAdmissionViolationEvidenceRow {
    family: S6PostAdmissionViolationFamily,
    cause: S6PostAdmissionViolationCause,
    observed_violations: u64,
    counter_strength: S6MaterializedCounterStrength,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoreOwnedS6CertificationMaterializationSources {
    backend_witness: AdmittedBackendCapabilityWitness,
    foreground_receipt: ForegroundReservationReceipt,
    background_outcome: BackgroundPacingOutcome,
    queue_outcome: QueueExecutionOutcome,
    secure_io_preservation: SecureIoPreservationReceipt,
    access_policy_rows: Vec<S6AccessPolicyEvidenceRow>,
    post_admission_violations: Vec<S6PostAdmissionViolationEvidenceRow>,
    flush_durability: Vec<S6FlushDurabilityEvidenceRow>,
    harness_closeout: S6IoPressureHarnessCloseoutEvidence,
    qualification_matrix: S6BackendQualificationMatrixCertification,
    later_handoffs: S6LaterReadinessHandoffCertification,
    latency_interference: Option<S6LatencyInterferenceEvidence>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct S6CertificationEvidenceSources {
    binding: S6StoreExecutionEvidenceBinding,
    backend_admission: S6BackendCapabilityAdmissionCertificationEvidence,
    foreground_reservation: S6ForegroundReservationCertificationEvidence,
    background_pacing: S6BackgroundPacingCertificationEvidence,
    queue_execution: S6CertifiedQueueExecutionEvidence,
    secure_io_preservation: SecureIoPreservationReceipt,
    access_policy_rows: Vec<S6AccessPolicyEvidenceRow>,
    post_admission_violations: Vec<S6PostAdmissionViolationEvidenceRow>,
    flush_durability: Vec<S6FlushDurabilityEvidenceRow>,
    harness_closeout: S6IoPressureHarnessCloseoutEvidence,
    qualification_matrix: S6BackendQualificationMatrixCertification,
    later_handoffs: S6LaterReadinessHandoffCertification,
    latency_interference: Option<S6LatencyInterferenceEvidence>,
}

impl S6PostAdmissionViolationEvidenceRow {
    pub fn from_queue_execution_outcome(outcome: &QueueExecutionOutcome) -> Option<Self> {
        let QueueExecutionOutcome::Violation(violation) = outcome else {
            return None;
        };
        Some(Self {
            family: S6PostAdmissionViolationFamily::QueueExecution,
            cause: S6PostAdmissionViolationCause::QueueExecution(violation.cause()),
            observed_violations: violation.counters().violation_events(),
            counter_strength: S6MaterializedCounterStrength::Derived,
        })
    }

    pub fn from_background_pacing_outcome(outcome: BackgroundPacingOutcome) -> Option<Self> {
        let BackgroundPacingOutcome::Violation(violation) = outcome else {
            return None;
        };
        Some(Self {
            family: S6PostAdmissionViolationFamily::BackgroundPacing,
            cause: S6PostAdmissionViolationCause::BackgroundPacing(violation.causal_debt().kind()),
            observed_violations: violation.counters().violation_events(),
            counter_strength: S6MaterializedCounterStrength::Derived,
        })
    }

    pub fn from_access_policy_row(row: S6AccessPolicyEvidenceRow) -> Option<Self> {
        let S6AccessPolicyEvidenceOutcomeKind::Violated(cause) = row.outcome() else {
            return None;
        };
        Some(Self {
            family: S6PostAdmissionViolationFamily::AccessPolicy,
            cause: S6PostAdmissionViolationCause::AccessPolicy(cause),
            observed_violations: row.counters().violations(),
            counter_strength: row.counters().strength().into(),
        })
    }

    pub const fn family(self) -> S6PostAdmissionViolationFamily {
        self.family
    }

    pub const fn cause(self) -> S6PostAdmissionViolationCause {
        self.cause
    }

    pub const fn observed_violations(self) -> u64 {
        self.observed_violations
    }

    pub const fn counter_strength(self) -> S6MaterializedCounterStrength {
        self.counter_strength
    }
}

impl StoreOwnedS6CertificationMaterializationSources {
    #[allow(clippy::too_many_arguments)]
    pub fn from_bound_store_execution(
        backend_witness: AdmittedBackendCapabilityWitness,
        foreground_receipt: ForegroundReservationReceipt,
        background_outcome: BackgroundPacingOutcome,
        queue_outcome: QueueExecutionOutcome,
        secure_io_preservation: SecureIoPreservationReceipt,
        access_policy_rows: Vec<S6AccessPolicyEvidenceRow>,
        flush_durability: Vec<S6FlushDurabilityEvidenceRow>,
        harness_closeout: S6IoPressureHarnessCloseoutEvidence,
        qualification_matrix: S6BackendQualificationMatrixCertification,
        later_handoffs: S6LaterReadinessHandoffCertification,
        latency_interference: Option<S6LatencyInterferenceEvidence>,
    ) -> Result<Self, S6CertificationMaterializationDenial> {
        reject_backend_mismatch(&backend_witness, foreground_receipt, secure_io_preservation)?;
        reject_access_policy_mismatch(&backend_witness, foreground_receipt, &access_policy_rows)?;
        reject_later_handoff_mismatch(&backend_witness, &later_handoffs)?;
        let post_admission_violations = derive_post_admission_violations(
            &queue_outcome,
            background_outcome,
            &access_policy_rows,
        );
        Ok(Self {
            backend_witness,
            foreground_receipt,
            background_outcome,
            queue_outcome,
            secure_io_preservation,
            access_policy_rows,
            post_admission_violations,
            flush_durability,
            harness_closeout,
            qualification_matrix,
            later_handoffs,
            latency_interference,
        })
    }
}

fn derive_post_admission_violations(
    queue_outcome: &QueueExecutionOutcome,
    background_outcome: BackgroundPacingOutcome,
    access_policy_rows: &[S6AccessPolicyEvidenceRow],
) -> Vec<S6PostAdmissionViolationEvidenceRow> {
    let mut rows = Vec::new();
    if let Some(row) =
        S6PostAdmissionViolationEvidenceRow::from_queue_execution_outcome(queue_outcome)
    {
        rows.push(row);
    }
    if let Some(row) =
        S6PostAdmissionViolationEvidenceRow::from_background_pacing_outcome(background_outcome)
    {
        rows.push(row);
    }
    rows.extend(
        access_policy_rows
            .iter()
            .copied()
            .filter_map(S6PostAdmissionViolationEvidenceRow::from_access_policy_row),
    );
    rows
}

impl S6CertificationEvidenceSources {
    pub(crate) fn from_store_owned(
        sources: StoreOwnedS6CertificationMaterializationSources,
    ) -> Result<Self, S6CertificationMaterializationDenial> {
        if sources.access_policy_rows.is_empty() {
            return Err(S6CertificationMaterializationDenial::MissingAccessPolicyEvidence);
        }
        if sources.post_admission_violations.is_empty() {
            return Err(
                S6CertificationMaterializationDenial::MissingPostAdmissionViolationEvidence,
            );
        }
        if sources
            .post_admission_violations
            .iter()
            .any(|row| row.observed_violations() == 0)
        {
            return Err(
                S6CertificationMaterializationDenial::MissingPostAdmissionViolationEvidence,
            );
        }
        if sources
            .access_policy_rows
            .iter()
            .all(|row| !matches!(row.outcome(), S6AccessPolicyEvidenceOutcomeKind::Executed))
        {
            return Err(S6CertificationMaterializationDenial::MissingAccessPolicyEvidence);
        }
        if sources.secure_io_preservation.counters().scope_checks() == 0 {
            return Err(S6CertificationMaterializationDenial::MissingSecureIoPreservationEvidence);
        }
        if sources.flush_durability.is_empty() {
            return Err(S6CertificationMaterializationDenial::MissingFlushDurabilityEvidence);
        }
        if sources.qualification_matrix.row_count() == 0 {
            return Err(S6CertificationMaterializationDenial::EmptyQualificationMatrix);
        }
        if sources
            .harness_closeout
            .harness_evidence()
            .executed_replay_coverage_rows()
            .rows()
            .is_empty()
        {
            return Err(S6CertificationMaterializationDenial::MissingHarnessReplayEvidence);
        }
        let readiness = crate::publish_s6_backend_capability_readiness(&sources.backend_witness);
        let backend_admission =
            certify_s6_backend_capability_admission(&sources.backend_witness, &readiness)
                .ok_or(S6CertificationMaterializationDenial::BackendAdmissionReadinessMismatch)?;
        let foreground_reservation = certify_s6_foreground_reservation(
            sources.foreground_receipt,
            sources.foreground_receipt,
        )?;
        let background_pacing =
            certify_s6_background_pacing(sources.background_outcome, sources.background_outcome)?;
        let queue_execution =
            S6CertifiedQueueExecutionEvidence::from_outcome(&sources.queue_outcome)?;
        Ok(Self {
            binding: S6StoreExecutionEvidenceBinding::from_materialized_lanes(
                &backend_admission,
                &foreground_reservation,
                &background_pacing,
                &queue_execution,
                sources.secure_io_preservation,
                &sources.access_policy_rows,
                &sources.post_admission_violations,
                &sources.flush_durability,
                &sources.harness_closeout,
                &sources.qualification_matrix,
                &sources.later_handoffs,
            )?,
            backend_admission,
            foreground_reservation,
            background_pacing,
            queue_execution,
            secure_io_preservation: sources.secure_io_preservation,
            access_policy_rows: sources.access_policy_rows,
            post_admission_violations: sources.post_admission_violations,
            flush_durability: sources.flush_durability,
            harness_closeout: sources.harness_closeout,
            qualification_matrix: sources.qualification_matrix,
            later_handoffs: sources.later_handoffs,
            latency_interference: sources.latency_interference,
        })
    }

    pub const fn binding(&self) -> S6StoreExecutionEvidenceBinding {
        self.binding
    }

    pub const fn backend_admission(&self) -> &S6BackendCapabilityAdmissionCertificationEvidence {
        &self.backend_admission
    }

    pub const fn foreground_reservation(&self) -> &S6ForegroundReservationCertificationEvidence {
        &self.foreground_reservation
    }

    pub const fn background_pacing(&self) -> &S6BackgroundPacingCertificationEvidence {
        &self.background_pacing
    }

    pub const fn queue_execution(&self) -> &S6CertifiedQueueExecutionEvidence {
        &self.queue_execution
    }

    pub const fn secure_io_preservation(&self) -> SecureIoPreservationReceipt {
        self.secure_io_preservation
    }

    pub fn access_policy_rows(&self) -> &[S6AccessPolicyEvidenceRow] {
        &self.access_policy_rows
    }

    pub fn post_admission_violations(&self) -> &[S6PostAdmissionViolationEvidenceRow] {
        &self.post_admission_violations
    }

    pub fn flush_durability(&self) -> &[S6FlushDurabilityEvidenceRow] {
        &self.flush_durability
    }

    pub const fn harness_closeout(&self) -> &S6IoPressureHarnessCloseoutEvidence {
        &self.harness_closeout
    }

    pub const fn qualification_matrix(&self) -> &S6BackendQualificationMatrixCertification {
        &self.qualification_matrix
    }

    pub const fn later_handoffs(&self) -> &S6LaterReadinessHandoffCertification {
        &self.later_handoffs
    }

    pub const fn latency_interference(&self) -> Option<&S6LatencyInterferenceEvidence> {
        self.latency_interference.as_ref()
    }
}

fn reject_backend_mismatch(
    backend_witness: &AdmittedBackendCapabilityWitness,
    foreground_receipt: ForegroundReservationReceipt,
    secure_io_preservation: SecureIoPreservationReceipt,
) -> Result<(), S6CertificationMaterializationDenial> {
    if foreground_receipt.backend_profile() != backend_witness.profile()
        || foreground_receipt.backend_evidence_class() != backend_witness.evidence_class()
        || secure_io_preservation.backend_profile() != backend_witness.profile()
        || secure_io_preservation.backend_evidence_class() != backend_witness.evidence_class()
    {
        return Err(S6CertificationMaterializationDenial::StoreEvidenceBackendBindingMismatch);
    }
    if secure_io_preservation.identity() != foreground_receipt.security_scope_identity() {
        return Err(
            S6CertificationMaterializationDenial::StoreEvidenceSecurityScopeBindingMismatch,
        );
    }
    Ok(())
}

fn reject_access_policy_mismatch(
    backend_witness: &AdmittedBackendCapabilityWitness,
    foreground_receipt: ForegroundReservationReceipt,
    rows: &[S6AccessPolicyEvidenceRow],
) -> Result<(), S6CertificationMaterializationDenial> {
    for row in rows {
        if let Some(profile) = row.profile() {
            if profile != backend_witness.profile() {
                return Err(
                    S6CertificationMaterializationDenial::StoreEvidenceBackendBindingMismatch,
                );
            }
        }
        if let Some(evidence_class) = row.evidence_class() {
            if evidence_class != backend_witness.evidence_class() {
                return Err(
                    S6CertificationMaterializationDenial::StoreEvidenceBackendBindingMismatch,
                );
            }
        }
        if let Some(scope) = row.security_scope() {
            if scope != foreground_receipt.security_scope_identity() {
                return Err(
                    S6CertificationMaterializationDenial::StoreEvidenceSecurityScopeBindingMismatch,
                );
            }
        }
    }
    Ok(())
}

fn reject_later_handoff_mismatch(
    backend_witness: &AdmittedBackendCapabilityWitness,
    handoffs: &S6LaterReadinessHandoffCertification,
) -> Result<(), S6CertificationMaterializationDenial> {
    let operator = handoffs.operator();
    if operator.backend_profile() != backend_witness.profile()
        || operator.backend_evidence_class() != backend_witness.evidence_class()
    {
        return Err(S6CertificationMaterializationDenial::StoreEvidenceBackendBindingMismatch);
    }
    Ok(())
}
