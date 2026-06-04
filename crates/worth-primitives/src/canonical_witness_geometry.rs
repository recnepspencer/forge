const DEFAULT_HOLE_RADIUS: f64 = 0.4;
const DEFAULT_HOLE_CLEARANCE: f64 = 0.2;
const DEFAULT_BOUNDARY_CLEARANCE: f64 = 0.2;

pub const CANONICAL_SIMPLEX_LATERAL_RATIO: f64 = std::f64::consts::FRAC_1_SQRT_2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitivePlanarWitnessAuthority {
    CanonicalScaffoldWitness,
    RequestDerivedWitness,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveCanonicalWitnessGeometry {
    planar_authority: PrimitivePlanarWitnessAuthority,
    local_vertices: Vec<[f64; 3]>,
}

impl PrimitiveCanonicalWitnessGeometry {
    pub fn new(
        planar_authority: PrimitivePlanarWitnessAuthority,
        local_vertices: Vec<[f64; 3]>,
    ) -> Self {
        Self {
            planar_authority,
            local_vertices,
        }
    }

    pub fn planar_authority(&self) -> PrimitivePlanarWitnessAuthority {
        self.planar_authority
    }

    pub fn local_vertices(&self) -> &[[f64; 3]] {
        &self.local_vertices
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShellWithHoleWitnessLayoutPolicy {
    hole_radius: f64,
    hole_clearance: f64,
    boundary_clearance: f64,
}

impl Default for ShellWithHoleWitnessLayoutPolicy {
    fn default() -> Self {
        Self {
            hole_radius: DEFAULT_HOLE_RADIUS,
            hole_clearance: DEFAULT_HOLE_CLEARANCE,
            boundary_clearance: DEFAULT_BOUNDARY_CLEARANCE,
        }
    }
}

impl ShellWithHoleWitnessLayoutPolicy {
    pub fn hole_radius(self) -> f64 {
        self.hole_radius
    }

    pub fn hole_clearance(self) -> f64 {
        self.hole_clearance
    }

    pub fn boundary_clearance(self) -> f64 {
        self.boundary_clearance
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShellWithHoleWitnessLayout {
    outer_radius: f64,
    hole_radius: f64,
    hole_centers: Vec<[f64; 2]>,
}

impl ShellWithHoleWitnessLayout {
    pub fn outer_radius(&self) -> f64 {
        self.outer_radius
    }

    pub fn hole_radius(&self) -> f64 {
        self.hole_radius
    }

    pub fn hole_centers(&self) -> &[[f64; 2]] {
        &self.hole_centers
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShellWithHoleLayoutLegality {
    outer_apothem: f64,
    minimum_center_spacing: f64,
    maximum_center_radius: f64,
}

impl ShellWithHoleLayoutLegality {
    pub fn outer_apothem(&self) -> f64 {
        self.outer_apothem
    }

    pub fn minimum_center_spacing(&self) -> f64 {
        self.minimum_center_spacing
    }

    pub fn maximum_center_radius(&self) -> f64 {
        self.maximum_center_radius
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShellWithHoleWitnessLayoutError {
    OuterLoopTooSmall,
    HoleLoopTooSmall,
    MissingHoleLoop,
}

pub fn canonical_simplex_vertices(
    scale: f64,
    auxiliary_altitude_component: f64,
) -> PrimitiveCanonicalWitnessGeometry {
    PrimitiveCanonicalWitnessGeometry::new(
        PrimitivePlanarWitnessAuthority::RequestDerivedWitness,
        vec![
            [0.0, 0.0, scale],
            [0.0, scale, -scale],
            [-scale * CANONICAL_SIMPLEX_LATERAL_RATIO, -scale * 0.5, -scale],
            [
                scale * CANONICAL_SIMPLEX_LATERAL_RATIO,
                -scale * 0.5,
                -scale + auxiliary_altitude_component,
            ],
        ],
    )
}

pub fn canonical_orthotope_vertices(half_extents: [f64; 3]) -> PrimitiveCanonicalWitnessGeometry {
    let [hx, hy, hz] = half_extents;
    PrimitiveCanonicalWitnessGeometry::new(
        PrimitivePlanarWitnessAuthority::RequestDerivedWitness,
        vec![
            [-hx, -hy, -hz],
            [-hx, -hy, hz],
            [-hx, hy, -hz],
            [-hx, hy, hz],
            [hx, -hy, -hz],
            [hx, -hy, hz],
            [hx, hy, -hz],
            [hx, hy, hz],
        ],
    )
}

pub fn canonical_prism_vertices(
    sides: u32,
    radius: f64,
    height: f64,
) -> PrimitiveCanonicalWitnessGeometry {
    let half_height = height / 2.0;
    PrimitiveCanonicalWitnessGeometry::new(
        PrimitivePlanarWitnessAuthority::RequestDerivedWitness,
        (0..sides)
            .flat_map(|index| {
                let angle = std::f64::consts::TAU * index as f64 / sides as f64;
                let x = angle.cos() * radius;
                let y = angle.sin() * radius;
                [[x, y, -half_height], [x, y, half_height]]
            })
            .collect(),
    )
}

pub fn canonical_pyramid_vertices(
    sides: u32,
    radius: f64,
    height: f64,
) -> PrimitiveCanonicalWitnessGeometry {
    let mut vertices = regular_polygon_vertices([0.0, 0.0, 0.0], sides, radius);
    vertices.push([0.0, 0.0, height]);
    PrimitiveCanonicalWitnessGeometry::new(
        PrimitivePlanarWitnessAuthority::RequestDerivedWitness,
        vertices,
    )
}

pub fn canonical_wire_body_vertices(edge_count: u32) -> PrimitiveCanonicalWitnessGeometry {
    PrimitiveCanonicalWitnessGeometry::new(
        PrimitivePlanarWitnessAuthority::CanonicalScaffoldWitness,
        regular_polygon_vertices([0.0, 0.0, 0.0], edge_count, 1.5),
    )
}

pub fn derive_shell_with_hole_layout(
    outer_loop_edge_count: u32,
    hole_loop_edge_counts: &[u32],
    policy: ShellWithHoleWitnessLayoutPolicy,
) -> Result<(ShellWithHoleWitnessLayout, ShellWithHoleLayoutLegality), ShellWithHoleWitnessLayoutError>
{
    if outer_loop_edge_count < 3 {
        return Err(ShellWithHoleWitnessLayoutError::OuterLoopTooSmall);
    }
    if hole_loop_edge_counts.is_empty() {
        return Err(ShellWithHoleWitnessLayoutError::MissingHoleLoop);
    }
    if hole_loop_edge_counts.iter().any(|count| *count < 3) {
        return Err(ShellWithHoleWitnessLayoutError::HoleLoopTooSmall);
    }

    let minimum_center_spacing = 2.0 * policy.hole_radius() + policy.hole_clearance();
    let ring_radius = if hole_loop_edge_counts.len() == 1 {
        0.0
    } else {
        let angle = std::f64::consts::PI / hole_loop_edge_counts.len() as f64;
        minimum_center_spacing / (2.0 * angle.sin())
    };
    let outer_apothem = ring_radius + policy.hole_radius() + policy.boundary_clearance();
    let outer_radius = outer_apothem / (std::f64::consts::PI / outer_loop_edge_count as f64).cos();
    let layout = ShellWithHoleWitnessLayout {
        outer_radius,
        hole_radius: policy.hole_radius(),
        hole_centers: hole_loop_centers(hole_loop_edge_counts.len(), ring_radius),
    };
    let legality = ShellWithHoleLayoutLegality {
        outer_apothem,
        minimum_center_spacing,
        maximum_center_radius: outer_apothem - policy.hole_radius() - policy.boundary_clearance(),
    };
    Ok((layout, legality))
}

pub fn shell_with_hole_vertices_from_layout(
    outer_loop_edge_count: u32,
    hole_loop_edge_counts: &[u32],
    layout: &ShellWithHoleWitnessLayout,
) -> PrimitiveCanonicalWitnessGeometry {
    let mut vertices =
        regular_polygon_vertices([0.0, 0.0, 0.0], outer_loop_edge_count, layout.outer_radius());
    for (index, edge_count) in hole_loop_edge_counts.iter().copied().enumerate() {
        let center = layout.hole_centers()[index];
        vertices.extend(regular_polygon_vertices(
            [center[0], center[1], 0.0],
            edge_count,
            layout.hole_radius(),
        ));
    }
    PrimitiveCanonicalWitnessGeometry::new(
        PrimitivePlanarWitnessAuthority::CanonicalScaffoldWitness,
        vertices,
    )
}

fn regular_polygon_vertices(center: [f64; 3], sides: u32, radius: f64) -> Vec<[f64; 3]> {
    (0..sides)
        .map(|index| {
            let angle = std::f64::consts::TAU * index as f64 / sides as f64;
            [
                center[0] + angle.cos() * radius,
                center[1] + angle.sin() * radius,
                center[2],
            ]
        })
        .collect()
}

fn hole_loop_centers(count: usize, ring_radius: f64) -> Vec<[f64; 2]> {
    if count == 1 {
        return vec![[0.0, 0.0]];
    }
    (0..count)
        .map(|index| {
            let angle = std::f64::consts::TAU * index as f64 / count as f64;
            [angle.cos() * ring_radius, angle.sin() * ring_radius]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        canonical_simplex_vertices, derive_shell_with_hole_layout,
        shell_with_hole_vertices_from_layout, PrimitivePlanarWitnessAuthority,
        ShellWithHoleWitnessLayoutPolicy, CANONICAL_SIMPLEX_LATERAL_RATIO,
    };

    #[test]
    fn canonical_simplex_vertices_use_named_ratio_surface() {
        let vertices = canonical_simplex_vertices(2.0, 0.0);

        assert_eq!(
            vertices.local_vertices()[2],
            [
                -2.0 * CANONICAL_SIMPLEX_LATERAL_RATIO,
                -1.0,
                -2.0,
            ]
        );
    }

    #[test]
    fn shell_with_hole_layout_scales_with_hole_growth_and_stays_legal() {
        let (layout_small, legality_small) =
            derive_shell_with_hole_layout(6, &[3], ShellWithHoleWitnessLayoutPolicy::default())
                .expect("single hole layout");
        let (layout_large, legality_large) = derive_shell_with_hole_layout(
            12,
            &[5, 5, 5, 5, 5, 5],
            ShellWithHoleWitnessLayoutPolicy::default(),
        )
        .expect("multi-hole layout");

        assert!(layout_large.outer_radius() > layout_small.outer_radius());
        assert!(legality_large.outer_apothem() > legality_small.outer_apothem());
        assert_eq!(
            shell_with_hole_vertices_from_layout(12, &[5, 5, 5, 5, 5, 5], &layout_large)
                .planar_authority(),
            PrimitivePlanarWitnessAuthority::CanonicalScaffoldWitness
        );
    }
}
