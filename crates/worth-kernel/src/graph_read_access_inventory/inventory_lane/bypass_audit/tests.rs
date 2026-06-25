use std::collections::BTreeSet;

use forge_query::facade::consumer_kit::{
    graph_read_bypass_audit, ForgeQueryBoundaryAuditSourceSet,
};

use super::super::{
    current_worth_graph_read_access_surface_inventory_for_tests,
    graph_read_bypass_residue_cap_inventory, graph_read_bypass_residue_manifest_for_report,
    WorthGraphReadAccessClassification, WorthGraphReadAccessInventoryErrorKind,
    WorthGraphReadAccessInventorySeed,
};

#[test]
fn graph_read_bypass_rollover_uses_new_inventory_after_cutover() {
    let closeout = current_worth_graph_read_access_surface_inventory_for_tests(
        WorthGraphReadAccessInventorySeed::for_tests(),
    )
    .expect("new graph-read inventory closeout should certify bypass adoption");
    let new_report = closeout.graph_read_bypass_adoption_report();
    let deleted_source_report = closeout.deleted_source_report();

    assert_eq!(new_report.unclassified_finding_count(), 0);
    assert_eq!(deleted_source_report.deleted_source_count(), 1);
    assert_eq!(deleted_source_report.existing_deleted_source_count(), 0);
    assert!(deleted_source_report
        .deleted_source_paths()
        .iter()
        .any(|path| path == "crates/worth-kernel/src/query_adoption/graph_read_access"));
    assert!(!new_report.covered_roots().iter().any(|new_root| {
        new_root.ends_with("crates/worth-kernel/src/query_adoption/graph_read_access")
    }));
    assert!(!new_report.adoption_manifest_digest().is_empty());
    assert!(new_report
        .residue_rows()
        .iter()
        .all(|row| !row.owner().is_empty()
            && !row.blocker().is_empty()
            && !row.removal_trigger().is_empty()
            && !row.row_digest().is_empty()));
}

#[test]
fn graph_read_residue_manifest_cannot_grow_without_cap_update() {
    let cap = graph_read_bypass_residue_cap_inventory()
        .iter()
        .find(|cap| cap.class().as_str() == "manual-relation-row-loop")
        .expect("manual relation loop cap must stay explicit");
    let report = graph_read_bypass_audit("worth-kernel-graph-read-access-inventory-test")
        .required_sources(
            ForgeQueryBoundaryAuditSourceSet::new("worth-kernel").source_file(
                "hostile-growth",
                "src/hostile_growth.rs",
                hostile_manual_relation_loop_source(cap.must_not_exceed_count() + 1),
            ),
        )
        .evaluate()
        .expect("hostile bypass source should produce a Query audit report");

    let error = graph_read_bypass_residue_manifest_for_report(&report)
        .expect_err("residue cap growth needs an explicit cap update");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessInventoryErrorKind::ResidueGrowthRequiresCapUpdate
    );
}

#[test]
fn consumer_kit_bypass_audit_covers_all_inventory_roots() {
    let closeout = current_worth_graph_read_access_surface_inventory_for_tests(
        WorthGraphReadAccessInventorySeed::for_tests(),
    )
    .expect("new graph-read inventory closeout should certify bypass adoption");
    let report = closeout.graph_read_bypass_adoption_report();
    let audited_roots = report.covered_roots().iter().collect::<BTreeSet<_>>();

    assert_eq!(audited_roots.len(), report.covered_roots().len());
    assert_eq!(
        report.required_root_coverage().len(),
        report.covered_roots().len()
    );
    assert!(report
        .required_root_coverage()
        .iter()
        .all(|coverage| coverage.has_source_files()
            && coverage.source_file_count() == coverage.audited_source_labels().len()
            && !coverage.required_root().is_empty()));
    for row in closeout.rows().iter().filter(|row| {
        row.classification() != WorthGraphReadAccessClassification::DeletionTarget
            && row.classification() != WorthGraphReadAccessClassification::CappedResidue
    }) {
        assert!(
            report
                .covered_roots()
                .iter()
                .any(|root| audit_root_covers_source(root, row.source_path())),
            "Consumer Kit bypass audit missed inventory root `{}`",
            row.source_path()
        );
    }
    for deleted_source in closeout.deleted_source_report().deleted_source_paths() {
        assert!(
            !report
                .covered_roots()
                .iter()
                .any(|root| root.ends_with(deleted_source)),
            "deleted graph-read source `{deleted_source}` must not be audited as a live root"
        );
    }
    assert_eq!(
        report.residue_certified_finding_count(),
        report.finding_count()
    );
    assert_eq!(
        report
            .residue_rows()
            .iter()
            .map(|row| row.current_count())
            .sum::<usize>(),
        report.finding_count()
    );
    assert!(report
        .residue_rows()
        .iter()
        .all(|row| row.current_count() <= row.must_not_exceed_count()));
    for cap in graph_read_bypass_residue_cap_inventory() {
        assert_report_contains_residue_class(report, cap.class().as_str());
        let row = report
            .residue_rows()
            .iter()
            .find(|row| row.class() == cap.class().as_str())
            .expect("residue row should exist for every explicit cap");
        assert_eq!(row.owner(), cap.owner());
        assert_eq!(row.introduced_in(), cap.introduced_in());
        assert_eq!(row.must_not_exceed_count(), cap.must_not_exceed_count());
        assert_eq!(row.blocker(), cap.blocker());
        assert_eq!(row.removal_trigger(), cap.removal_trigger());
    }
}

fn assert_report_contains_residue_class(
    report: &super::WorthGraphReadBypassAdoptionReport,
    expected_class: &str,
) {
    assert!(
        report
            .residue_rows()
            .iter()
            .any(|row| row.class() == expected_class),
        "bypass residue ledger must expose class `{expected_class}`"
    );
}

fn audit_root_covers_source(root: &str, source_path: &str) -> bool {
    let Some(workspace_relative_root) = root.split("crates/").last() else {
        return root.ends_with(source_path);
    };
    let workspace_relative_root = format!("crates/{workspace_relative_root}");
    source_path.starts_with(&workspace_relative_root)
}

fn hostile_manual_relation_loop_source(loop_count: usize) -> String {
    let mut source = String::from("fn hostile(relation_rows: &[usize]) {\n");
    for index in 0..loop_count {
        source.push_str(&format!(
            "    for relation_{index} in relation_rows.iter() {{ let _ = relation_{index}; }}\n"
        ));
    }
    source.push_str("}\n");
    source
}
