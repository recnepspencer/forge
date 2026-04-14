mod diagnostics;
mod geometry;
mod lineage;
mod naming;
mod topology;

use serde::{Deserialize, Serialize};

pub use diagnostics::WorthDiagnosticsInvariantGroup;
pub use geometry::WorthGeometryInvariantGroup;
pub use lineage::WorthLineageInvariantGroup;
pub use naming::WorthNamingInvariantGroup;
pub use topology::WorthTopologyInvariantGroup;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthInvariantGroup {
    Topology(WorthTopologyInvariantGroup),
    Geometry(WorthGeometryInvariantGroup),
    Lineage(WorthLineageInvariantGroup),
    Naming(WorthNamingInvariantGroup),
    Diagnostics(WorthDiagnosticsInvariantGroup),
}

impl WorthInvariantGroup {
    pub const ALL: [Self; 20] = [
        Self::Topology(WorthTopologyInvariantGroup::OwnershipConsistency),
        Self::Topology(WorthTopologyInvariantGroup::RequiredSingleRelationPresence),
        Self::Topology(WorthTopologyInvariantGroup::LoopEntryCoherence),
        Self::Topology(WorthTopologyInvariantGroup::HalfEdgeNextCoherence),
        Self::Topology(WorthTopologyInvariantGroup::HalfEdgeRadialCoherence),
        Self::Topology(WorthTopologyInvariantGroup::EdgeIncidenceLegality),
        Self::Topology(WorthTopologyInvariantGroup::VertexOriginLegality),
        Self::Geometry(WorthGeometryInvariantGroup::BindingCoverage),
        Self::Geometry(WorthGeometryInvariantGroup::CarrierCompatibility),
        Self::Geometry(WorthGeometryInvariantGroup::UvAnchoringContinuity),
        Self::Geometry(WorthGeometryInvariantGroup::ApproximationBounded),
        Self::Geometry(WorthGeometryInvariantGroup::ToleranceRegimeValidity),
        Self::Geometry(WorthGeometryInvariantGroup::ProvenanceCompleteness),
        Self::Geometry(WorthGeometryInvariantGroup::PrecisionEscalationDeclared),
        Self::Geometry(WorthGeometryInvariantGroup::FallbackDispositionDeclared),
        Self::Geometry(WorthGeometryInvariantGroup::FallbackProofSufficiency),
        Self::Lineage(WorthLineageInvariantGroup::ProvenanceCompleteness),
        Self::Naming(WorthNamingInvariantGroup::PersistentNameStability),
        Self::Naming(WorthNamingInvariantGroup::PersistentNameUniqueness),
        Self::Diagnostics(WorthDiagnosticsInvariantGroup::DecisionTraceCoverage),
    ];
}
