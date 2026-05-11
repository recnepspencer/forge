use super::error::TopologyDomainQueryError;
use crate::projection::runtime_boundary::read_lowering::schema::TopologyDomainTraversalRelation;
use forge_query::facade::RelationName;

use crate::projection::diagnostic_surfaces::read_proof::report::TopologyDomainQueryRequestFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TopologyDomainQueryRequest {
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
pub(crate) struct TopologyDomainQueryTraversalStep {
    relation: TopologyDomainTraversalRelation,
    depth: u8,
}

impl TopologyDomainQueryTraversalStep {
    pub(crate) fn new(relation: TopologyDomainTraversalRelation, depth: u8) -> Self {
        Self { relation, depth }
    }

    pub(crate) fn relation_name(self) -> RelationName {
        self.relation.relation_name()
    }

    pub(crate) fn depth(self) -> u8 {
        self.depth
    }

    fn validate(
        self,
        request_family: TopologyDomainQueryRequestFamily,
    ) -> Result<(), TopologyDomainQueryError> {
        let maximum_supported_depth = usize::from(self.relation.max_depth());
        let requested_depth = usize::from(self.depth);
        if requested_depth == 0 || requested_depth > maximum_supported_depth {
            return Err(TopologyDomainQueryError::unsupported_traversal_depth(
                request_family,
                requested_depth,
                maximum_supported_depth,
            ));
        }
        Ok(())
    }
}

impl TopologyDomainQueryRequest {
    pub(crate) fn validate(&self) -> Result<(), TopologyDomainQueryError> {
        let request_family = self.family();
        for step in self.traversal_steps() {
            step.validate(request_family)?;
        }
        Ok(())
    }

    pub(crate) fn family(&self) -> TopologyDomainQueryRequestFamily {
        match self {
            Self::HalfEdgeSharedVertexNeighborhood { .. } => {
                TopologyDomainQueryRequestFamily::HalfEdgeSharedVertexNeighborhood
            }
            Self::HalfEdgeRadialNeighborhood { .. } => {
                TopologyDomainQueryRequestFamily::HalfEdgeRadialNeighborhood
            }
            Self::LoopCycleNeighborhood { .. } => {
                TopologyDomainQueryRequestFamily::LoopCycleNeighborhood
            }
            Self::LocalRewireNeighborhood { .. } => {
                TopologyDomainQueryRequestFamily::LocalRewireNeighborhood
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

    pub(crate) fn traversal_steps(&self) -> Vec<TopologyDomainQueryTraversalStep> {
        match self {
            Self::HalfEdgeSharedVertexNeighborhood { .. } => vec![
                TopologyDomainQueryTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgeStartsAtVertex,
                    1,
                ),
                TopologyDomainQueryTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgeEndsAtVertex,
                    1,
                ),
            ],
            Self::HalfEdgeRadialNeighborhood { .. } => vec![
                TopologyDomainQueryTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgeRadialNext,
                    1,
                ),
                TopologyDomainQueryTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgeUsesEdge,
                    1,
                ),
            ],
            Self::LoopCycleNeighborhood { depth, .. } => {
                vec![TopologyDomainQueryTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgeNext,
                    *depth,
                )]
            }
            Self::LocalRewireNeighborhood { cycle_depth, .. } => vec![
                TopologyDomainQueryTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgeNext,
                    *cycle_depth,
                ),
                TopologyDomainQueryTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgePrev,
                    1,
                ),
            ],
        }
    }
}
