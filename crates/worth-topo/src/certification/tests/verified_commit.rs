use super::*;
use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent, TopologyMutation};

#[test]
fn public_facade_exports_closeout_field_types() {
    fn _accepts_surface_types(
        _digest: DeterministicDigest,
        _coverage: PrimitiveCorpusCoverageMatrix,
        _parity: PrimitiveCorpusParityReport,
        _sweeps: AdmittedRangeSweepReport,
        _failures: FailureLocalityReport,
        _bridge_family_coverage: crate::facade::BridgeFamilyCoverageReport,
        _bridge: BridgeProofReport,
        _counters: MilestoneOneCounters,
    ) {
    }

    fn _closeout_fields_are_publicly_reachable(report: crate::facade::MilestoneOneCloseoutReport) {
        let _: AdmittedRangeSweepReport = report.admitted_range_sweep_report;
        let _: FailureLocalityReport = report.failure_locality_report;
        let _: crate::facade::BridgeFamilyCoverageReport = report.bridge_family_coverage_report;
        let _: BridgeProofReport = report.bridge_proof_report;
        let _: MilestoneOneCounters = report.milestone_1_counter_report;
    }

    fn _milestone_two_closeout_fields_are_publicly_reachable(report: MilestoneTwoCloseoutReport) {
        let _: DerivedValidatorCoverageReport = report.derived_validator_coverage_report;
        let _: DerivedInvalidationAggregateReport = report.derived_invalidation_report;
        let _: DerivedRebuildAggregateReport = report.derived_rebuild_report;
        let _: DerivedEquivalenceContractAggregateReport =
            report.derived_equivalence_contract_report;
        let _: DerivedFallbackAggregateReport = report.derived_fallback_report;
        let _: MilestoneTwoCounters = report.milestone_2_counter_report;
    }
}

#[test]
fn verified_topology_commit_is_the_canonical_certification_input() {
    let mut runtime = crate::validation::reference_integrity::milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();

    let seeded = seeded_bootstrap(&mut runtime, "cert-verified-commit").expect("seed  topology");
    let verified = crate::committed_artifact::TopologyCommittedArtifact::empty_on_main(
        seeded.snapshot,
        MutationOrigin::LocalEdit,
    );

    let report = certify_verified_topology_commit_traced(&mut runtime, &verified)
        .expect("verified commit certification should succeed")
        .into_primary_result();

    assert!(report.named_truth_validated);
    assert!(report.topology_validated);
    assert_eq!(
        report.read_artifact.snapshot,
        verified.persisted_truth().snapshot
    );
    assert_eq!(
        report.branch_local_topology_report.mutation_origin,
        MutationOrigin::LocalEdit
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
        ReplayParityStatus::NotChecked
    );
    assert!(verified.commits().is_empty());
}

#[test]
fn branch_local_verified_commit_certifies_against_the_feature_branch_truth_basis() {
    let mut runtime = crate::validation::reference_integrity::milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();

    let seeded = seeded_bootstrap(&mut runtime, "cert-branch-local").expect("seed  topology");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("feature branch");

    let verified = crate::committed_artifact::TopologyCommittedArtifact::empty_from_intent(
        seeded.snapshot,
        BranchId("feature".to_string()),
        RawTopologyIntent::new(
            Vec::<TopologyMutation>::new(),
            MutationOrigin::BranchLocalApplication,
        ),
    );

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
        ReplayParityStatus::NotChecked
    );
    assert!(verified.commits().is_empty());
}

#[test]
fn verified_commit_certification_runs_relational_replay_when_commit_exists() {
    let mut runtime = crate::validation::reference_integrity::milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();

    let verified = verified_primitive(
        &mut runtime,
        "replay-backed-certification",
        &MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
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
        ReplayParityStatus::Match
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
