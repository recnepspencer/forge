mod basis;
mod basis_identity;
mod certificate;
mod containment;
mod counters;
mod denial;
mod loop_basis;
mod validation;
mod winding;

pub use basis::CertifiedPolygonWinding2DBasis;
pub use certificate::CertifiedPolygonWinding2DReceipt;
pub use containment::CertifiedLoopContainment;
pub use counters::CertifiedPolygonWinding2DPerformanceCounters;
pub use denial::{
    CertifiedPolygonWinding2DDenial, CertifiedPolygonWinding2DDenialBasisLocus,
    CertifiedPolygonWinding2DDenialKind,
};
pub use loop_basis::CertifiedTopologyLoopBasis2D;
pub use winding::CertifiedLoopWinding;

pub(crate) use basis::{CertifiedLoopWindingSummary, ProjectedLoopVertexSnapshot};
pub(crate) use basis_identity::certified_polygon_winding_2d_identity_entries;
pub(crate) use containment::point_strictly_inside_loop;
