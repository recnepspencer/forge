use crate::construction::admitted_scaffold::family_birth_input::geometry::{
    orthotope_vertices, prism_vertices, pyramid_vertices, shell_with_hole_vertices,
    simplex_vertices, wire_body_vertices,
};
use worth_geom::facade::{
    build_direct_realization_report, realize_tetrahedron_support_with_altitude_component,
};
use worth_primitives::{
    canonical_orthotope_vertices, canonical_prism_vertices, canonical_pyramid_vertices,
    canonical_simplex_vertices, canonical_wire_body_vertices, derive_shell_with_hole_layout,
    shell_with_hole_vertices_from_layout, ShellWithHoleWitnessLayoutPolicy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveCanonicalWitnessParityReport {
    simplex_solid_verified: bool,
    orthotope_verified: bool,
    regular_prism_verified: bool,
    regular_pyramid_verified: bool,
    wire_body_verified: bool,
    shell_with_hole_verified: bool,
}

impl PrimitiveCanonicalWitnessParityReport {
    pub fn covers_expected_families(&self) -> bool {
        self.simplex_solid_verified
            && self.orthotope_verified
            && self.regular_prism_verified
            && self.regular_pyramid_verified
            && self.wire_body_verified
            && self.shell_with_hole_verified
    }
}

pub fn prepare_primitive_canonical_witness_parity_report() -> PrimitiveCanonicalWitnessParityReport
{
    let shell_layout =
        derive_shell_with_hole_layout(6, &[3, 4], ShellWithHoleWitnessLayoutPolicy::default())
            .expect("shell layout")
            .0;
    let canonical_simplex = canonical_simplex_vertices(1.0, 0.0)
        .local_vertices()
        .to_vec();
    let simplex_realization =
        realize_tetrahedron_support_with_altitude_component([0.0, 0.0, 0.0], 1.0, 0.0)
            .expect("simplex realization");
    let simplex_solid_verified = canonical_simplex == simplex_vertices(1.0, 0.0);
    let _ = build_direct_realization_report(
        "simplex_solid",
        &canonical_simplex,
        simplex_realization.planes(),
    )
    .geometry_digest()
    .to_string();
    let orthotope_verified = canonical_orthotope_vertices([1.0, 2.0, 3.0]).local_vertices()
        == orthotope_vertices([1.0, 2.0, 3.0]).as_slice();
    let regular_prism_verified = canonical_prism_vertices(6, 1.0, 2.0).local_vertices()
        == prism_vertices(6, 1.0, 2.0).as_slice();
    let regular_pyramid_verified = canonical_pyramid_vertices(5, 1.0, 2.0).local_vertices()
        == pyramid_vertices(5, 1.0, 2.0).as_slice();
    let wire_body_verified =
        canonical_wire_body_vertices(8).local_vertices() == wire_body_vertices(8, 1.5).as_slice();
    let shell_with_hole_verified = shell_with_hole_vertices_from_layout(6, &[3, 4], &shell_layout)
        .local_vertices()
        == shell_with_hole_vertices(6, &[3, 4])
            .expect("shell vertices")
            .as_slice();
    PrimitiveCanonicalWitnessParityReport {
        simplex_solid_verified,
        orthotope_verified,
        regular_prism_verified,
        regular_pyramid_verified,
        wire_body_verified,
        shell_with_hole_verified,
    }
}
