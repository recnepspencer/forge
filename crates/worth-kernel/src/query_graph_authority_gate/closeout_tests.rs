use super::{
    closeout_report::{
        certify_worth_graph_authority_closeout,
        certify_worth_graph_authority_closeout_with_evidence,
    },
    current_worth_graph_authority_closeout_report, current_worth_graph_authority_gate_report,
    WorthGraphAuthorityCloseoutBypassClass, WorthGraphAuthorityCloseoutReport,
    WorthGraphAuthorityCloseoutViolation, WorthGraphAuthorityPublicFacadeEvidence,
    WorthGraphAuthorityPublicFacadeProof,
};

#[test]
fn closeout_report_covers_audited_sources_and_deletion_classes() {
    let report = current_worth_graph_authority_closeout_report()
        .expect("closeout report should certify the gate matrix");
    let _: &WorthGraphAuthorityCloseoutReport = &report;
    let counters = report.counters();

    assert_eq!(counters.inventory_matrix_rows(), report.matrix().len());
    assert_eq!(counters.deletion_target_classes(), 11);
    assert_eq!(counters.deleted_surfaces(), 2);
    assert_eq!(counters.collapsed_canonical_query_proofs(), 4);
    assert_eq!(counters.collapsed_split_ledger_receipts(), 1);
    assert_eq!(counters.collapsed_loop_ledger_receipts(), 1);
    assert_eq!(counters.certification_only_boundaries(), 1);
    assert_eq!(counters.explicit_residue_rows(), 3);
    assert_eq!(counters.query_capability_gaps(), 0);
    assert_eq!(counters.lower_authority_rejection_fixtures(), 8);
    assert_eq!(counters.rejected_bypass_classes(), 6);
    assert_eq!(counters.public_facade_proofs(), 4);
    assert_eq!(counters.deletion_line_removal_classes(), 11);
    assert_eq!(counters.deletion_removal_ledger_rows(), 13);
    assert!(counters.deletion_affected_source_files() >= 11);
    assert!(counters.deletion_affected_source_lines() > 0);
    assert!(counters.zero_silent_covered_lane_bypass());
}

#[test]
fn closeout_report_exposes_public_posture_without_raw_certifier() {
    let report = current_worth_graph_authority_closeout_report()
        .expect("closeout report should certify the gate matrix");

    for row in report.matrix() {
        assert!(!row.ordinary_public_facade().is_empty());
        assert!(!row
            .ordinary_public_facade()
            .contains("certify_worth_graph_authority"));
        assert!(!row.proof_boundary().is_empty());
        assert!(!row.disposition().label().is_empty());
        assert!(!row.public_facade_evidence().ordinary_api().is_empty());
        assert!(!row.public_facade_evidence().posture_accessor().is_empty());
        assert!(!row.public_facade_evidence().contract_test_path().is_empty());
    }
    for proof in WorthGraphAuthorityPublicFacadeProof::ALL {
        assert!(report
            .matrix()
            .iter()
            .any(|row| row.public_facade_evidence().proof() == proof));
    }
    for bypass_evidence in report.bypass_evidence() {
        assert!(!bypass_evidence.bypass_class().label().is_empty());
        assert!(!bypass_evidence.compile_fail_path().is_empty());
        assert!(!bypass_evidence.rejected_artifact().is_empty());
        assert!(!bypass_evidence.required_authority_type().is_empty());
    }
    for deletion_evidence in report.deletion_class_evidence() {
        assert!(deletion_evidence.removal_ledger_rows() > 0);
        assert!(deletion_evidence.affected_source_files() > 0);
        assert!(deletion_evidence.affected_source_lines() > 0);
    }
}

#[test]
fn closeout_rejects_stale_public_facade_contract_symbols() {
    let gate = current_worth_graph_authority_gate_report()
        .expect("gate report should certify before facade sabotage");
    let report = current_worth_graph_authority_closeout_report()
        .expect("closeout report should certify before facade sabotage");
    let mut matrix = report.matrix().to_vec();
    let sabotaged_source_id = matrix[0].source_id();
    matrix[0].public_facade_evidence = WorthGraphAuthorityPublicFacadeEvidence::new(
        matrix[0].public_facade_evidence().proof(),
        matrix[0].public_facade_evidence().ordinary_api(),
        matrix[0].public_facade_evidence().posture_accessor(),
        "crates/worth-kernel/src/query_graph_authority_gate/closeout_facade.rs",
        &["definitely_missing_closeout_facade_probe"],
    );

    let err = certify_worth_graph_authority_closeout_with_evidence(
        &gate,
        matrix,
        report.bypass_evidence().to_vec(),
        report.deletion_class_evidence().to_vec(),
        include_str!("../../../../_docs/worth/worth-query-graph-authority-hardening-closeout.md"),
    )
    .expect_err("stale public facade symbol evidence must fail closeout");

    assert!(matches!(
        err,
        WorthGraphAuthorityCloseoutViolation::PublicFacadeContractSymbolMissing {
            source_id,
            ..
        } if source_id == sabotaged_source_id
    ));
}

#[test]
fn closeout_rejects_private_public_facade_namespace() {
    let gate = current_worth_graph_authority_gate_report()
        .expect("gate report should certify before facade namespace sabotage");
    let report = current_worth_graph_authority_closeout_report()
        .expect("closeout report should certify before facade namespace sabotage");
    let mut matrix = report.matrix().to_vec();
    let topology_row = matrix
        .iter_mut()
        .find(|row| {
            row.public_facade_evidence().proof()
                == WorthGraphAuthorityPublicFacadeProof::TopologyOperatorQuerySurface
        })
        .expect("topology facade proof row should exist");
    let sabotaged_source_id = topology_row.source_id();
    topology_row.public_facade_evidence = WorthGraphAuthorityPublicFacadeEvidence::new(
        WorthGraphAuthorityPublicFacadeProof::TopologyOperatorQuerySurface,
        "worth_topo::topology_operators::topology_operator_graph_obligation_catalog",
        "TopologyOperatorGraphObligationCatalog::rows",
        "crates/worth-topo/src/certification/public_facade_contracts/contracts/public_api_topology_operator_surface.rs",
        &[
            "topology_operator_graph_obligation_catalog",
            "TopologyOperatorGraphObligationCatalog",
            "rows",
        ],
    );

    let err = certify_worth_graph_authority_closeout_with_evidence(
        &gate,
        matrix,
        report.bypass_evidence().to_vec(),
        report.deletion_class_evidence().to_vec(),
        include_str!("../../../../_docs/worth/worth-query-graph-authority-hardening-closeout.md"),
    )
    .expect_err("private topology namespace must fail closeout");

    assert!(matches!(
        err,
        WorthGraphAuthorityCloseoutViolation::PublicFacadeRootMismatch {
            source_id,
            expected_prefix: "worth_topo::facade::",
            ..
        } if source_id == sabotaged_source_id
    ));
}

#[test]
fn closeout_rejects_stale_public_facade_api_path_with_valid_terminal_symbol() {
    let gate = current_worth_graph_authority_gate_report()
        .expect("gate report should certify before stale facade path sabotage");
    let report = current_worth_graph_authority_closeout_report()
        .expect("closeout report should certify before stale facade path sabotage");
    let mut matrix = report.matrix().to_vec();
    let topology_row = matrix
        .iter_mut()
        .find(|row| {
            row.public_facade_evidence().proof()
                == WorthGraphAuthorityPublicFacadeProof::TopologyOperatorQuerySurface
        })
        .expect("topology facade proof row should exist");
    let sabotaged_source_id = topology_row.source_id();
    topology_row.public_facade_evidence = WorthGraphAuthorityPublicFacadeEvidence::new(
        WorthGraphAuthorityPublicFacadeProof::TopologyOperatorQuerySurface,
        "worth_topo::facade::stale::topology_operator_graph_obligation_catalog",
        "TopologyOperatorGraphObligationCatalog::rows",
        "crates/worth-topo/src/certification/public_facade_contracts/contracts/public_api_topology_operator_surface.rs",
        &[
            "topology_operator_graph_obligation_catalog",
            "TopologyOperatorGraphObligationCatalog",
            "rows",
        ],
    );

    let err = certify_worth_graph_authority_closeout_with_evidence(
        &gate,
        matrix,
        report.bypass_evidence().to_vec(),
        report.deletion_class_evidence().to_vec(),
        include_str!("../../../../_docs/worth/worth-query-graph-authority-hardening-closeout.md"),
    )
    .expect_err("same-terminal stale facade path must fail closeout");

    assert!(matches!(
        err,
        WorthGraphAuthorityCloseoutViolation::PublicFacadeApiMismatch {
            source_id,
            expected_api: "worth_topo::facade::topology_operator_graph_obligation_catalog",
            ..
        } if source_id == sabotaged_source_id
    ));
}

#[test]
fn closeout_rejects_missing_lower_authority_bypass_class() {
    let gate = current_worth_graph_authority_gate_report()
        .expect("gate report should certify before closeout sabotage");
    let report = current_worth_graph_authority_closeout_report()
        .expect("closeout report should certify before closeout sabotage");
    let mut bypass_evidence = report.bypass_evidence().to_vec();
    bypass_evidence.retain(|evidence| {
        evidence.bypass_class() != WorthGraphAuthorityCloseoutBypassClass::RawEvidenceVector
    });

    let err = certify_worth_graph_authority_closeout_with_evidence(
        &gate,
        report.matrix().to_vec(),
        bypass_evidence,
        report.deletion_class_evidence().to_vec(),
        include_str!("../../../../_docs/worth/worth-query-graph-authority-hardening-closeout.md"),
    )
    .expect_err("missing raw-vector rejection must fail closeout");

    assert_eq!(
        err,
        WorthGraphAuthorityCloseoutViolation::MissingBypassRejection(
            WorthGraphAuthorityCloseoutBypassClass::RawEvidenceVector,
        )
    );
}

#[test]
fn closeout_rejects_stale_residue_and_query_gap_doc() {
    let gate = current_worth_graph_authority_gate_report()
        .expect("gate report should certify before closeout doc sabotage");
    let stale_doc = "Explicit residue rows: 0\nQuery capability gaps: 0\n";

    let err = certify_worth_graph_authority_closeout(&gate, stale_doc)
        .expect_err("stale closeout doc must fail certification");

    assert_eq!(
        err,
        WorthGraphAuthorityCloseoutViolation::CloseoutDocMissingClaim("Audited sources covered")
    );
}

#[test]
fn closeout_rejects_stale_deletion_line_count_doc() {
    let gate = current_worth_graph_authority_gate_report()
        .expect("gate report should certify before closeout doc sabotage");
    let stale_doc =
        include_str!("../../../../_docs/worth/worth-query-graph-authority-hardening-closeout.md")
            .replace(
                "Deletion affected source lines: 49601",
                "Deletion affected source lines: 0",
            );

    let err = certify_worth_graph_authority_closeout(&gate, &stale_doc)
        .expect_err("stale deletion line count must fail certification");

    assert_eq!(
        err,
        WorthGraphAuthorityCloseoutViolation::CloseoutDocMissingClaim(
            "Deletion affected source lines"
        )
    );
}
