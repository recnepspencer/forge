use forge_foundational::{
    CertificationPostureProfile, FoundationalProfileSetInput, SupportPostureProfile,
};

fn requires_certification(_: CertificationPostureProfile) {}

fn main() {
    requires_certification(SupportPostureProfile::CertificationReady);

    let _ = FoundationalProfileSetInput {
        diagnostic_richness: forge_foundational::DiagnosticRichnessProfile::Standard,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: forge_foundational::CompatibilityPostureProfile::NativeOnly,
        admission_readiness: forge_foundational::AdmissionReadinessProfile::Admitted,
        retention_delivery: forge_foundational::RetentionDeliveryProfile::Retained,
        certification_posture: SupportPostureProfile::CertificationReady,
    };
}
