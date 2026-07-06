use crate::declaration::stable_text_digest;
use crate::evidence::{
    UiMeasurementBasis, UiMeasurementDependencyMap, UiMeasurementNeighborhoodClassHint,
};

use super::UiGraphNodeIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphMeasurementNeighborhoodHint {
    graph_node_identity: UiGraphNodeIdentity,
    world_identity_digest: u64,
    basis_identity_digest: u64,
    dependency_map: UiMeasurementDependencyMap,
    neighborhood_class_hint: UiMeasurementNeighborhoodClassHint,
    identity_digest: u64,
}

impl UiGraphMeasurementNeighborhoodHint {
    pub(crate) fn from_basis(basis: &UiMeasurementBasis) -> Self {
        Self::new(
            basis.graph_node_identity(),
            basis.world_profile().identity_digest(),
            basis.identity_digest(),
            basis.dependency_map().clone(),
            basis.neighborhood_class_hint(),
        )
    }

    pub(crate) fn new(
        graph_node_identity: UiGraphNodeIdentity,
        world_identity_digest: u64,
        basis_identity_digest: u64,
        dependency_map: UiMeasurementDependencyMap,
        neighborhood_class_hint: UiMeasurementNeighborhoodClassHint,
    ) -> Self {
        let identity_digest = stable_text_digest("graph-measurement-neighborhood-hint")
            ^ graph_node_identity.digest().rotate_left(7)
            ^ world_identity_digest.rotate_left(11)
            ^ basis_identity_digest.rotate_left(13)
            ^ dependency_map.identity_digest().rotate_left(19)
            ^ (neighborhood_class_hint as u64).rotate_left(23);

        Self {
            graph_node_identity,
            world_identity_digest,
            basis_identity_digest,
            dependency_map,
            neighborhood_class_hint,
            identity_digest,
        }
    }

    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn basis_identity_digest(&self) -> u64 {
        self.basis_identity_digest
    }

    pub fn world_identity_digest(&self) -> u64 {
        self.world_identity_digest
    }

    pub fn dependency_map(&self) -> &UiMeasurementDependencyMap {
        &self.dependency_map
    }

    pub fn neighborhood_class_hint(&self) -> UiMeasurementNeighborhoodClassHint {
        self.neighborhood_class_hint
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}
