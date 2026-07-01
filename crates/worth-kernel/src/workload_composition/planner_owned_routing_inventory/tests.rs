use super::classification::{
    PlannerOwnedRoutingDisplacedLane as DisplacedLane,
    PlannerOwnedRoutingDisposition as Disposition, PlannerOwnedRoutingLifecycleRole as Role,
    PlannerOwnedRoutingReplacementLane as Lane,
};
use super::current_planner_owned_routing_inventory;
use super::row::{
    PlannerOwnedRoutingInventoryRow as Row, PlannerOwnedRoutingSurfaceIdentity as Surface,
};
use super::source_scan;

#[test]
fn planner_owned_routing_inventory_is_scope_complete() {
    let closeout = current_planner_owned_routing_inventory().expect("inventory");
    let rows = closeout.report().rows();

    for lane in [
        DisplacedLane::KernelPublicCloseout,
        DisplacedLane::KernelSourceFirewall,
        DisplacedLane::TopoDiagnosticSurfaces,
        DisplacedLane::TopoQueryBackedConsumerCutover,
        DisplacedLane::SpatialEvidenceLookupPublicCloseout,
    ] {
        for surface in source_scan::displaced_lane_covered_surfaces(lane).expect("lane surfaces") {
            assert!(
                rows.iter()
                    .any(|row| row.displaced_lane() == lane && row.surface_name() == surface),
                "missing inventory row for {lane:?} covered surface {surface}"
            );
        }
    }
}

#[test]
fn ordinary_public_surfaces_cannot_hide_local_route_rediscovery() {
    let closeout = current_planner_owned_routing_inventory().expect("inventory");
    for row in closeout.report().rows() {
        if row.ordinary_path() {
            assert_ne!(row.disposition(), Disposition::Cap);
            assert!(!row.blocker().is_empty());
            assert!(!row.removal_trigger().is_empty());
        }
    }
}

#[test]
fn every_lifecycle_role_is_represented() {
    let closeout = current_planner_owned_routing_inventory().expect("inventory");
    let rows = closeout.report().rows();
    for role in Role::ALL {
        assert!(rows.iter().any(|row| row.lifecycle_role() == role));
    }
}

#[test]
fn every_row_names_displaced_and_replacement_lanes() {
    let closeout = current_planner_owned_routing_inventory().expect("inventory");
    for row in closeout.report().rows() {
        assert!(!row.displaced_lane().path().is_empty());
        assert!(row
            .replacement_lane()
            .path()
            .contains("planner_owned_routing"));
        assert!(!row.current_authority_sources().is_empty());
        for authority_source in row.current_authority_sources() {
            assert!(!authority_source.contains(' '));
        }
    }
}

#[test]
fn query_rows_are_marked_as_query_gaps() {
    let closeout = current_planner_owned_routing_inventory().expect("inventory");
    for row in closeout
        .report()
        .rows()
        .iter()
        .filter(|row| row.displaced_lane() == DisplacedLane::ForgeQueryDocs)
    {
        assert_eq!(row.disposition(), Disposition::QueryGap);
        assert!(row.query_gap().is_some());
    }
}

#[test]
fn high_risk_surfaces_keep_exact_phase_one_semantics() {
    let closeout = current_planner_owned_routing_inventory().expect("inventory");
    let rows = closeout.report().rows();

    assert_row_semantics(
        rows,
        Surface::DerivedReadDiagnostics,
        Role::DerivedDiagnosticProjection,
        Disposition::Migrate,
        Lane::TopoDiagnosticProjectionInput,
    );
    assert_row_semantics(
        rows,
        Surface::DerivedValidationExecutionReport,
        Role::DerivedDiagnosticProjection,
        Disposition::Migrate,
        Lane::TopoDiagnosticProjectionInput,
    );
    assert_row_semantics(
        rows,
        Surface::DerivedInvalidationTargetRow,
        Role::DerivedDiagnosticProjection,
        Disposition::Migrate,
        Lane::TopoDiagnosticProjectionInput,
    );
    assert_row_semantics(
        rows,
        Surface::AdmitTopologyQueryBackedConsumerCutover,
        Role::FamilyRouteProduct,
        Disposition::Migrate,
        Lane::TopoQueryBackedReadFamily,
    );
    assert_row_semantics(
        rows,
        Surface::CurrentTopologyQueryBackedConsumerCutover,
        Role::ForbiddenLegacyExplainer,
        Disposition::Delete,
        Lane::TopoQueryBackedReadFamily,
    );
    assert_row_semantics(
        rows,
        Surface::QueryWorkspacePublicSupportMatrix,
        Role::PriorProofInputConsumer,
        Disposition::QueryGap,
        Lane::TopoQueryBackedReadFamily,
    );
}

fn assert_row_semantics(
    rows: &[Row],
    surface: Surface,
    expected_role: Role,
    expected_disposition: Disposition,
    expected_replacement_lane: Lane,
) {
    let row = find_row(rows, surface);
    assert_eq!(row.lifecycle_role(), expected_role);
    assert_eq!(row.disposition(), expected_disposition);
    assert_eq!(row.replacement_lane(), expected_replacement_lane);
}

fn find_row(rows: &[Row], surface: Surface) -> &Row {
    rows.iter()
        .find(|row| row.surface_identity() == surface)
        .unwrap_or_else(|| panic!("missing planner-owned routing inventory row for {surface:?}"))
}
