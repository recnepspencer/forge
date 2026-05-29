use super::*;
use crate::certification::support::parity::build_derived_equivalence_contract;
use crate::facade::build_derived_read_diagnostics;
use crate::projection::runtime_boundary::read_stage::stage_topology_read_from_view;

#[test]
fn seeded_bootstrap_earns_milestone_one_certification_report() {
    let mut runtime = crate::validation::reference_integrity::milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();

    let seeded = seeded_bootstrap(&mut runtime, "cert-harness").expect("seed  topology");
    let report = certify_milestone_one_read_basis_traced(&mut runtime, seeded.read_basis().clone())
        .expect("milestone one certification should succeed")
        .into_primary_result();

    assert!(report.named_truth_validated);
    assert!(report.topology_validated);
    assert_eq!(report.topology_truth_digest.algorithm, "fnv1a64");
    assert!(report.topology_truth_digest.row_count > 0);
    assert_eq!(report.naming_truth_digest.algorithm, "fnv1a64");
    assert_eq!(report.topology_validation_digest.algorithm, "fnv1a64");
    assert_eq!(report.topology_validation_report.rows.len(), 5);
    assert!(report
        .topology_validation_report
        .rows
        .iter()
        .any(|row| row.validator == "ownership" && row.status == "passed"));
    assert!(report.naming_attachment_report.fully_named);
    assert_eq!(
        report.branch_local_topology_report.mutation_origin,
        schema::facade::platform::authority::MutationOrigin::Seed
    );
    assert!(!report.branch_local_topology_report.branch_local);
    assert_eq!(report.branch_local_topology_report.branch_id.0, "main");
    assert_eq!(
        report.milestone_1_replay_parity_report.parity_status,
        ReplayParityStatus::NotChecked
    );
    assert_eq!(report.milestone_1_replay_parity_report.branch_id.0, "main");
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
    assert!(report
        .milestone_1_replay_parity_report
        .replayed_commit_id
        .is_none());
    assert_eq!(report.milestone_1_replay_parity_report.mismatch_count, 0);
    assert!(report
        .milestone_1_replay_parity_report
        .replay_failure
        .is_none());
    assert!(
        report
            .milestone_1_replay_parity_report
            .interpretation_digest_match
    );
    assert!(report.milestone_1_replay_parity_report.truth_digest_match);
    assert!(
        report
            .milestone_1_replay_parity_report
            .validation_digest_match
    );
    assert_eq!(report.counters.topology_entity_upsert_count, 0);
    assert_eq!(report.counters.topology_relation_upsert_count, 0);
    assert_eq!(report.counters.commit_boundary_validator_count, 6);
    assert_eq!(report.counters.naming_target_lookup_count, 11);
    assert_eq!(report.read_artifact.snapshot, seeded.snapshot);
    assert_eq!(report.read_artifact.interpretations.wires.len(), 1);
    assert_eq!(report.read_artifact.interpretations.shells.len(), 1);
    assert_eq!(
        report.read_artifact.interpretations.wires[0].class,
        WireInterpretationClass::OpenChain
    );
    assert_eq!(
        report.read_artifact.interpretations.shells[0].class,
        ShellInterpretationClass::OpenSheet
    );
    assert_eq!(
        report.certified_interpretation.interpretations,
        report.read_artifact.interpretations
    );
    assert!(report
        .primitive_family_coverage_matrix
        .entries
        .iter()
        .any(|entry| entry.family == "WireOpen(n)" && entry.observed));
    assert!(report
        .primitive_family_coverage_matrix
        .entries
        .iter()
        .any(|entry| entry.family == "SheetDisk(n)" && entry.observed));
}

#[test]
fn seeded_bootstrap_earns_direct_milestone_two_read_report() {
    let mut runtime = crate::validation::reference_integrity::milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();

    let seeded = seeded_bootstrap(&mut runtime, "cert-m2-read").expect("seed  topology");
    let report = certify_milestone_two_read_basis_traced(&mut runtime, seeded.read_basis().clone())
        .expect("milestone two read certification should succeed")
        .into_primary_result();

    assert_eq!(report.materialized_topology_digest.algorithm, "fnv1a64");
    assert_eq!(report.interpreted_topology_digest.algorithm, "fnv1a64");
    assert_eq!(report.derived_validation_digest.algorithm, "fnv1a64");
    assert!(report.derived_invalidation_report.topology_touched);
    assert!(report.derived_rebuild_report.whole_view_rebuild);
    assert!(report.derived_fallback_report.whole_view_materialization);
    assert_eq!(report.milestone_2_counter_report.derived_read_count, 1);
    assert_eq!(
        report.read_artifact.interpretations,
        report.certified_interpretation.interpretations
    );
}

#[test]
fn certification_read_view_matches_traced_reader_diagnostics_on_same_basis() {
    let mut runtime = crate::validation::reference_integrity::milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();

    let seeded = seeded_bootstrap(&mut runtime, "cert-reader-parity").expect("seed  topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect(" snapshot read");

    let report = certify_milestone_one_read_basis_traced(&mut runtime, seeded.read_basis().clone())
        .expect("milestone one certification should succeed")
        .into_primary_result();
    let staged = stage_topology_read_from_view(&read_view).expect("read stage should succeed");
    let traced_diagnostics = build_derived_read_diagnostics(
        &seeded.read_basis(),
        staged.materialized(),
        staged.interpreted(),
        staged.validation(),
    );
    let traced_equivalence = build_derived_equivalence_contract(
        &seeded.read_basis(),
        staged.materialized(),
        staged.interpreted(),
        staged.validation(),
    );

    assert_eq!(
        report.derived_read_diagnostics.invalidation_report,
        traced_diagnostics.invalidation_report
    );
    assert_eq!(
        report.derived_read_diagnostics.rebuild_report,
        traced_diagnostics.rebuild_report
    );
    assert_eq!(
        report.derived_read_diagnostics.fallback_report,
        traced_diagnostics.fallback_report
    );
    assert_eq!(
        report
            .derived_equivalence_contract_report
            .truth_basis_digest_hex,
        traced_equivalence.truth_basis_digest_hex
    );
}




