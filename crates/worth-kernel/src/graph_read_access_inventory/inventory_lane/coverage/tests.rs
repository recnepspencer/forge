use std::collections::BTreeMap;

use super::super::{
    covered_graph_read_sources, current_worth_graph_read_access_surface_inventory_for_tests,
    validate_discovered_graph_read_surfaces, WorthGraphReadAccessClassification,
    WorthGraphReadAccessCloseoutOwner, WorthGraphReadAccessDiscoveredSurface,
    WorthGraphReadAccessFollowOnWork, WorthGraphReadAccessInventoryErrorKind,
    WorthGraphReadAccessInventorySeed, WorthGraphReadAccessOwner,
    WorthGraphReadAccessScopeExpectation, WorthGraphReadAccessScopeFamily,
    WorthGraphReadAccessScopeKind,
};
use super::assert_exact_scope_plan;

#[test]
fn graph_read_inventory_covers_topology_spatial_kernel_and_test_surfaces() {
    let closeout = current_worth_graph_read_access_surface_inventory_for_tests(
        WorthGraphReadAccessInventorySeed::for_tests(),
    )
    .expect("current graph-read access surface inventory should close");

    let covered_sources = covered_graph_read_sources().unwrap();
    let guard_report = closeout.guard_report();
    assert_eq!(
        closeout.closeout_owner(),
        WorthGraphReadAccessCloseoutOwner::WorthKernel
    );
    assert_eq!(guard_report.unclassified_surface_count(), 0);
    assert_eq!(guard_report.production_shaped_test_support_gap_count(), 0);
    assert_eq!(guard_report.covered_source_count(), covered_sources.len());
    assert!(guard_report.discovered_surface_count() > 0);
    assert_eq!(
        guard_report.discovered_surface_count(),
        guard_report.admitted_surface_count()
    );
    assert_eq!(closeout.deleted_source_report().deleted_source_count(), 1);
    assert_eq!(
        closeout
            .deleted_source_report()
            .existing_deleted_source_count(),
        0
    );
    let scope_report = closeout.scope_report();
    assert_eq!(scope_report.scoped_row_count(), closeout.rows().len());
    assert_eq!(scope_report.unscoped_row_count(), 0);
    assert_eq!(scope_report.selected_obligation_scoped_count(), 2);
    assert_eq!(scope_report.touched_authority_scoped_count(), 1);
    assert_eq!(scope_report.touch_descriptor_scoped_count(), 1);
    assert_eq!(scope_report.topology_read_proof_scoped_count(), 1);
    assert_eq!(scope_report.spatial_continuation_scoped_count(), 2);
    assert_eq!(scope_report.deleted_graph_read_source_scoped_count(), 1);
    assert_eq!(scope_report.certification_only_scoped_count(), 4);
    assert_eq!(scope_report.out_of_scope_count(), 0);
    assert_eq!(
        closeout.scope_plan_report().entries().len(),
        closeout.rows().len()
    );

    let rows_by_source = rows_by_source_path(closeout.rows());
    for covered_source in covered_sources {
        assert_covered_once(
            &rows_by_source,
            covered_source.source_path(),
            covered_source.owner(),
            covered_source.classification(),
        );
    }
    assert_all_rows_have_expected_follow_on_work(closeout.rows());
    assert_all_rows_have_matching_scope_bindings(closeout.rows());
    assert_all_graph_read_rows_have_scope_provenance(closeout.rows());
    assert_exact_scope_plan(closeout.scope_plan_report());

    assert_covered_once(
        &rows_by_source,
        "crates/worth-topo/src/projection/read_views/domain",
        WorthGraphReadAccessOwner::WorthTopo,
        WorthGraphReadAccessClassification::QueryDeclarationCandidate,
    );
    assert_covered_once(
        &rows_by_source,
        "crates/worth-topo/src/projection/runtime_boundary/read_execution",
        WorthGraphReadAccessOwner::WorthTopo,
        WorthGraphReadAccessClassification::QueryDeclarationCandidate,
    );
    assert_covered_once(
        &rows_by_source,
        "crates/worth-kernel/src/query_adoption/graph_read_access",
        WorthGraphReadAccessOwner::WorthKernel,
        WorthGraphReadAccessClassification::DeletionTarget,
    );
    assert_covered_once(
        &rows_by_source,
        "crates/worth-kernel/src/workload_composition",
        WorthGraphReadAccessOwner::WorthKernel,
        WorthGraphReadAccessClassification::QueryDeclarationCandidate,
    );
    assert_covered_once(
        &rows_by_source,
        "crates/worth-spatial/src/workload_platform/evidence_ledger",
        WorthGraphReadAccessOwner::WorthSpatial,
        WorthGraphReadAccessClassification::QueryDeclarationCandidate,
    );
    assert_covered_once(
        &rows_by_source,
        "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction",
        WorthGraphReadAccessOwner::WorthSpatial,
        WorthGraphReadAccessClassification::QueryAccessCapabilityGap,
    );
    assert_covered_once(
        &rows_by_source,
        "crates/worth-kernel/src/binding/tests",
        WorthGraphReadAccessOwner::WorthKernel,
        WorthGraphReadAccessClassification::CertificationOnlySupport,
    );
}

#[test]
fn graph_read_inventory_rows_require_selected_obligation_or_certification_scope() {
    let closeout = current_worth_graph_read_access_surface_inventory_for_tests(
        WorthGraphReadAccessInventorySeed::for_tests(),
    )
    .expect("current graph-read access surface inventory should close");

    assert_all_graph_read_rows_have_scope_provenance(closeout.rows());
}

#[test]
fn unclassified_graph_read_surface_fails_inventory() {
    let covered_sources = covered_graph_read_sources().unwrap();
    let discovered = [WorthGraphReadAccessDiscoveredSurface::new(
        "crates/worth-spatial/src/workload_platform/new_boolean_frontier",
        "RELATION LOOP with a LOCAL CACHE over planar boolean fragments",
        false,
    )];

    let error = validate_discovered_graph_read_surfaces(&discovered, covered_sources)
        .expect_err("unclassified production graph-read shape must fail");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessInventoryErrorKind::UnclassifiedGraphReadSurface
    );
}

#[test]
fn test_support_cannot_hide_production_read_folklore() {
    let covered_sources = covered_graph_read_sources().unwrap();
    let discovered = [WorthGraphReadAccessDiscoveredSurface::new(
        "crates/worth-kernel/src/binding/tests/support/copied_neighborhood_helper.rs",
        "test helper fabricates neighborhood receipt and local topology replacement",
        true,
    )];

    let error = validate_discovered_graph_read_surfaces(&discovered, covered_sources)
        .expect_err("production-shaped test graph-read support must be classified");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessInventoryErrorKind::ProductionShapedTestSupportUnclassified
    );
}

fn rows_by_source_path(
    rows: &[super::super::WorthGraphReadAccessInventoryRow],
) -> BTreeMap<&str, Vec<&super::super::WorthGraphReadAccessInventoryRow>> {
    let mut rows_by_source = BTreeMap::new();
    for row in rows {
        rows_by_source
            .entry(row.source_path())
            .or_insert_with(Vec::new)
            .push(row);
    }
    rows_by_source
}

fn assert_covered_once(
    rows_by_source: &BTreeMap<&str, Vec<&super::super::WorthGraphReadAccessInventoryRow>>,
    source_path: &str,
    owner: WorthGraphReadAccessOwner,
    classification: WorthGraphReadAccessClassification,
) {
    let rows = rows_by_source
        .get(source_path)
        .unwrap_or_else(|| panic!("missing inventory row for {source_path}"));
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].owner(), owner);
    assert_eq!(rows[0].classification(), classification);
}

fn assert_all_rows_have_expected_follow_on_work(
    rows: &[super::super::WorthGraphReadAccessInventoryRow],
) {
    for row in rows {
        assert_eq!(
            row.follow_on_work(),
            expected_follow_on_work(row.classification()),
            "unexpected follow-on work for {}",
            row.source_path()
        );
    }
}

fn assert_all_rows_have_matching_scope_bindings(
    rows: &[super::super::WorthGraphReadAccessInventoryRow],
) {
    for row in rows {
        assert_eq!(
            row.scope_binding().source_path(),
            row.source_path(),
            "scope binding must bind the admitted row source path for {}",
            row.source_path()
        );
    }
}

fn assert_all_graph_read_rows_have_scope_provenance(
    rows: &[super::super::WorthGraphReadAccessInventoryRow],
) {
    for row in rows {
        let binding = row.scope_binding();
        if row.classification() == WorthGraphReadAccessClassification::CertificationOnlySupport {
            assert_eq!(
                binding.scope_expectation(),
                WorthGraphReadAccessScopeExpectation::CertificationOnlyBoundary
            );
            assert_eq!(
                binding.scope_family(),
                WorthGraphReadAccessScopeFamily::CertificationBoundary
            );
            continue;
        }

        if row.classification() == WorthGraphReadAccessClassification::DeletionTarget {
            assert_eq!(
                binding.scope_expectation(),
                WorthGraphReadAccessScopeExpectation::DeletionOnlyResidue
            );
            assert_eq!(
                binding.scope_family(),
                WorthGraphReadAccessScopeFamily::DeletedGraphReadSource
            );
            continue;
        }

        assert!(
            binding.selected_obligation_index().is_some(),
            "graph-read migration scope must bind selected-obligation provenance for {}",
            row.source_path()
        );
        assert!(binding.authority_digest().is_some());
        assert!(binding.touch_descriptor_digest().is_some());
        assert!(binding.execution_proof_digest().is_some());
        if binding.scope_kind() == WorthGraphReadAccessScopeKind::SelectedObligation {
            assert!(binding.selected_registration_digest().is_some());
        }
    }
}

fn expected_follow_on_work(
    classification: WorthGraphReadAccessClassification,
) -> WorthGraphReadAccessFollowOnWork {
    match classification {
        WorthGraphReadAccessClassification::QueryDeclarationCandidate => {
            WorthGraphReadAccessFollowOnWork::MilestoneSevenDeclaration
        }
        WorthGraphReadAccessClassification::QueryAccessCapabilityGap
        | WorthGraphReadAccessClassification::CappedResidue => {
            WorthGraphReadAccessFollowOnWork::MilestoneEightAccessPlanAdoption
        }
        WorthGraphReadAccessClassification::DeletionTarget => {
            WorthGraphReadAccessFollowOnWork::DeletionOnlyCleanup
        }
        WorthGraphReadAccessClassification::CertificationOnlySupport => {
            WorthGraphReadAccessFollowOnWork::CertificationOnly
        }
        WorthGraphReadAccessClassification::OutOfScopeNonGraphRead => {
            WorthGraphReadAccessFollowOnWork::OutOfScope
        }
    }
}
