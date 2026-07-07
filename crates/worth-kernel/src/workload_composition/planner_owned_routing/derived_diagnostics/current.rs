use crate::workload_composition::performance_trace::trace_scope;
#[cfg(test)]
use crate::workload_composition::planner_owned_routing::selected_route::WorthTouchedGraphConflictSelectedRoutePacket;
use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_selected_route_packet, PlannerOwnedRoutingError,
};

use super::projection::{
    WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
    WorthTouchedGraphConflictDerivedDiagnosticProjection,
};
use super::selection::select_rich_localization;

#[cfg(test)]
pub fn current_worth_touched_graph_conflict_derived_diagnostic_projection(
) -> Result<WorthTouchedGraphConflictDerivedDiagnosticProjection, PlannerOwnedRoutingError> {
    current_worth_touched_graph_conflict_derived_diagnostic_projection_with_artifact_policy(
        WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::RichLocalization,
    )
}

pub fn current_worth_touched_graph_conflict_derived_diagnostic_projection_with_artifact_policy(
    artifact_policy: WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
) -> Result<WorthTouchedGraphConflictDerivedDiagnosticProjection, PlannerOwnedRoutingError> {
    trace_scope(
        "current_worth_touched_graph_conflict_derived_diagnostic_projection",
        || {
            build_current_worth_touched_graph_conflict_derived_diagnostic_projection(
                current_worth_touched_graph_conflict_selected_route_packet()?,
                artifact_policy,
            )
        },
    )
}

#[cfg(test)]
pub(crate) fn current_worth_touched_graph_conflict_derived_diagnostic_projection_with_packet_loader(
    packet_loader: impl FnOnce() -> Result<
        WorthTouchedGraphConflictSelectedRoutePacket,
        PlannerOwnedRoutingError,
    >,
) -> Result<WorthTouchedGraphConflictDerivedDiagnosticProjection, PlannerOwnedRoutingError> {
    build_current_worth_touched_graph_conflict_derived_diagnostic_projection(
        packet_loader()?,
        WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::RichLocalization,
    )
}

fn build_current_worth_touched_graph_conflict_derived_diagnostic_projection(
    selected_route_packet: crate::workload_composition::planner_owned_routing::selected_route::WorthTouchedGraphConflictSelectedRoutePacket,
    artifact_policy: WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
) -> Result<WorthTouchedGraphConflictDerivedDiagnosticProjection, PlannerOwnedRoutingError> {
    trace_scope(
        "build_current_worth_touched_graph_conflict_derived_diagnostic_projection",
        || {
            let rich_localization = trace_scope("select_rich_localization", || {
                select_rich_localization(artifact_policy, &selected_route_packet)
            });

            Ok(
                WorthTouchedGraphConflictDerivedDiagnosticProjection::from_selected_route_packet(
                    &selected_route_packet,
                    artifact_policy,
                    rich_localization,
                ),
            )
        },
    )
}
