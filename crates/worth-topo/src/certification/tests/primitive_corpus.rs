use super::*;
use crate::facade::TopologyBranchAuthoringBoundary;

#[test]
fn primitive_corpus_certification_runs_cases_through_authority_and_reports_family_coverage() {
    let corpus = certify_milestone_one_primitive_corpus(
        || {
            crate::validation::reference_integrity::milestone_one_runtime_builder()
                .expect(" milestone one runtime builder")
                .build()
        },
        "cert-corpus",
        &[
            MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
            MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 4 },
            MilestoneOnePrimitiveCase::WireBranch { branch_count: 4 },
            MilestoneOnePrimitiveCase::SheetDisk { edge_count: 5 },
            MilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
            MilestoneOnePrimitiveCase::SolidShell { face_count: 4 },
            MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
        ],
    )
    .expect("primitive corpus certification should succeed");

    assert_eq!(corpus.cases.len(), 7);
    assert!(corpus
        .cases
        .iter()
        .all(|case| case.certification.named_truth_validated));
    assert!(corpus
        .cases
        .iter()
        .all(|case| case.certification.topology_validated));
    assert_eq!(corpus.coverage_matrix.entries.len(), 7);
    assert!(corpus.coverage_matrix.entries.iter().all(|entry| {
        entry.admitted_generic_count == 1
            && entry.admitted_smallest_count == 0
            && entry.admitted_hostile_count == 0
            && entry.rejected_out_of_class_count == 0
            && !entry.role_closure_complete
    }));
    assert_eq!(corpus.parity_report.entries.len(), 7);
    assert!(corpus.parity_report.entries.iter().all(|entry| {
        entry.mainline_case_count == 1
            && entry.branch_local_case_count == 0
            && entry.mainline_replay_checked_case_count == 1
            && entry.mainline_replay_verified_case_count == 1
            && entry.mainline_digest_parity_case_count == 1
            && entry.branch_local_replay_checked_case_count == 0
            && entry.branch_local_replay_verified_case_count == 0
            && entry.branch_local_digest_parity_case_count == 0
            && entry.cross_branch_parity_case_count == 0
            && !entry.parity_closure_complete
    }));
    assert!(corpus.cases.iter().any(|case| {
        matches!(case.primitive, MilestoneOnePrimitiveCase::WireOpen { .. })
            && case
                .certification
                .primitive_family_coverage_matrix
                .entries
                .iter()
                .any(|entry| entry.family == "WireOpen(n)" && entry.observed)
    }));
    assert!(corpus.cases.iter().any(|case| {
        matches!(case.primitive, MilestoneOnePrimitiveCase::SolidShell { .. })
            && case
                .certification
                .primitive_family_coverage_matrix
                .entries
                .iter()
                .any(|entry| entry.family == "SolidShell(f)" && entry.observed)
    }));
    assert!(corpus.cases.iter().any(|case| {
        matches!(case.primitive, MilestoneOnePrimitiveCase::NmtEdgeFan { .. })
            && case
                .certification
                .primitive_family_coverage_matrix
                .entries
                .iter()
                .any(|entry| entry.family == "NmtEdgeFan(k)" && entry.observed)
    }));
}

#[test]
fn primitive_corpus_reports_keep_the_full_canonical_family_set_even_when_input_is_partial() {
    let corpus = certify_milestone_one_primitive_corpus(
        || {
            crate::validation::reference_integrity::milestone_one_runtime_builder()
                .expect(" milestone one runtime builder")
                .build()
        },
        "cert-partial-corpus",
        &[MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 }],
    )
    .expect("partial primitive corpus certification should succeed");

    assert_eq!(corpus.coverage_matrix.entries.len(), 7);
    assert_eq!(corpus.parity_report.entries.len(), 7);
    assert!(corpus
        .coverage_matrix
        .entries
        .iter()
        .any(|entry| entry.family == "WireOpen(n)" && entry.admitted_generic_count == 1));
    assert!(corpus
        .coverage_matrix
        .entries
        .iter()
        .any(|entry| entry.family == "SolidShell(f)" && !entry.role_closure_complete));
    assert!(corpus
        .parity_report
        .entries
        .iter()
        .any(|entry| entry.family == "NmtEdgeFan(k)" && !entry.parity_closure_complete));
}

#[test]
fn default_primitive_corpus_includes_smallest_generic_hostile_and_out_of_class_members() {
    let corpus = certify_milestone_one_default_primitive_corpus(
        || {
            crate::validation::reference_integrity::milestone_one_runtime_builder()
                .expect(" milestone one runtime builder")
                .build()
        },
        "cert-default-corpus",
    )
    .expect("default primitive corpus certification should succeed");

    assert!(corpus
        .cases
        .iter()
        .any(|case| case.role == MilestoneOnePrimitiveRole::Smallest));
    assert!(corpus
        .cases
        .iter()
        .any(|case| case.role == MilestoneOnePrimitiveRole::Generic));
    assert!(corpus
        .cases
        .iter()
        .any(|case| case.role == MilestoneOnePrimitiveRole::HostileAdmitted));
    assert!(corpus
        .rejected_cases
        .iter()
        .all(|case| case.expected_outcome == MilestoneOnePrimitiveExpectedOutcome::Reject));
    assert!(corpus
        .rejected_cases
        .iter()
        .all(|case| !case.rejection.detail.is_empty()));
    assert!(corpus.rejected_cases.iter().all(|case| {
        matches!(
            case.rejection.rejection_class.as_str(),
            "OutOfClass" | "IllegalAdmittedTopology" | "AuthorityBlocked" | "InvariantFailure"
        )
    }));
    assert!(corpus
        .rejected_cases
        .iter()
        .any(|case| case.role == MilestoneOnePrimitiveRole::OutOfClass));
    assert!(corpus
        .rejected_cases
        .iter()
        .any(|case| case.family == "WireClosed(n)"));
    assert!(corpus.coverage_matrix.entries.iter().all(|entry| {
        entry.admitted_smallest_count >= 1
            && entry.admitted_generic_count >= 1
            && entry.admitted_hostile_count >= 1
            && entry.rejected_out_of_class_count >= 1
            && entry.role_closure_complete
    }));
    assert!(corpus.parity_report.entries.iter().all(|entry| {
        entry.mainline_case_count >= 3
            && entry.branch_local_case_count == entry.mainline_case_count
            && entry.mainline_replay_checked_case_count == entry.mainline_case_count
            && entry.mainline_replay_verified_case_count == entry.mainline_case_count
            && entry.mainline_digest_parity_case_count == entry.mainline_case_count
            && entry.branch_local_replay_checked_case_count == entry.branch_local_case_count
            && entry.branch_local_replay_verified_case_count == entry.branch_local_case_count
            && entry.branch_local_digest_parity_case_count == entry.branch_local_case_count
            && entry.cross_branch_parity_case_count == entry.mainline_case_count
            && entry.parity_closure_complete
    }));
    assert!(corpus
        .parity_report
        .entries
        .iter()
        .all(|entry| entry.branch_ids.iter().any(|branch| branch == "main")));
    assert!(corpus
        .parity_report
        .entries
        .iter()
        .all(|entry| entry.branch_ids.iter().any(|branch| branch == "feature")));
}

#[test]
fn branch_local_default_primitive_corpus_preserves_branch_local_reports_for_admitted_cases() {
    let scenarios = milestone_one_default_branch_local_admitted_scenarios();

    let corpus = certify_milestone_one_branch_local_primitive_scenarios(
        &mut || {
            crate::validation::reference_integrity::milestone_one_runtime_builder()
                .expect(" milestone one runtime builder")
                .build()
        },
        "cert-branch-corpus",
        "feature",
        &scenarios,
    )
    .expect("branch-local primitive corpus certification should succeed");

    assert!(!corpus.cases.is_empty());
    assert!(corpus.rejected_cases.is_empty());
    assert!(corpus
        .cases
        .iter()
        .all(|case| case.certification.branch_local_topology_report.branch_local));
    assert!(corpus.cases.iter().all(|case| {
        case.certification
            .branch_local_topology_report
            .branch_authoring_boundary
            == Some(TopologyBranchAuthoringBoundary::SchemaTopologyAuthoring)
    }));
    assert!(corpus.cases.iter().all(|case| {
        case.certification.branch_local_topology_report.branch_id.0 == "feature"
            && case
                .certification
                .milestone_1_replay_parity_report
                .branch_id
                .0
                == "feature"
    }));
    assert!(corpus.coverage_matrix.entries.iter().all(|entry| {
        entry.admitted_smallest_count >= 1
            && entry.admitted_generic_count >= 1
            && entry.admitted_hostile_count >= 1
            && entry.rejected_out_of_class_count == 0
            && !entry.role_closure_complete
    }));
    assert!(corpus.parity_report.entries.iter().all(|entry| {
        entry.mainline_case_count >= 3
            && entry.branch_local_case_count == entry.mainline_case_count
            && entry.mainline_replay_checked_case_count == entry.mainline_case_count
            && entry.mainline_replay_verified_case_count == entry.mainline_case_count
            && entry.mainline_digest_parity_case_count == entry.mainline_case_count
            && entry.branch_local_replay_checked_case_count == entry.branch_local_case_count
            && entry.branch_local_replay_verified_case_count == entry.branch_local_case_count
            && entry.branch_local_digest_parity_case_count == entry.branch_local_case_count
            && entry.cross_branch_parity_case_count == 0
            && entry.parity_closure_complete
    }));
}
