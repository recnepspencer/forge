mod binding_identity;
mod canonical_entries;
mod identity_basis;

pub use binding_identity::SpatialBindingIdentity;
pub(crate) use identity_basis::{
    coedge_pcurve_basis, edge_curve_basis, face_surface_basis, vertex_geometry_basis,
};
