use crate::facade::WorthUiApp;
use crate::graph::{
    UiGraphAxisParticipation, UiGraphCoreIndexes, UiGraphGeneration, UiGraphNode,
    UiGraphNodeIdentity, UiGraphParticipationAxis, UiGraphSnapshot,
};

pub(crate) fn snapshot_with_admitted_layout(
    app: &WorthUiApp,
    admitted_nodes: &[UiGraphNodeIdentity],
) -> UiGraphSnapshot {
    let snapshot = app.graph_snapshot();
    let nodes = snapshot
        .nodes()
        .iter()
        .map(|node| {
            let participation_posture = if admitted_nodes.contains(&node.graph_node_identity()) {
                node.participation_posture().with_axis(
                    UiGraphParticipationAxis::Layout,
                    UiGraphAxisParticipation::runtime_mutation(
                        crate::graph::UiGraphParticipationStatus::Admitted,
                    ),
                )
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
                participation_posture,
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
