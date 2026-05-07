use super::error::WorthTopologyDomainQueryError;
use forge_query::facade::RelationName;

use super::report::WorthTopologyDomainQueryRequestFamily;
use super::schema::WorthTopologyDomainTraversalRelation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WorthTopologyDomainQueryRequest {
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
pub(crate) struct WorthTopologyDomainQueryTraversalStep {
    relation: WorthTopologyDomainTraversalRelation,
    depth: u8,
}

impl WorthTopologyDomainQueryTraversalStep {
    pub(crate) fn new(relation: WorthTopologyDomainTraversalRelation, depth: u8) -> Self {
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
        request_family: WorthTopologyDomainQueryRequestFamily,
    ) -> Result<(), WorthTopologyDomainQueryError> {
        let maximum_supported_depth = usize::from(self.relation.max_depth());
        let requested_depth = usize::from(self.depth);
        if requested_depth == 0 || requested_depth > maximum_supported_depth {
            return Err(WorthTopologyDomainQueryError::unsupported_traversal_depth(
                request_family,
                requested_depth,
                maximum_supported_depth,
            ));
        }
        Ok(())
    }
}

impl WorthTopologyDomainQueryRequest {
    pub(crate) fn validate(&self) -> Result<(), WorthTopologyDomainQueryError> {
        let request_family = self.family();
        for step in self.traversal_steps() {
            step.validate(request_family)?;
        }
        Ok(())
    }

    pub(crate) fn family(&self) -> WorthTopologyDomainQueryRequestFamily {
        match self {
            Self::HalfEdgeSharedVertexNeighborhood { .. } => {
                WorthTopologyDomainQueryRequestFamily::HalfEdgeSharedVertexNeighborhood
            }
            Self::HalfEdgeRadialNeighborhood { .. } => {
                WorthTopologyDomainQueryRequestFamily::HalfEdgeRadialNeighborhood
            }
            Self::LoopCycleNeighborhood { .. } => {
                WorthTopologyDomainQueryRequestFamily::LoopCycleNeighborhood
            }
            Self::LocalRewireNeighborhood { .. } => {
                WorthTopologyDomainQueryRequestFamily::LocalRewireNeighborhood
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

    pub(crate) fn traversal_steps(&self) -> Vec<WorthTopologyDomainQueryTraversalStep> {
        match self {
            Self::HalfEdgeSharedVertexNeighborhood { .. } => vec![
                WorthTopologyDomainQueryTraversalStep::new(
                    WorthTopologyDomainTraversalRelation::HalfEdgeStartsAtVertex,
                    1,
                ),
                WorthTopologyDomainQueryTraversalStep::new(
                    WorthTopologyDomainTraversalRelation::HalfEdgeEndsAtVertex,
                    1,
                ),
            ],
            Self::HalfEdgeRadialNeighborhood { .. } => vec![
                WorthTopologyDomainQueryTraversalStep::new(
                    WorthTopologyDomainTraversalRelation::HalfEdgeRadialNext,
                    1,
                ),
                WorthTopologyDomainQueryTraversalStep::new(
                    WorthTopologyDomainTraversalRelation::HalfEdgeUsesEdge,
                    1,
                ),
            ],
            Self::LoopCycleNeighborhood { depth, .. } => {
                vec![WorthTopologyDomainQueryTraversalStep::new(
                    WorthTopologyDomainTraversalRelation::HalfEdgeNext,
                    *depth,
                )]
            }
            Self::LocalRewireNeighborhood { cycle_depth, .. } => vec![
                WorthTopologyDomainQueryTraversalStep::new(
                    WorthTopologyDomainTraversalRelation::HalfEdgeNext,
                    *cycle_depth,
                ),
                WorthTopologyDomainQueryTraversalStep::new(
                    WorthTopologyDomainTraversalRelation::HalfEdgePrev,
                    1,
                ),
            ],
        }
    }
}
