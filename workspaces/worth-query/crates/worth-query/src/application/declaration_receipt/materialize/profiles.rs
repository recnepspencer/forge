use worth_foundational::facade::{
    profiles, AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, MaterializedFoundationalProfileSet, RetentionDeliveryProfile,
    SupportPostureProfile,
};
use worth_proof::TransitionOutcome;

use crate::application::{
    materialized_profile_for_tier as orchestration_materialized_profile_for_tier,
    WorthQueryDeclarationEntryOrchestrationMaterializationTier,
};

pub(crate) fn default_receipt_materialized_profile() -> &'static MaterializedFoundationalProfileSet
{
    static ONCE: std::sync::OnceLock<MaterializedFoundationalProfileSet> =
        std::sync::OnceLock::new();
    ONCE.get_or_init(build_default_receipt_materialized_profile)
}

fn build_default_receipt_materialized_profile() -> MaterializedFoundationalProfileSet {
    let requested = profiles()
        .set()
        .diagnostic_richness(DiagnosticRichnessProfile::Standard)
        .support_posture(SupportPostureProfile::SupportReady)
        .compatibility_posture(CompatibilityPostureProfile::CompatibilityLowered)
        .admission_readiness(AdmissionReadinessProfile::Admitted)
        .retention_delivery(RetentionDeliveryProfile::Retained)
        .certification_posture(CertificationPostureProfile::Uncertified)
        .request()
        .expect("static receipt profile should compose");
    let admitted = match profiles().progression().admit_same(requested) {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("receipt profile admission should succeed: {outcome:?}"),
    };
    match profiles().progression().materialize_same(admitted) {
        TransitionOutcome::Success(value) => *value.payload(),
        outcome => panic!("receipt profile materialization should succeed: {outcome:?}"),
    }
}

pub(crate) fn receipt_materialized_profile_for_tier(
    tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
) -> MaterializedFoundationalProfileSet {
    orchestration_materialized_profile_for_tier(tier)
}
