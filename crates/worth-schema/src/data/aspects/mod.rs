//! Worth aspect catalogs for platform bootstrap and descriptor assembly.
//!
//! This module is the authority for Worth-specific aspect identity and lower
//! publication contract data. It is not the ordinary Query lifecycle entry
//! surface; downstream runtime work should enter through `forge-query`.

mod diagnostics;
mod geometry;
mod lineage;
mod naming;
mod topology;

use forge_relational::facade::publication::AspectKey;
use forge_relational::facade::symbols::InternedString;
use serde::{Deserialize, Serialize};

pub use diagnostics::DiagnosticsAspect;
pub use geometry::GeometryAspect;
pub use lineage::LineageAspect;
pub use naming::NamingAspect;
pub use topology::TopologyAspect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Aspect {
    Topology(TopologyAspect),
    Geometry(GeometryAspect),
    Lineage(LineageAspect),
    Naming(NamingAspect),
    Diagnostics(DiagnosticsAspect),
}

impl Aspect {
    pub const ALL: [Self; 15] = [
        Self::Topology(TopologyAspect::Structure),
        Self::Topology(TopologyAspect::Ownership),
        Self::Topology(TopologyAspect::Boundary),
        Self::Topology(TopologyAspect::Radial),
        Self::Geometry(GeometryAspect::Binding),
        Self::Geometry(GeometryAspect::Embedding),
        Self::Geometry(GeometryAspect::Provenance),
        Self::Geometry(GeometryAspect::Approximation),
        Self::Geometry(GeometryAspect::UvAnchoring),
        Self::Geometry(GeometryAspect::Carrier),
        Self::Geometry(GeometryAspect::Precision),
        Self::Geometry(GeometryAspect::Fallback),
        Self::Lineage(LineageAspect::Provenance),
        Self::Naming(NamingAspect::PersistentName),
        Self::Diagnostics(DiagnosticsAspect::Decisions),
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Topology(aspect) => aspect.as_str(),
            Self::Geometry(aspect) => aspect.as_str(),
            Self::Lineage(aspect) => aspect.as_str(),
            Self::Naming(aspect) => aspect.as_str(),
            Self::Diagnostics(aspect) => aspect.as_str(),
        }
    }

    pub fn aspect_key(self) -> AspectKey {
        AspectKey(InternedString::Raw(self.as_str().to_string()))
    }
}
