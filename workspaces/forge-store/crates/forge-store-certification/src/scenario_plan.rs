use forge_store_contracts::StableArtifactId;
use forge_store_test_support::LargeStorePressureFixture;

use crate::scenario_plan_rules::{
    capability_tier_for_lane, cost_class_for_lane, counter_expectations_for_lane,
    default_oracles_for_lane, denial_boundary_for_lane, driver_requirements_for_lane,
    fixture_posture_for_lane, footprint_for_lane, forbidden_shortcuts_for_lane,
    observer_requirements_for_lane, runtime_relationship_for_definition,
    storage_crossings_for_steps,
};
use crate::{
    FixtureAdversaryPosture, LaneFamilyExtension, PhysicalProofOracleKind,
    PhysicalScenarioDefinition, PhysicalScenarioDriverRequirement,
    PhysicalScenarioObserverRequirement, PhysicalStoryStep, RoadmapLaneFamily,
    RuntimeVerifierRelationship, ScenarioCounterExpectation, ScenarioDenialBoundary, ScenarioLane,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScenarioPlanIdentity {
    scenario_name: StableArtifactId,
    lane: ScenarioLane,
    lane_family: RoadmapLaneFamily,
}

impl PhysicalScenarioPlanIdentity {
    pub(crate) const fn new(scenario_name: StableArtifactId, lane: ScenarioLane) -> Self {
        Self {
            scenario_name,
            lane,
            lane_family: lane.family(),
        }
    }

    pub const fn scenario_name(&self) -> &StableArtifactId {
        &self.scenario_name
    }

    pub const fn lane(&self) -> ScenarioLane {
        self.lane
    }

    pub const fn lane_family(&self) -> RoadmapLaneFamily {
        self.lane_family
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactPolicy {
    MinimalCertificationRecord,
    FullEvidenceTranscript,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadScale {
    DeveloperSmoke,
    CertificationMatrix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalScenarioCapabilityTier {
    PlatformGradePhysicalSubstrate,
    DeniedLegacyPlatformClaim,
    RoadmapFollowOnExtension,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalScenarioCostClass {
    BoundedPhysicalLocate,
    CertificationExtension,
    LegacyProbeOnly,
    LargeStoreMemoryPressure,
    ManifestBoundedVerifierParity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedPhysicalFootprint {
    SinglePageAuthority,
    HostileReferenceProbe,
    HostileFormatProbe,
    LegacyBackendClaimProbe,
    OfflineManifestRead,
    LocalityScaleSample,
    FoundationalEvidenceExport,
    LargeStorePressureFixture,
    RoadmapFamilyExtension(RoadmapLaneFamily),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageBoundaryCrossing {
    AppendPhysicalRecord,
    EvidenceExport,
    LegacyBackendProbe,
    OfflineVerifierRead,
    ReopenFromBytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScenarioPlan {
    identity: PhysicalScenarioPlanIdentity,
    physical_law: String,
    story_steps: Vec<PhysicalStoryStep>,
    driver_requirements: Vec<PhysicalScenarioDriverRequirement>,
    observer_requirements: Vec<PhysicalScenarioObserverRequirement>,
    required_oracles: Vec<PhysicalProofOracleKind>,
    expected_counters: Vec<ScenarioCounterExpectation>,
    expected_denial_boundary: Option<ScenarioDenialBoundary>,
    forbidden_shortcuts: Vec<ScenarioDenialBoundary>,
    resolved_capability: PhysicalScenarioCapabilityTier,
    cost_class: PhysicalScenarioCostClass,
    expected_physical_footprint: ExpectedPhysicalFootprint,
    runtime_verifier_relationship: RuntimeVerifierRelationship,
    fixture_adversary_posture: FixtureAdversaryPosture,
    artifact_policy: ArtifactPolicy,
    workload_scale: WorkloadScale,
    storage_boundary_crossings: Vec<StorageBoundaryCrossing>,
    large_store_pressure_fixture: Option<LargeStorePressureFixture>,
}

impl PhysicalScenarioPlan {
    pub(crate) fn from_definition(
        definition: PhysicalScenarioDefinition,
        extensions: &[LaneFamilyExtension],
    ) -> Result<Self, PhysicalScenarioPlanDenial> {
        let lane = definition.lane();
        let physical_lane = lane.physical_substrate_lane();
        let mut required_oracles = definition.required_oracles().to_vec();
        push_unique(
            &mut required_oracles,
            PhysicalProofOracleKind::ScenarioPlanOwnsStrategy,
        );
        push_unique(
            &mut required_oracles,
            PhysicalProofOracleKind::TranscriptPreservesEvidence,
        );
        for oracle in default_oracles_for_lane(lane, extensions) {
            push_unique(&mut required_oracles, oracle);
        }

        Ok(Self {
            identity: PhysicalScenarioPlanIdentity::new(definition.name().clone(), lane),
            physical_law: definition.physical_law().to_string(),
            story_steps: definition.steps().to_vec(),
            driver_requirements: driver_requirements_for_lane(lane, extensions)?,
            observer_requirements: observer_requirements_for_lane(lane, extensions)?,
            required_oracles: required_oracles.clone(),
            expected_counters: counter_expectations_for_lane(
                physical_lane,
                definition.large_store_pressure_fixture(),
            ),
            expected_denial_boundary: denial_boundary_for_lane(
                physical_lane,
                definition.large_store_pressure_fixture(),
            ),
            forbidden_shortcuts: forbidden_shortcuts_for_lane(
                physical_lane,
                definition.large_store_pressure_fixture(),
            ),
            resolved_capability: capability_tier_for_lane(lane),
            cost_class: cost_class_for_lane(lane, definition.large_store_pressure_fixture()),
            expected_physical_footprint: footprint_for_lane(
                lane,
                definition.large_store_pressure_fixture(),
            ),
            runtime_verifier_relationship: runtime_relationship_for_definition(
                lane,
                &required_oracles,
            ),
            fixture_adversary_posture: fixture_posture_for_lane(
                physical_lane,
                definition.large_store_pressure_fixture(),
            ),
            artifact_policy: ArtifactPolicy::FullEvidenceTranscript,
            workload_scale: WorkloadScale::CertificationMatrix,
            storage_boundary_crossings: storage_crossings_for_steps(definition.steps(), lane),
            large_store_pressure_fixture: definition.large_store_pressure_fixture(),
        })
    }

    pub const fn identity(&self) -> &PhysicalScenarioPlanIdentity {
        &self.identity
    }

    pub fn physical_law(&self) -> &str {
        &self.physical_law
    }

    pub fn story_steps(&self) -> &[PhysicalStoryStep] {
        &self.story_steps
    }

    pub fn driver_requirements(&self) -> &[PhysicalScenarioDriverRequirement] {
        &self.driver_requirements
    }

    pub fn observer_requirements(&self) -> &[PhysicalScenarioObserverRequirement] {
        &self.observer_requirements
    }

    pub fn required_oracles(&self) -> &[PhysicalProofOracleKind] {
        &self.required_oracles
    }

    pub fn expected_counters(&self) -> &[ScenarioCounterExpectation] {
        &self.expected_counters
    }

    pub const fn expected_denial_boundary(&self) -> Option<ScenarioDenialBoundary> {
        self.expected_denial_boundary
    }

    pub fn forbidden_shortcuts(&self) -> &[ScenarioDenialBoundary] {
        &self.forbidden_shortcuts
    }

    pub const fn resolved_capability(&self) -> PhysicalScenarioCapabilityTier {
        self.resolved_capability
    }

    pub const fn cost_class(&self) -> PhysicalScenarioCostClass {
        self.cost_class
    }

    pub const fn expected_physical_footprint(&self) -> ExpectedPhysicalFootprint {
        self.expected_physical_footprint
    }

    pub const fn runtime_verifier_relationship(&self) -> RuntimeVerifierRelationship {
        self.runtime_verifier_relationship
    }

    pub const fn fixture_adversary_posture(&self) -> FixtureAdversaryPosture {
        self.fixture_adversary_posture
    }

    pub const fn artifact_policy(&self) -> ArtifactPolicy {
        self.artifact_policy
    }

    pub const fn workload_scale(&self) -> WorkloadScale {
        self.workload_scale
    }

    pub fn storage_boundary_crossings(&self) -> &[StorageBoundaryCrossing] {
        &self.storage_boundary_crossings
    }

    pub const fn large_store_pressure_fixture(&self) -> Option<LargeStorePressureFixture> {
        self.large_store_pressure_fixture
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalScenarioPlanDenial {
    MissingExecutionDriver,
    UnregisteredLaneFamily,
}

fn push_unique<T: PartialEq>(items: &mut Vec<T>, item: T) {
    if !items.contains(&item) {
        items.push(item);
    }
}
