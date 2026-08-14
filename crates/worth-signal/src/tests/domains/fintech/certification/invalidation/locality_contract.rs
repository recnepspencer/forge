use crate::tests::domains::fintech::world::{
    ordinary_locality_cases, scheduled_locality_cases, FinancialLocalityScenario,
    LocalityCaseContract, LocalityScaleTuple,
};

const LOCALITY_IDENTITY_SCHEMA_VERSION: u16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LocalityExpectationDimension {
    QueriedBucketKeys,
    CandidateDependencies,
    CanonicalCauses,
    CanonicalWork,
    NecessaryEvaluations,
    UnchangedOutputStops,
    PeakReadyWidth,
}

impl LocalityExpectationDimension {
    const ALL: [Self; 7] = [
        Self::QueriedBucketKeys,
        Self::CandidateDependencies,
        Self::CanonicalCauses,
        Self::CanonicalWork,
        Self::NecessaryEvaluations,
        Self::UnchangedOutputStops,
        Self::PeakReadyWidth,
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LocalityIdentityAxis {
    ScenarioAndSchema,
    FinancialSeedAndScale,
    MutationTraceAndStep,
    ConsumerComparatorPolicy,
    ProducerOutputEquivalence,
    DiagnosticsTier,
    ExecutionPosture,
    ExpectedSemanticWork,
    ExpectedCounterContract,
}

impl LocalityIdentityAxis {
    const ALL: [Self; 9] = [
        Self::ScenarioAndSchema,
        Self::FinancialSeedAndScale,
        Self::MutationTraceAndStep,
        Self::ConsumerComparatorPolicy,
        Self::ProducerOutputEquivalence,
        Self::DiagnosticsTier,
        Self::ExecutionPosture,
        Self::ExpectedSemanticWork,
        Self::ExpectedCounterContract,
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LocalityGeneratorInvariant {
    ExactCompiledSemanticOutputCount,
    EveryOutputHasFinancialAuditOrReportingOwner,
    EveryNonSourceHasCompilerDeclaredDependency,
    BaselineIsCausallyComplete,
    DependencySnapshotsMatchDeclaredTopology,
    RelevantDepthSixteenChainIsStable,
    CurveRegionsAreIndependentlyOwned,
    MatchingMembershipAndInstrumentDensityVarySeparately,
    FourDistinctImmediateProducersConverge,
    DenseAffectedSetIsDeclaredBeforeExecution,
    TopologyChurnUsesProductionMutationAuthority,
    RestoreStartsWithAuthoritativePendingState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LocalityMutationContract {
    QuoteBump,
    CurveDetailBump,
    FourFactorCommitPermutation,
    DeclaredDenseMarketClose,
    AcceptedRewireAndRejectedCycle,
    BranchShockCaptureRestoreReplay,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LocalityRedMutation {
    RestoreTransitiveSubscriberClosure,
    QueryProducerWideSubscriberList,
    FilterAspectAfterMutation,
    OmitRejectedCandidatesFromCounters,
    ReplaceOwnedFanoutWithDisconnectedPadding,
    ScanAllRegions,
    FlattenAspectScopeCorrelation,
    EnumerateUnrelatedDetailBuckets,
    RetainOnlyLastChangedDetail,
    RejectScopeAfterMutationOrEnqueue,
    TrustDriftedDerivedIndex,
    DeduplicateByNodeOrAspectSlotOnly,
    ReplaceCanonicalCauseSetWithLastCause,
    EvaluateConvergedTargetTwice,
    ReconstructProvenanceAfterMerge,
    MakeCanonicalizationSeedOrderDependent,
    CapDenseWork,
    DropNecessaryDenseWork,
    RelabelFullGraphWalkAsEvaluation,
    OmitQueueWidthOrAllocationRows,
    CompareStrategiesOnDifferentStreams,
    ExecuteStalePreRewireWork,
    AdvanceRevisionOnRejectedCycle,
    SerializeAndExecuteReadyAuthority,
    SubstitutePendingCauseAtEqualCardinality,
    CountRecoveryAsOrdinaryHotWork,
    CompareReplayByKindsOrFinalValuesOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LocalityIdentitySidecarContract {
    schema_version: u16,
    seed: u64,
    scale: LocalityScaleTuple,
    mutation: LocalityMutationContract,
    identity_axes: &'static [LocalityIdentityAxis],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FinancialLocalityCaseDefinition {
    case: LocalityCaseContract,
    identity: LocalityIdentitySidecarContract,
    invariants: &'static [LocalityGeneratorInvariant],
    expectation_dimensions: &'static [LocalityExpectationDimension],
    red_mutations: &'static [LocalityRedMutation],
}

impl FinancialLocalityCaseDefinition {
    fn freeze(case: LocalityCaseContract, seed: u64) -> Self {
        let scenario = case.scenario();
        Self {
            case,
            identity: LocalityIdentitySidecarContract {
                schema_version: LOCALITY_IDENTITY_SCHEMA_VERSION,
                seed,
                scale: case.scale,
                mutation: mutation_contract(scenario),
                identity_axes: &LocalityIdentityAxis::ALL,
            },
            invariants: generator_invariants(scenario),
            expectation_dimensions: &LocalityExpectationDimension::ALL,
            red_mutations: red_mutations(scenario),
        }
    }
}

fn mutation_contract(scenario: FinancialLocalityScenario) -> LocalityMutationContract {
    match scenario {
        FinancialLocalityScenario::SparseBookFanout => LocalityMutationContract::QuoteBump,
        FinancialLocalityScenario::PartitionedCurveUniverse => {
            LocalityMutationContract::CurveDetailBump
        }
        FinancialLocalityScenario::ConvergentFactorBatch => {
            LocalityMutationContract::FourFactorCommitPermutation
        }
        FinancialLocalityScenario::DenseMarketClose => {
            LocalityMutationContract::DeclaredDenseMarketClose
        }
        FinancialLocalityScenario::PortfolioDependencyChurn => {
            LocalityMutationContract::AcceptedRewireAndRejectedCycle
        }
        FinancialLocalityScenario::BranchRestoreLocalityReplay => {
            LocalityMutationContract::BranchShockCaptureRestoreReplay
        }
    }
}

use LocalityGeneratorInvariant as GeneratorInvariant;

const BASE_GENERATOR_INVARIANTS: &[GeneratorInvariant] = &[
    GeneratorInvariant::ExactCompiledSemanticOutputCount,
    GeneratorInvariant::EveryOutputHasFinancialAuditOrReportingOwner,
    GeneratorInvariant::EveryNonSourceHasCompilerDeclaredDependency,
    GeneratorInvariant::BaselineIsCausallyComplete,
    GeneratorInvariant::DependencySnapshotsMatchDeclaredTopology,
];

const SPARSE_GENERATOR_INVARIANTS: &[GeneratorInvariant] = &[
    BASE_GENERATOR_INVARIANTS[0],
    BASE_GENERATOR_INVARIANTS[1],
    BASE_GENERATOR_INVARIANTS[2],
    BASE_GENERATOR_INVARIANTS[3],
    BASE_GENERATOR_INVARIANTS[4],
    GeneratorInvariant::RelevantDepthSixteenChainIsStable,
];

const PARTITION_GENERATOR_INVARIANTS: &[GeneratorInvariant] = &[
    BASE_GENERATOR_INVARIANTS[0],
    BASE_GENERATOR_INVARIANTS[1],
    BASE_GENERATOR_INVARIANTS[2],
    BASE_GENERATOR_INVARIANTS[3],
    BASE_GENERATOR_INVARIANTS[4],
    GeneratorInvariant::CurveRegionsAreIndependentlyOwned,
    GeneratorInvariant::MatchingMembershipAndInstrumentDensityVarySeparately,
];

const CONVERGENT_GENERATOR_INVARIANTS: &[GeneratorInvariant] = &[
    BASE_GENERATOR_INVARIANTS[0],
    BASE_GENERATOR_INVARIANTS[1],
    BASE_GENERATOR_INVARIANTS[2],
    BASE_GENERATOR_INVARIANTS[3],
    BASE_GENERATOR_INVARIANTS[4],
    GeneratorInvariant::FourDistinctImmediateProducersConverge,
];

const DENSE_GENERATOR_INVARIANTS: &[GeneratorInvariant] = &[
    BASE_GENERATOR_INVARIANTS[0],
    BASE_GENERATOR_INVARIANTS[1],
    BASE_GENERATOR_INVARIANTS[2],
    BASE_GENERATOR_INVARIANTS[3],
    BASE_GENERATOR_INVARIANTS[4],
    GeneratorInvariant::DenseAffectedSetIsDeclaredBeforeExecution,
];

const CHURN_GENERATOR_INVARIANTS: &[GeneratorInvariant] = &[
    BASE_GENERATOR_INVARIANTS[0],
    BASE_GENERATOR_INVARIANTS[1],
    BASE_GENERATOR_INVARIANTS[2],
    BASE_GENERATOR_INVARIANTS[3],
    BASE_GENERATOR_INVARIANTS[4],
    GeneratorInvariant::TopologyChurnUsesProductionMutationAuthority,
];

const RESTORE_GENERATOR_INVARIANTS: &[GeneratorInvariant] = &[
    BASE_GENERATOR_INVARIANTS[0],
    BASE_GENERATOR_INVARIANTS[1],
    BASE_GENERATOR_INVARIANTS[2],
    BASE_GENERATOR_INVARIANTS[3],
    BASE_GENERATOR_INVARIANTS[4],
    GeneratorInvariant::RestoreStartsWithAuthoritativePendingState,
];

fn generator_invariants(
    scenario: FinancialLocalityScenario,
) -> &'static [LocalityGeneratorInvariant] {
    match scenario {
        FinancialLocalityScenario::SparseBookFanout => SPARSE_GENERATOR_INVARIANTS,
        FinancialLocalityScenario::PartitionedCurveUniverse => PARTITION_GENERATOR_INVARIANTS,
        FinancialLocalityScenario::ConvergentFactorBatch => CONVERGENT_GENERATOR_INVARIANTS,
        FinancialLocalityScenario::DenseMarketClose => DENSE_GENERATOR_INVARIANTS,
        FinancialLocalityScenario::PortfolioDependencyChurn => CHURN_GENERATOR_INVARIANTS,
        FinancialLocalityScenario::BranchRestoreLocalityReplay => RESTORE_GENERATOR_INVARIANTS,
    }
}

fn red_mutations(scenario: FinancialLocalityScenario) -> &'static [LocalityRedMutation] {
    use LocalityRedMutation as M;
    match scenario {
        FinancialLocalityScenario::SparseBookFanout => &[
            M::RestoreTransitiveSubscriberClosure,
            M::QueryProducerWideSubscriberList,
            M::FilterAspectAfterMutation,
            M::OmitRejectedCandidatesFromCounters,
            M::ReplaceOwnedFanoutWithDisconnectedPadding,
        ],
        FinancialLocalityScenario::PartitionedCurveUniverse => &[
            M::ScanAllRegions,
            M::FlattenAspectScopeCorrelation,
            M::EnumerateUnrelatedDetailBuckets,
            M::RetainOnlyLastChangedDetail,
            M::RejectScopeAfterMutationOrEnqueue,
            M::TrustDriftedDerivedIndex,
        ],
        FinancialLocalityScenario::ConvergentFactorBatch => &[
            M::DeduplicateByNodeOrAspectSlotOnly,
            M::ReplaceCanonicalCauseSetWithLastCause,
            M::EvaluateConvergedTargetTwice,
            M::ReconstructProvenanceAfterMerge,
            M::MakeCanonicalizationSeedOrderDependent,
        ],
        FinancialLocalityScenario::DenseMarketClose => &[
            M::CapDenseWork,
            M::DropNecessaryDenseWork,
            M::RelabelFullGraphWalkAsEvaluation,
            M::OmitQueueWidthOrAllocationRows,
            M::CompareStrategiesOnDifferentStreams,
        ],
        FinancialLocalityScenario::PortfolioDependencyChurn => &[
            M::ExecuteStalePreRewireWork,
            M::AdvanceRevisionOnRejectedCycle,
        ],
        FinancialLocalityScenario::BranchRestoreLocalityReplay => &[
            M::SerializeAndExecuteReadyAuthority,
            M::SubstitutePendingCauseAtEqualCardinality,
            M::CountRecoveryAsOrdinaryHotWork,
            M::CompareReplayByKindsOrFinalValuesOnly,
        ],
    }
}

#[test]
fn every_frozen_scale_case_has_generator_expectation_identity_and_mutation_contracts() {
    let cases = ordinary_locality_cases()
        .into_iter()
        .chain(scheduled_locality_cases());
    let mut observed = std::collections::BTreeSet::new();
    for case in cases {
        let definition = FinancialLocalityCaseDefinition::freeze(case, 41);
        observed.insert(definition.case.scenario());
        assert_eq!(definition.identity.schema_version, 1);
        assert_eq!(definition.identity.seed, 41);
        assert_eq!(definition.identity.scale, case.scale);
        assert_eq!(
            definition.identity.mutation,
            mutation_contract(definition.case.scenario())
        );
        assert_eq!(definition.identity.identity_axes.len(), 9);
        assert_eq!(definition.expectation_dimensions.len(), 7);
        assert!(!definition.invariants.is_empty());
        assert!(!definition.red_mutations.is_empty());
    }
    assert_eq!(observed, FinancialLocalityScenario::ALL.into());
}
