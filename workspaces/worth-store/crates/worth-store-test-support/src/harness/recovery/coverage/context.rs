use crate::admitted_developer_smoke_driver_contracts;
use worth_store_physical_certification::{
    ForbiddenShortcutSet, PhysicalSimulationCapabilitySet, PhysicalSimulationProfile,
    PhysicalSimulationProfileSet, SimulationEvidencePolicy, SimulationPlanningContext,
    SupportedObserverSet, SupportedOracleFamilySet, SupportedPhysicalDriverSet,
};

pub(super) fn complete_context_for_profile(
    profile: PhysicalSimulationProfile,
) -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(profile)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(
            PhysicalSimulationCapabilitySet::physical_isolation_readiness_shape_probe(),
        )
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_drivers(SupportedPhysicalDriverSet::all_for_developer_smoke())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::physical_certification_baseline())
}
