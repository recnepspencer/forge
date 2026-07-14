use worth_foundational::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalProfileSet, FoundationalProfileSetInput,
    RetentionDeliveryProfile, SupportPostureProfile,
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

impl S6FoundationalProfileEvidence {
    pub(crate) fn from_sources(sources: &S6CertificationEvidenceSources) -> Self {
        Self {
            profile_set: FoundationalProfileSet::new(FoundationalProfileSetInput {
                diagnostic_richness: DiagnosticRichnessProfile::OperationalMinimal,
                support_posture: SupportPostureProfile::SupportReady,
                compatibility_posture: CompatibilityPostureProfile::NativeOnly,
                admission_readiness: AdmissionReadinessProfile::Admitted,
                retention_delivery: RetentionDeliveryProfile::Retained,
                certification_posture: CertificationPostureProfile::EvidenceBacked,
            })
            .expect("S6 materialization profile is retained evidence-backed support"),
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
