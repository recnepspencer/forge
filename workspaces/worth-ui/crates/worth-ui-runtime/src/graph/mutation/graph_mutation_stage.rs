use std::collections::BTreeMap;

use crate::graph::{
    materialize_graph_mounted_receipts, materialize_graph_participation_posture,
    materialize_graph_topology, UiGraphCoreIndexes, UiGraphDeclarationCorrespondence,
    UiGraphGeneration, UiGraphInstantiationPlan, UiGraphMountedReceiptAuthoritySeedStore,
    UiGraphNode, UiGraphNodeIdentity, UiGraphSnapshot, UiGraphTopology, UiGraphWorldProfile,
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
            node_to_authored_provenance.insert(graph_node_identity, entry.authored_provenance_digest());
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
