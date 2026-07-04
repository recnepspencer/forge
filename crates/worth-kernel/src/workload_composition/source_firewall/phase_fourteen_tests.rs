use std::collections::BTreeSet;

use super::semantic_source_registry::{
    phase_fourteen_raw_construction_semantic_source_coverages,
    phase_fourteen_reintroduction_semantic_source_coverages,
};
use super::{
    current_worth_touched_graph_conflict_source_firewall_closeout,
    WorthTouchedGraphConflictForbiddenSurface,
    WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind,
};
use topology::certification::{
    current_topology_public_facade_compile_fail_closeout,
    topology_public_facade_compile_fail_closeout_excluding_fence_class_for_tests,
};
use worth_spatial::certification::{
    current_spatial_public_facade_compile_fail_closeout,
    spatial_public_facade_compile_fail_closeout_excluding_fence_class_for_tests,
};

#[test]
fn phase_fourteen_firewall_registries_cover_reintroduction_and_raw_construction_families() {
    let reintroduction = phase_fourteen_reintroduction_semantic_source_coverages();
    let raw_construction = phase_fourteen_raw_construction_semantic_source_coverages();
    assert!(!reintroduction.is_empty());
    assert!(!raw_construction.is_empty());

    let reintroduction_surfaces = reintroduction
        .iter()
        .map(|coverage| coverage.forbidden_surface())
        .collect::<BTreeSet<_>>();
    let raw_construction_surfaces = raw_construction
        .iter()
        .map(|coverage| coverage.forbidden_surface())
        .collect::<BTreeSet<_>>();

    assert!(reintroduction_surfaces
        .contains(&WorthTouchedGraphConflictForbiddenSurface::PlannerRouteConstruction));
    assert!(reintroduction_surfaces
        .contains(&WorthTouchedGraphConflictForbiddenSurface::SupportWrapperShortcut));
    assert!(reintroduction_surfaces
        .contains(&WorthTouchedGraphConflictForbiddenSurface::LegacyExplainerImport));
    assert!(raw_construction_surfaces
        .contains(&WorthTouchedGraphConflictForbiddenSurface::CallerOwnedReuseDecision));
    assert!(raw_construction_surfaces
        .contains(&WorthTouchedGraphConflictForbiddenSurface::LocalPublicProofFabrication));
    assert!(raw_construction_surfaces
        .contains(&WorthTouchedGraphConflictForbiddenSurface::LocalDiagnosticAuthorityFabrication));
}

#[test]
fn source_firewall_closeout_rejects_missing_phase_fourteen_public_fence_classes() {
    let report = super::current_worth_touched_graph_conflict_source_firewall_report()
        .expect("current source firewall report should load");
    let deletion_closeout =
        crate::workload_composition::current_worth_touched_graph_conflict_deletion_closeout()
            .expect("current deletion closeout should load");
    let hostile_topology_closeout =
        topology_public_facade_compile_fail_closeout_excluding_fence_class_for_tests(
            "route-rediscovery",
        )
        .expect("hostile topology closeout should lower");
    let hostile_spatial_closeout =
        spatial_public_facade_compile_fail_closeout_excluding_fence_class_for_tests(
            "readiness-constructor",
        )
        .expect("hostile spatial closeout should lower");

    let error = super::closeout::closeout_from_products(
        &report,
        &deletion_closeout,
        &hostile_topology_closeout,
        &hostile_spatial_closeout,
    )
    .expect_err("missing phase 14 fence classes should fail closeout");
    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind::PublicFacadeCertificationMismatch
    );
}

#[test]
fn phase_fourteen_closeout_is_bound_into_current_source_firewall_closeout() {
    let closeout = current_worth_touched_graph_conflict_source_firewall_closeout()
        .expect("phase 14 source firewall closeout should load");
    assert!(closeout
        .covered_forbidden_surfaces()
        .contains(&WorthTouchedGraphConflictForbiddenSurface::PlannerRouteConstruction));
    assert!(closeout
        .covered_forbidden_surfaces()
        .contains(&WorthTouchedGraphConflictForbiddenSurface::LocalPublicProofFabrication));

    let topology_closeout = current_topology_public_facade_compile_fail_closeout()
        .expect("topology compile-fail closeout should load");
    let spatial_closeout = current_spatial_public_facade_compile_fail_closeout()
        .expect("spatial compile-fail closeout should load");
    assert!(topology_closeout
        .covered_fence_classes()
        .iter()
        .any(|class| class == "route-rediscovery"));
    assert!(topology_closeout
        .covered_fence_classes()
        .iter()
        .any(|class| class == "readiness-constructor"));
    assert!(spatial_closeout
        .covered_fence_classes()
        .iter()
        .any(|class| class == "readiness-constructor"));
}
