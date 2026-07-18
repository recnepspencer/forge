use std::collections::BTreeMap;

#[cfg(any(test, feature = "certification-support"))]
use crate::graph::UiGraphParticipationMutation;
use crate::graph::{
    materialize_graph_mounted_receipts, materialize_graph_participation_posture,
    materialize_graph_topology, UiGraphAxisParticipation, UiGraphCoreIndexes,
    UiGraphDeclarationCorrespondence, UiGraphGeneration, UiGraphInstantiationPlan,
    UiGraphMountedReceiptAuthoritySeedStore, UiGraphMountedReceiptTransition, UiGraphNode,
    UiGraphNodeIdentity, UiGraphParticipationAxis, UiGraphParticipationPosture,
    UiGraphParticipationStatus, UiGraphSnapshot, UiGraphTopology, UiGraphWorldProfile,
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
            let node = UiGraphNode::new(crate::graph::UiGraphNodeInput {
                graph_node_identity,
                declaration_identity: entry.declaration_identity().clone(),
                structural_digest: entry.topology_seed().structural_digest(),
                structural_role: entry.topology_seed().role(),
                operator_kind: entry.topology_seed().operator_kind(),
                repetition_posture: entry.topology_seed().repetition_posture(),
                measurement_constraint_modifier: entry.measurement_constraint_modifier(),
                authored_provenance_digest: entry.authored_provenance_digest(),
                repeated_instance_basis: entry.repeated_instance_basis().clone(),
                attachment_posture: entry.attachment_posture(),
                participation_posture: materialize_graph_participation_posture(entry),
            });

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
                clone_node_with_posture(node, posture)
            })
            .collect::<Vec<_>>();

        Self::successor_with_nodes(prior_snapshot, nodes)
    }

    pub(crate) fn mounted_layout_admitted_successor(
        prior_snapshot: &UiGraphSnapshot,
        transitions: &[UiGraphMountedReceiptTransition],
    ) -> Self {
        let transitions = transitions
            .iter()
            .map(|transition| {
                (
                    transition.authority_record().graph_node_identity(),
                    *transition,
                )
            })
            .collect::<BTreeMap<_, _>>();
        let nodes = prior_snapshot
            .nodes()
            .iter()
            .map(|node| {
                let Some(transition) = transitions.get(&node.graph_node_identity()) else {
                    return node.clone();
                };
                let posture = node
                    .participation_posture()
                    .with_axis(
                        UiGraphParticipationAxis::Mounted,
                        transition.next_mounted_axis_participation(),
                    )
                    .with_axis(
                        UiGraphParticipationAxis::Layout,
                        UiGraphAxisParticipation::runtime_mutation(
                            UiGraphParticipationStatus::Admitted,
                        ),
                    );
                clone_node_with_posture(node, posture)
            })
            .collect::<Vec<_>>();

        Self::successor_with_nodes(prior_snapshot, nodes)
    }

    fn successor_with_nodes(prior_snapshot: &UiGraphSnapshot, nodes: Vec<UiGraphNode>) -> Self {
        let core_indexes = UiGraphCoreIndexes::rebuild_participation_for_successor(
            &nodes,
            prior_snapshot.topology(),
            prior_snapshot.core_indexes(),
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

fn clone_node_with_posture(
    node: &UiGraphNode,
    participation_posture: UiGraphParticipationPosture,
) -> UiGraphNode {
    UiGraphNode::new(crate::graph::UiGraphNodeInput {
        graph_node_identity: node.graph_node_identity(),
        declaration_identity: node.declaration_identity().clone(),
        structural_digest: node.structural_digest(),
        structural_role: node.structural_role(),
        operator_kind: node.operator_kind(),
        repetition_posture: node.repetition_posture(),
        measurement_constraint_modifier: node.measurement_constraint_modifier(),
        authored_provenance_digest: node.authored_provenance_digest(),
        repeated_instance_basis: node.repeated_instance_basis().clone(),
        attachment_posture: node.attachment_posture(),
        participation_posture,
    })
}
