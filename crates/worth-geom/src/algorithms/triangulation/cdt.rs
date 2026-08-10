//! 2D Constrained Delaunay Triangulation (CDT) via Bowyer-Watson.
//!
//! DOMAIN: Triangulate a 2D polygon with constraint edges using incremental
//! point insertion (Bowyer-Watson) followed by constraint edge enforcement.
//!
//! ALGORITHM:
//! 1. Create a super-triangle containing all points.
//! 2. Insert each point by finding and removing triangles whose circumcircle
//!    contains the point, then retriangulate the resulting cavity.
//! 3. Enforce constraint edges by flipping non-constraint edges that cross them.
//! 4. Remove triangles connected to the super-triangle vertices.
//! 5. Remove triangles outside the polygon boundary.
//!
//! INVARIANTS:
//! - All orientation tests use `orient2d` from `worth_math` for exact predicates.
//! - Constraint edges appear as triangle edges in the final triangulation.
//! - No floating-point comparisons drive topology; only exact sign predicates.

use worth_math::predicates::incircle::incircle;
use worth_math::predicates::orient2d::orient2d;
use worth_math::sign::TriSign;

mod geometry;

/// Result of a CDT computation.
pub struct CdtResult {
    /// Triangle index triples into the original vertex array.
    pub triangles: Vec<[usize; 3]>,
}

/// Perform constrained Delaunay triangulation on a 2D polygon.
///
/// `vertices`: 2D points (polygon boundary + interior points).
/// `constraints`: pairs of vertex indices that must appear as edges.
/// `boundary`: ordered vertex indices forming the polygon boundary (closed loop).
pub fn triangulate_polygon_2d(
    vertices: &[[f64; 2]],
    constraints: &[[usize; 2]],
    boundary: &[usize],
) -> Result<CdtResult, worth_math::error::MathError> {
    let mut cdt = CdtState::new(vertices);

    for (i, &pt) in vertices.iter().enumerate() {
        cdt.insert_point(i, pt)?;
    }

    for &[a, b] in constraints {
        cdt.enforce_constraint(a, b)?;
    }

    cdt.remove_super_triangle();
    cdt.remove_exterior_triangles(boundary)?;

    Ok(CdtResult {
        triangles: cdt.collect_triangles(),
    })
}

/// Convenience: triangulate a face polygon with a single cut line.
///
/// `face_verts`: 2D positions of the polygon vertices (in order).
/// `cut_a`, `cut_b`: indices into `face_verts` for the cut line endpoints.
///
/// Returns triangles partitioning the polygon, with the cut line
/// as an internal edge sequence.
pub fn triangulate_face_with_cut(
    face_verts: &[[f64; 2]],
    cut_a: usize,
    cut_b: usize,
) -> Result<CdtResult, worth_math::error::MathError> {
    let n = face_verts.len();
    let boundary: Vec<usize> = (0..n).collect();

    let mut constraints = Vec::new();
    for i in 0..n {
        constraints.push([i, (i + 1) % n]);
    }
    constraints.push([cut_a, cut_b]);

    triangulate_polygon_2d(face_verts, &constraints, &boundary)
}

// ── Internal CDT state ───────────────────────────────────────────────────────

/// Index into the extended vertex array (original + 3 super-triangle vertices).
type VIdx = usize;

/// A triangle is three vertex indices.
#[derive(Clone, Copy, Debug)]
struct Triangle {
    v: [VIdx; 3],
    alive: bool,
}

struct CdtState {
    vertices: Vec<[f64; 2]>,
    triangles: Vec<Triangle>,
    n_original: usize,
}

impl CdtState {
    fn new(original_verts: &[[f64; 2]]) -> Self {
        let n = original_verts.len();

        let (min, max) = geometry::bounding_box(original_verts);
        let dx = (max[0] - min[0]).max(1e-15);
        let dy = (max[1] - min[1]).max(1e-15);
        let margin = (dx + dy) * 10.0;

        let cx = (min[0] + max[0]) * 0.5;
        let cy = (min[1] + max[1]) * 0.5;

        let s0 = [cx - margin * 2.0, cy - margin];
        let s1 = [cx + margin * 2.0, cy - margin];
        let s2 = [cx, cy + margin * 2.0];

        let mut vertices = original_verts.to_vec();
        vertices.push(s0);
        vertices.push(s1);
        vertices.push(s2);

        let super_tri = Triangle {
            v: [n, n + 1, n + 2],
            alive: true,
        };

        Self {
            vertices,
            triangles: vec![super_tri],
            n_original: n,
        }
    }

    /// Insert a point into the triangulation via Bowyer-Watson.
    fn insert_point(
        &mut self,
        _original_idx: usize,
        pt: [f64; 2],
    ) -> Result<(), worth_math::error::MathError> {
        let vidx = _original_idx;

        let bad_triangles = self.find_bad_triangles(pt);
        if bad_triangles.is_empty() {
            return Ok(());
        }

        let cavity_edges = self.find_cavity_boundary(&bad_triangles);

        for &ti in &bad_triangles {
            self.triangles[ti].alive = false;
        }

        for (a, b) in cavity_edges {
            self.triangles.push(Triangle {
                v: [vidx, a, b],
                alive: true,
            });
        }

        Ok(())
    }

    /// Find all triangles whose circumcircle contains `pt`.
    fn find_bad_triangles(&self, pt: [f64; 2]) -> Vec<usize> {
        let mut bad = Vec::new();
        for (i, tri) in self.triangles.iter().enumerate() {
            if tri.alive && self.in_circumcircle(tri, pt) {
                bad.push(i);
            }
        }
        bad
    }

    /// Find the boundary edges of the cavity formed by removing bad triangles.
    ///
    /// An edge is on the boundary if exactly one of its adjacent triangles
    /// is in the bad set.
    fn find_cavity_boundary(&self, bad: &[usize]) -> Vec<(VIdx, VIdx)> {
        let _bad_set: std::collections::BTreeSet<usize> = bad.iter().copied().collect();
        let mut boundary = Vec::new();

        for &ti in bad {
            let tri = &self.triangles[ti];
            for edge_idx in 0..3 {
                let a = tri.v[edge_idx];
                let b = tri.v[(edge_idx + 1) % 3];

                let shared_with_other_bad = bad.iter().any(|&other| {
                    other != ti
                        && self.triangles[other].alive
                        && geometry::triangle_has_edge(&self.triangles[other], a, b)
                });

                if !shared_with_other_bad {
                    boundary.push((a, b));
                }
            }
        }

        boundary
    }

    /// Test if point `p` is inside the circumcircle of triangle `tri`.
    ///
    /// Uses the robust Shewchuk incircle predicate for exact classification.
    fn in_circumcircle(&self, tri: &Triangle, p: [f64; 2]) -> bool {
        let a = self.vertices[tri.v[0]];
        let b = self.vertices[tri.v[1]];
        let c = self.vertices[tri.v[2]];

        let orient = orient2d(a, b, c);
        let orient_sign = match orient {
            Ok((s, _)) => s.sign(),
            Err(_) => return false,
        };

        if orient_sign == TriSign::Zero {
            return false;
        }

        let incircle_sign = match incircle(a, b, c, p) {
            Ok((s, _)) => s.sign(),
            Err(_) => return false,
        };

        if orient_sign == TriSign::Pos {
            incircle_sign == TriSign::Pos
        } else {
            incircle_sign == TriSign::Neg
        }
    }

    /// Enforce a constraint edge by flipping crossing edges.
    fn enforce_constraint(
        &mut self,
        a: usize,
        b: usize,
    ) -> Result<(), worth_math::error::MathError> {
        if self.edge_exists(a, b) {
            return Ok(());
        }

        let crossing = self.find_crossing_edges(a, b);

        for (ti, edge_a, edge_b) in crossing {
            if !self.triangles[ti].alive {
                continue;
            }
            self.flip_edge_toward_constraint(ti, edge_a, edge_b, a, b)?;
        }

        Ok(())
    }

    /// Check if edge (a, b) already exists in some alive triangle.
    fn edge_exists(&self, a: usize, b: usize) -> bool {
        self.triangles
            .iter()
            .any(|tri| tri.alive && geometry::triangle_has_edge(tri, a, b))
    }

    /// Find triangle edges that cross the constraint edge (a, b).
    fn find_crossing_edges(&self, a: usize, b: usize) -> Vec<(usize, VIdx, VIdx)> {
        let pa = self.vertices[a];
        let pb = self.vertices[b];
        let mut result = Vec::new();

        for (ti, tri) in self.triangles.iter().enumerate() {
            if !tri.alive {
                continue;
            }
            for edge_idx in 0..3 {
                let ea = tri.v[edge_idx];
                let eb = tri.v[(edge_idx + 1) % 3];
                if ea == a || ea == b || eb == a || eb == b {
                    continue;
                }
                if geometry::segments_cross(pa, pb, self.vertices[ea], self.vertices[eb]) {
                    result.push((ti, ea, eb));
                }
            }
        }

        result
    }

    /// Flip an edge that crosses the constraint, moving it toward the constraint line.
    fn flip_edge_toward_constraint(
        &mut self,
        tri_idx: usize,
        edge_a: VIdx,
        edge_b: VIdx,
        _constraint_a: usize,
        _constraint_b: usize,
    ) -> Result<(), worth_math::error::MathError> {
        let adj_idx = self.find_adjacent_triangle(tri_idx, edge_a, edge_b);
        let Some(adj) = adj_idx else {
            return Ok(());
        };

        let tri = self.triangles[tri_idx];
        let adj_tri = self.triangles[adj];

        let apex_1 = geometry::triangle_opposite_vertex(&tri, edge_a, edge_b);
        let apex_2 = geometry::triangle_opposite_vertex(&adj_tri, edge_a, edge_b);

        let Some(v1) = apex_1 else { return Ok(()) };
        let Some(v2) = apex_2 else { return Ok(()) };

        self.triangles[tri_idx].alive = false;
        self.triangles[adj].alive = false;

        self.triangles.push(Triangle {
            v: [v1, v2, edge_a],
            alive: true,
        });
        self.triangles.push(Triangle {
            v: [v1, v2, edge_b],
            alive: true,
        });

        Ok(())
    }

    /// Find the triangle sharing edge (a, b) with triangle `ti`.
    fn find_adjacent_triangle(&self, ti: usize, a: VIdx, b: VIdx) -> Option<usize> {
        for (i, tri) in self.triangles.iter().enumerate() {
            if i != ti && tri.alive && geometry::triangle_has_edge(tri, a, b) {
                return Some(i);
            }
        }
        None
    }

    /// Remove all triangles that reference super-triangle vertices.
    fn remove_super_triangle(&mut self) {
        let s0 = self.n_original;
        let s1 = self.n_original + 1;
        let s2 = self.n_original + 2;

        for tri in &mut self.triangles {
            if tri.alive {
                let has_super = tri.v.iter().any(|&v| v == s0 || v == s1 || v == s2);
                if has_super {
                    tri.alive = false;
                }
            }
        }
    }

    /// Remove triangles whose centroid is outside the polygon boundary.
    fn remove_exterior_triangles(
        &mut self,
        boundary: &[usize],
    ) -> Result<(), worth_math::error::MathError> {
        let n = boundary.len();
        if n < 3 {
            return Ok(());
        }

        for tri in &mut self.triangles {
            if !tri.alive {
                continue;
            }
            let centroid = geometry::triangle_centroid(
                self.vertices[tri.v[0]],
                self.vertices[tri.v[1]],
                self.vertices[tri.v[2]],
            );

            let inside = geometry::point_in_polygon_2d(&centroid, &self.vertices, boundary)?;
            if !inside {
                tri.alive = false;
            }
        }

        Ok(())
    }

    /// Collect surviving triangles as index triples into the original vertex array.
    fn collect_triangles(&self) -> Vec<[usize; 3]> {
        self.triangles
            .iter()
            .filter(|t| t.alive)
            .map(|t| t.v)
            .collect()
    }
}

#[cfg(test)]
mod tests;
