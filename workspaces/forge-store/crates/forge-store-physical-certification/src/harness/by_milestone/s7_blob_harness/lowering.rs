use crate::scenario::{certify_scenario_definition, PhysicalSimulationScenarioDefinition};
use crate::{
    lower_physical_simulation_plan, AdmittedDriverContractSet, CertifiedPhysicalScenario,
    ForbiddenShortcutSet, PhysicalScenarioActor, PhysicalScenarioDefinitionDenial,
    PhysicalScenarioExpectation, PhysicalScenarioFault, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet, PhysicalSimulationPlan,
    PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily, SimulationEvidencePolicy,
    SimulationPlanDenial, SimulationPlanningContext, SupportedObserverSet,
    SupportedOracleFamilySet, SupportedPhysicalDriverSet,
};

use super::fixture::blob_harness_seed_fixture;
use super::foundational_profile::BlobHarnessMaterializedProfile;
use super::scenario_seed::BlobHarnessScenarioSeed;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobHarnessLoweredSeedPlan {
    scenario: CertifiedPhysicalScenario,
    plan: PhysicalSimulationPlan,
    materialized_profile: BlobHarnessMaterializedProfile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobHarnessLoweringDenial {
    ScenarioDefinition(PhysicalScenarioDefinitionDenial),
    DriverAdmission(crate::DriverAdmissionDenial),
    MissingYieldpointBinding,
    SimulationPlan(SimulationPlanDenial),
}

pub fn lower_blob_simulation_seed_plan(
    seed: BlobHarnessScenarioSeed,
) -> Result<BlobHarnessLoweredSeedPlan, BlobHarnessLoweringDenial> {
    let materialized_profile = seed.profile().materialize_foundational_profile();
    let scenario = certify_blob_seed_scenario(&seed, &materialized_profile)?;
    let plan =
        lower_physical_simulation_plan(scenario.clone(), blob_harness_planning_context(&seed)?)
            .map_err(BlobHarnessLoweringDenial::SimulationPlan)?;
    Ok(BlobHarnessLoweredSeedPlan {
        scenario,
        plan,
        materialized_profile,
    })
}

impl BlobHarnessLoweredSeedPlan {
    pub const fn scenario(&self) -> &CertifiedPhysicalScenario {
        &self.scenario
    }

    pub const fn plan(&self) -> &PhysicalSimulationPlan {
        &self.plan
    }

    pub const fn materialized_profile(&self) -> &BlobHarnessMaterializedProfile {
        &self.materialized_profile
    }

    pub fn replay_identity(&self) -> &[u8; 32] {
        self.plan.identity().digest_bytes()
    }
}

fn certify_blob_seed_scenario(
    seed: &BlobHarnessScenarioSeed,
    materialized_profile: &BlobHarnessMaterializedProfile,
) -> Result<CertifiedPhysicalScenario, BlobHarnessLoweringDenial> {
    let definition = PhysicalSimulationScenarioDefinition::from_native_parts(
        "s7.blob-harness.seed".to_owned(),
        PhysicalSimulationScenarioFamily::S7BlobHarnessSeed,
        PhysicalScenarioIntent::S7BlobHarnessSeed,
        vec![blob_harness_seed_fixture(seed, materialized_profile)],
        vec![PhysicalScenarioActor::recovery_driver("blob-seed-replay")],
        PhysicalScenarioSchedule::named_boundary_yieldpoint("memory-pressure-boundary"),
        PhysicalScenarioFault::no_fault(),
        PhysicalScenarioExpectation::non_claiming_s7_blob_harness_seed(seed.topology()),
    )
    .map_err(BlobHarnessLoweringDenial::ScenarioDefinition)?;
    certify_scenario_definition(definition).map_err(BlobHarnessLoweringDenial::ScenarioDefinition)
}

fn blob_harness_planning_context(
    seed: &BlobHarnessScenarioSeed,
) -> Result<SimulationPlanningContext, BlobHarnessLoweringDenial> {
    let driver_contracts = AdmittedDriverContractSet::ci_certification()
        .map_err(BlobHarnessLoweringDenial::DriverAdmission)?;
    Ok(
        SimulationPlanningContext::for_profile(seed.profile().physical_profile())
            .with_supported_profiles(PhysicalSimulationProfileSet::all())
            .with_capabilities(PhysicalSimulationCapabilitySet::all_for_developer_smoke())
            .with_driver_contracts(driver_contracts)
            .with_supported_drivers(SupportedPhysicalDriverSet::all_for_ci_certification())
            .with_supported_observers(SupportedObserverSet::all_for_ci_certification())
            .with_supported_oracle_families(SupportedOracleFamilySet::all_for_ci_certification())
            .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
            .with_forbidden_shortcuts(ForbiddenShortcutSet::roadmap2_baseline()),
    )
}
