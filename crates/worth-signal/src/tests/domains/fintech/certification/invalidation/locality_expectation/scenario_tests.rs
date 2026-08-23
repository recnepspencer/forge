use super::recovery_assertions::assert_restore_checkpoint_authority;
use super::*;
mod normalization;
use crate::tests::domains::fintech::world::{
    ordinary_locality_cases, scheduled_locality_cases, DensityRatio, FinancialLocalityAction,
    FinancialLocalityDefinition, FinancialLocalityScenario, FinancialLocalityTraceIdentity,
    FinancialWorldDefinition, LocalityCaseContract, LocalityScaleTuple,
};
use normalization::{normalized_cause_shape, normalized_work_shape};

#[test]
fn every_scenario_has_constructible_ordinary_and_scheduled_complete_trace_manifests() {
    for cases in [ordinary_locality_cases(), scheduled_locality_cases()] {
        for scenario in FinancialLocalityScenario::ALL {
            let case = representative_case(&cases, scenario);
            let world = FinancialWorldDefinition::locality_case(41, case);
            let definition = world.locality().unwrap();
            definition.validate_generator_invariants();
            let manifest = FinancialLocalityExpectationManifest::derive(definition, 73);

            assert_eq!(manifest.scenario(), scenario);
            assert_eq!(definition.workload().execution_posture(), case.lane);
            assert!(!manifest.canonical_work().is_empty());
            assert!(!manifest.necessary_evaluations().is_empty());
            assert!(manifest.peak_ready_width() > 0);
            assert_eq!(
                manifest.counter_manifest().rows().len(),
                ExpectedLocalityCounterRow::ALL.len()
            );
            assert_candidate_partition(&manifest);
            assert_ready_counter_conservation(&manifest);
            assert_current_cause_keys_are_unique(&manifest);
            for (work, origins) in manifest.canonical_work() {
                assert_eq!(work.graph.graph_instance, 73, "{scenario:?}");
                assert_eq!(work.graph.seed, 41, "{scenario:?}");
                assert_eq!(work.graph.scale, case.scale, "{scenario:?}");
                assert_ne!(work.readiness_epoch, 0, "{scenario:?}");
                assert!(
                    origins.iter().all(origin_binding_matches),
                    "{scenario:?} has an incomplete origin binding: {origins:?}"
                );
            }
            assert_scenario_contract(definition, &manifest);
        }
    }
}

#[test]
fn convergent_permutations_and_retries_are_real_ordered_action_traces() {
    let case = scheduled_locality_cases()
        .into_iter()
        .find(|case| case.scenario() == FinancialLocalityScenario::ConvergentFactorBatch)
        .unwrap();
    let world = FinancialWorldDefinition::locality_case(41, case);
    let definition = world.locality().unwrap();
    let mut orders = BTreeSet::new();
    let mut canonical_cause_shape = None;
    let mut canonical_work_shape = None;

    assert_eq!(definition.action_traces().len(), 24);
    for trace in definition.action_traces() {
        let order = trace
            .actions()
            .iter()
            .filter_map(|action| match action {
                FinancialLocalityAction::CommitFactor(mutation) => Some(mutation.producer),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(order.len(), 4);
        assert!(orders.insert(order));
        let manifest =
            FinancialLocalityExpectationManifest::derive_for_trace(definition, trace, 73);
        assert_eq!(manifest.action_trace(), trace.identity());
        assert_eq!(manifest.duplicate_admission_attempts(), 8);
        assert_eq!(
            counter(&manifest, ExpectedLocalityCounterRow::WorkItemsMerged),
            8
        );
        assert_eq!(
            counter(&manifest, ExpectedLocalityCounterRow::ReadyItemsEnqueued),
            5
        );
        let cause_shape = normalized_cause_shape(&manifest);
        let work_shape = normalized_work_shape(&manifest);
        if let Some(expected) = &canonical_cause_shape {
            assert_eq!(&cause_shape, expected);
            assert_eq!(&work_shape, canonical_work_shape.as_ref().unwrap());
        } else {
            canonical_cause_shape = Some(cause_shape);
            canonical_work_shape = Some(work_shape);
        }
    }
    assert_eq!(orders.len(), 24);
}

#[test]
fn partition_twins_freeze_detail_whole_and_correlated_scope_truth() {
    let world = FinancialWorldDefinition::partitioned_curve_universe(41, 16, 4, 8);
    let definition = world.locality().unwrap();
    let primary =
        manifest_for_identity(definition, FinancialLocalityTraceIdentity::PrimaryMutation);
    let whole = manifest_for_identity(
        definition,
        FinancialLocalityTraceIdentity::PartitionWholeRegion,
    );
    let correlated = manifest_for_identity(
        definition,
        FinancialLocalityTraceIdentity::PartitionCorrelatedScopes,
    );

    assert_eq!(
        source_curve_scopes(&primary),
        [
            None,
            Some(LocalityScope::partition(0)),
            Some(LocalityScope::detail(0, 0))
        ]
        .into()
    );
    assert_eq!(
        source_curve_scopes(&whole),
        [
            None,
            Some(LocalityScope::partition(0)),
            Some(LocalityScope::detail(0, 0)),
            Some(LocalityScope::detail(0, 1)),
        ]
        .into()
    );
    let pairs = correlated
        .canonical_causes()
        .iter()
        .filter(|cause| {
            matches!(
                cause.aspect,
                FinancialAspect::Price | FinancialAspect::Volatility
            )
        })
        .map(|cause| (cause.aspect, cause.changed_scopes.clone()))
        .collect::<BTreeSet<_>>();
    assert!(pairs.contains(&(FinancialAspect::Price, vec![LocalityScope::detail(500, 1)])));
    assert!(pairs.contains(&(
        FinancialAspect::Volatility,
        vec![LocalityScope::detail(501, 2)]
    )));
}

fn assert_candidate_partition(manifest: &FinancialLocalityExpectationManifest) {
    let counters = manifest.counter_manifest();
    assert_eq!(
        counters.value(ExpectedLocalityCounterRow::ReverseIndexCandidatesReturned),
        counters.value(ExpectedLocalityCounterRow::DirectSettlementsProduced)
            + counters.value(ExpectedLocalityCounterRow::CandidatesRejectedByAspectContract)
            + counters.value(ExpectedLocalityCounterRow::CandidatesRejectedByScope)
            + counters.value(ExpectedLocalityCounterRow::CandidatesRejectedByComparator)
            + manifest.causality_rejections()
    );
}

fn assert_ready_counter_conservation(manifest: &FinancialLocalityExpectationManifest) {
    let admitted = counter(manifest, ExpectedLocalityCounterRow::WorkItemsAdmitted);
    let merged = counter(manifest, ExpectedLocalityCounterRow::WorkItemsMerged);
    let enqueued = counter(manifest, ExpectedLocalityCounterRow::ReadyItemsEnqueued);
    let popped = counter(manifest, ExpectedLocalityCounterRow::ReadyItemsPopped);
    let retained = counter(
        manifest,
        ExpectedLocalityCounterRow::RetainedReadyFrontierWidth,
    );
    let maximum = counter(
        manifest,
        ExpectedLocalityCounterRow::MaximumReadyFrontierWidth,
    );
    assert_eq!(admitted, merged + enqueued);
    assert_eq!(enqueued, popped + retained);
    assert!(retained <= maximum);
}

fn assert_current_cause_keys_are_unique(manifest: &FinancialLocalityExpectationManifest) {
    let keys = manifest
        .canonical_causes()
        .iter()
        .map(|cause| {
            (
                cause.consumer,
                cause.producer,
                cause.aspect,
                cause.edge_scope,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        keys.iter().copied().collect::<BTreeSet<_>>().len(),
        keys.len()
    );
}

fn manifest_for_identity(
    definition: &FinancialLocalityDefinition,
    identity: FinancialLocalityTraceIdentity,
) -> FinancialLocalityExpectationManifest {
    let trace = definition
        .action_traces()
        .iter()
        .find(|trace| trace.identity() == identity)
        .unwrap();
    FinancialLocalityExpectationManifest::derive_for_trace(definition, trace, 73)
}

fn source_curve_scopes(
    manifest: &FinancialLocalityExpectationManifest,
) -> BTreeSet<Option<LocalityScope>> {
    manifest
        .queried_bucket_keys()
        .iter()
        .filter(|key| {
            key.producer == LocalitySemanticOutputId::new(0) && key.aspect == FinancialAspect::Curve
        })
        .map(|key| key.scope)
        .collect()
}

fn representative_case(
    cases: &[LocalityCaseContract],
    scenario: FinancialLocalityScenario,
) -> LocalityCaseContract {
    *cases
        .iter()
        .find(|case| case.scenario() == scenario)
        .expect("each lane freezes every locality scenario")
}

fn origin_binding_matches(origin: &ExpectedSealedOriginBinding) -> bool {
    match origin {
        ExpectedSealedOriginBinding::SourceRecompute {
            admission_generation,
        } => *admission_generation > 0,
        ExpectedSealedOriginBinding::DependencyCommit {
            producer_commit_ordinals,
            ..
        } => !producer_commit_ordinals.is_empty(),
        ExpectedSealedOriginBinding::StructuralRecompute {
            structural_generation,
        } => *structural_generation > 0,
    }
}

fn assert_scenario_contract(
    definition: &FinancialLocalityDefinition,
    manifest: &FinancialLocalityExpectationManifest,
) {
    match definition.scale() {
        LocalityScaleTuple::ConvergentFactorBatch { .. } => {
            assert_eq!(definition.mutations().len(), 4);
            assert_eq!(manifest.canonical_causes().len(), 4);
            assert_eq!(manifest.canonical_work().len(), 5);
            assert_eq!(manifest.peak_ready_width(), 1);
            assert!(manifest.canonical_work().values().any(|origins| {
                origins.iter().any(|origin| {
                    matches!(
                        origin,
                        ExpectedSealedOriginBinding::DependencyCommit {
                            producer_commit_ordinals,
                            ..
                        } if producer_commit_ordinals.len() == 4
                    )
                })
            }));
        }
        LocalityScaleTuple::DenseMarketClose {
            total_outputs,
            affected_ratio,
        } => {
            let expected = match affected_ratio {
                DensityRatio::OneInOneHundred => total_outputs / 100,
                DensityRatio::OneInFour => total_outputs / 4,
                DensityRatio::FourInFive => total_outputs / 5 * 4,
            };
            assert_eq!(manifest.necessary_evaluations().len(), expected as usize);
        }
        LocalityScaleTuple::PortfolioDependencyChurn { rounds, .. } => {
            assert_churn_contract(manifest, rounds)
        }
        LocalityScaleTuple::BranchRestoreLocalityReplay { .. } => {
            assert!(manifest
                .canonical_work()
                .keys()
                .all(|work| work.readiness_epoch == 2));
            assert_restore_checkpoint_authority(manifest);
        }
        LocalityScaleTuple::SparseBookFanout { .. }
        | LocalityScaleTuple::PartitionedCurveUniverse { .. } => {}
    }
}

fn assert_churn_contract(manifest: &FinancialLocalityExpectationManifest, rounds: u16) {
    let final_revision = u64::from(rounds) * 3 + 1;
    let valuation = LocalitySemanticOutputId::new(2);
    let valuation_work = manifest
        .canonical_work()
        .iter()
        .filter(|(work, _)| work.target == valuation)
        .collect::<Vec<_>>();
    assert_eq!(valuation_work.len(), 1);
    assert_eq!(valuation_work[0].0.dependency_revision, final_revision);
    assert!(valuation_work[0].1.iter().any(|origin| matches!(
        origin,
        ExpectedSealedOriginBinding::StructuralRecompute { .. }
    )));
    assert!(!valuation_work[0]
        .1
        .iter()
        .any(|origin| matches!(origin, ExpectedSealedOriginBinding::DependencyCommit { .. })));
    assert!(manifest
        .canonical_causes()
        .iter()
        .filter(|cause| cause.consumer == valuation)
        .all(|cause| cause.dependency_revision == final_revision));
    assert_eq!(
        counter(manifest, ExpectedLocalityCounterRow::StaleWorkRejected),
        u64::from(rounds)
    );
    assert_eq!(
        counter(
            manifest,
            ExpectedLocalityCounterRow::TopologyRevisionRevalidations
        ),
        u64::from(rounds) * 3
    );
    assert_eq!(
        counter(
            manifest,
            ExpectedLocalityCounterRow::RejectedTopologyMutations
        ),
        u64::from(rounds)
    );
    assert_eq!(manifest.action_checkpoints().len(), usize::from(rounds) * 6);
    assert!(manifest
        .committed_output_ordinals()
        .windows(2)
        .all(|pair| pair[0] < pair[1]));
    let accepted = manifest
        .action_checkpoints()
        .iter()
        .filter_map(|checkpoint| match checkpoint.kind {
            ExpectedActionCheckpointKind::TopologyAccepted(structural) => {
                Some(structural.topology_mutation_ordinal)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(accepted, (1..=u64::from(rounds) * 3).collect::<Vec<_>>());
    for checkpoint in manifest.action_checkpoints() {
        if let ExpectedActionCheckpointKind::CycleRejected {
            attempted_topology_ordinal,
            ..
        } = checkpoint.kind
        {
            let prior = accepted
                .iter()
                .copied()
                .filter(|ordinal| *ordinal < attempted_topology_ordinal)
                .max()
                .unwrap();
            assert_eq!(attempted_topology_ordinal, prior + 1);
        }
    }
}

fn counter(
    manifest: &FinancialLocalityExpectationManifest,
    row: ExpectedLocalityCounterRow,
) -> u64 {
    manifest.counter_manifest().value(row)
}
