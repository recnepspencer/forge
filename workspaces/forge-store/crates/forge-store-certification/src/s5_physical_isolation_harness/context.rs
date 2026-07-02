use forge_store_physical_certification::{
    ForbiddenShortcutSet, PhysicalSimulationCapabilitySet, PhysicalSimulationProfile,
    PhysicalSimulationProfileSet, S5PhysicalIsolationCertificationLaneRegistration,
    SimulationEvidencePolicy, SimulationPlanningContext, SupportedObserverSet,
    SupportedOracleFamilySet, SupportedPhysicalDriverSet,
};
use forge_store_physical_isolation::CompactionMutationLaneOrigin;
use forge_store_test_support::{
    admitted_ci_certification_driver_contracts, admitted_developer_smoke_driver_contracts,
};

pub fn s5_physical_isolation_planning_context(
    registration: S5PhysicalIsolationCertificationLaneRegistration,
    compaction_mutation_origin: CompactionMutationLaneOrigin,
) -> SimulationPlanningContext {
    s5_physical_isolation_context_without_lane_registration(compaction_mutation_origin)
        .with_s5_physical_isolation_lane_registration(registration)
}

pub fn s5_physical_isolation_ci_certification_planning_context(
    registration: S5PhysicalIsolationCertificationLaneRegistration,
    compaction_mutation_origin: CompactionMutationLaneOrigin,
) -> SimulationPlanningContext {
    s5_physical_isolation_ci_certification_context_without_lane_registration(
        compaction_mutation_origin,
    )
    .with_s5_physical_isolation_lane_registration(registration)
}

pub fn s5_physical_isolation_context_without_lane_registration(
    compaction_mutation_origin: CompactionMutationLaneOrigin,
) -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(PhysicalSimulationProfile::DeveloperSmoke)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(PhysicalSimulationCapabilitySet::s5_readiness_shape_probe())
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_drivers(SupportedPhysicalDriverSet::all_for_developer_smoke())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::roadmap2_baseline())
        .with_s5_compaction_mutation_origin(compaction_mutation_origin)
}

pub fn s5_physical_isolation_ci_certification_context_without_lane_registration(
    compaction_mutation_origin: CompactionMutationLaneOrigin,
) -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(PhysicalSimulationProfile::CiCertification)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(PhysicalSimulationCapabilitySet::s5_ci_certification())
        .with_driver_contracts(admitted_ci_certification_driver_contracts().unwrap())
        .with_supported_drivers(SupportedPhysicalDriverSet::all_for_ci_certification())
        .with_supported_observers(SupportedObserverSet::all_for_ci_certification())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_ci_certification())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::roadmap2_baseline())
        .with_s5_compaction_mutation_origin(compaction_mutation_origin)
}
