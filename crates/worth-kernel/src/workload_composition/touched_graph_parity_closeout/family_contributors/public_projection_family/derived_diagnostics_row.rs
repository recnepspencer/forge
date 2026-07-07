use crate::workload_composition::planner_owned_routing::{
    WorthTouchedGraphConflictDerivedDiagnosticProjection, WorthTouchedGraphConflictPublicFacade,
};
use super::error::PublicProjectionContributorCatalogError;
use super::row::{PublicProjectionContributorCatalogRow, PublicProjectionContributorRowKind};

pub(super) fn derived_diagnostics_contributor_row_from_public_facade(
    public_facade: &WorthTouchedGraphConflictPublicFacade,
) -> Result<PublicProjectionContributorCatalogRow, PublicProjectionContributorCatalogError> {
    derived_diagnostics_contributor_row_from_projection(public_facade.derived_diagnostics())
}

fn derived_diagnostics_contributor_row_from_projection(
    projection: &WorthTouchedGraphConflictDerivedDiagnosticProjection,
) -> Result<PublicProjectionContributorCatalogRow, PublicProjectionContributorCatalogError> {
    PublicProjectionContributorCatalogRow::new(
        PublicProjectionContributorRowKind::DerivedDiagnostics,
        "current_worth_touched_graph_conflict_selected_route_packet",
        "current_worth_touched_graph_conflict_derived_diagnostic_projection_with_artifact_policy::{selected_route_identity_digest,decision_trace_identity_digest,selected_witness_identity_digest}",
        "current_worth_workload_ordinary_consumer_sweep_closeout",
        "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/closeout.rs",
        projection.selected_route_identity_digest().to_string(),
        projection.selected_family_identity().to_string(),
        projection.selected_product_identity_digest().to_string(),
        projection.selected_witness_identity_digest().map(str::to_string),
        None,
        None,
        None,
        None,
        Some(projection.decision_trace_identity_digest().to_string()),
        &[
            "selected_route_identity_digest",
            "decision_trace_identity_digest",
            "selected_family_identity",
            "selected_product_identity_digest",
            "selected_witness_identity_digest",
        ],
        "current_worth_touched_graph_conflict_derived_diagnostic_projection_with_artifact_policy",
        "crates/worth-kernel/src/workload_composition/planner_owned_routing/derived_diagnostics/current.rs",
    )
}
