use super::error::TopologyReadError;
use crate::projection::runtime_boundary::read_lowering::schema::TopologyDomainTraversalRelation;
use forge_query::facade::RelationName;

use crate::projection::read_views::domain::read_proof::report::TopologyReadRequestFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TopologyReadRequest {
    HalfEdgeSharedVertexNeighborhood {
        source_half_edge_identity: String,
    },
    HalfEdgeRadialNeighborhood {
        source_half_edge_identity: String,
    },
    LoopCycleNeighborhood {
        start_half_edge_identity: String,
        depth: u8,
    },
    LocalRewireNeighborhood {
        moved_half_edge_identity: String,
        cycle_depth: u8,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TopologyReadTraversalStep {
    relation: TopologyDomainTraversalRelation,
    depth: u8,
}

impl TopologyReadTraversalStep {
    pub(crate) fn new(relation: TopologyDomainTraversalRelation, depth: u8) -> Self {
        Self { relation, depth }
    }

    pub(crate) fn relation_name(self) -> RelationName {
        self.relation.relation_name()
    }

    pub(crate) fn depth(self) -> u8 {
        self.depth
    }

    fn validate(self, request_family: TopologyReadRequestFamily) -> Result<(), TopologyReadError> {
        let maximum_supported_depth = usize::from(self.relation.max_depth());
        let requested_depth = usize::from(self.depth);
        if requested_depth == 0 || requested_depth > maximum_supported_depth {
            return Err(TopologyReadError::unsupported_traversal_depth(
                request_family,
                requested_depth,
                maximum_supported_depth,
            ));
        }
        Ok(())
    }
}

impl TopologyReadRequest {
    pub(crate) fn validate(&self) -> Result<(), TopologyReadError> {
        let request_family = self.family();
        for step in self.traversal_steps() {
            step.validate(request_family)?;
        }
        Ok(())
    }

    pub(crate) fn family(&self) -> TopologyReadRequestFamily {
        match self {
            Self::HalfEdgeSharedVertexNeighborhood { .. } => {
                TopologyReadRequestFamily::HalfEdgeSharedVertexNeighborhood
            }
            Self::HalfEdgeRadialNeighborhood { .. } => {
                TopologyReadRequestFamily::HalfEdgeRadialNeighborhood
            }
            Self::LoopCycleNeighborhood { .. } => TopologyReadRequestFamily::LoopCycleNeighborhood,
            Self::LocalRewireNeighborhood { .. } => {
                TopologyReadRequestFamily::LocalRewireNeighborhood
            }
        }
    }

    pub(crate) fn anchor_identity(&self) -> &str {
        match self {
            Self::HalfEdgeSharedVertexNeighborhood {
                source_half_edge_identity,
            }
            | Self::HalfEdgeRadialNeighborhood {
                source_half_edge_identity,
            } => source_half_edge_identity.as_str(),
            Self::LoopCycleNeighborhood {
                start_half_edge_identity,
                ..
            } => start_half_edge_identity.as_str(),
            Self::LocalRewireNeighborhood {
                moved_half_edge_identity,
                ..
            } => moved_half_edge_identity.as_str(),
        }
    }

    pub(crate) fn traversal_steps(&self) -> Vec<TopologyReadTraversalStep> {
        match self {
            Self::HalfEdgeSharedVertexNeighborhood { .. } => vec![
                TopologyReadTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgeStartsAtVertex,
                    1,
                ),
                TopologyReadTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgeEndsAtVertex,
                    1,
                ),
            ],
            Self::HalfEdgeRadialNeighborhood { .. } => vec![
                TopologyReadTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgeRadialNext,
                    1,
                ),
                TopologyReadTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgeUsesEdge,
                    1,
                ),
            ],
            Self::LoopCycleNeighborhood { depth, .. } => {
                vec![TopologyReadTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgeNext,
                    *depth,
                )]
            }
            Self::LocalRewireNeighborhood { cycle_depth, .. } => vec![
                TopologyReadTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgeNext,
                    *cycle_depth,
                ),
                TopologyReadTraversalStep::new(TopologyDomainTraversalRelation::HalfEdgePrev, 1),
            ],
        }
    }
}
