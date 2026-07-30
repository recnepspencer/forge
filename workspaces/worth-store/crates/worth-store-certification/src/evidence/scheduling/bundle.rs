use super::{
    S6CanonicalEvidenceBasis, S6CertificationEvidenceSources, S6CertificationMaterializationDenial,
    S6CertificationProofTrace, S6CounterStrengthDeclaration, S6CounterStrengthFamily,
    S6FoundationalPerformanceReceipts, S6FoundationalProfileEvidence,
    S6MaterializedCounterStrength, S6PostAdmissionViolationEvidenceRow,
    StoreOwnedS6CertificationMaterializationSources,
};
use crate::{IoPressureHarnessCloseoutEvidence, S6AccessPolicyEvidenceRow};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct S6MaterializedCertificationEvidenceBundle {
    sources: S6CertificationEvidenceSources,
    profiles: S6FoundationalProfileEvidence,
    performance: S6FoundationalPerformanceReceipts,
    canonical: S6CanonicalEvidenceBasis,
    proof: S6CertificationProofTrace,
    counter_strengths: Vec<S6CounterStrengthDeclaration>,
}

pub fn materialize_io_qos_certification_evidence(
    sources: StoreOwnedS6CertificationMaterializationSources,
) -> Result<S6MaterializedCertificationEvidenceBundle, S6CertificationMaterializationDenial> {
    S6MaterializedCertificationEvidenceBundle::from_store_owned_sources(sources)
}

impl S6MaterializedCertificationEvidenceBundle {
    pub fn from_store_owned_sources(
        sources: StoreOwnedS6CertificationMaterializationSources,
    ) -> Result<Self, S6CertificationMaterializationDenial> {
        Self::from_sources(S6CertificationEvidenceSources::from_store_owned(sources)?)
    }

    pub(crate) fn from_sources(
        sources: S6CertificationEvidenceSources,
    ) -> Result<Self, S6CertificationMaterializationDenial> {
        if sources.flush_durability().is_empty() {
            return Err(S6CertificationMaterializationDenial::MissingFlushDurabilityEvidence);
        }
        if sources.qualification_matrix().row_count() == 0 {
            return Err(S6CertificationMaterializationDenial::EmptyQualificationMatrix);
        }
        if sources
            .harness_closeout()
            .harness_evidence()
            .executed_replay_coverage_rows()
            .rows()
            .is_empty()
        {
            return Err(S6CertificationMaterializationDenial::MissingHarnessReplayEvidence);
        }
        Ok(Self {
            profiles: S6FoundationalProfileEvidence::from_sources(&sources),
            performance: S6FoundationalPerformanceReceipts::from_sources(&sources)?,
            canonical: S6CanonicalEvidenceBasis::from_sources(&sources)?,
            proof: S6CertificationProofTrace::from_sources(&sources),
            counter_strengths: counter_strengths(&sources),
            sources,
        })
    }

    pub const fn profiles(&self) -> &S6FoundationalProfileEvidence {
        &self.profiles
    }

    pub const fn performance(&self) -> &S6FoundationalPerformanceReceipts {
        &self.performance
    }

    pub const fn canonical(&self) -> &S6CanonicalEvidenceBasis {
        &self.canonical
    }

    pub const fn proof(&self) -> &S6CertificationProofTrace {
        &self.proof
    }

    pub(crate) const fn qualification_matrix(
        &self,
    ) -> &crate::S6BackendQualificationMatrixCertification {
        self.sources.qualification_matrix()
    }

    pub fn counter_strengths(&self) -> &[S6CounterStrengthDeclaration] {
        &self.counter_strengths
    }

    pub fn access_policy_rows(&self) -> &[S6AccessPolicyEvidenceRow] {
        self.sources.access_policy_rows()
    }

    pub fn post_admission_violations(&self) -> &[S6PostAdmissionViolationEvidenceRow] {
        self.sources.post_admission_violations()
    }

    pub const fn harness_closeout(&self) -> &IoPressureHarnessCloseoutEvidence {
        self.sources.harness_closeout()
    }

    pub fn is_courtroom_evidence_over_executed_store_law(&self) -> bool {
        self.performance.has_required_counter_contracts()
            && self.proof.is_checked_from_executed_store_law()
            && self.profiles.authority_boundary()
                == super::S6FoundationalAuthorityBoundary::CertificationEvidenceOnly
    }
}

fn counter_strengths(
    sources: &S6CertificationEvidenceSources,
) -> Vec<S6CounterStrengthDeclaration> {
    let latency_rows = sources
        .latency_interference()
        .map(|evidence| evidence.rows().len())
        .unwrap_or_default();
    vec![
        S6CounterStrengthDeclaration::new(
            S6CounterStrengthFamily::ForegroundReservation,
            sources
                .binding()
                .counter_strengths()
                .foreground_reservation(),
            nonzero_counter_rows(&[
                sources
                    .foreground_reservation()
                    .counters()
                    .requested()
                    .queue_slots(),
                sources
                    .foreground_reservation()
                    .counters()
                    .admitted_budget()
                    .queue_slots(),
                sources
                    .foreground_reservation()
                    .counters()
                    .denied_budget()
                    .queue_slots(),
                sources
                    .foreground_reservation()
                    .counters()
                    .denied_capacity_events(),
            ]),
        ),
        S6CounterStrengthDeclaration::new(
            S6CounterStrengthFamily::BackgroundPacing,
            sources.binding().counter_strengths().background_pacing(),
            nonzero_counter_rows(&[
                sources.background_pacing().counters().yield_events(),
                sources.background_pacing().counters().deferred_events(),
                sources.background_pacing().counters().denied_events(),
                sources.background_pacing().counters().revoke_events(),
                sources.background_pacing().counters().throttle_events(),
                sources
                    .background_pacing()
                    .counters()
                    .admitted_with_debt_events(),
                sources.background_pacing().counters().violation_events(),
                sources
                    .background_pacing()
                    .counters()
                    .foreground_pressure_events(),
            ]),
        ),
        S6CounterStrengthDeclaration::new(
            S6CounterStrengthFamily::QueueExecution,
            sources.binding().counter_strengths().queue_execution(),
            sources
                .queue_execution()
                .counter_backed_receipt()
                .counter_rows()
                .len(),
        ),
        S6CounterStrengthDeclaration::new(
            S6CounterStrengthFamily::FlushDurability,
            sources.flush_durability()[0].counters().strength().into(),
            sources.flush_durability().len(),
        ),
        S6CounterStrengthDeclaration::new(
            S6CounterStrengthFamily::LatencyInterference,
            if latency_rows == 0 {
                S6MaterializedCounterStrength::Unavailable
            } else {
                S6MaterializedCounterStrength::Derived
            },
            latency_rows,
        ),
        S6CounterStrengthDeclaration::new(
            S6CounterStrengthFamily::SecureIoPreservation,
            sources
                .secure_io_preservation()
                .counters()
                .strength()
                .into(),
            nonzero_counter_rows(&[
                sources.secure_io_preservation().counters().scope_checks(),
                sources
                    .secure_io_preservation()
                    .counters()
                    .backend_posture_checks(),
                sources.secure_io_preservation().counters().denied_checks(),
            ]),
        ),
        S6CounterStrengthDeclaration::new(
            S6CounterStrengthFamily::AccessPolicy,
            access_policy_strength(sources),
            sources.access_policy_rows().len(),
        ),
        S6CounterStrengthDeclaration::new(
            S6CounterStrengthFamily::PostAdmissionViolation,
            sources
                .binding()
                .counter_strengths()
                .post_admission_violation(),
            sources.post_admission_violations().len(),
        ),
        S6CounterStrengthDeclaration::new(
            S6CounterStrengthFamily::QualificationMatrix,
            S6MaterializedCounterStrength::CertificationOnly,
            sources.qualification_matrix().row_count(),
        ),
    ]
}

fn nonzero_counter_rows(values: &[u64]) -> usize {
    values.iter().filter(|value| **value > 0).count()
}

fn access_policy_strength(
    sources: &S6CertificationEvidenceSources,
) -> S6MaterializedCounterStrength {
    sources
        .access_policy_rows()
        .iter()
        .fold(S6MaterializedCounterStrength::Exact, |strongest, row| {
            weakest_strength(strongest, row.counters().strength().into())
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
