use worth_foundational::facade::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, ExecutionObjectiveProfile, FoundationalProfileSet,
    FoundationalProfileSetInput, ObservationActivationProfile, RetentionDeliveryProfile,
    SupportPostureProfile,
};

pub(in crate::estate_capability_admission) fn publication_profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Standard,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::CompatibilityLowered,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
        execution_objective: ExecutionObjectiveProfile::Balanced,
        observation_activation: ObservationActivationProfile::Continuous,
    })
    .unwrap()
}
