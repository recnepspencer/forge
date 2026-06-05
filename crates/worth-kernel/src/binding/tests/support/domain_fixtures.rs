use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
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
