use worth_primitives::PrimitiveConstructionFamilyKey;

pub type PrimitiveConstructionFamily = PrimitiveConstructionFamilyKey;

pub const PRIMITIVE_CONSTRUCTION_FAMILIES: [PrimitiveConstructionFamily; 6] = [
    PrimitiveConstructionFamily::SimplexSolid,
    PrimitiveConstructionFamily::Orthotope,
    PrimitiveConstructionFamily::RegularPrism,
    PrimitiveConstructionFamily::RegularPyramid,
    PrimitiveConstructionFamily::WireBody,
    PrimitiveConstructionFamily::ShellWithHole,
];

#[cfg(test)]
pub fn primitive_construction_topology_birth_class(
    family: PrimitiveConstructionFamily,
) -> &'static str {
    match family {
        PrimitiveConstructionFamily::SimplexSolid => "closed_simplex_body",
        PrimitiveConstructionFamily::Orthotope => "closed_orthotope_body",
        PrimitiveConstructionFamily::RegularPrism => "closed_regular_prism_body",
        PrimitiveConstructionFamily::RegularPyramid => "closed_regular_pyramid_body",
        PrimitiveConstructionFamily::WireBody => "planar_wire_body",
        PrimitiveConstructionFamily::ShellWithHole => "planar_shell_with_hole_body",
    }
}
