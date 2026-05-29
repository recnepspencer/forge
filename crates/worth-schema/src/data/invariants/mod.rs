mod diagnostics;
mod geometry;
mod lineage;
mod naming;
mod topology;

use serde::{Deserialize, Serialize};

pub use diagnostics::DiagnosticsInvariantGroup;
pub use geometry::GeometryInvariantGroup;
pub use lineage::LineageInvariantGroup;
pub use naming::NamingInvariantGroup;
pub use topology::TopologyInvariantGroup;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum InvariantGroup {
    Topology(TopologyInvariantGroup),
    Geometry(GeometryInvariantGroup),
    Lineage(LineageInvariantGroup),
    Naming(NamingInvariantGroup),
    Diagnostics(DiagnosticsInvariantGroup),
}

impl InvariantGroup {
    pub const ALL: [Self; 20] = [
        Self::Topology(TopologyInvariantGroup::OwnershipConsistency),
        Self::Topology(TopologyInvariantGroup::RequiredSingleRelationPresence),
        Self::Topology(TopologyInvariantGroup::LoopEntryCoherence),
        Self::Topology(TopologyInvariantGroup::HalfEdgeNextCoherence),
        Self::Topology(TopologyInvariantGroup::HalfEdgeRadialCoherence),
        Self::Topology(TopologyInvariantGroup::EdgeIncidenceLegality),
        Self::Topology(TopologyInvariantGroup::VertexOriginLegality),
        Self::Geometry(GeometryInvariantGroup::BindingCoverage),
        Self::Geometry(GeometryInvariantGroup::CarrierCompatibility),
        Self::Geometry(GeometryInvariantGroup::UvAnchoringContinuity),
        Self::Geometry(GeometryInvariantGroup::ApproximationBounded),
        Self::Geometry(GeometryInvariantGroup::ToleranceRegimeValidity),
        Self::Geometry(GeometryInvariantGroup::ProvenanceCompleteness),
        Self::Geometry(GeometryInvariantGroup::PrecisionEscalationDeclared),
        Self::Geometry(GeometryInvariantGroup::FallbackDispositionDeclared),
        Self::Geometry(GeometryInvariantGroup::FallbackProofSufficiency),
        Self::Lineage(LineageInvariantGroup::ProvenanceCompleteness),
        Self::Naming(NamingInvariantGroup::PersistentNameStability),
        Self::Naming(NamingInvariantGroup::PersistentNameUniqueness),
        Self::Diagnostics(DiagnosticsInvariantGroup::DecisionTraceCoverage),
    ];
}
