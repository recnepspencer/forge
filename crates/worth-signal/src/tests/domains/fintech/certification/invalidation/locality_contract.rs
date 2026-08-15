use crate::tests::domains::fintech::world::{
    ordinary_locality_cases, scheduled_locality_cases, FinancialLocalityComparisonPolicy,
    FinancialLocalityDefinition, FinancialLocalityOutputPolicy, FinancialLocalityScenario,
    FinancialWorldDefinition, LocalityCaseContract, LocalityScaleTuple,
};

mod mutations;
use mutations::red_mutations;

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
enum LocalityComparatorContract {
    ExactAspectVersion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LocalityDiagnosticsContract {
    DevelopmentIdentitySidecar,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LocalityExpectationContract {
    CompleteTraceManifestV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LocalityCounterContract {
    ExactQueriedCandidateCauseWorkEvaluationStopAndPeakRows,
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
struct LocalityIdentitySidecarContract {
    schema_version: u16,
    seed: u64,
    scale: LocalityScaleTuple,
    mutation: LocalityMutationContract,
    consumer_comparator: LocalityComparatorContract,
    producer_output_equivalence: LocalityComparatorContract,
    diagnostics: LocalityDiagnosticsContract,
    execution_posture: crate::tests::domains::fintech::world::LocalityLane,
    expected_semantic_work: LocalityExpectationContract,
    expected_counters: LocalityCounterContract,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FinancialLocalityCaseDefinition {
    case: LocalityCaseContract,
    identity: LocalityIdentitySidecarContract,
    invariants: &'static [LocalityGeneratorInvariant],
    expectation_dimensions: &'static [LocalityExpectationDimension],
    red_mutations: &'static [mutations::LocalityRedMutation],
}

impl FinancialLocalityCaseDefinition {
    fn freeze(case: LocalityCaseContract, definition: &FinancialLocalityDefinition) -> Self {
        assert_eq!(definition.scale(), case.scale);
        assert_eq!(definition.workload().execution_posture(), case.lane);
        let scenario = definition.scenario();
        let exact_policy = exact_policy_contract(definition);
        Self {
            case,
            identity: LocalityIdentitySidecarContract {
                schema_version: LOCALITY_IDENTITY_SCHEMA_VERSION,
                seed: definition.seed(),
                scale: case.scale,
                mutation: mutation_contract(scenario),
                consumer_comparator: exact_policy,
                producer_output_equivalence: exact_policy,
                diagnostics: LocalityDiagnosticsContract::DevelopmentIdentitySidecar,
                execution_posture: case.lane,
                expected_semantic_work: LocalityExpectationContract::CompleteTraceManifestV1,
                expected_counters:
                    LocalityCounterContract::ExactQueriedCandidateCauseWorkEvaluationStopAndPeakRows,
            },
            invariants: generator_invariants(scenario),
            expectation_dimensions: &LocalityExpectationDimension::ALL,
            red_mutations: red_mutations(scenario),
        }
    }
}

fn exact_policy_contract(definition: &FinancialLocalityDefinition) -> LocalityComparatorContract {
    for output in definition.outputs() {
        let policy = output.execution_policy();
        assert_eq!(
            policy.dependency_comparison,
            FinancialLocalityComparisonPolicy::ExactEconomicRevision
        );
        assert_eq!(
            policy.output_equivalence,
            FinancialLocalityOutputPolicy::ExactEconomicRevision
        );
    }
    LocalityComparatorContract::ExactAspectVersion
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

#[test]
fn every_frozen_scale_case_has_generator_expectation_identity_and_mutation_contracts() {
    let cases = ordinary_locality_cases()
        .into_iter()
        .chain(scheduled_locality_cases());
    let mut observed = std::collections::BTreeSet::new();
    for case in cases {
        let world = FinancialWorldDefinition::locality_case(41, case);
        let frozen = FinancialLocalityCaseDefinition::freeze(
            case,
            world
                .locality()
                .expect("locality case must own locality meaning"),
        );
        observed.insert(frozen.case.scenario());
        assert_eq!(frozen.identity.schema_version, 1);
        assert_eq!(frozen.identity.seed, 41);
        assert_eq!(frozen.identity.scale, case.scale);
        assert_eq!(
            frozen.identity.mutation,
            mutation_contract(frozen.case.scenario())
        );
        assert_eq!(
            frozen.identity.consumer_comparator,
            LocalityComparatorContract::ExactAspectVersion
        );
        assert_eq!(
            frozen.identity.producer_output_equivalence,
            LocalityComparatorContract::ExactAspectVersion
        );
        assert_eq!(
            frozen.identity.diagnostics,
            LocalityDiagnosticsContract::DevelopmentIdentitySidecar
        );
        assert_eq!(frozen.identity.execution_posture, case.lane);
        assert_eq!(
            frozen.identity.expected_semantic_work,
            LocalityExpectationContract::CompleteTraceManifestV1
        );
        assert_eq!(
            frozen.identity.expected_counters,
            LocalityCounterContract::ExactQueriedCandidateCauseWorkEvaluationStopAndPeakRows
        );
        assert_eq!(frozen.expectation_dimensions.len(), 7);
        assert!(!frozen.invariants.is_empty());
        assert!(!frozen.red_mutations.is_empty());
    }
    assert_eq!(observed, FinancialLocalityScenario::ALL.into());
}
