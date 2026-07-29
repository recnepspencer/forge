use std::collections::BTreeMap;

use crate::declaration::UiDeclarationIdentity;
use crate::graph::UiGraphNodeIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphDeclarationCorrespondence {
    declaration_to_nodes: BTreeMap<u64, Vec<UiGraphNodeIdentity>>,
    node_to_declaration: BTreeMap<UiGraphNodeIdentity, UiDeclarationIdentity>,
    authored_provenance_to_nodes: BTreeMap<u64, Vec<UiGraphNodeIdentity>>,
    node_to_authored_provenance: BTreeMap<UiGraphNodeIdentity, u64>,
}

impl UiGraphDeclarationCorrespondence {
    pub(crate) fn rebuild(nodes: &[crate::graph::UiGraphNode]) -> Self {
        let mut declaration_to_nodes = BTreeMap::<u64, Vec<UiGraphNodeIdentity>>::new();
        let mut node_to_declaration = BTreeMap::new();
        let mut authored_provenance_to_nodes = BTreeMap::<u64, Vec<UiGraphNodeIdentity>>::new();
        let mut node_to_authored_provenance = BTreeMap::new();

        for node in nodes {
            let graph_node_identity = node.graph_node_identity();
            declaration_to_nodes
                .entry(node.declaration_identity().digest().raw())
                .or_default()
                .push(graph_node_identity);
            node_to_declaration.insert(graph_node_identity, node.declaration_identity().clone());
            authored_provenance_to_nodes
                .entry(node.authored_provenance_digest())
                .or_default()
                .push(graph_node_identity);
            node_to_authored_provenance
                .insert(graph_node_identity, node.authored_provenance_digest());
        }

        Self::new(
            declaration_to_nodes,
            node_to_declaration,
            authored_provenance_to_nodes,
            node_to_authored_provenance,
        )
    }

    pub(crate) fn new(
        declaration_to_nodes: BTreeMap<u64, Vec<UiGraphNodeIdentity>>,
        node_to_declaration: BTreeMap<UiGraphNodeIdentity, UiDeclarationIdentity>,
        authored_provenance_to_nodes: BTreeMap<u64, Vec<UiGraphNodeIdentity>>,
        node_to_authored_provenance: BTreeMap<UiGraphNodeIdentity, u64>,
    ) -> Self {
        Self {
            declaration_to_nodes,
            node_to_declaration,
            authored_provenance_to_nodes,
            node_to_authored_provenance,
        }
    }

    pub fn graph_node_ids_for(
        &self,
        declaration_identity: &UiDeclarationIdentity,
    ) -> &[UiGraphNodeIdentity] {
        self.declaration_to_nodes
            .get(&declaration_identity.digest().raw())
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn single_graph_node_for(
        &self,
        declaration_identity: &UiDeclarationIdentity,
    ) -> Option<UiGraphNodeIdentity> {
        match self.graph_node_ids_for(declaration_identity) {
            [graph_node_identity] => Some(*graph_node_identity),
            _ => None,
        }
    }

    pub fn declaration_identity_for(
        &self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<&UiDeclarationIdentity> {
        self.node_to_declaration.get(&graph_node_identity)
    }

    pub fn graph_node_ids_for_authored_provenance(
        &self,
        authored_provenance_digest: u64,
    ) -> &[UiGraphNodeIdentity] {
        self.authored_provenance_to_nodes
            .get(&authored_provenance_digest)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn authored_provenance_digest_for(
        &self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<u64> {
        self.node_to_authored_provenance
            .get(&graph_node_identity)
            .copied()
    }

    pub fn declaration_instance_count(
        &self,
        declaration_identity: &UiDeclarationIdentity,
    ) -> usize {
        self.graph_node_ids_for(declaration_identity).len()
    }

    pub(crate) fn declaration_digests(&self) -> impl Iterator<Item = u64> + '_ {
        self.declaration_to_nodes.keys().copied()
    }
}
