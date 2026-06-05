use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveCurvedSupportIdentity,
    PrimitiveGeometryIdentityBundle, PrimitiveSupportPlaneIdentity,
    PrimitiveTriaxialEllipsoidIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};

pub(crate) fn canonical_geometry(vertices: [[f64; 3]; 2]) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(
        vec![PrimitiveSupportPlaneIdentity::new(
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
        )],
        vertices
            .into_iter()
            .map(PrimitiveVertexIdentity::from_position)
            .collect(),
    )
}

pub(crate) fn triaxial_ellipsoid_geometry(
    axis_u: [f64; 3],
    axis_v: [f64; 3],
    axis_w: [f64; 3],
    radii: [f64; 3],
    vertices: [[f64; 3]; 2],
) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::with_curved_support(
        vec![],
        vec![PrimitiveCurvedSupportIdentity::TriaxialEllipsoid(
            PrimitiveTriaxialEllipsoidIdentity::new(
                [0.0, 0.0, 0.0],
                axis_u,
                axis_v,
                axis_w,
                radii[0],
                radii[1],
                radii[2],
            ),
        )],
        vertices
            .into_iter()
            .map(PrimitiveVertexIdentity::from_position)
            .collect(),
    )
}

pub(crate) fn orthotope_contract() -> worth_primitives::PrimitiveConstructionBirthSynopsisContract {
    PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    )
}

pub(crate) fn shell_with_hole_contract(
) -> worth_primitives::PrimitiveConstructionBirthSynopsisContract {
    PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::ShellWithHole {
            outer_loop_edge_count: 4,
            hole_loop_edge_counts: vec![3],
        },
    )
}
