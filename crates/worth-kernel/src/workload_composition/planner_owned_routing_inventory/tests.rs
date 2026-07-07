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
        DisplacedLane::TopoQueryBackedConsumerCutover,
        DisplacedLane::TopoDiagnosticProjectionInputResidue,
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
        assert!(
            row.replacement_lane()
                .path()
                .contains("planner_owned_routing")
                || row
                    .replacement_lane()
                    .path()
                    .contains("touched_graph_parity_closeout")
        );
        assert!(!row.current_authority_sources().is_empty());
        for authority_source in row.current_authority_sources() {
            assert!(!authority_source.contains(' '));
        }
    }
}

#[test]
fn capped_rows_name_a_real_residue_path_instead_of_self_replacing() {
    let closeout = current_planner_owned_routing_inventory().expect("inventory");
    for row in closeout
        .report()
        .rows()
        .iter()
        .filter(|row| row.disposition() == Disposition::Cap)
    {
        assert_ne!(
            row.displaced_lane().path(),
            row.replacement_lane().path(),
            "capped residue surface `{}` must name an exact surviving residue path instead of claiming the same path as its replacement lane",
            row.surface_name()
        );
    }
}

#[test]
fn phase_twelve_names_exact_closeout_replacement_lanes_for_topology_and_spatial_authority() {
    let closeout = current_planner_owned_routing_inventory().expect("inventory");

    for lane in [
        Lane::TopoQueryBackedReadFamily,
        Lane::TopoInvalidationRoute,
        Lane::SpatialEvidenceLookupRoute,
        Lane::SpatialPublicCloseoutRoute,
    ] {
        assert!(
            lane.path().contains("touched_graph_parity_closeout"),
            "phase 12 replacement lane must point at the final parity closeout directory: {}",
            lane.path()
        );
    }

    assert!(closeout.report().rows().iter().any(|row| {
        row.replacement_lane() == Lane::TopoQueryBackedReadFamily
            && row.displaced_lane() == DisplacedLane::TopoQueryBackedConsumerCutover
    }));
    assert!(closeout.report().rows().iter().any(|row| {
        row.replacement_lane() == Lane::SpatialPublicCloseoutRoute
            && row.displaced_lane() == DisplacedLane::SpatialEvidenceLookupPublicCloseout
    }));
}

#[test]
fn high_risk_surfaces_keep_exact_phase_one_semantics() {
    let closeout = current_planner_owned_routing_inventory().expect("inventory");
    let rows = closeout.report().rows();

    assert_row_semantics(
        rows,
        Surface::DerivedReadDiagnostics,
        Role::DerivedDiagnosticProjection,
        Disposition::Cap,
        Lane::TopoDiagnosticProjectionInput,
    );
    assert_row_semantics(
        rows,
        Surface::DerivedValidationExecutionReport,
        Role::DerivedDiagnosticProjection,
        Disposition::Cap,
        Lane::TopoDiagnosticProjectionInput,
    );
    assert_row_semantics(
        rows,
        Surface::DerivedInvalidationTargetRow,
        Role::DerivedDiagnosticProjection,
        Disposition::Cap,
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
        Surface::BuildDerivedReadDiagnostics,
        Role::DerivedDiagnosticProjection,
        Disposition::Cap,
        Lane::TopoDiagnosticProjectionInput,
    );
    assert_row_semantics(
        rows,
        Surface::DeriveTopologyValidationReport,
        Role::DerivedDiagnosticProjection,
        Disposition::Cap,
        Lane::TopoDiagnosticProjectionInput,
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
