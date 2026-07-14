use crate::{
    ExpectedPhysicalFootprint, FixtureAdversaryPosture, LaneFamilyExtension,
    PhysicalCounterExpectationKind, PhysicalProofOracleKind, PhysicalScenarioCapabilityTier,
    PhysicalScenarioCostClass, PhysicalScenarioDriverKind, PhysicalScenarioDriverRequirement,
    PhysicalScenarioObserverKind, PhysicalScenarioObserverRequirement, PhysicalScenarioPlanDenial,
    PhysicalStoryStep, PhysicalSubstrateLane, RoadmapLaneFamily, RuntimeVerifierRelationship,
    ScenarioCounterExpectation, ScenarioDenialBoundary, ScenarioLane, StorageBoundaryCrossing,
};
use worth_store_test_support::{LargeStorePressureClass, LargeStorePressureFixture};

pub(crate) fn driver_requirements_for_lane(
    lane: ScenarioLane,
    extensions: &[LaneFamilyExtension],
) -> Result<Vec<PhysicalScenarioDriverRequirement>, PhysicalScenarioPlanDenial> {
    use PhysicalScenarioDriverKind as Driver;
    let Some(physical_lane) = lane.physical_substrate_lane() else {
        if extensions.is_empty() {
            return Err(PhysicalScenarioPlanDenial::UnregisteredLaneFamily);
        }
        let mut drivers = Vec::new();
        for extension in extensions {
            push_unique(&mut drivers, extension.driver());
        }
        return Ok(drivers
            .into_iter()
            .map(PhysicalScenarioDriverRequirement::new)
            .collect());
    };
    let drivers = match physical_lane {
        PhysicalSubstrateLane::HappyAuthority => vec![
            Driver::PlatformBackendCandidate,
            Driver::PersistedFileDevice,
        ],
        PhysicalSubstrateLane::HostileReference | PhysicalSubstrateLane::HostileFormat => {
            vec![
                Driver::PlatformBackendCandidate,
                Driver::AdversarialByteDevice,
            ]
        }
        PhysicalSubstrateLane::LegacyOverclaim => vec![Driver::LegacyBackendProbe],
        PhysicalSubstrateLane::OfflineVerifier => {
            vec![Driver::PlatformBackendCandidate, Driver::VerifierOnlyReader]
        }
        PhysicalSubstrateLane::ScaleLocality
        | PhysicalSubstrateLane::FoundationalExport
        | PhysicalSubstrateLane::S2Handoff => vec![Driver::PlatformBackendCandidate],
    };
    Ok(drivers
        .into_iter()
        .map(PhysicalScenarioDriverRequirement::new)
        .collect())
}

pub(crate) fn observer_requirements_for_lane(
    lane: ScenarioLane,
    extensions: &[LaneFamilyExtension],
) -> Result<Vec<PhysicalScenarioObserverRequirement>, PhysicalScenarioPlanDenial> {
    use PhysicalScenarioObserverKind as Observer;
    let Some(physical_lane) = lane.physical_substrate_lane() else {
        if extensions.is_empty() {
            return Err(PhysicalScenarioPlanDenial::UnregisteredLaneFamily);
        }
        let mut observers = Vec::new();
        for extension in extensions {
            push_unique(&mut observers, extension.observer());
        }
        return Ok(observers
            .into_iter()
            .map(PhysicalScenarioObserverRequirement::new)
            .collect());
    };
    let mut observers = vec![
        Observer::CounterBundle,
        Observer::DenialBoundary,
        Observer::MaterializationShortcut,
        Observer::StorageBoundary,
    ];
    if matches!(
        physical_lane,
        PhysicalSubstrateLane::OfflineVerifier | PhysicalSubstrateLane::FoundationalExport
    ) {
        observers.push(Observer::RuntimeLayout);
        observers.push(Observer::OfflineVerifier);
        observers.push(Observer::EvidenceExport);
    }
    Ok(observers
        .into_iter()
        .map(PhysicalScenarioObserverRequirement::new)
        .collect())
}

pub(crate) fn default_oracles_for_lane(
    lane: ScenarioLane,
    extensions: &[LaneFamilyExtension],
) -> Vec<PhysicalProofOracleKind> {
    let Some(physical_lane) = lane.physical_substrate_lane() else {
        let mut oracles = Vec::new();
        for extension in extensions {
            push_unique(&mut oracles, extension.oracle());
        }
        return oracles;
    };
    match physical_lane {
        PhysicalSubstrateLane::LegacyOverclaim => {
            vec![PhysicalProofOracleKind::ForbiddenLegacyPlatformClaim]
        }
        PhysicalSubstrateLane::OfflineVerifier | PhysicalSubstrateLane::FoundationalExport => {
            vec![PhysicalProofOracleKind::VerifierRuntimeLayoutParity]
        }
        _ => vec![PhysicalProofOracleKind::BoundedPhysicalLocate],
    }
}

pub(crate) fn counter_expectations_for_lane(
    lane: Option<PhysicalSubstrateLane>,
    fixture: Option<LargeStorePressureFixture>,
) -> Vec<ScenarioCounterExpectation> {
    let mut counters = vec![
        ScenarioCounterExpectation::new(
            PhysicalCounterExpectationKind::WholeStoreMaterializationAttempts,
            0,
        ),
        ScenarioCounterExpectation::new(
            PhysicalCounterExpectationKind::LogicalDecodeBeforeHeaderValidation,
            0,
        ),
    ];
    if lane == Some(PhysicalSubstrateLane::LegacyOverclaim) {
        counters.push(ScenarioCounterExpectation::new(
            PhysicalCounterExpectationKind::LegacyPlatformClaimRejections,
            1,
        ));
    }
    if lane == Some(PhysicalSubstrateLane::OfflineVerifier) {
        counters.push(ScenarioCounterExpectation::new(
            PhysicalCounterExpectationKind::RuntimeVerifierParityComparisons,
            1,
        ));
    }
    if let Some(fixture) = fixture {
        counters.extend(pressure_counter_expectations(fixture));
    }
    counters
}

pub(crate) fn denial_boundary_for_lane(
    lane: Option<PhysicalSubstrateLane>,
    fixture: Option<LargeStorePressureFixture>,
) -> Option<ScenarioDenialBoundary> {
    if let Some(fixture) = fixture {
        return pressure_denial_boundary(fixture);
    }
    match lane {
        Some(PhysicalSubstrateLane::HostileReference) => {
            Some(ScenarioDenialBoundary::StaleGeneration)
        }
        Some(PhysicalSubstrateLane::HostileFormat) => {
            Some(ScenarioDenialBoundary::HeaderBeforePayload)
        }
        Some(PhysicalSubstrateLane::LegacyOverclaim) => {
            Some(ScenarioDenialBoundary::LegacyPlatformClaim)
        }
        Some(PhysicalSubstrateLane::S2Handoff) => Some(ScenarioDenialBoundary::WeakerS2Handoff),
        _ => None,
    }
}

pub(crate) fn forbidden_shortcuts_for_lane(
    lane: Option<PhysicalSubstrateLane>,
    fixture: Option<LargeStorePressureFixture>,
) -> Vec<ScenarioDenialBoundary> {
    let mut shortcuts = vec![
        ScenarioDenialBoundary::BackendResidueGuessing,
        ScenarioDenialBoundary::WholeStoreMaterialization,
    ];
    if fixture.is_some() {
        shortcuts.push(ScenarioDenialBoundary::BypassedLoweredPlan);
        shortcuts.push(ScenarioDenialBoundary::BypassedObserverTrace);
        shortcuts.push(ScenarioDenialBoundary::TestSupportOwnedMeaning);
    }
    if lane == Some(PhysicalSubstrateLane::FoundationalExport) {
        shortcuts.push(ScenarioDenialBoundary::FoundationalLookalike);
    }
    shortcuts
}

pub(crate) fn runtime_relationship_for_definition(
    lane: ScenarioLane,
    required_oracles: &[PhysicalProofOracleKind],
) -> RuntimeVerifierRelationship {
    match (
        lane.physical_substrate_lane(),
        required_oracles.contains(&PhysicalProofOracleKind::VerifierRuntimeLayoutParity),
    ) {
        (
            Some(
                PhysicalSubstrateLane::OfflineVerifier | PhysicalSubstrateLane::FoundationalExport,
            ),
            _,
        ) => RuntimeVerifierRelationship::RuntimeMustMatchVerifier,
        (Some(PhysicalSubstrateLane::HappyAuthority), true) => {
            RuntimeVerifierRelationship::RuntimeMustMatchVerifier
        }
        _ => RuntimeVerifierRelationship::NotApplicable,
    }
}

pub(crate) fn fixture_posture_for_lane(
    lane: Option<PhysicalSubstrateLane>,
    fixture: Option<LargeStorePressureFixture>,
) -> FixtureAdversaryPosture {
    if let Some(fixture) = fixture {
        return match fixture.class() {
            LargeStorePressureClass::FragmentedPressure => {
                FixtureAdversaryPosture::FragmentedResidentPressure
            }
            LargeStorePressureClass::ProtectedPressure => {
                FixtureAdversaryPosture::ProtectedResidentPressure
            }
            LargeStorePressureClass::StreamingPressure => {
                FixtureAdversaryPosture::StreamingPressure
            }
            _ => FixtureAdversaryPosture::LargeStorePressure,
        };
    }
    match lane {
        Some(PhysicalSubstrateLane::HostileReference) => FixtureAdversaryPosture::HostileReference,
        Some(PhysicalSubstrateLane::HostileFormat) => FixtureAdversaryPosture::HostileFormat,
        Some(PhysicalSubstrateLane::LegacyOverclaim) => FixtureAdversaryPosture::LegacyOverclaim,
        _ => FixtureAdversaryPosture::Clean,
    }
}

pub(crate) fn storage_crossings_for_steps(
    steps: &[PhysicalStoryStep],
    lane: ScenarioLane,
) -> Vec<StorageBoundaryCrossing> {
    let mut crossings = Vec::new();
    for step in steps {
        match step {
            PhysicalStoryStep::WhenAuthoritativeRecordIsAppended => push_unique(
                &mut crossings,
                StorageBoundaryCrossing::AppendPhysicalRecord,
            ),
            PhysicalStoryStep::WhenStoreClosesAndReopensFromBytes => {
                push_unique(&mut crossings, StorageBoundaryCrossing::ReopenFromBytes)
            }
            PhysicalStoryStep::WhenOfflineVerifierReadsManifest => {
                push_unique(&mut crossings, StorageBoundaryCrossing::OfflineVerifierRead)
            }
            PhysicalStoryStep::WhenLegacyClaimAsksForPlatformGrade => {
                push_unique(&mut crossings, StorageBoundaryCrossing::LegacyBackendProbe)
            }
            _ => {}
        }
    }
    if lane.physical_substrate_lane() == Some(PhysicalSubstrateLane::FoundationalExport) {
        push_unique(&mut crossings, StorageBoundaryCrossing::EvidenceExport);
    }
    crossings
}

pub(crate) fn capability_tier_for_lane(lane: ScenarioLane) -> PhysicalScenarioCapabilityTier {
    match lane.physical_substrate_lane() {
        Some(PhysicalSubstrateLane::LegacyOverclaim) => {
            PhysicalScenarioCapabilityTier::DeniedLegacyPlatformClaim
        }
        Some(_) => PhysicalScenarioCapabilityTier::PlatformGradePhysicalSubstrate,
        None => PhysicalScenarioCapabilityTier::RoadmapFollowOnExtension,
    }
}

pub(crate) fn cost_class_for_lane(
    lane: ScenarioLane,
    fixture: Option<LargeStorePressureFixture>,
) -> PhysicalScenarioCostClass {
    if fixture.is_some()
        && lane.family() == RoadmapLaneFamily::BufferPool
        && lane.physical_substrate_lane().is_none()
    {
        return PhysicalScenarioCostClass::LargeStoreMemoryPressure;
    }
    match lane.physical_substrate_lane() {
        Some(PhysicalSubstrateLane::LegacyOverclaim) => PhysicalScenarioCostClass::LegacyProbeOnly,
        Some(
            PhysicalSubstrateLane::OfflineVerifier | PhysicalSubstrateLane::FoundationalExport,
        ) => PhysicalScenarioCostClass::ManifestBoundedVerifierParity,
        Some(_) => PhysicalScenarioCostClass::BoundedPhysicalLocate,
        None => PhysicalScenarioCostClass::CertificationExtension,
    }
}

pub(crate) fn footprint_for_lane(
    lane: ScenarioLane,
    fixture: Option<LargeStorePressureFixture>,
) -> ExpectedPhysicalFootprint {
    if fixture.is_some()
        && lane.family() == RoadmapLaneFamily::BufferPool
        && lane.physical_substrate_lane().is_none()
    {
        return ExpectedPhysicalFootprint::LargeStorePressureFixture;
    }
    match lane.physical_substrate_lane() {
        Some(PhysicalSubstrateLane::HappyAuthority) => {
            ExpectedPhysicalFootprint::SinglePageAuthority
        }
        Some(PhysicalSubstrateLane::HostileReference) => {
            ExpectedPhysicalFootprint::HostileReferenceProbe
        }
        Some(PhysicalSubstrateLane::HostileFormat) => ExpectedPhysicalFootprint::HostileFormatProbe,
        Some(PhysicalSubstrateLane::LegacyOverclaim) => {
            ExpectedPhysicalFootprint::LegacyBackendClaimProbe
        }
        Some(PhysicalSubstrateLane::OfflineVerifier) => {
            ExpectedPhysicalFootprint::OfflineManifestRead
        }
        Some(PhysicalSubstrateLane::ScaleLocality) => {
            ExpectedPhysicalFootprint::LocalityScaleSample
        }
        Some(PhysicalSubstrateLane::FoundationalExport) => {
            ExpectedPhysicalFootprint::FoundationalEvidenceExport
        }
        Some(PhysicalSubstrateLane::S2Handoff) => {
            ExpectedPhysicalFootprint::RoadmapFamilyExtension(RoadmapLaneFamily::BufferPool)
        }
        None => ExpectedPhysicalFootprint::RoadmapFamilyExtension(lane.family()),
    }
}

fn push_unique<T: PartialEq>(items: &mut Vec<T>, item: T) {
    if !items.contains(&item) {
        items.push(item);
    }
}

fn pressure_counter_expectations(
    fixture: LargeStorePressureFixture,
) -> Vec<ScenarioCounterExpectation> {
    let pinned_pages = fixture.protected_page_count();
    let copied_payload_bytes = if fixture.class() == LargeStorePressureClass::StreamingPressure {
        fixture.streaming_window_bytes()
    } else {
        0
    };
    vec![
        ScenarioCounterExpectation::new(
            PhysicalCounterExpectationKind::PressureFixtureStoreBytes,
            fixture.declared_store_bytes(),
        ),
        ScenarioCounterExpectation::new(
            PhysicalCounterExpectationKind::PressureFixtureResidentBudgetBytes,
            fixture.resident_budget_bytes(),
        ),
        ScenarioCounterExpectation::new(
            PhysicalCounterExpectationKind::ResidentBytesPeak,
            fixture.resident_budget_bytes(),
        ),
        ScenarioCounterExpectation::new(
            PhysicalCounterExpectationKind::PinnedPagesPeak,
            pinned_pages,
        ),
        ScenarioCounterExpectation::new(PhysicalCounterExpectationKind::DirtyPagesPeak, 0),
        ScenarioCounterExpectation::new(
            PhysicalCounterExpectationKind::AllocationBytesPeak,
            fixture.allocation_envelope_bytes(),
        ),
        ScenarioCounterExpectation::new(
            PhysicalCounterExpectationKind::CopiedPayloadBytes,
            copied_payload_bytes,
        ),
        ScenarioCounterExpectation::new(
            PhysicalCounterExpectationKind::DomainObjectConstructions,
            0,
        ),
        ScenarioCounterExpectation::new(
            PhysicalCounterExpectationKind::UnboundedAllocationAttempts,
            0,
        ),
        ScenarioCounterExpectation::new(
            PhysicalCounterExpectationKind::DiagnosticMaterializationBytes,
            0,
        ),
    ]
}

fn pressure_denial_boundary(fixture: LargeStorePressureFixture) -> Option<ScenarioDenialBoundary> {
    match fixture.class() {
        LargeStorePressureClass::ProtectedPressure => {
            Some(ScenarioDenialBoundary::ProtectedResidentPressure)
        }
        LargeStorePressureClass::StreamingPressure => {
            Some(ScenarioDenialBoundary::StreamingWindowPressure)
        }
        _ => None,
    }
}
