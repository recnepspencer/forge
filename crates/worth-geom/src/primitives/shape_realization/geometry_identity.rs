use crate::primitives::plane::Plane;
use worth_primitives::{
    PrimitiveGeometryIdentityBundle, PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity,
};

pub(super) fn geometry_identity_bundle(
    planes: &[Plane],
    vertices: &[[f64; 3]],
) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(
        planes.iter().map(plane_identity).collect(),
        vertices
            .iter()
            .copied()
            .map(PrimitiveVertexIdentity::from_position)
            .collect(),
    )
}

fn plane_identity(plane: &Plane) -> PrimitiveSupportPlaneIdentity {
    let (a, b, c, d) = plane.exact_coefficients();
    PrimitiveSupportPlaneIdentity::new(a.to_string(), b.to_string(), c.to_string(), d.to_string())
}
