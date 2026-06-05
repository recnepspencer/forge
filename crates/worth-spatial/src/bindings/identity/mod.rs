mod binding_identity;
mod identity_basis;

pub use binding_identity::SpatialBindingIdentity;
pub(crate) use identity_basis::{
    coedge_pcurve_basis, direction_anchor_basis, edge_curve_basis, face_surface_basis,
    point_anchor_basis, vertex_geometry_basis,
};
