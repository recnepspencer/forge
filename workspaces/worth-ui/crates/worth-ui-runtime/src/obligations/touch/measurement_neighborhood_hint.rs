use crate::declaration::stable_text_digest;
use crate::evidence::{UiMeasurementDependencyMap, UiMeasurementNeighborhoodClassHint};
use crate::graph::{UiGraphMeasurementNeighborhoodHint, UiGraphNodeIdentity};

use super::UiGraphTouchDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphTouchMeasurementNeighborhoodHint {
    touch_identity_digest: u64,
    graph_node_identity: UiGraphNodeIdentity,
    world_identity_digest: u64,
    basis_identity_digest: u64,
    dependency_map: UiMeasurementDependencyMap,
    neighborhood_class_hint: UiMeasurementNeighborhoodClassHint,
    identity_digest: u64,
}

impl UiGraphTouchMeasurementNeighborhoodHint {
    pub(crate) fn from_touch(
        touch: &UiGraphTouchDescriptor,
        neighborhood_hint: &UiGraphMeasurementNeighborhoodHint,
    ) -> Option<Self> {
        if touch.target().graph_node_identity() != neighborhood_hint.graph_node_identity()
            || touch.world().world_profile().identity_digest()
                != neighborhood_hint.world_identity_digest()
        {
            return None;
        }

        Some(Self::new(
            touch.identity_digest(),
            neighborhood_hint.graph_node_identity(),
            neighborhood_hint.world_identity_digest(),
            neighborhood_hint.basis_identity_digest(),
            neighborhood_hint.dependency_map().clone(),
            neighborhood_hint.neighborhood_class_hint(),
        ))
    }

    pub(crate) fn new(
        touch_identity_digest: u64,
        graph_node_identity: UiGraphNodeIdentity,
        world_identity_digest: u64,
        basis_identity_digest: u64,
        dependency_map: UiMeasurementDependencyMap,
        neighborhood_class_hint: UiMeasurementNeighborhoodClassHint,
    ) -> Self {
        let identity_digest = stable_text_digest("touch-measurement-neighborhood-hint")
            ^ touch_identity_digest.rotate_left(7)
            ^ graph_node_identity.digest().rotate_left(13)
            ^ world_identity_digest.rotate_left(17)
            ^ basis_identity_digest.rotate_left(19)
            ^ dependency_map.identity_digest().rotate_left(23)
            ^ (neighborhood_class_hint as u64).rotate_left(29);

        Self {
            touch_identity_digest,
            graph_node_identity,
            world_identity_digest,
            basis_identity_digest,
            dependency_map,
            neighborhood_class_hint,
            identity_digest,
        }
    }

    pub fn touch_identity_digest(&self) -> u64 {
        self.touch_identity_digest
    }

    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn world_identity_digest(&self) -> u64 {
        self.world_identity_digest
    }

    pub fn basis_identity_digest(&self) -> u64 {
        self.basis_identity_digest
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
