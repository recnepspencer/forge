//! SUPPORT AUTHORITY — layout participation seeding for certification fixtures.
//!
//! Applies layout admission through named participation transitions, then commits a
//! same-world successor snapshot. Not a production graph mutation owner.

use crate::facade::WorthUiApp;
use crate::graph::{
    UiGraphAxisParticipation, UiGraphCoreIndexes, UiGraphGeneration, UiGraphNode,
    UiGraphNodeIdentity, UiGraphParticipationAxis, UiGraphParticipationMutation,
    UiGraphParticipationStatus, UiGraphSnapshot,
};

/// Seed layout-admitted participation for support fixtures.
///
/// Each admitted node transitions Layout → Admitted via
/// [`UiGraphParticipationMutation::axis_transition`], then the snapshot is rebuilt as a
/// same-world successor. Production callers must not use this path.
pub(crate) fn snapshot_after_layout_admission_support(
    app: &WorthUiApp,
    admitted_nodes: &[UiGraphNodeIdentity],
) -> UiGraphSnapshot {
    let snapshot = app.graph_snapshot();
    let nodes = snapshot
        .nodes()
        .iter()
        .map(|node| {
            let posture = if admitted_nodes.contains(&node.graph_node_identity()) {
                let page = snapshot
                    .lookup()
                    .topology_node(node.graph_node_identity())
                    .and_then(|row| row.value().page_membership())
                    .map(|membership| membership.page_node_identity())
                    .unwrap_or_else(|| node.graph_node_identity());
                UiGraphParticipationMutation::axis_transition(
                    node.graph_node_identity(),
                    page,
                    node.participation_posture(),
                    UiGraphParticipationAxis::Layout,
                    UiGraphAxisParticipation::runtime_mutation(UiGraphParticipationStatus::Admitted),
                )
                .updated_posture()
            } else {
                node.participation_posture()
            };
            UiGraphNode::new(
                node.graph_node_identity(),
                node.declaration_identity().clone(),
                node.structural_digest(),
                node.structural_role(),
                node.operator_kind(),
                node.repetition_posture(),
                node.measurement_constraint_modifier(),
                node.authored_provenance_digest(),
                node.repeated_instance_basis().clone(),
                node.attachment_posture(),
                posture,
            )
        })
        .collect::<Vec<_>>();

    let core_indexes = UiGraphCoreIndexes::build_without_aspects(
        &nodes,
        snapshot.core_indexes().declaration_correspondence().clone(),
        snapshot.topology(),
        snapshot.mounted_receipts(),
    );

    UiGraphSnapshot::new(
        UiGraphGeneration::successor_of(snapshot.generation()),
        snapshot.world_profile().clone(),
        nodes,
        snapshot.topology().clone(),
        snapshot.mounted_receipts().clone(),
        core_indexes,
    )
}
