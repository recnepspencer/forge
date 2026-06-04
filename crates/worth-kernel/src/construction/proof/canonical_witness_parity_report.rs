use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
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
pub struct PrimitiveCanonicalWitnessParityRow {
    family: &'static str,
    shared_geometry_digest: String,
    kernel_geometry_digest: String,
    geom_geometry_digest: Option<String>,
}

impl PrimitiveCanonicalWitnessParityRow {
    pub fn family(&self) -> &str {
        self.family
    }

    pub fn shared_geometry_digest(&self) -> &str {
        &self.shared_geometry_digest
    }

    pub fn kernel_geometry_digest(&self) -> &str {
        &self.kernel_geometry_digest
    }

    pub fn geom_geometry_digest(&self) -> Option<&str> {
        self.geom_geometry_digest.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveCanonicalWitnessParityReport {
    rows: Vec<PrimitiveCanonicalWitnessParityRow>,
    report_digest: String,
}

impl PrimitiveCanonicalWitnessParityReport {
    pub fn rows(&self) -> &[PrimitiveCanonicalWitnessParityRow] {
        &self.rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_canonical_witness_parity_report(
) -> PrimitiveCanonicalWitnessParityReport {
    let shell_layout = derive_shell_with_hole_layout(
        6,
        &[3, 4],
        ShellWithHoleWitnessLayoutPolicy::default(),
    )
    .expect("shell layout")
    .0;
    let canonical_simplex = canonical_simplex_vertices(1.0, 0.0).local_vertices().to_vec();
    let simplex_realization =
        realize_tetrahedron_support_with_altitude_component([0.0, 0.0, 0.0], 1.0, 0.0)
            .expect("simplex realization");
    let rows = vec![
        row_with_geom(
            "simplex_solid",
            &canonical_simplex,
            &simplex_vertices(1.0, 0.0),
            build_direct_realization_report(
                "simplex_solid",
                &canonical_simplex,
                simplex_realization.planes(),
            )
            .geometry_digest()
            .to_string(),
        ),
        row(
            "orthotope",
            canonical_orthotope_vertices([1.0, 2.0, 3.0]).local_vertices(),
            &orthotope_vertices([1.0, 2.0, 3.0]),
        ),
        row(
            "regular_prism",
            canonical_prism_vertices(6, 1.0, 2.0).local_vertices(),
            &prism_vertices(6, 1.0, 2.0),
        ),
        row(
            "regular_pyramid",
            canonical_pyramid_vertices(5, 1.0, 2.0).local_vertices(),
            &pyramid_vertices(5, 1.0, 2.0),
        ),
        row(
            "wire_body",
            canonical_wire_body_vertices(8).local_vertices(),
            &wire_body_vertices(8, 1.5),
        ),
        row(
            "shell_with_hole",
            shell_with_hole_vertices_from_layout(6, &[3, 4], &shell_layout).local_vertices(),
            &shell_with_hole_vertices(6, &[3, 4]).expect("shell vertices"),
        ),
    ];
    assert_eq!(
        canonical_simplex,
        simplex_vertices(1.0, 0.0),
        "kernel simplex witness drifted from the shared canonical source"
    );
    assert_eq!(
        canonical_orthotope_vertices([1.0, 2.0, 3.0]).local_vertices(),
        orthotope_vertices([1.0, 2.0, 3.0]).as_slice(),
        "kernel orthotope witness drifted from the shared canonical source"
    );
    assert_eq!(
        canonical_prism_vertices(6, 1.0, 2.0).local_vertices(),
        prism_vertices(6, 1.0, 2.0).as_slice(),
        "kernel prism witness drifted from the shared canonical source"
    );
    assert_eq!(
        canonical_pyramid_vertices(5, 1.0, 2.0).local_vertices(),
        pyramid_vertices(5, 1.0, 2.0).as_slice(),
        "kernel pyramid witness drifted from the shared canonical source"
    );
    assert_eq!(
        canonical_wire_body_vertices(8).local_vertices(),
        wire_body_vertices(8, 1.5).as_slice(),
        "kernel wire witness drifted from the shared canonical source"
    );
    assert_eq!(
        shell_with_hole_vertices_from_layout(6, &[3, 4], &shell_layout).local_vertices(),
        shell_with_hole_vertices(6, &[3, 4])
            .expect("shell vertices")
            .as_slice(),
        "kernel shell-with-hole witness drifted from the shared canonical source"
    );
    for row in &rows {
        assert_eq!(
            row.shared_geometry_digest(),
            row.kernel_geometry_digest(),
            "kernel witness digest drifted from shared canonical digest for {}",
            row.family()
        );
    }
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &rows
            .iter()
            .flat_map(|row| {
                [
                    row.family().to_string(),
                    row.shared_geometry_digest().to_string(),
                    row.kernel_geometry_digest().to_string(),
                    row.geom_geometry_digest().unwrap_or("kernel-only").to_string(),
                ]
            })
            .collect::<Vec<_>>(),
    );
    PrimitiveCanonicalWitnessParityReport { rows, report_digest }
}

fn row(
    family: &'static str,
    shared_vertices: &[[f64; 3]],
    kernel_vertices: &[[f64; 3]],
) -> PrimitiveCanonicalWitnessParityRow {
    PrimitiveCanonicalWitnessParityRow {
        family,
        shared_geometry_digest: witness_digest(shared_vertices),
        kernel_geometry_digest: witness_digest(kernel_vertices),
        geom_geometry_digest: None,
    }
}

fn row_with_geom(
    family: &'static str,
    shared_vertices: &[[f64; 3]],
    kernel_vertices: &[[f64; 3]],
    geom_geometry_digest: String,
) -> PrimitiveCanonicalWitnessParityRow {
    PrimitiveCanonicalWitnessParityRow {
        family,
        shared_geometry_digest: witness_digest(shared_vertices),
        kernel_geometry_digest: witness_digest(kernel_vertices),
        geom_geometry_digest: Some(geom_geometry_digest),
    }
}

fn witness_digest(vertices: &[[f64; 3]]) -> String {
    digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &vertices
            .iter()
            .flat_map(|vertex| vertex.iter().map(|component| component.to_bits().to_string()))
            .collect::<Vec<_>>(),
    )
}
