use std::collections::BTreeMap;

use crate::graph::{
    materialize_graph_mounted_receipts, materialize_graph_participation_posture,
    materialize_graph_topology, UiGraphCoreIndexes, UiGraphDeclarationCorrespondence,
    UiGraphGeneration, UiGraphInstantiationPlan, UiGraphMountedReceiptAuthoritySeedStore,
    UiGraphNode, UiGraphNodeIdentity, UiGraphSnapshot, UiGraphTopology, UiGraphWorldProfile,
};
#[cfg(any(test, feature = "certification-support"))]
use crate::graph::{
    UiGraphAxisParticipation, UiGraphParticipationAxis, UiGraphParticipationMutation,
    UiGraphParticipationStatus,
};

pub(crate) struct UiGraphMutationStage {
    generation: UiGraphGeneration,
    world_profile: UiGraphWorldProfile,
    nodes: Vec<UiGraphNode>,
    topology: UiGraphTopology,
    mounted_receipts: UiGraphMountedReceiptAuthoritySeedStore,
    core_indexes: UiGraphCoreIndexes,
}

impl UiGraphMutationStage {
    pub(crate) fn from_initial_plan(
        plan: &UiGraphInstantiationPlan,
        world_profile: UiGraphWorldProfile,
    ) -> Self {
        let mounted_receipt_reservations = plan.mounted_receipt_reservations(world_profile.clone());
        let mut declaration_to_nodes = BTreeMap::<u64, Vec<UiGraphNodeIdentity>>::new();
        let mut node_to_declaration = BTreeMap::new();
        let mut authored_provenance_to_nodes = BTreeMap::<u64, Vec<UiGraphNodeIdentity>>::new();
        let mut node_to_authored_provenance = BTreeMap::new();
        let mut nodes = Vec::with_capacity(plan.node_entries().len());
        let mut node_identities = Vec::with_capacity(plan.node_entries().len());

        for (entry, reservation) in plan
            .node_entries()
            .iter()
            .zip(mounted_receipt_reservations.iter().copied())
        {
            let graph_node_identity = reservation.graph_node_identity();
            let node = UiGraphNode::new(
                graph_node_identity,
                entry.declaration_identity().clone(),
                entry.topology_seed().structural_digest(),
                entry.topology_seed().role(),
                entry.topology_seed().operator_kind(),
                entry.topology_seed().repetition_posture(),
                entry.measurement_constraint_modifier(),
                entry.authored_provenance_digest(),
                entry.repeated_instance_basis().clone(),
                entry.attachment_posture(),
                materialize_graph_participation_posture(entry),
            );

            declaration_to_nodes
                .entry(entry.declaration_identity().digest().raw())
                .or_default()
                .push(graph_node_identity);
            authored_provenance_to_nodes
                .entry(entry.authored_provenance_digest())
                .or_default()
                .push(graph_node_identity);
            node_to_declaration.insert(graph_node_identity, entry.declaration_identity().clone());
            node_to_authored_provenance
                .insert(graph_node_identity, entry.authored_provenance_digest());
            node_identities.push(graph_node_identity);
            nodes.push(node);
        }

        let declaration_correspondence = UiGraphDeclarationCorrespondence::new(
            declaration_to_nodes,
            node_to_declaration,
            authored_provenance_to_nodes,
            node_to_authored_provenance,
        );
        let topology = materialize_graph_topology(plan, &node_identities);
        let mounted_receipts = materialize_graph_mounted_receipts(&mounted_receipt_reservations);
        let core_indexes = UiGraphCoreIndexes::build(
            plan.node_entries(),
            &nodes,
            declaration_correspondence,
            &topology,
            &mounted_receipts,
        );

        Self {
            generation: UiGraphGeneration::initial(),
            world_profile,
            nodes,
            topology,
            mounted_receipts,
            core_indexes,
        }
    }

    pub(crate) fn from_successor_plan(
        prior_snapshot: &UiGraphSnapshot,
        plan: &UiGraphInstantiationPlan,
    ) -> Self {
        let mut stage = Self::from_initial_plan(plan, prior_snapshot.world_profile().clone());
        stage.generation = UiGraphGeneration::successor_of(prior_snapshot.generation());
        stage
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn layout_admitted_successor(
        prior_snapshot: &UiGraphSnapshot,
        admitted_nodes: &[UiGraphNodeIdentity],
    ) -> Self {
        let nodes = prior_snapshot
            .nodes()
            .iter()
            .map(|node| {
                let posture = if admitted_nodes.contains(&node.graph_node_identity()) {
                    let page = prior_snapshot
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
                        UiGraphAxisParticipation::runtime_mutation(
                            UiGraphParticipationStatus::Admitted,
                        ),
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
            prior_snapshot
                .core_indexes()
                .declaration_correspondence()
                .clone(),
            prior_snapshot.topology(),
            prior_snapshot.mounted_receipts(),
        );

        Self {
            generation: UiGraphGeneration::successor_of(prior_snapshot.generation()),
            world_profile: prior_snapshot.world_profile().clone(),
            nodes,
            topology: prior_snapshot.topology().clone(),
            mounted_receipts: prior_snapshot.mounted_receipts().clone(),
            core_indexes,
        }
    }

    pub(crate) fn commit(self) -> UiGraphSnapshot {
        UiGraphSnapshot::new(
            self.generation,
            self.world_profile,
            self.nodes,
            self.topology,
            self.mounted_receipts,
            self.core_indexes,
        )
    }
}
