use worth_store_physical_backend::BackendCapabilitySupportPosture;
use worth_store_physical_certification::{
    BackendQualificationMatrixDenial, PublishedQualificationPosture,
    QualificationResidualDebtReason,
};
use worth_store_readiness::{
    S6LaterMilestoneDestination, S6MaterializedCertificationAdoptionDenial,
    S6ReadinessCertificationCounterEvidence, S6ReadinessCertificationCounterFamily,
    S6ReadinessCertificationCounterStrength, S6ReadinessCertificationProofSummary,
    S6ReadinessCertificationProofTopology, S6ReadinessResidualDebtEvidenceKind,
    S6ReadinessResidualDebtEvidenceRow,
};

use super::{
    S6CounterStrengthFamily, S6FoundationalAuthorityBoundary,
    S6MaterializedCertificationEvidenceBundle, S6MaterializedCounterStrength,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct S6CertificationEvidenceAdoptionReceipt {
    canonical_execution_identity_tag: u64,
    proof_execution_identity_tag: u64,
    canonical_lane_binding_mask: u16,
    proof_lane_binding_mask: u16,
    profile_count: usize,
    profile_boundary_certification_only: bool,
    performance_receipt_count: usize,
    counter_strengths: Vec<S6ReadinessCertificationCounterEvidence>,
    canonical_access_policy_rows: usize,
    canonical_post_admission_violation_rows: usize,
    proof: S6ReadinessCertificationProofSummary,
    proof_topology: S6ReadinessCertificationProofTopology,
    residual_debt_rows: Vec<S6ReadinessResidualDebtEvidenceRow>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6CertificationRuntimeAuthorityDenial {
    CertificationEvidenceCannotAdmitForeground,
    CertificationEvidenceCannotAdmitBackground,
    CertificationEvidenceCannotPublishLaterReadiness(S6LaterMilestoneDestination),
    CertificationEvidenceCannotStrengthenBackendCapability,
    CertificationEvidenceCannotSatisfyCloseout,
}

pub fn adopt_materialized_s6_certification_evidence_for_closeout(
    bundle: &S6MaterializedCertificationEvidenceBundle,
) -> Result<S6CertificationEvidenceAdoptionReceipt, S6CertificationRuntimeAuthorityDenial> {
    closeout_receipt_from_bundle(bundle).map_err(S6CertificationRuntimeAuthorityDenial::from)
}

pub const fn reject_materialized_s6_certification_as_runtime_authority(
    _bundle: &S6MaterializedCertificationEvidenceBundle,
) -> S6CertificationRuntimeAuthorityDenial {
    S6CertificationRuntimeAuthorityDenial::CertificationEvidenceCannotStrengthenBackendCapability
}

impl From<S6MaterializedCertificationAdoptionDenial> for S6CertificationRuntimeAuthorityDenial {
    fn from(denial: S6MaterializedCertificationAdoptionDenial) -> Self {
        match denial {
            S6MaterializedCertificationAdoptionDenial::CertificationEvidenceCannotStrengthenRuntimeAuthority => {
                Self::CertificationEvidenceCannotStrengthenBackendCapability
            }
            S6MaterializedCertificationAdoptionDenial::CertificationEvidenceCannotSatisfyCloseout => {
                Self::CertificationEvidenceCannotSatisfyCloseout
            }
        }
    }
}

pub fn closeout_receipt_from_bundle(
    bundle: &S6MaterializedCertificationEvidenceBundle,
) -> Result<S6CertificationEvidenceAdoptionReceipt, S6MaterializedCertificationAdoptionDenial> {
    let proof = bundle.proof().projection().payload();
    S6CertificationEvidenceAdoptionReceipt::from_materialized_bundle_evidence(
        bundle.canonical().execution_identity_tag(),
        proof.execution_identity_tag(),
        bundle.canonical().lane_binding_mask(),
        proof.lane_binding_mask(),
        6,
        bundle.profiles().authority_boundary()
            == S6FoundationalAuthorityBoundary::CertificationEvidenceOnly,
        5,
        readiness_counter_strengths(bundle),
        bundle.canonical().access_policy_rows(),
        bundle.canonical().post_admission_violation_rows(),
        S6ReadinessCertificationProofSummary::new(
            proof.checked_execution(),
            proof.readmission_boundaries(),
            proof.access_policy_rows(),
            proof.post_admission_violation_rows(),
        ),
        proof.readiness_proof_topology(),
        residual_debt_rows_from_qualification(bundle),
    )
}

impl S6CertificationEvidenceAdoptionReceipt {
    #[allow(clippy::too_many_arguments)]
    fn from_materialized_bundle_evidence(
        canonical_execution_identity_tag: u64,
        proof_execution_identity_tag: u64,
        canonical_lane_binding_mask: u16,
        proof_lane_binding_mask: u16,
        profile_count: usize,
        profile_boundary_certification_only: bool,
        performance_receipt_count: usize,
        counter_strengths: Vec<S6ReadinessCertificationCounterEvidence>,
        canonical_access_policy_rows: usize,
        canonical_post_admission_violation_rows: usize,
        proof: S6ReadinessCertificationProofSummary,
        proof_topology: S6ReadinessCertificationProofTopology,
        residual_debt_rows: Vec<S6ReadinessResidualDebtEvidenceRow>,
    ) -> Result<Self, S6MaterializedCertificationAdoptionDenial> {
        let receipt = Self {
            canonical_execution_identity_tag,
            proof_execution_identity_tag,
            canonical_lane_binding_mask,
            proof_lane_binding_mask,
            profile_count,
            profile_boundary_certification_only,
            performance_receipt_count,
            counter_strengths,
            canonical_access_policy_rows,
            canonical_post_admission_violation_rows,
            proof,
            proof_topology,
            residual_debt_rows,
        };
        reject_unbound_closeout_evidence(&receipt)?;
        Ok(receipt)
    }

    pub const fn canonical_execution_identity_tag(&self) -> u64 {
        self.canonical_execution_identity_tag
    }

    pub const fn proof_execution_identity_tag(&self) -> u64 {
        self.proof_execution_identity_tag
    }

    pub const fn canonical_lane_binding_mask(&self) -> u16 {
        self.canonical_lane_binding_mask
    }

    pub const fn proof_lane_binding_mask(&self) -> u16 {
        self.proof_lane_binding_mask
    }

    pub const fn profile_count(&self) -> usize {
        self.profile_count
    }

    pub const fn profile_boundary_certification_only(&self) -> bool {
        self.profile_boundary_certification_only
    }

    pub const fn performance_receipt_count(&self) -> usize {
        self.performance_receipt_count
    }

    pub fn counter_strengths(&self) -> &[S6ReadinessCertificationCounterEvidence] {
        &self.counter_strengths
    }

    pub const fn canonical_access_policy_rows(&self) -> usize {
        self.canonical_access_policy_rows
    }

    pub const fn canonical_post_admission_violation_rows(&self) -> usize {
        self.canonical_post_admission_violation_rows
    }

    pub const fn proof(&self) -> S6ReadinessCertificationProofSummary {
        self.proof
    }

    pub const fn proof_topology(&self) -> S6ReadinessCertificationProofTopology {
        self.proof_topology
    }

    pub fn residual_debt_rows(&self) -> &[S6ReadinessResidualDebtEvidenceRow] {
        &self.residual_debt_rows
    }
}

fn reject_unbound_closeout_evidence(
    evidence: &S6CertificationEvidenceAdoptionReceipt,
) -> Result<(), S6MaterializedCertificationAdoptionDenial> {
    let proof = evidence.proof();
    let identity = evidence.canonical_execution_identity_tag();
    let lane_mask = evidence.canonical_lane_binding_mask();
    if evidence.counter_strengths().is_empty()
        || identity == 0
        || identity != evidence.proof_execution_identity_tag()
        || lane_mask != evidence.proof_lane_binding_mask()
        || lane_mask.count_ones() != 11
        || evidence.profile_count() != 6
        || !evidence.profile_boundary_certification_only()
        || evidence.performance_receipt_count() != 5
        || evidence.canonical_access_policy_rows() == 0
        || evidence.canonical_post_admission_violation_rows() == 0
        || proof.access_policy_rows() != evidence.canonical_access_policy_rows()
        || proof.post_admission_violation_rows()
            != evidence.canonical_post_admission_violation_rows()
        || proof.readmission_boundaries() != 5
        || !proof.checked_execution()
        || !evidence.proof_topology().is_checked_for_closeout(proof)
        || !has_required_residual_debt(evidence.residual_debt_rows())
    {
        return Err(
            S6MaterializedCertificationAdoptionDenial::CertificationEvidenceCannotSatisfyCloseout,
        );
    }
    Ok(())
}

fn readiness_counter_strengths(
    bundle: &S6MaterializedCertificationEvidenceBundle,
) -> Vec<S6ReadinessCertificationCounterEvidence> {
    bundle
        .counter_strengths()
        .iter()
        .map(|row| {
            S6ReadinessCertificationCounterEvidence::new(
                row.family().into(),
                row.strength().into(),
                row.observed_rows(),
            )
        })
        .collect()
}

impl From<S6CounterStrengthFamily> for S6ReadinessCertificationCounterFamily {
    fn from(family: S6CounterStrengthFamily) -> Self {
        match family {
            S6CounterStrengthFamily::ForegroundReservation => Self::ForegroundReservation,
            S6CounterStrengthFamily::BackgroundPacing => Self::BackgroundPacing,
            S6CounterStrengthFamily::QueueExecution => Self::QueueExecution,
            S6CounterStrengthFamily::FlushDurability => Self::FlushDurability,
            S6CounterStrengthFamily::LatencyInterference => Self::LatencyInterference,
            S6CounterStrengthFamily::LaterReadinessHandoff => Self::LaterReadinessHandoff,
            S6CounterStrengthFamily::SecureIoPreservation => Self::SecureIoPreservation,
            S6CounterStrengthFamily::AccessPolicy => Self::AccessPolicy,
            S6CounterStrengthFamily::PostAdmissionViolation => Self::PostAdmissionViolation,
            S6CounterStrengthFamily::QualificationMatrix => Self::QualificationMatrix,
        }
    }
}

impl From<S6MaterializedCounterStrength> for S6ReadinessCertificationCounterStrength {
    fn from(strength: S6MaterializedCounterStrength) -> Self {
        match strength {
            S6MaterializedCounterStrength::Exact => Self::Exact,
            S6MaterializedCounterStrength::Bounded => Self::Bounded,
            S6MaterializedCounterStrength::Sampled => Self::Sampled,
            S6MaterializedCounterStrength::Derived => Self::Derived,
            S6MaterializedCounterStrength::CertificationOnly => Self::CertificationOnly,
            S6MaterializedCounterStrength::Unavailable => Self::Unavailable,
        }
    }
}

fn residual_debt_rows_from_qualification(
    bundle: &S6MaterializedCertificationEvidenceBundle,
) -> Vec<S6ReadinessResidualDebtEvidenceRow> {
    use S6ReadinessResidualDebtEvidenceKind::{
        DegradedBackendPosture, DeniedClaim, RebindRequired, ResidualQualificationDebt,
        StaleEvidence, UnavailableEvidence, UnsupportedBackendProfile,
    };
    let required = [
        UnsupportedBackendProfile,
        UnavailableEvidence,
        DegradedBackendPosture,
        DeniedClaim,
        StaleEvidence,
        RebindRequired,
        ResidualQualificationDebt,
    ];
    let row_kinds: Vec<Vec<S6ReadinessResidualDebtEvidenceKind>> = bundle
        .qualification_matrix()
        .row_outcomes()
        .into_iter()
        .map(|outcome| {
            debt_kinds_for_qualification_row(outcome.row(), outcome.certified_support().err())
        })
        .collect();

    required
        .into_iter()
        .filter_map(|kind| {
            let observed = row_kinds
                .iter()
                .filter(|kinds| kinds.iter().any(|candidate| *candidate == kind))
                .count();
            (observed > 0).then(|| S6ReadinessResidualDebtEvidenceRow::new(kind, observed))
        })
        .collect()
}

fn debt_kinds_for_qualification_row(
    row: worth_store_physical_certification::BackendQualificationRow,
    certified_support: Option<BackendQualificationMatrixDenial>,
) -> Vec<S6ReadinessResidualDebtEvidenceKind> {
    use S6ReadinessResidualDebtEvidenceKind::{
        DegradedBackendPosture, DeniedClaim, RebindRequired, ResidualQualificationDebt,
        StaleEvidence, UnavailableEvidence, UnsupportedBackendProfile,
    };
    let mut kinds = Vec::new();
    match row.support_posture() {
        BackendCapabilitySupportPosture::Unsupported | BackendCapabilitySupportPosture::Unknown => {
            kinds.push(UnsupportedBackendProfile);
        }
        BackendCapabilitySupportPosture::Unavailable => kinds.push(UnavailableEvidence),
        BackendCapabilitySupportPosture::Stale => kinds.push(StaleEvidence),
        BackendCapabilitySupportPosture::RebindRequired => kinds.push(RebindRequired),
        BackendCapabilitySupportPosture::Supported => {}
    }
    match row.published_posture() {
        PublishedQualificationPosture::Degraded => kinds.push(DegradedBackendPosture),
        PublishedQualificationPosture::Unsupported | PublishedQualificationPosture::Unknown => {
            kinds.push(UnsupportedBackendProfile);
        }
        PublishedQualificationPosture::Unavailable => kinds.push(UnavailableEvidence),
        PublishedQualificationPosture::Stale => kinds.push(StaleEvidence),
        PublishedQualificationPosture::RebindRequired => kinds.push(RebindRequired),
        PublishedQualificationPosture::Supported => {}
    }
    match row.residual_debt().reason() {
        QualificationResidualDebtReason::MissingEvidence => {
            kinds.push(UnavailableEvidence);
            kinds.push(ResidualQualificationDebt);
        }
        QualificationResidualDebtReason::BackendSpecificDenial => {
            kinds.push(DeniedClaim);
            kinds.push(ResidualQualificationDebt);
        }
        QualificationResidualDebtReason::DegradedOperation => {
            kinds.push(DegradedBackendPosture);
            kinds.push(ResidualQualificationDebt);
        }
        QualificationResidualDebtReason::StaleEvidence => {
            kinds.push(StaleEvidence);
            kinds.push(ResidualQualificationDebt);
        }
        QualificationResidualDebtReason::None => {}
    }
    if certified_support.is_some() {
        kinds.push(DeniedClaim);
    }
    kinds.sort_by_key(|kind| *kind as u8);
    kinds.dedup();
    kinds
}

fn has_required_residual_debt(rows: &[S6ReadinessResidualDebtEvidenceRow]) -> bool {
    use S6ReadinessResidualDebtEvidenceKind::{
        DegradedBackendPosture, DeniedClaim, RebindRequired, ResidualQualificationDebt,
        StaleEvidence, UnavailableEvidence, UnsupportedBackendProfile,
    };
    [
        UnsupportedBackendProfile,
        UnavailableEvidence,
        DegradedBackendPosture,
        DeniedClaim,
        StaleEvidence,
        RebindRequired,
        ResidualQualificationDebt,
    ]
    .into_iter()
    .all(|kind| {
        rows.iter()
            .any(|row| row.kind() == kind && row.observed_claims() > 0)
    })
}
