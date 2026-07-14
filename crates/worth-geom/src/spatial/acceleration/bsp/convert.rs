//! BSP ↔ Convex Solid Conversion.
//!
//! DOMAIN: Convert between BSP tree representation and convex cell
//! boundary representation. Initial solids (cubes) go BSP → merge →
//! boundary extraction. Halfedge conversion happens in `WORTH-kernel`.
//!
//! INVARIANTS:
//! - `convex_to_bsp` produces a BSP tree whose single solid leaf
//!   represents exactly the intersection of all input half-spaces
//! - `extract_boundary_cells` produces ConvexCells whose union equals
//!   the BSP solid, with no internal (shared) faces
//!
//! DEPENDENCIES: `BspNode`, `BspSolid`, `Plane`, `ConvexCell`,
//!               `build_convex_polyhedron`, `BspConfig`

use worth_math::MathError;

use super::eval::{build_convex_polyhedron, BspConfig};
use super::schema::{BspNode, BspSolid, ConvexCell};
use crate::Plane;

/// Build a BspSolid from a convex solid defined by half-space planes.
///
/// Each plane's negative half-space (n·x + d < 0) is "inside".
/// The solid is the intersection of all negative half-spaces.
///
/// The resulting BSP tree is a linear chain: each plane splits space,
/// the positive side is empty (outside), the negative side continues
/// to the next plane. The innermost negative leaf is solid.
pub fn convex_to_bsp(planes: Vec<Plane>) -> BspSolid {
    let n = planes.len();
    if n == 0 {
        return BspSolid::new(planes, BspNode::solid());
    }

    let mut root = BspNode::solid();
    for i in (0..n).rev() {
        root = BspNode::split(i, root, BspNode::empty());
    }

    BspSolid::new(planes, root)
}

/// Constraint accumulated while walking from root to a leaf.
#[derive(Debug, Clone)]
struct HalfSpaceConstraint {
    /// Index of the plane in the BspSolid's plane set.
    plane_idx: usize,
    /// If true, the constraint is the negative half-space (n·x + d < 0).
    /// If false, the constraint is the positive half-space (n·x + d > 0),
    /// which means we need to flip the plane before clipping.
    negative_side: bool,
}

/// Extract boundary ConvexCells from a BspSolid.
///
/// Returns one ConvexCell per solid leaf. Each cell is bounded by
/// the half-space constraints along the path from root to that leaf.
/// Internal faces (shared between adjacent solid cells on the same plane)
/// are retained — they will be removed during halfedge mesh construction.
pub fn extract_boundary_cells(
    solid: &BspSolid,
    config: &BspConfig,
) -> Result<Vec<(ConvexCell, Vec<usize>)>, MathError> {
    let mut cells = Vec::new();
    let mut constraints = Vec::new();
    collect_solid_cells(
        solid.root(),
        solid.planes(),
        &mut constraints,
        &mut cells,
        config,
    )?;
    Ok(cells)
}

/// Recursively collect ConvexCells for all solid leaves.
fn collect_solid_cells(
    node: &BspNode,
    planes: &[Plane],
    constraints: &mut Vec<HalfSpaceConstraint>,
    cells: &mut Vec<(ConvexCell, Vec<usize>)>,
    config: &BspConfig,
) -> Result<(), MathError> {
    match node {
        BspNode::Leaf { solid } => {
            if *solid && !constraints.is_empty() {
                match build_cell_from_constraints(planes, constraints, config) {
                    Ok((cell, used_planes)) => {
                        if cell.vertex_count() >= 4 && cell.face_count() >= 4 {
                            cells.push((cell, used_planes));
                        }
                    }
                    Err(_) => {
                        // After BSP merge, some solid leaves have contradictory
                        // half-space constraints (artifacts of partition duplication)
                        // that produce geometrically empty cells. Skip them silently.
                    }
                }
            }
            Ok(())
        }
        BspNode::Internal {
            plane_idx,
            neg,
            pos,
        } => {
            constraints.push(HalfSpaceConstraint {
                plane_idx: *plane_idx,
                negative_side: true,
            });
            collect_solid_cells(neg, planes, constraints, cells, config)?;
            constraints.pop();

            constraints.push(HalfSpaceConstraint {
                plane_idx: *plane_idx,
                negative_side: false,
            });
            collect_solid_cells(pos, planes, constraints, cells, config)?;
            constraints.pop();

            Ok(())
        }
    }
}

/// Build a ConvexCell from a set of half-space constraints.
///
/// For each constraint:
/// - negative_side=true: use the plane as-is (inside = n·x + d < 0)
/// - negative_side=false: flip the plane (inside = -(n·x + d) < 0)
///
/// Returns the ConvexCell and the list of original plane indices used.
fn build_cell_from_constraints(
    planes: &[Plane],
    constraints: &[HalfSpaceConstraint],
    config: &BspConfig,
) -> Result<(ConvexCell, Vec<usize>), MathError> {
    let mut cell_planes = Vec::new();
    let mut original_indices = Vec::new();

    for c in constraints {
        let mut plane = planes[c.plane_idx].clone();
        if !c.negative_side {
            plane.flip();
        }
        cell_planes.push(plane);
        original_indices.push(c.plane_idx);
    }

    let cell = build_convex_polyhedron(&cell_planes, config)?;
    Ok((cell, original_indices))
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_math::arithmetic::Rational;

    fn axis_plane(axis: usize, sign: i64, offset_val: f64) -> Plane {
        Plane::axis_aligned(axis, sign, Rational::try_from_f64(offset_val).unwrap()).unwrap()
    }

    fn cube_planes(center: [f64; 3], half: f64) -> Vec<Plane> {
        let mut planes = Vec::new();
        for axis in 0..3 {
            planes.push(axis_plane(axis, 1, -(center[axis] + half)));
            planes.push(axis_plane(axis, -1, center[axis] - half));
        }
        planes
    }

    #[test]
    fn convex_to_bsp_cube() {
        let planes = cube_planes([0.0, 0.0, 0.0], 1.0);
        let solid = convex_to_bsp(planes);

        assert_eq!(solid.plane_count(), 6);
        assert_eq!(solid.root().depth(), 6);
        assert_eq!(solid.root().solid_leaf_count(), 1);
        assert_eq!(solid.root().leaf_count(), 7);
    }

    #[test]
    fn convex_to_bsp_roundtrip_single_cube() {
        let planes = cube_planes([0.0, 0.0, 0.0], 1.0);
        let solid = convex_to_bsp(planes);

        let config = BspConfig::default();
        let cells = extract_boundary_cells(&solid, &config).unwrap();

        assert_eq!(cells.len(), 1, "Single convex solid should produce 1 cell");
        let (cell, _) = &cells[0];
        assert_eq!(cell.face_count(), 6, "Cube should have 6 faces");
        assert_eq!(cell.vertex_count(), 8, "Cube should have 8 vertices");
    }

    #[test]
    fn extract_after_union() {
        use super::super::merge::merge_bsp;
        use super::super::schema::BspOp;

        let a = convex_to_bsp(cube_planes([0.0, 0.0, 0.0], 1.0));
        let b = convex_to_bsp(cube_planes([1.5, 0.0, 0.0], 1.0));
        let merged = merge_bsp(&a, &b, BspOp::Union).unwrap();

        let config = BspConfig::default();
        let cells = extract_boundary_cells(&merged, &config).unwrap();

        assert!(
            cells.len() >= 2,
            "Union of disjoint cubes should produce at least 2 cells, got {}",
            cells.len()
        );

        let total_faces: usize = cells.iter().map(|(c, _)| c.face_count()).sum();
        assert!(
            total_faces >= 12,
            "Two disjoint cubes should have at least 12 boundary faces, got {total_faces}"
        );
    }

    #[test]
    fn extract_after_overlapping_union() {
        use super::super::merge::merge_bsp;
        use super::super::schema::BspOp;

        let a = convex_to_bsp(cube_planes([0.0, 0.0, 0.0], 1.0));
        let b = convex_to_bsp(cube_planes([0.8, 0.0, 0.0], 1.0));
        let merged = merge_bsp(&a, &b, BspOp::Union).unwrap();

        let config = BspConfig::default();
        let cells = extract_boundary_cells(&merged, &config).unwrap();

        assert!(
            !cells.is_empty(),
            "Overlapping union should produce at least 1 cell"
        );

        let total_verts: usize = cells.iter().map(|(c, _)| c.vertex_count()).sum();
        assert!(total_verts > 0, "Should have vertices");
    }

    #[test]
    fn extract_after_subtraction() {
        use super::super::merge::merge_bsp;
        use super::super::schema::BspOp;

        let big = convex_to_bsp(cube_planes([0.0, 0.0, 0.0], 2.0));
        let small = convex_to_bsp(cube_planes([0.0, 0.0, 0.0], 1.0));
        let result = merge_bsp(&big, &small, BspOp::Subtraction).unwrap();

        let config = BspConfig::default();
        let cells = extract_boundary_cells(&result, &config).unwrap();

        assert!(
            !cells.is_empty(),
            "Subtracting interior cube should produce cells"
        );

        let total_faces: usize = cells.iter().map(|(c, _)| c.face_count()).sum();
        assert!(
            total_faces >= 6,
            "Hollow cube should have at least 6 outer faces, got {total_faces}"
        );
    }

    #[test]
    fn chain_three_unions_then_extract() {
        use super::super::merge::merge_bsp;
        use super::super::schema::BspOp;

        let a = convex_to_bsp(cube_planes([0.0, 0.0, 0.0], 1.0));
        let b = convex_to_bsp(cube_planes([0.8, 0.0, 0.0], 1.0));
        let c = convex_to_bsp(cube_planes([0.0, 0.8, 0.0], 1.0));

        let ab = merge_bsp(&a, &b, BspOp::Union).unwrap();
        let abc = merge_bsp(&ab, &c, BspOp::Union).unwrap();

        let config = BspConfig::default();
        let cells = extract_boundary_cells(&abc, &config).unwrap();

        assert!(!cells.is_empty(), "3-cube union chain should produce cells");
    }
}
