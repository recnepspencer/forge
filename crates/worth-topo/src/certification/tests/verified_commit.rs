use super::*;
use worth_schema::facade::topology_authoring::{
    verify_topology_intent, verify_topology_intent_on_branch,
};

#[test]
fn public_facade_exports_closeout_field_types() {
    fn _accepts_surface_types(
        _digest: WorthDeterministicDigest,
        _coverage: WorthPrimitiveCorpusCoverageMatrix,
        _parity: WorthPrimitiveCorpusParityReport,
        _sweeps: WorthAdmittedRangeSweepReport,
        _failures: WorthFailureLocalityReport,
        _bridge_family_coverage: crate::facade::WorthBridgeFamilyCoverageReport,
        _bridge: WorthBridgeProofReport,
        _counters: WorthMilestoneOneCounters,
    ) {
    }

    fn _closeout_fields_are_publicly_reachable(
        report: crate::facade::WorthMilestoneOneCloseoutReport,
    ) {
        let _: WorthAdmittedRangeSweepReport = report.admitted_range_sweep_report;
        let _: WorthFailureLocalityReport = report.failure_locality_report;
        let _: crate::facade::WorthBridgeFamilyCoverageReport =
            report.bridge_family_coverage_report;
        let _: WorthBridgeProofReport = report.bridge_proof_report;
        let _: WorthMilestoneOneCounters = report.milestone_1_counter_report;
    }

    fn _milestone_two_closeout_fields_are_publicly_reachable(
        report: WorthMilestoneTwoCloseoutReport,
    ) {
        let _: WorthDerivedValidatorCoverageReport = report.derived_validator_coverage_report;
        let _: WorthDerivedInvalidationAggregateReport = report.derived_invalidation_report;
        let _: WorthDerivedRebuildAggregateReport = report.derived_rebuild_report;
        let _: WorthDerivedEquivalenceContractAggregateReport =
            report.derived_equivalence_contract_report;
        let _: WorthDerivedFallbackAggregateReport = report.derived_fallback_report;
        let _: WorthMilestoneTwoCounters = report.milestone_2_counter_report;
    }
}

#[test]
fn verified_topology_commit_is_the_canonical_certification_input() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();

    let _seeded =
        seeded_bootstrap(&mut runtime, "cert-verified-commit").expect("seed worth topology");
    let verified = verify_topology_intent(
        &mut runtime,
        RawWorthTopologyIntent::new(
            Vec::<WorthTopologyMutation>::new(),
            WorthMutationOrigin::LocalEdit,
        ),
    )
    .expect("verified topology commit");

    let report = certify_verified_topology_commit_traced(&mut runtime, &verified)
        .expect("verified commit certification should succeed")
        .into_primary_result();

    assert!(report.named_truth_validated);
    assert!(report.topology_validated);
    assert_eq!(
        report.read_artifact.snapshot,
        verified.persisted_truth.snapshot
    );
    assert_eq!(
        report.branch_local_topology_report.mutation_origin,
        WorthMutationOrigin::LocalEdit
    );
    assert_eq!(report.branch_local_topology_report.branch_id.0, "main");
    assert!(
        !report
            .milestone_1_replay_parity_report
            .relational_replay_checked
    );
    assert!(
        !report
            .milestone_1_replay_parity_report
            .relational_replay_verified
    );
    assert_eq!(
        report.milestone_1_replay_parity_report.parity_status,
        WorthReplayParityStatus::NotChecked
    );
    assert!(verified.commits.is_empty());
}

#[test]
fn branch_local_verified_commit_certifies_against_the_feature_branch_truth_basis() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();

    let _seeded = seeded_bootstrap(&mut runtime, "cert-branch-local").expect("seed worth topology");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("feature branch");

    let verified = verify_topology_intent_on_branch(
        &mut runtime,
        RawWorthTopologyIntent::new(
            Vec::<WorthTopologyMutation>::new(),
            WorthMutationOrigin::BranchLocalApplication,
        ),
        BranchId("feature".to_string()),
    )
    .expect("branch-local verified topology commit");

    let report = certify_verified_topology_commit_traced(&mut runtime, &verified)
        .expect("branch-local certification should succeed")
        .into_primary_result();

    assert!(report.named_truth_validated);
    assert!(report.topology_validated);
    assert!(report.branch_local_topology_report.branch_local);
    assert_eq!(report.branch_local_topology_report.branch_id.0, "feature");
    assert_eq!(
        report.milestone_1_replay_parity_report.branch_id.0,
        "feature"
    );
    assert!(
        !report
            .milestone_1_replay_parity_report
            .relational_replay_checked
    );
    assert!(
        !report
            .milestone_1_replay_parity_report
            .relational_replay_verified
    );
    assert_eq!(
        report.milestone_1_replay_parity_report.parity_status,
        WorthReplayParityStatus::NotChecked
    );
    assert!(verified.commits.is_empty());
}

#[test]
fn verified_commit_certification_runs_relational_replay_when_commit_exists() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();

    let verified = verified_primitive(
        &mut runtime,
        "replay-backed-certification",
        &WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("verified admitted primitive commit");

    let report = certify_verified_topology_commit_traced(&mut runtime, &verified)
        .expect("verified commit certification should succeed")
        .into_primary_result();

    assert!(
        report
            .milestone_1_replay_parity_report
            .relational_replay_checked
    );
    assert!(
        report
            .milestone_1_replay_parity_report
            .relational_replay_verified
    );
    assert_eq!(
        report.milestone_1_replay_parity_report.parity_status,
        WorthReplayParityStatus::Match
    );
    assert!(report
        .milestone_1_replay_parity_report
        .replayed_commit_id
        .is_some());
    assert_eq!(report.milestone_1_replay_parity_report.mismatch_count, 0);
    assert!(report
        .milestone_1_replay_parity_report
        .replay_failure
        .is_none());
}
