//! Fluent verification chain for solid assertions.
//!
//! DOMAIN: Accumulates all assertion failures before panicking, so you
//! see every violation at once instead of fixing them one at a time.
//! Integrates with `dump` module to auto-export OBJ on failure.
//!
//! All geometry computations delegate to production algorithms in
//! `geometry::logic::measurements` — no novel algorithms here.
//!
//! ```rust,ignore
//! verify(&envelope)
//!     .euler(2)
//!     .faces(6)
//!     .manifold()
//!     .volume_approx(8.0, 1e-6)
//!     .pass();
//! ```

use crate::engine::facade::SolidEnvelope;
use crate::geometry::facade::{
    GeometryView, face_area, solid_volume,
};

/// Create a verifier for a `SolidEnvelope`.
pub fn verify(env: &SolidEnvelope) -> Verifier<'_> {
    Verifier {
        env,
        failures: Vec::new(),
        test_name: String::new(),
    }
}

/// Fluent assertion builder that accumulates failures.
pub struct Verifier<'a> {
    env: &'a SolidEnvelope,
    failures: Vec<String>,
    test_name: String,
}

impl<'a> Verifier<'a> {
    /// Set a test name for diagnostics and OBJ dump filenames.
    pub fn named(mut self, name: &str) -> Self {
        self.test_name = name.to_string();
        self
    }

    // ── Topology checks ──────────────────────────────────────────────────

    /// Assert Euler characteristic V - E + F = expected.
    pub fn euler(mut self, expected: i64) -> Self {
        let arena = self.env.topology().arena();
        let v = arena.vertex_count() as i64;
        let e = arena.edge_count() as i64;
        let f = arena.face_count() as i64;
        let chi = v - e + f;
        if chi != expected {
            self.failures.push(format!(
                "Euler: V({v}) - E({e}) + F({f}) = {chi}, expected {expected}"
            ));
        }
        self
    }

    /// Assert exact face count.
    pub fn faces(mut self, expected: usize) -> Self {
        let actual = self.env.topology().arena().face_count();
        if actual != expected {
            self.failures.push(format!("Faces: {actual}, expected {expected}"));
        }
        self
    }

    /// Assert exact vertex count.
    pub fn vertices(mut self, expected: usize) -> Self {
        let actual = self.env.topology().arena().vertex_count();
        if actual != expected {
            self.failures.push(format!("Vertices: {actual}, expected {expected}"));
        }
        self
    }

    /// Assert exact edge count.
    pub fn edges(mut self, expected: usize) -> Self {
        let actual = self.env.topology().arena().edge_count();
        if actual != expected {
            self.failures.push(format!("Edges: {actual}, expected {expected}"));
        }
        self
    }

    /// Assert the solid is manifold (all structural invariants pass).
    pub fn manifold(mut self) -> Self {
        let arena = self.env.topology().arena();
        if let Err(msg) = check_manifold(arena) {
            self.failures.push(format!("Manifold: {msg}"));
        }
        self
    }

    // ── Geometry checks (delegates to production algorithms) ─────────────

    /// Assert all faces have a plane and all vertices have a position.
    pub fn geometry_complete(mut self) -> Self {
        let arena = self.env.topology().arena();
        let geom = self.env.geometry();

        for (fid, _) in arena.iter_faces() {
            if geom.get_face_plane(fid).is_none() {
                self.failures.push(format!(
                    "Geometry: Face F#{} missing plane", fid.index()
                ));
            }
        }
        for (vid, _) in arena.iter_vertices() {
            if geom.get_vertex_position(vid).is_none() {
                self.failures.push(format!(
                    "Geometry: Vertex V#{} missing position", vid.index()
                ));
            }
        }
        self
    }

    /// Assert all face areas are above a minimum threshold.
    ///
    /// Delegates to `geometry::facade::face_area`.
    pub fn all_face_areas_above(mut self, min_area: f64) -> Self {
        let arena = self.env.topology().arena();
        let geom = self.env.geometry();

        for (fid, _) in arena.iter_faces() {
            let area = face_area(arena, geom, fid);
            if area <= min_area {
                self.failures.push(format!(
                    "Face area: F#{} has area {:.2e} ≤ {:.2e}",
                    fid.index(), area, min_area
                ));
            }
        }
        self
    }

    /// Assert the solid's volume is approximately `expected ± tol`.
    ///
    /// Delegates to `geometry::facade::solid_volume`.
    pub fn volume_approx(mut self, expected: f64, tol: f64) -> Self {
        let volume = solid_volume(self.env.topology().arena(), self.env.geometry());
        if (volume - expected).abs() > tol {
            self.failures.push(format!(
                "Volume: {volume:.6}, expected {expected:.6} ± {tol:.2e} (diff: {:.2e})",
                (volume - expected).abs()
            ));
        }
        self
    }

    /// Assert vertices within bounding box.
    pub fn bounds(mut self, min: [f64; 3], max: [f64; 3], tol: f64) -> Self {
        let arena = self.env.topology().arena();
        let geom = self.env.geometry();

        for (vid, _) in arena.iter_vertices() {
            if let Some(pos) = geom.get_vertex_position(vid) {
                for axis in 0..3 {
                    if pos[axis] < min[axis] - tol || pos[axis] > max[axis] + tol {
                        self.failures.push(format!(
                            "Bounds: V#{} axis[{}] = {:.6} outside [{:.6}, {:.6}]",
                            vid.index(), axis, pos[axis], min[axis], max[axis]
                        ));
                    }
                }
            }
        }
        self
    }

    // ── Terminal ──────────────────────────────────────────────────────────

    /// Consume the verifier and panic if any checks failed.
    ///
    /// On failure, dumps the solid to OBJ for visual inspection.
    pub fn pass(self) {
        if self.failures.is_empty() {
            return;
        }

        let name = if self.test_name.is_empty() {
            "unnamed_test".to_string()
        } else {
            self.test_name.clone()
        };

        // Try to dump OBJ for visual debugging
        let dump_msg = match super::dump::dump_to_obj(self.env, &name) {
            Ok(path) => format!("\n\nMesh dumped to: {}", path.display()),
            Err(e) => format!("\n\n(OBJ dump failed: {e})"),
        };

        panic!(
            "{} verification failure(s):{}{}",
            self.failures.len(),
            self.failures
                .iter()
                .enumerate()
                .map(|(i, f)| format!("\n  {}. {}", i + 1, f))
                .collect::<String>(),
            dump_msg
        );
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Check manifold invariants non-destructively, returning Err on failure.
fn check_manifold(arena: &forge_topo::b_rep::TopologyArena) -> Result<(), String> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.radial_next();
        if he_id != twin_id {
            let twin_data = arena.get_half_edge(twin_id)
                .map_err(|e| format!(
                    "Twin {} of he {} not found: {:?}",
                    twin_id.index(), he_id.index(), e
                ))?;
            if twin_data.radial_next() != he_id {
                return Err(format!(
                    "Twin reciprocity broken at he[{}]", he_id.index()
                ));
            }
        }
    }

    for (face_id, _) in arena.iter_faces() {
        let hes = arena.halfedges_of_face(face_id);
        if hes.is_empty() {
            return Err(format!("Face {} has no halfedges", face_id.index()));
        }
        let start = hes[0];
        let mut current = arena.get_half_edge(start)
            .map_err(|e| format!("{:?}", e))?.next();
        let mut count = 1;
        while current != start && count < 1000 {
            current = arena.get_half_edge(current)
                .map_err(|e| format!("{:?}", e))?.next();
            count += 1;
        }
        if current != start {
            return Err(format!("Face {} loop not closed", face_id.index()));
        }
    }

    Ok(())
}
