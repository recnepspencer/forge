use super::error::TopologyReadError;
use crate::projection::runtime_boundary::read_lowering::schema::TopologyDomainTraversalRelation;
use forge_query::facade::{ForgeQueryEntityIdentity, RelationName};

use crate::projection::read_views::domain::read_proof::report::TopologyReadRequestFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadAnchorIdentity {
    value: String,
    authority: TopologyReadAnchorAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TopologyReadAnchorAuthority {
    QueryEntityIdentity,
    RuntimeRowLabel,
}

impl TopologyReadAnchorIdentity {
    pub fn from_query_entity_identity(
        identity: &ForgeQueryEntityIdentity,
    ) -> Result<Self, TopologyReadError> {
        let Some(parts) = identity.relational_entity_record_parts() else {
            if identity.relational_record_parts().is_some() {
                return Err(authority_denial(
                    TopologyReadAnchorAuthority::QueryEntityIdentity,
                    "query relation identity cannot anchor topology entity read",
                ));
            }
            return Err(authority_denial(
                TopologyReadAnchorAuthority::QueryEntityIdentity,
                "non-relational query entity identity",
            ));
        };
        Ok(Self {
            value: format!(
                "entity:{}:{}:{}",
                parts.partition_id(),
                parts.local_slot(),
                parts.generation()
            ),
            authority: TopologyReadAnchorAuthority::QueryEntityIdentity,
        })
    }

    pub(crate) fn from_runtime_row_label(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            authority: TopologyReadAnchorAuthority::RuntimeRowLabel,
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        self.value.as_str()
    }

    fn validate(&self) -> Result<(), TopologyReadError> {
        let trimmed = self.value.trim();
        if trimmed.is_empty() {
            return Err(authority_denial(
                self.authority,
                "empty topology read anchor identity",
            ));
        }
        if trimmed.starts_with("projection:")
            || trimmed.starts_with("cached:")
            || trimmed.contains("terminal_projection")
        {
            return Err(authority_denial(
                self.authority,
                "projection-reconstructed topology read anchor identity",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TopologyReadRequest {
    HalfEdgeSharedVertexNeighborhood {
        source_half_edge_identity: TopologyReadAnchorIdentity,
    },
    HalfEdgeRadialNeighborhood {
        source_half_edge_identity: TopologyReadAnchorIdentity,
    },
    ShellBoundaryNeighborhood {
        source_half_edge_identity: TopologyReadAnchorIdentity,
    },
    LoopCycleNeighborhood {
        start_half_edge_identity: TopologyReadAnchorIdentity,
        depth: u8,
    },
    LocalRewireNeighborhood {
        moved_half_edge_identity: TopologyReadAnchorIdentity,
        cycle_depth: u8,
    },
    WireNeighborhood {
        source_half_edge_identity: TopologyReadAnchorIdentity,
        wire_depth: u8,
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
        self.anchor().validate()?;
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
            Self::ShellBoundaryNeighborhood { .. } => {
                TopologyReadRequestFamily::ShellBoundaryNeighborhood
            }
            Self::LoopCycleNeighborhood { .. } => TopologyReadRequestFamily::LoopCycleNeighborhood,
            Self::LocalRewireNeighborhood { .. } => {
                TopologyReadRequestFamily::LocalRewireNeighborhood
            }
            Self::WireNeighborhood { .. } => TopologyReadRequestFamily::WireNeighborhood,
        }
    }

    pub(crate) fn anchor_identity(&self) -> &str {
        self.anchor().as_str()
    }

    fn anchor(&self) -> &TopologyReadAnchorIdentity {
        match self {
            Self::HalfEdgeSharedVertexNeighborhood {
                source_half_edge_identity,
            }
            | Self::HalfEdgeRadialNeighborhood {
                source_half_edge_identity,
            }
            | Self::ShellBoundaryNeighborhood {
                source_half_edge_identity,
            } => source_half_edge_identity,
            Self::LoopCycleNeighborhood {
                start_half_edge_identity,
                ..
            } => start_half_edge_identity,
            Self::LocalRewireNeighborhood {
                moved_half_edge_identity,
                ..
            } => moved_half_edge_identity,
            Self::WireNeighborhood {
                source_half_edge_identity,
                ..
            } => source_half_edge_identity,
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
            Self::ShellBoundaryNeighborhood { .. } => vec![
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
            Self::WireNeighborhood { wire_depth, .. } => vec![
                TopologyReadTraversalStep::new(
                    TopologyDomainTraversalRelation::HalfEdgeNext,
                    *wire_depth,
                ),
                TopologyReadTraversalStep::new(TopologyDomainTraversalRelation::HalfEdgePrev, 1),
            ],
        }
    }
}

fn authority_denial(
    authority: TopologyReadAnchorAuthority,
    reason: &'static str,
) -> TopologyReadError {
    let authority_label = match authority {
        TopologyReadAnchorAuthority::QueryEntityIdentity => "query_entity_identity",
        TopologyReadAnchorAuthority::RuntimeRowLabel => "runtime_row_label",
    };
    TopologyReadError::runtime_boundary_authority_denied(format!(
        "worth-topo/runtime_boundary/read_lowering rejected {authority_label}: {reason}"
    ))
}

fn wire_neighborhood_request_shape() -> TopologyReadRequest {
    TopologyReadRequest::WireNeighborhood {
        source_half_edge_identity: TopologyReadAnchorIdentity::from_runtime_row_label(
            "wire-neighborhood-shape",
        ),
        wire_depth: 1,
    }
}

const _: fn() -> TopologyReadRequest = wire_neighborhood_request_shape;
