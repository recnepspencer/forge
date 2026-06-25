use crate::{
    ExpectedPhysicalFootprint, FixtureAdversaryPosture, LaneFamilyExtension,
    PhysicalCounterExpectationKind, PhysicalProofOracleKind, PhysicalScenarioCapabilityTier,
    PhysicalScenarioCostClass, PhysicalScenarioDriverKind, PhysicalScenarioDriverRequirement,
    PhysicalScenarioObserverKind, PhysicalScenarioObserverRequirement, PhysicalScenarioPlanDenial,
    PhysicalStoryStep, PhysicalSubstrateLane, RoadmapLaneFamily, RuntimeVerifierRelationship,
    ScenarioCounterExpectation, ScenarioDenialBoundary, ScenarioLane, StorageBoundaryCrossing,
};

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
    counters
}

pub(crate) fn denial_boundary_for_lane(
    lane: PhysicalSubstrateLane,
) -> Option<ScenarioDenialBoundary> {
    match lane {
        PhysicalSubstrateLane::HostileReference => Some(ScenarioDenialBoundary::StaleGeneration),
        PhysicalSubstrateLane::HostileFormat => Some(ScenarioDenialBoundary::HeaderBeforePayload),
        PhysicalSubstrateLane::LegacyOverclaim => Some(ScenarioDenialBoundary::LegacyPlatformClaim),
        PhysicalSubstrateLane::S2Handoff => Some(ScenarioDenialBoundary::WeakerS2Handoff),
        _ => None,
    }
}

pub(crate) fn forbidden_shortcuts_for_lane(
    lane: Option<PhysicalSubstrateLane>,
) -> Vec<ScenarioDenialBoundary> {
    let mut shortcuts = vec![
        ScenarioDenialBoundary::BackendResidueGuessing,
        ScenarioDenialBoundary::WholeStoreMaterialization,
    ];
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
) -> FixtureAdversaryPosture {
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

pub(crate) fn cost_class_for_lane(lane: ScenarioLane) -> PhysicalScenarioCostClass {
    match lane.physical_substrate_lane() {
        Some(PhysicalSubstrateLane::LegacyOverclaim) => PhysicalScenarioCostClass::LegacyProbeOnly,
        Some(
            PhysicalSubstrateLane::OfflineVerifier | PhysicalSubstrateLane::FoundationalExport,
        ) => PhysicalScenarioCostClass::ManifestBoundedVerifierParity,
        Some(_) => PhysicalScenarioCostClass::BoundedPhysicalLocate,
        None => PhysicalScenarioCostClass::CertificationExtension,
    }
}

pub(crate) fn footprint_for_lane(lane: ScenarioLane) -> ExpectedPhysicalFootprint {
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
