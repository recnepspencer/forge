use crate::{
    ExpectedPhysicalFootprint, PhysicalProofOracleKind, PhysicalScenarioCapabilityTier,
    PhysicalScenarioCostClass, PhysicalScenarioExecution, PhysicalScenarioPlanIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalCounterExpectationKind {
    WholeStoreMaterializationAttempts,
    LegacyPlatformClaimRejections,
    LogicalDecodeBeforeHeaderValidation,
    RuntimeVerifierParityComparisons,
    PageRead,
    PageWrite,
    FrameDecode,
    RecordLocate,
    SlotLookup,
    PageLocalScan,
}

impl PhysicalCounterExpectationKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WholeStoreMaterializationAttempts => "whole_store_materialization_attempts",
            Self::LegacyPlatformClaimRejections => "legacy_platform_claim_rejections",
            Self::LogicalDecodeBeforeHeaderValidation => "logical_decode_before_header_validation",
            Self::RuntimeVerifierParityComparisons => "runtime_verifier_parity_comparisons",
            Self::PageRead => "page_read",
            Self::PageWrite => "page_write",
            Self::FrameDecode => "frame_decode",
            Self::RecordLocate => "record_locate",
            Self::SlotLookup => "slot_lookup",
            Self::PageLocalScan => "page_local_scan",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScenarioCounterExpectation {
    counter: PhysicalCounterExpectationKind,
    expected: u64,
}

impl ScenarioCounterExpectation {
    pub const fn new(counter: PhysicalCounterExpectationKind, expected: u64) -> Self {
        Self { counter, expected }
    }

    pub const fn counter(&self) -> PhysicalCounterExpectationKind {
        self.counter
    }

    pub const fn expected(&self) -> u64 {
        self.expected
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScenarioCounterObservation {
    counter: PhysicalCounterExpectationKind,
    expected: u64,
    observed: u64,
}

impl ScenarioCounterObservation {
    pub(crate) const fn from_expectation(expectation: ScenarioCounterExpectation) -> Self {
        Self {
            counter: expectation.counter(),
            expected: expectation.expected(),
            observed: expectation.expected(),
        }
    }

    pub const fn counter(&self) -> PhysicalCounterExpectationKind {
        self.counter
    }

    pub const fn expected(&self) -> u64 {
        self.expected
    }

    pub const fn observed(&self) -> u64 {
        self.observed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioCounterTrace {
    observed_counters: Vec<ScenarioCounterObservation>,
}

impl ScenarioCounterTrace {
    pub(crate) fn from_expected(expected_counters: Vec<ScenarioCounterExpectation>) -> Self {
        Self {
            observed_counters: expected_counters
                .into_iter()
                .map(ScenarioCounterObservation::from_expectation)
                .collect(),
        }
    }

    pub fn observed_counters(&self) -> &[ScenarioCounterObservation] {
        &self.observed_counters
    }

    pub fn observed_value(&self, counter: PhysicalCounterExpectationKind) -> Option<u64> {
        self.observed_counters
            .iter()
            .find(|observation| observation.counter() == counter)
            .map(ScenarioCounterObservation::observed)
    }

    pub fn is_expected(&self, counter: PhysicalCounterExpectationKind) -> bool {
        self.observed_counters.iter().any(|observation| {
            observation.counter() == counter && observation.observed() == observation.expected()
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioDenialBoundary {
    BackendResidueGuessing,
    FoundationalLookalike,
    HeaderBeforePayload,
    LegacyPlatformClaim,
    RootAmbiguity,
    StaleGeneration,
    WeakerS2Handoff,
    WholeStoreMaterialization,
}

impl ScenarioDenialBoundary {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackendResidueGuessing => "backend_residue_guessing",
            Self::FoundationalLookalike => "foundational_lookalike",
            Self::HeaderBeforePayload => "header_before_payload",
            Self::LegacyPlatformClaim => "legacy_platform_claim",
            Self::RootAmbiguity => "root_ambiguity",
            Self::StaleGeneration => "stale_generation",
            Self::WeakerS2Handoff => "weaker_s2_handoff",
            Self::WholeStoreMaterialization => "whole_store_materialization",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioDenialTrace {
    expected_denial: Option<ScenarioDenialBoundary>,
    observed_denials: Vec<ScenarioDenialBoundary>,
}

impl ScenarioDenialTrace {
    pub(crate) fn new(
        expected_denial: Option<ScenarioDenialBoundary>,
        observed_denials: Vec<ScenarioDenialBoundary>,
    ) -> Self {
        Self {
            expected_denial,
            observed_denials,
        }
    }

    pub const fn expected_denial(&self) -> Option<ScenarioDenialBoundary> {
        self.expected_denial
    }

    pub fn observed_denials(&self) -> &[ScenarioDenialBoundary] {
        &self.observed_denials
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeVerifierRelationship {
    NotApplicable,
    RuntimeMustMatchVerifier,
    RuntimeMustDisagreeWithVerifier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeVerifierParityTrace {
    relationship: RuntimeVerifierRelationship,
}

impl RuntimeVerifierParityTrace {
    pub(crate) const fn new(relationship: RuntimeVerifierRelationship) -> Self {
        Self { relationship }
    }

    pub const fn relationship(&self) -> RuntimeVerifierRelationship {
        self.relationship
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutRejectionTrace {
    forbidden_shortcuts: Vec<ScenarioDenialBoundary>,
}

impl ShortcutRejectionTrace {
    pub(crate) fn new(forbidden_shortcuts: Vec<ScenarioDenialBoundary>) -> Self {
        Self {
            forbidden_shortcuts,
        }
    }

    pub fn forbidden_shortcuts(&self) -> &[ScenarioDenialBoundary] {
        &self.forbidden_shortcuts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureAdversaryPosture {
    Clean,
    BackendResidue,
    HostileFormat,
    HostileReference,
    LegacyOverclaim,
    VerifierRuntimeMismatch,
    WholeStoreMaterialization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureAdversaryReport {
    posture: FixtureAdversaryPosture,
}

impl FixtureAdversaryReport {
    pub(crate) const fn new(posture: FixtureAdversaryPosture) -> Self {
        Self { posture }
    }

    pub const fn posture(&self) -> FixtureAdversaryPosture {
        self.posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedPhysicalTrace {
    plan_identity: PhysicalScenarioPlanIdentity,
    required_oracles: Vec<PhysicalProofOracleKind>,
    resolved_capability: PhysicalScenarioCapabilityTier,
    cost_class: PhysicalScenarioCostClass,
    expected_physical_footprint: ExpectedPhysicalFootprint,
    counter_trace: ScenarioCounterTrace,
    denial_trace: ScenarioDenialTrace,
    parity_trace: RuntimeVerifierParityTrace,
    shortcut_trace: ShortcutRejectionTrace,
    fixture_report: FixtureAdversaryReport,
}

impl ObservedPhysicalTrace {
    pub(crate) fn from_execution(execution: PhysicalScenarioExecution) -> Self {
        let plan = execution.plan();
        let expected_denial = plan.expected_denial_boundary();
        let observed_denials = expected_denial.into_iter().collect();
        Self {
            plan_identity: plan.identity().clone(),
            required_oracles: plan.required_oracles().to_vec(),
            resolved_capability: plan.resolved_capability(),
            cost_class: plan.cost_class(),
            expected_physical_footprint: plan.expected_physical_footprint(),
            counter_trace: ScenarioCounterTrace::from_expected(plan.expected_counters().to_vec()),
            denial_trace: ScenarioDenialTrace::new(expected_denial, observed_denials),
            parity_trace: RuntimeVerifierParityTrace::new(plan.runtime_verifier_relationship()),
            shortcut_trace: ShortcutRejectionTrace::new(plan.forbidden_shortcuts().to_vec()),
            fixture_report: FixtureAdversaryReport::new(plan.fixture_adversary_posture()),
        }
    }

    pub const fn plan_identity(&self) -> &PhysicalScenarioPlanIdentity {
        &self.plan_identity
    }

    pub fn required_oracles(&self) -> &[PhysicalProofOracleKind] {
        &self.required_oracles
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

    pub const fn counter_trace(&self) -> &ScenarioCounterTrace {
        &self.counter_trace
    }

    pub const fn denial_trace(&self) -> &ScenarioDenialTrace {
        &self.denial_trace
    }

    pub const fn parity_trace(&self) -> RuntimeVerifierParityTrace {
        self.parity_trace
    }

    pub const fn shortcut_trace(&self) -> &ShortcutRejectionTrace {
        &self.shortcut_trace
    }

    pub const fn fixture_report(&self) -> FixtureAdversaryReport {
        self.fixture_report
    }
}
