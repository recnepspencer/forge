use crate::{
    ExpectedPhysicalFootprint, ObservedPhysicalTrace, PhysicalCounterExpectationKind,
    PhysicalScenarioCapabilityTier, PhysicalScenarioCostClass, RuntimeVerifierRelationship,
    ScenarioDenialBoundary,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalProofOracleKind {
    BoundedPhysicalLocate,
    ForbiddenLegacyPlatformClaim,
    NoWholeStoreMaterialization,
    ScenarioPlanOwnsStrategy,
    TranscriptPreservesEvidence,
    VerifierRuntimeLayoutParity,
}

impl PhysicalProofOracleKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundedPhysicalLocate => "bounded_physical_locate",
            Self::ForbiddenLegacyPlatformClaim => "forbidden_legacy_platform_claim",
            Self::NoWholeStoreMaterialization => "no_whole_store_materialization",
            Self::ScenarioPlanOwnsStrategy => "scenario_plan_owns_strategy",
            Self::TranscriptPreservesEvidence => "transcript_preserves_evidence",
            Self::VerifierRuntimeLayoutParity => "verifier_runtime_layout_parity",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalOracleDenialKind {
    CounterMismatch(PhysicalCounterExpectationKind),
    MissingCounterTrace(PhysicalCounterExpectationKind),
    MissingExpectedDenial(ScenarioDenialBoundary),
    MissingResolvedPlanStrategy,
    MissingRuntimeVerifierParity,
    MissingTranscriptEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalOracleOutcome {
    Satisfied,
    Denied(PhysicalOracleDenialKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalOracleJudgment {
    oracle: PhysicalProofOracleKind,
    outcome: PhysicalOracleOutcome,
}

impl PhysicalOracleJudgment {
    pub(crate) const fn new(
        oracle: PhysicalProofOracleKind,
        outcome: PhysicalOracleOutcome,
    ) -> Self {
        Self { oracle, outcome }
    }

    pub const fn oracle(&self) -> PhysicalProofOracleKind {
        self.oracle
    }

    pub const fn outcome(&self) -> PhysicalOracleOutcome {
        self.outcome
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalProofOracleVerdict {
    observed_trace: ObservedPhysicalTrace,
    judgments: Vec<PhysicalOracleJudgment>,
}

impl PhysicalProofOracleVerdict {
    pub(crate) fn from_trace(observed_trace: ObservedPhysicalTrace) -> Self {
        let judgments = observed_trace
            .required_oracles()
            .iter()
            .copied()
            .map(|oracle| {
                PhysicalOracleJudgment::new(oracle, judge_oracle(oracle, &observed_trace))
            })
            .collect();
        Self {
            observed_trace,
            judgments,
        }
    }

    pub const fn observed_trace(&self) -> &ObservedPhysicalTrace {
        &self.observed_trace
    }

    pub fn judgments(&self) -> &[PhysicalOracleJudgment] {
        &self.judgments
    }
}

fn judge_oracle(
    oracle: PhysicalProofOracleKind,
    trace: &ObservedPhysicalTrace,
) -> PhysicalOracleOutcome {
    match oracle {
        PhysicalProofOracleKind::BoundedPhysicalLocate => judge_bounded_physical_locate(trace),
        PhysicalProofOracleKind::ForbiddenLegacyPlatformClaim => {
            judge_forbidden_legacy_platform_claim(trace)
        }
        PhysicalProofOracleKind::NoWholeStoreMaterialization => {
            judge_no_whole_store_materialization(trace)
        }
        PhysicalProofOracleKind::ScenarioPlanOwnsStrategy => judge_plan_owns_strategy(trace),
        PhysicalProofOracleKind::TranscriptPreservesEvidence => judge_transcript_evidence(trace),
        PhysicalProofOracleKind::VerifierRuntimeLayoutParity => {
            judge_runtime_verifier_parity(trace)
        }
    }
}

fn judge_bounded_physical_locate(trace: &ObservedPhysicalTrace) -> PhysicalOracleOutcome {
    if !trace
        .counter_trace()
        .is_expected(PhysicalCounterExpectationKind::LogicalDecodeBeforeHeaderValidation)
    {
        return PhysicalOracleOutcome::Denied(PhysicalOracleDenialKind::CounterMismatch(
            PhysicalCounterExpectationKind::LogicalDecodeBeforeHeaderValidation,
        ));
    }
    judge_no_whole_store_materialization(trace)
}

fn judge_forbidden_legacy_platform_claim(trace: &ObservedPhysicalTrace) -> PhysicalOracleOutcome {
    if !trace
        .denial_trace()
        .observed_denials()
        .contains(&ScenarioDenialBoundary::LegacyPlatformClaim)
    {
        return PhysicalOracleOutcome::Denied(PhysicalOracleDenialKind::MissingExpectedDenial(
            ScenarioDenialBoundary::LegacyPlatformClaim,
        ));
    }
    if !trace
        .counter_trace()
        .is_expected(PhysicalCounterExpectationKind::LegacyPlatformClaimRejections)
    {
        return PhysicalOracleOutcome::Denied(PhysicalOracleDenialKind::CounterMismatch(
            PhysicalCounterExpectationKind::LegacyPlatformClaimRejections,
        ));
    }
    PhysicalOracleOutcome::Satisfied
}

fn judge_no_whole_store_materialization(trace: &ObservedPhysicalTrace) -> PhysicalOracleOutcome {
    match trace
        .counter_trace()
        .observed_value(PhysicalCounterExpectationKind::WholeStoreMaterializationAttempts)
    {
        Some(0) => PhysicalOracleOutcome::Satisfied,
        Some(_) => PhysicalOracleOutcome::Denied(PhysicalOracleDenialKind::CounterMismatch(
            PhysicalCounterExpectationKind::WholeStoreMaterializationAttempts,
        )),
        None => PhysicalOracleOutcome::Denied(PhysicalOracleDenialKind::MissingCounterTrace(
            PhysicalCounterExpectationKind::WholeStoreMaterializationAttempts,
        )),
    }
}

fn judge_plan_owns_strategy(trace: &ObservedPhysicalTrace) -> PhysicalOracleOutcome {
    let has_capability = matches!(
        trace.resolved_capability(),
        PhysicalScenarioCapabilityTier::PlatformGradePhysicalSubstrate
            | PhysicalScenarioCapabilityTier::DeniedLegacyPlatformClaim
            | PhysicalScenarioCapabilityTier::RoadmapFollowOnExtension
    );
    let has_cost = matches!(
        trace.cost_class(),
        PhysicalScenarioCostClass::BoundedPhysicalLocate
            | PhysicalScenarioCostClass::CertificationExtension
            | PhysicalScenarioCostClass::LegacyProbeOnly
            | PhysicalScenarioCostClass::ManifestBoundedVerifierParity
    );
    let has_footprint = matches!(
        trace.expected_physical_footprint(),
        ExpectedPhysicalFootprint::SinglePageAuthority
            | ExpectedPhysicalFootprint::HostileReferenceProbe
            | ExpectedPhysicalFootprint::HostileFormatProbe
            | ExpectedPhysicalFootprint::LegacyBackendClaimProbe
            | ExpectedPhysicalFootprint::OfflineManifestRead
            | ExpectedPhysicalFootprint::LocalityScaleSample
            | ExpectedPhysicalFootprint::FoundationalEvidenceExport
            | ExpectedPhysicalFootprint::RoadmapFamilyExtension(_)
    );
    if has_capability && has_cost && has_footprint && !trace.required_oracles().is_empty() {
        PhysicalOracleOutcome::Satisfied
    } else {
        PhysicalOracleOutcome::Denied(PhysicalOracleDenialKind::MissingResolvedPlanStrategy)
    }
}

fn judge_transcript_evidence(trace: &ObservedPhysicalTrace) -> PhysicalOracleOutcome {
    if trace.counter_trace().observed_counters().is_empty() || trace.required_oracles().is_empty() {
        return PhysicalOracleOutcome::Denied(PhysicalOracleDenialKind::MissingTranscriptEvidence);
    }
    PhysicalOracleOutcome::Satisfied
}

fn judge_runtime_verifier_parity(trace: &ObservedPhysicalTrace) -> PhysicalOracleOutcome {
    if trace.parity_trace().relationship() == RuntimeVerifierRelationship::RuntimeMustMatchVerifier
    {
        PhysicalOracleOutcome::Satisfied
    } else {
        PhysicalOracleOutcome::Denied(PhysicalOracleDenialKind::MissingRuntimeVerifierParity)
    }
}
