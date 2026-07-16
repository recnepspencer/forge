use crate::scenario::{
    certify_scenario_definition, BlobHarnessScenarioMetadata, NativeScenarioDefinitionParts,
    PhysicalSimulationScenarioDefinition,
};
use crate::{
    lower_physical_simulation_plan, AdmittedDriverContractSet, CertifiedPhysicalScenario,
    ForbiddenShortcutSet, OracleFamilyKind, PhysicalScenarioActor,
    PhysicalScenarioDefinitionDenial, PhysicalScenarioExpectation, PhysicalScenarioFault,
    PhysicalScenarioIntent, PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet,
    PhysicalSimulationPlan, PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    SimulationEvidencePolicy, SimulationPlanDenial, SimulationPlanningContext,
    SupportedObserverSet, SupportedOracleFamilySet, SupportedPhysicalDriverSet,
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
    let definition =
        PhysicalSimulationScenarioDefinition::from_native_parts(NativeScenarioDefinitionParts {
            label: "s7.blob-harness.seed".to_owned(),
            family: PhysicalSimulationScenarioFamily::BlobHarnessSeed,
            intent: PhysicalScenarioIntent::BlobHarnessSeed,
            fixtures: vec![blob_harness_seed_fixture(seed, materialized_profile)],
            actors: blob_seed_actors(seed),
            schedule: blob_seed_schedule(seed),
            fault: blob_seed_fault(seed),
            expectation: PhysicalScenarioExpectation::non_claiming_blob_harness_seed(
                seed.topology(),
                blob_seed_metadata(seed),
            ),
        })
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
            .with_supported_oracle_families(blob_supported_oracle_families())
            .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
            .with_forbidden_shortcuts(ForbiddenShortcutSet::physical_certification_baseline()),
    )
}

fn blob_seed_metadata(seed: &BlobHarnessScenarioSeed) -> BlobHarnessScenarioMetadata {
    BlobHarnessScenarioMetadata::new(
        seed.size_class(),
        seed.chunk_size_class(),
        seed.placement_class(),
        seed.security_scope(),
        seed.access_mode(),
        seed.failure_point(),
        seed.actor_mix(),
    )
}

fn blob_seed_actors(seed: &BlobHarnessScenarioSeed) -> Vec<PhysicalScenarioActor> {
    match seed.actor_mix() {
        worth_store_blob_chunks::BlobHarnessActorMix::SeedReplayOnly => {
            vec![PhysicalScenarioActor::recovery_driver("blob-seed-replay")]
        }
        worth_store_blob_chunks::BlobHarnessActorMix::IngestReadVerify => vec![
            PhysicalScenarioActor::blob_ingest_actor("blob-ingest"),
            PhysicalScenarioActor::blob_read_actor("blob-read"),
            PhysicalScenarioActor::blob_verify_actor("blob-verify"),
        ],
        worth_store_blob_chunks::BlobHarnessActorMix::ResumeRecovery => vec![
            PhysicalScenarioActor::blob_resume_actor("blob-resume"),
            PhysicalScenarioActor::recovery_driver("blob-recovery"),
        ],
        worth_store_blob_chunks::BlobHarnessActorMix::DedupeReclaim => vec![
            PhysicalScenarioActor::blob_dedupe_actor("blob-dedupe"),
            PhysicalScenarioActor::blob_reclaim_actor("blob-reclaim"),
        ],
        worth_store_blob_chunks::BlobHarnessActorMix::ExportImport => vec![
            PhysicalScenarioActor::blob_export_actor("blob-export"),
            PhysicalScenarioActor::blob_import_actor("blob-import"),
        ],
        worth_store_blob_chunks::BlobHarnessActorMix::PlacementMovePartialReplication => vec![
            PhysicalScenarioActor::blob_placement_move_actor("blob-placement-move"),
            PhysicalScenarioActor::blob_partial_replication_actor("blob-partial-replication"),
        ],
    }
}

fn blob_seed_schedule(seed: &BlobHarnessScenarioSeed) -> PhysicalScenarioSchedule {
    let yieldpoint = match seed.failure_point() {
        worth_store_blob_chunks::BlobHarnessFailurePoint::NoFaultSeed => "memory-pressure-boundary",
        worth_store_blob_chunks::BlobHarnessFailurePoint::AfterChunkWrite => {
            "wal-append-before-flush"
        }
        worth_store_blob_chunks::BlobHarnessFailurePoint::AfterSessionCheckpoint => {
            "fresh-runtime-replay-open"
        }
        worth_store_blob_chunks::BlobHarnessFailurePoint::AfterRootPublication => {
            "root-publication-before-observe"
        }
        worth_store_blob_chunks::BlobHarnessFailurePoint::DuringTierMove => "io-pressure-boundary",
        worth_store_blob_chunks::BlobHarnessFailurePoint::DuringExport => {
            "offline-verifier-layout-walk-before-runtime-recovery"
        }
        worth_store_blob_chunks::BlobHarnessFailurePoint::DuringReclaim => {
            "shortcut-rejection-boundary"
        }
    };
    PhysicalScenarioSchedule::named_boundary_yieldpoint(yieldpoint)
}

fn blob_seed_fault(seed: &BlobHarnessScenarioSeed) -> PhysicalScenarioFault {
    match seed.failure_point() {
        worth_store_blob_chunks::BlobHarnessFailurePoint::NoFaultSeed => {
            PhysicalScenarioFault::no_fault()
        }
        worth_store_blob_chunks::BlobHarnessFailurePoint::AfterChunkWrite => {
            PhysicalScenarioFault::blob_crash_after_chunk_write()
        }
        worth_store_blob_chunks::BlobHarnessFailurePoint::AfterSessionCheckpoint => {
            PhysicalScenarioFault::blob_crash_after_session_checkpoint()
        }
        worth_store_blob_chunks::BlobHarnessFailurePoint::AfterRootPublication => {
            PhysicalScenarioFault::blob_crash_after_root_publication()
        }
        worth_store_blob_chunks::BlobHarnessFailurePoint::DuringTierMove => {
            PhysicalScenarioFault::blob_tier_move_interruption()
        }
        worth_store_blob_chunks::BlobHarnessFailurePoint::DuringExport => {
            PhysicalScenarioFault::blob_export_interruption()
        }
        worth_store_blob_chunks::BlobHarnessFailurePoint::DuringReclaim => {
            PhysicalScenarioFault::blob_reclaim_interruption()
        }
    }
}

fn blob_supported_oracle_families() -> SupportedOracleFamilySet {
    SupportedOracleFamilySet::all_for_ci_certification().with_added([
        OracleFamilyKind::BlobHarnessEvidence,
        OracleFamilyKind::BlobHeavyQualification,
    ])
}
