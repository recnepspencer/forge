use forge_store_test_support::harness::recovery::coverage as coverage_support;

use forge_store_physical_certification::{
    CoverageGapDenial, CoverageRowDimension, CoverageSurfaceKind, HarnessCoverageStage,
    PhysicalIsolationCompactionMutationKind,
};

#[test]
fn compaction_mutation_coverage_requires_all_physical_isolation_interleaving_mutants() {
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let evidence = coverage_support::compaction_mutation_evidence(&replay).unwrap();

    assert_eq!(
        evidence.sequence(),
        HarnessCoverageStage::SimulationAdmission
    );
    assert_eq!(evidence.plan_identity(), plan.identity().digest_bytes());
    let observed = evidence
        .compaction_mutations()
        .iter()
        .map(|row| row.kind())
        .collect::<Vec<_>>();
    assert_eq!(
        observed.as_slice(),
        PhysicalIsolationCompactionMutationKind::REQUIRED_FOR_S5_INTERLEAVING.as_slice()
    );
}

#[test]
fn compaction_mutation_coverage_rejects_replay_without_physical_isolation_interleaving_mutants() {
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle_without_compaction_mutations(&plan);

    assert_eq!(
        coverage_support::compaction_mutation_evidence(&replay).unwrap_err(),
        CoverageGapDenial::MissingMutationResult
    );
}

#[test]
fn compaction_mutation_observation_rejects_each_missing_physical_isolation_interleaving_mutant() {
    let plan = coverage_support::lowered_ci_plan();
    let schedule = coverage_support::schedule(&plan);
    for missing in PhysicalIsolationCompactionMutationKind::REQUIRED_FOR_S5_INTERLEAVING {
        let denial = coverage_support::compaction_mutation_lane_observation_set(
            &plan,
            &schedule,
            coverage_support::compaction_mutation_lanes_without(&plan, &schedule, missing).unwrap(),
        )
        .unwrap_err();
        assert_eq!(denial, CoverageGapDenial::MissingMutationResult);
    }
}

#[test]
fn compaction_mutation_lane_kind_is_derived_from_operation_receipt() {
    let plan = coverage_support::lowered_ci_plan();
    let schedule = coverage_support::schedule(&plan);
    let observed = coverage_support::complete_compaction_mutation_lanes(&plan, &schedule)
        .unwrap()
        .iter()
        .map(|lane| lane.kind())
        .collect::<Vec<_>>();

    assert_eq!(
        observed.as_slice(),
        PhysicalIsolationCompactionMutationKind::REQUIRED_FOR_S5_INTERLEAVING.as_slice()
    );
}

#[test]
fn compaction_mutation_observation_rejects_duplicate_physical_isolation_interleaving_mutant() {
    let plan = coverage_support::lowered_ci_plan();
    let schedule = coverage_support::schedule(&plan);
    let mut lanes = coverage_support::complete_compaction_mutation_lanes(&plan, &schedule).unwrap();
    lanes[1] = lanes[0].clone();

    assert_eq!(
        coverage_support::compaction_mutation_lane_observation_set(&plan, &schedule, lanes)
            .unwrap_err(),
        CoverageGapDenial::MissingMutationResult
    );
}

#[test]
fn compaction_mutation_observation_rejects_same_footprint_wrong_cutover_receipts() {
    let plan = coverage_support::lowered_ci_plan();
    let schedule = coverage_support::schedule(&plan);

    assert_eq!(
        coverage_support::compaction_mutation_lane_observation_set(
            &plan,
            &schedule,
            coverage_support::same_footprint_wrong_cutover_lanes(&plan, &schedule).unwrap(),
        )
        .unwrap_err(),
        CoverageGapDenial::MissingMutationResult
    );
}

#[test]
fn compaction_mutation_observation_rejects_unscheduled_lane_set() {
    let plan = coverage_support::lowered_ci_plan();

    assert_eq!(
        coverage_support::replay_bundle_with_unscheduled_compaction_mutations(&plan).unwrap_err(),
        CoverageGapDenial::MissingMutationResult
    );
}

#[test]
fn compaction_mutation_observation_rejects_detached_operation_receipts() {
    let plan = coverage_support::lowered_ci_plan();
    let schedule = coverage_support::schedule(&plan);

    assert_eq!(
        coverage_support::detached_compaction_mutation_lanes(&plan, &schedule).unwrap_err(),
        CoverageGapDenial::MissingMutationResult
    );
}

#[test]
fn coverage_matrix_names_compaction_mutation_dimensions() {
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let matrix = coverage_support::complete_registry(&plan, &replay)
        .generate_matrix()
        .unwrap();

    for kind in PhysicalIsolationCompactionMutationKind::REQUIRED_FOR_S5_INTERLEAVING {
        assert!(matrix.rows().iter().any(|row| {
            row.surface() == CoverageSurfaceKind::MutationResult
                && row.has_dimension(&CoverageRowDimension::CompactionMutation(kind))
        }));
    }
}
