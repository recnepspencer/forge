use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use worth_query::facade::consumer_kit::{
    query_consumer_residue_audit, query_test_backend_residue_audit,
    worth_query_consumer_residue_registry, worth_query_test_backend_residue_classes,
    WorthQueryConsumerResidueClass, WorthQueryConsumerResidueDetection,
    WorthQueryConsumerResidueReport,
};
use worth_query::WorthQueryEvidenceScope;

#[path = "consumer_residue_audit_support/fixtures.rs"]
mod consumer_residue_audit_fixtures;
use consumer_residue_audit_fixtures::{
    CLEAN_SOURCE, DEBUG_BINDING_SOURCE, FALSE_POSITIVE_CASES, HOSTILE_CLASS_CASES,
    LOCAL_QUERY_REPORT_SOURCE, RUNTIME_BRIDGE_SOURCE, SYNTAX_ROLE_CASES,
};

static WORKSPACE_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[test]
fn each_residue_class_has_an_independent_hostile_fixture() {
    for case in HOSTILE_CLASS_CASES {
        let workspace = residue_workspace(case.label, case.source);
        let report = query_consumer_residue_audit("downstream")
            .required_root(&workspace)
            .evaluate()
            .expect("hostile class fixture must parse");
        let finding = single_finding_for(&report, case.class);

        assert_eq!(finding.detection_key(), case.detection_key);
        assert_eq!(finding.replacement_lane(), case.replacement_lane);
        assert_eq!(
            finding.line(),
            expected_line(case.source, case.line_needle),
            "line should identify the hostile site for {}",
            case.label
        );
        assert!(finding.column() > 0);
        assert_eq!(
            report.finding_identities()[0].scope(),
            WorthQueryEvidenceScope::ConsumerResidueFinding
        );
        assert_eq!(
            report.report_identity().scope(),
            WorthQueryEvidenceScope::ConsumerResidueReport
        );
        assert_eq!(
            report.audited_source_paths().len(),
            report.scanned_file_count()
        );
        assert!(!report.source_inventory_digest().is_empty());
    }
}

#[test]
fn syntax_roles_detect_proof_folklore_without_source_substring_luck() {
    for case in SYNTAX_ROLE_CASES {
        let workspace = residue_workspace(case.label, case.source);
        let report = query_consumer_residue_audit("downstream")
            .required_root(&workspace)
            .evaluate()
            .expect("syntax role fixture must parse");

        let finding = single_finding_for(&report, case.class);
        assert_eq!(finding.detection_key(), case.detection_key);
        assert_eq!(finding.replacement_lane(), case.replacement_lane);
        assert_eq!(finding.line(), expected_line(case.source, case.line_needle));
    }
}

#[test]
fn false_positive_sources_stay_clean_under_rust_noise() {
    for case in FALSE_POSITIVE_CASES {
        let workspace = residue_workspace(case.label, case.source);
        let report = query_consumer_residue_audit("downstream")
            .required_root(&workspace)
            .evaluate()
            .expect("false-positive fixture must parse");

        assert_eq!(
            report.finding_count(),
            0,
            "{} should stay clean: {:?}",
            case.label,
            report.findings()
        );
        assert!(report.scanned_file_count() > 0);
        assert!(report.parsed_item_count() > 0);
        assert!(report.visited_node_count() > 0);
    }
}

#[test]
fn exact_text_detection_reports_repeated_sites_in_one_file() {
    let workspace = residue_workspace(
        "repeated-exact-text",
        r#"
fn residue() {
    let _ = RuntimeBridge::new();
    let _ = RuntimeBridge::new();
}
"#,
    );
    let report = query_consumer_residue_audit("downstream")
        .required_root(&workspace)
        .evaluate()
        .expect("repeated exact-text fixture must parse");
    let lines = finding_lines_for(
        &report,
        WorthQueryConsumerResidueClass::RuntimeBridgeHandAssembly,
    );

    assert_eq!(lines, vec![3, 4]);
    assert_eq!(report.skipped_non_rust_file_count(), 0);
}

#[test]
fn registry_rows_are_complete_unique_and_point_to_consumer_kit_lanes() {
    let rows = worth_query_consumer_residue_registry();
    let classes = rows
        .iter()
        .map(|row| row.class())
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(rows.len(), classes.len());
    for case in HOSTILE_CLASS_CASES {
        let row = rows
            .iter()
            .find(|row| row.class() == case.class)
            .expect("hostile class must have registry row");
        assert_eq!(row.detection_key(), case.detection_key);
        assert_eq!(row.replacement_lane(), case.replacement_lane);
        assert!(!row.explanation().is_empty());
        assert!(matches!(
            row.replacement_lane(),
            "evidence-report-kit" | "support-pinning" | "in-memory-test-backend"
        ));
        if case.class.is_proof_folklore_for_test() {
            assert_eq!(row.detection(), WorthQueryConsumerResidueDetection::Ast);
        }
    }
    let test_backend_classes = worth_query_test_backend_residue_classes()
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        test_backend_classes.contains(&WorthQueryConsumerResidueClass::RuntimeBridgeHandAssembly)
    );
    assert!(!test_backend_classes.contains(&WorthQueryConsumerResidueClass::LocalQueryReport));
}

#[test]
fn report_and_finding_identity_change_with_canonical_meaning() {
    let root = residue_workspace("identity-perturbation", DEBUG_BINDING_SOURCE);
    let first = query_consumer_residue_audit("downstream")
        .required_root(&root)
        .evaluate()
        .expect("identity fixture must parse");
    let repeated = query_consumer_residue_audit("downstream")
        .required_root(&root)
        .evaluate()
        .expect("identity fixture must parse repeatedly");
    assert_eq!(first.report_identity(), repeated.report_identity());

    let renamed_consumer = query_consumer_residue_audit("other-consumer")
        .required_root(&root)
        .evaluate()
        .expect("renamed consumer fixture must parse");
    assert_ne!(first.report_identity(), renamed_consumer.report_identity());

    let extra_root = residue_workspace("identity-extra-root", CLEAN_SOURCE);
    let wider_roots = query_consumer_residue_audit("downstream")
        .required_root(&root)
        .required_root(&extra_root)
        .evaluate()
        .expect("wider root fixture must parse");
    assert_ne!(first.report_identity(), wider_roots.report_identity());

    fs::write(root.join("lib.rs"), format!("\n{DEBUG_BINDING_SOURCE}"))
        .expect("identity fixture should be rewritable");
    let moved_finding = query_consumer_residue_audit("downstream")
        .required_root(&root)
        .evaluate()
        .expect("moved finding fixture must parse");
    assert_ne!(
        first.finding_identities()[0],
        moved_finding.finding_identities()[0]
    );
}

#[test]
fn consumer_residue_audit_rejects_invalid_inventory_inputs() {
    let empty_name = query_consumer_residue_audit(" ")
        .required_root(residue_workspace("empty-name", CLEAN_SOURCE))
        .evaluate()
        .expect_err("blank consumer name must fail");
    assert_eq!(
        empty_name.kind(),
        worth_query::facade::consumer_kit::WorthQueryBoundaryAuditErrorKind::EmptyCrateName
    );

    let empty_roots = query_consumer_residue_audit("downstream")
        .evaluate()
        .expect_err("empty root set must fail");
    assert_eq!(
        empty_roots.kind(),
        worth_query::facade::consumer_kit::WorthQueryBoundaryAuditErrorKind::MissingRequiredRoot
    );

    let missing_root = query_consumer_residue_audit("downstream")
        .required_root(std::env::temp_dir().join("worth-query-missing-consumer-residue-root"))
        .evaluate()
        .expect_err("missing root must fail");
    assert_eq!(
        missing_root.kind(),
        worth_query::facade::consumer_kit::WorthQueryBoundaryAuditErrorKind::MissingRequiredRoot
    );
}

#[test]
fn test_backend_residue_audit_remains_a_filtered_compatibility_lane() {
    let workspace = residue_workspace(
        "test-backend-compatibility",
        &format!("{RUNTIME_BRIDGE_SOURCE}\n{LOCAL_QUERY_REPORT_SOURCE}"),
    );
    let report = query_test_backend_residue_audit("downstream")
        .required_root(&workspace)
        .evaluate()
        .expect("compatibility fixture must parse");

    assert_eq!(report.finding_count(), 1);
    assert_eq!(
        report.findings()[0].residue_class(),
        WorthQueryConsumerResidueClass::RuntimeBridgeHandAssembly.as_str()
    );
    assert_eq!(
        report.report_identity().scope(),
        WorthQueryEvidenceScope::ConsumerTestBackendResidueReport
    );
    assert!(report.finding_identities().iter().all(|identity| {
        identity.scope() == WorthQueryEvidenceScope::ConsumerTestBackendResidueFinding
    }));
}

fn single_finding_for(
    report: &WorthQueryConsumerResidueReport,
    class: WorthQueryConsumerResidueClass,
) -> &worth_query::facade::consumer_kit::WorthQueryConsumerResidueFinding {
    let matches = report
        .findings()
        .iter()
        .filter(|finding| finding.residue_class() == class)
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "findings: {:?}", report.findings());
    matches[0]
}

fn finding_lines_for(
    report: &WorthQueryConsumerResidueReport,
    class: WorthQueryConsumerResidueClass,
) -> Vec<usize> {
    report
        .findings()
        .iter()
        .filter(|finding| finding.residue_class() == class)
        .map(|finding| finding.line())
        .collect()
}

fn expected_line(source: &str, needle: &str) -> usize {
    source
        .lines()
        .position(|line| line.contains(needle))
        .map(|index| index + 1)
        .expect("fixture should contain expected line needle")
}

fn residue_workspace(label: &str, source: &str) -> PathBuf {
    let unique = WORKSPACE_COUNTER.fetch_add(1, Ordering::SeqCst);
    let root = std::env::temp_dir()
        .join("worth-query-consumer-residue-audit")
        .join(format!("{label}-{}-{unique}", std::process::id()));
    fs::create_dir_all(&root).expect("residue fixture root should be creatable");
    fs::write(root.join("lib.rs"), source).expect("residue fixture source should be writable");
    root
}

trait ProofFolkloreClassForTest {
    fn is_proof_folklore_for_test(self) -> bool;
}

impl ProofFolkloreClassForTest for WorthQueryConsumerResidueClass {
    fn is_proof_folklore_for_test(self) -> bool {
        matches!(
            self,
            Self::LocalQueryReport
                | Self::LocalQueryProof
                | Self::RawSupportSnapshotRow
                | Self::SupportMatrixRowSearch
                | Self::DebugDerivedQueryProof
                | Self::DelimiterJoinedQueryProof
                | Self::DelimiterFormattedQueryProof
        )
    }
}
