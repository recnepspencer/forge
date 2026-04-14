mod diagnostics;
mod geometry;
mod lineage;
mod naming;
mod topology;

use forge_relational::facade::publication::AspectKey;
use forge_relational::facade::symbols::InternedString;
use serde::{Deserialize, Serialize};

pub use diagnostics::WorthDiagnosticsAspect;
pub use geometry::WorthGeometryAspect;
pub use lineage::WorthLineageAspect;
pub use naming::WorthNamingAspect;
pub use topology::WorthTopologyAspect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthAspect {
    Topology(WorthTopologyAspect),
    Geometry(WorthGeometryAspect),
    Lineage(WorthLineageAspect),
    Naming(WorthNamingAspect),
    Diagnostics(WorthDiagnosticsAspect),
}

impl WorthAspect {
    pub const ALL: [Self; 15] = [
        Self::Topology(WorthTopologyAspect::Structure),
        Self::Topology(WorthTopologyAspect::Ownership),
        Self::Topology(WorthTopologyAspect::Boundary),
        Self::Topology(WorthTopologyAspect::Radial),
        Self::Geometry(WorthGeometryAspect::Binding),
        Self::Geometry(WorthGeometryAspect::Embedding),
        Self::Geometry(WorthGeometryAspect::Provenance),
        Self::Geometry(WorthGeometryAspect::Approximation),
        Self::Geometry(WorthGeometryAspect::UvAnchoring),
        Self::Geometry(WorthGeometryAspect::Carrier),
        Self::Geometry(WorthGeometryAspect::Precision),
        Self::Geometry(WorthGeometryAspect::Fallback),
        Self::Lineage(WorthLineageAspect::Provenance),
        Self::Naming(WorthNamingAspect::PersistentName),
        Self::Diagnostics(WorthDiagnosticsAspect::Decisions),
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
