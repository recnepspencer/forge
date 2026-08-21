use super::families::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, ExecutionObjectiveProfile, ObservationActivationProfile,
    RetentionDeliveryProfile, SupportPostureProfile,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalProfileSetInput {
    pub diagnostic_richness: DiagnosticRichnessProfile,
    pub support_posture: SupportPostureProfile,
    pub compatibility_posture: CompatibilityPostureProfile,
    pub admission_readiness: AdmissionReadinessProfile,
    pub retention_delivery: RetentionDeliveryProfile,
    pub certification_posture: CertificationPostureProfile,
    pub execution_objective: ExecutionObjectiveProfile,
    pub observation_activation: ObservationActivationProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileCompositionDenial {
    InternalSupportCannotClaimCertifiedPosture,
    EvidenceBackedRequiresAdmittedReadiness,
    EvidenceBackedRequiresRetainedDelivery,
    ProductionCertifiedRequiresCertificationReadySupport,
    ProductionCertifiedRequiresProductionGateReadiness,
    ProductionCertifiedRequiresRetainedDelivery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalProfileSet {
    diagnostic_richness: DiagnosticRichnessProfile,
    support_posture: SupportPostureProfile,
    compatibility_posture: CompatibilityPostureProfile,
    admission_readiness: AdmissionReadinessProfile,
    retention_delivery: RetentionDeliveryProfile,
    certification_posture: CertificationPostureProfile,
    execution_objective: ExecutionObjectiveProfile,
    observation_activation: ObservationActivationProfile,
}

impl FoundationalProfileSet {
    pub fn new(
        input: FoundationalProfileSetInput,
    ) -> Result<Self, FoundationalProfileCompositionDenial> {
        validate_profile_composition(input)?;

        Ok(Self {
            diagnostic_richness: input.diagnostic_richness,
            support_posture: input.support_posture,
            compatibility_posture: input.compatibility_posture,
            admission_readiness: input.admission_readiness,
            retention_delivery: input.retention_delivery,
            certification_posture: input.certification_posture,
            execution_objective: input.execution_objective,
            observation_activation: input.observation_activation,
        })
    }

    pub const fn diagnostic_richness(&self) -> DiagnosticRichnessProfile {
        self.diagnostic_richness
    }

    pub const fn support_posture(&self) -> SupportPostureProfile {
        self.support_posture
    }

    pub const fn compatibility_posture(&self) -> CompatibilityPostureProfile {
        self.compatibility_posture
    }

    pub const fn admission_readiness(&self) -> AdmissionReadinessProfile {
        self.admission_readiness
    }

    pub const fn retention_delivery(&self) -> RetentionDeliveryProfile {
        self.retention_delivery
    }

    pub const fn certification_posture(&self) -> CertificationPostureProfile {
        self.certification_posture
    }

    pub const fn execution_objective(&self) -> ExecutionObjectiveProfile {
        self.execution_objective
    }

    pub const fn observation_activation(&self) -> ObservationActivationProfile {
        self.observation_activation
    }
}

fn validate_profile_composition(
    input: FoundationalProfileSetInput,
) -> Result<(), FoundationalProfileCompositionDenial> {
    if support_is_internal_while_certification_claims_strength(input) {
        return Err(
            FoundationalProfileCompositionDenial::InternalSupportCannotClaimCertifiedPosture,
        );
    }

    if evidence_backed_lacks_admitted_readiness(input) {
        return Err(FoundationalProfileCompositionDenial::EvidenceBackedRequiresAdmittedReadiness);
    }

    if evidence_backed_lacks_retained_delivery(input) {
        return Err(FoundationalProfileCompositionDenial::EvidenceBackedRequiresRetainedDelivery);
    }

    if production_certified_lacks_certification_ready_support(input) {
        return Err(
            FoundationalProfileCompositionDenial::ProductionCertifiedRequiresCertificationReadySupport,
        );
    }

    if production_certified_lacks_production_gate_readiness(input) {
        return Err(
            FoundationalProfileCompositionDenial::ProductionCertifiedRequiresProductionGateReadiness,
        );
    }

    if production_certified_lacks_retained_delivery(input) {
        return Err(
            FoundationalProfileCompositionDenial::ProductionCertifiedRequiresRetainedDelivery,
        );
    }

    Ok(())
}

fn support_is_internal_while_certification_claims_strength(
    input: FoundationalProfileSetInput,
) -> bool {
    input.support_posture == SupportPostureProfile::InternalOnly
        && input.certification_posture != CertificationPostureProfile::Uncertified
}

fn evidence_backed_lacks_admitted_readiness(input: FoundationalProfileSetInput) -> bool {
    input.certification_posture == CertificationPostureProfile::EvidenceBacked
        && input.admission_readiness == AdmissionReadinessProfile::CandidateOnly
}

fn evidence_backed_lacks_retained_delivery(input: FoundationalProfileSetInput) -> bool {
    input.certification_posture == CertificationPostureProfile::EvidenceBacked
        && input.retention_delivery == RetentionDeliveryProfile::Ephemeral
}

fn production_certified_lacks_certification_ready_support(
    input: FoundationalProfileSetInput,
) -> bool {
    input.certification_posture == CertificationPostureProfile::ProductionCertified
        && input.support_posture != SupportPostureProfile::CertificationReady
}

fn production_certified_lacks_production_gate_readiness(
    input: FoundationalProfileSetInput,
) -> bool {
    input.certification_posture == CertificationPostureProfile::ProductionCertified
        && input.admission_readiness != AdmissionReadinessProfile::ProductionGateReady
}

fn production_certified_lacks_retained_delivery(input: FoundationalProfileSetInput) -> bool {
    input.certification_posture == CertificationPostureProfile::ProductionCertified
        && input.retention_delivery == RetentionDeliveryProfile::Ephemeral
}
