use worth_foundational::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, ExecutionObjectiveProfile, FoundationalProfileSet,
    FoundationalProfileSetInput, ObservationActivationProfile, RetentionDeliveryProfile,
    SupportPostureProfile,
};
use worth_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};

use super::S6CertificationEvidenceSources;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6FoundationalAuthorityBoundary {
    StoreRuntimeAuthority,
    CertificationEvidenceOnly,
    FoundationalSupportPostureOnly,
    ProofProgressionOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct S6FoundationalProfileEvidence {
    profile_set: FoundationalProfileSet,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    authority_boundary: S6FoundationalAuthorityBoundary,
}

pub(crate) fn operational_evidence_profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::OperationalMinimal,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
        execution_objective: ExecutionObjectiveProfile::Throughput,
        observation_activation: ObservationActivationProfile::OnDemand,
    })
    .expect("S6 materialization profile is retained evidence-backed support")
}

impl S6FoundationalProfileEvidence {
    pub(crate) fn from_sources(sources: &S6CertificationEvidenceSources) -> Self {
        Self {
            profile_set: operational_evidence_profile(),
            backend_profile: sources.backend_admission().profile(),
            backend_evidence_class: sources.backend_admission().evidence_class(),
            authority_boundary: S6FoundationalAuthorityBoundary::CertificationEvidenceOnly,
        }
    }

    pub const fn profile_set(&self) -> &FoundationalProfileSet {
        &self.profile_set
    }

    pub const fn backend_profile(&self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn backend_evidence_class(&self) -> CapabilityEvidenceClass {
        self.backend_evidence_class
    }

    pub const fn authority_boundary(&self) -> S6FoundationalAuthorityBoundary {
        self.authority_boundary
    }
}

#[cfg(test)]
mod operational_axes {
    use super::operational_evidence_profile;
    use worth_foundational::{
        DiagnosticRichnessProfile, ExecutionObjectiveProfile, ObservationActivationProfile,
        RetentionDeliveryProfile,
    };

    #[test]
    fn operational_evidence_profile_is_throughput_on_demand_without_changing_retention() {
        let profile = operational_evidence_profile();
        assert_eq!(
            profile.execution_objective(),
            ExecutionObjectiveProfile::Throughput
        );
        assert_eq!(
            profile.observation_activation(),
            ObservationActivationProfile::OnDemand
        );
        assert_eq!(
            profile.retention_delivery(),
            RetentionDeliveryProfile::Retained
        );
        assert_eq!(
            profile.diagnostic_richness(),
            DiagnosticRichnessProfile::OperationalMinimal
        );
    }
}
