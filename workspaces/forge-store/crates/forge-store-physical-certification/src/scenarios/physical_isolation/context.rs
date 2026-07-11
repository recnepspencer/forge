use crate::{
    ForbiddenShortcutSet, PhysicalIsolationCertificationLaneRegistration,
    PhysicalSimulationCapabilitySet, PhysicalSimulationProfile, PhysicalSimulationProfileSet,
    SimulationEvidencePolicy, SimulationPlanningContext, SupportedObserverSet,
    SupportedOracleFamilySet, SupportedPhysicalDriverSet,
};
use forge_store_physical_isolation::CompactionMutationLaneOrigin;

pub fn physical_isolation_planning_context(
    registration: PhysicalIsolationCertificationLaneRegistration,
    compaction_mutation_origin: CompactionMutationLaneOrigin,
) -> SimulationPlanningContext {
    physical_isolation_context_without_lane_registration(compaction_mutation_origin)
        .with_physical_isolation_lane_registration(registration)
}

pub fn physical_isolation_ci_certification_planning_context(
    registration: PhysicalIsolationCertificationLaneRegistration,
    compaction_mutation_origin: CompactionMutationLaneOrigin,
) -> SimulationPlanningContext {
    physical_isolation_ci_certification_context_without_lane_registration(
        compaction_mutation_origin,
    )
    .with_physical_isolation_lane_registration(registration)
}

pub fn physical_isolation_context_without_lane_registration(
    compaction_mutation_origin: CompactionMutationLaneOrigin,
) -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(PhysicalSimulationProfile::DeveloperSmoke)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(
            PhysicalSimulationCapabilitySet::physical_isolation_readiness_shape_probe(),
        )
        .with_driver_contracts(crate::AdmittedDriverContractSet::developer_smoke().unwrap())
        .with_supported_drivers(SupportedPhysicalDriverSet::all_for_developer_smoke())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::physical_certification_baseline())
        .with_physical_isolation_compaction_mutation_origin(compaction_mutation_origin)
}

pub fn physical_isolation_ci_certification_context_without_lane_registration(
    compaction_mutation_origin: CompactionMutationLaneOrigin,
) -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(PhysicalSimulationProfile::CiCertification)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(PhysicalSimulationCapabilitySet::physical_isolation_ci_certification())
        .with_driver_contracts(crate::AdmittedDriverContractSet::ci_certification().unwrap())
        .with_supported_drivers(SupportedPhysicalDriverSet::all_for_ci_certification())
        .with_supported_observers(SupportedObserverSet::all_for_ci_certification())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_ci_certification())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::physical_certification_baseline())
        .with_physical_isolation_compaction_mutation_origin(compaction_mutation_origin)
}
