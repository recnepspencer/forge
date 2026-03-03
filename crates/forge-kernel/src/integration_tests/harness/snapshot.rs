//! Entity count snapshots for delta assertions.
//!
//! DOMAIN: Captures a snapshot of entity counts before an operation,
//! then asserts expected deltas after. Eliminates hardcoding absolute
//! counts — tests express "MEF adds 1 face, 1 edge, 2 halfedges"
//! instead of "after MEF there are exactly 7 faces."
//!
//! ```rust,ignore
//! let snap = Snapshot::capture(draft.arena());
//! draft.execute(MakeEdgeFace { ... }).unwrap();
//! snap.assert_delta(draft.arena(), Delta {
//!     faces: 1, vertices: 0, edges: 1, half_edges: 2,
//!     loops: 1, shells: 0, bodies: 0,
//! });
//! ```

use forge_topo::b_rep::TopologyArena;

/// Captured entity counts at a point in time.
#[derive(Debug, Clone, Copy)]
pub struct Snapshot {
    pub faces: usize,
    pub vertices: usize,
    pub edges: usize,
    pub half_edges: usize,
    pub loops: usize,
    pub shells: usize,
    pub bodies: usize,
}

/// Expected change in entity counts (signed).
#[derive(Debug, Clone, Copy, Default)]
pub struct Delta {
    pub faces: i64,
    pub vertices: i64,
    pub edges: i64,
    pub half_edges: i64,
    pub loops: i64,
    pub shells: i64,
    pub bodies: i64,
}

impl Snapshot {
    /// Capture a snapshot of the current entity counts.
    pub fn capture(arena: &TopologyArena) -> Self {
        Self {
            faces: arena.face_count(),
            vertices: arena.vertex_count(),
            edges: arena.edge_count(),
            half_edges: arena.half_edge_count(),
            loops: arena.loop_count(),
            shells: arena.shell_count(),
            bodies: arena.body_count(),
        }
    }

    /// Assert that the entity counts changed by exactly the expected delta.
    ///
    /// Panics with a summary of ALL mismatches (not just the first one).
    pub fn assert_delta(&self, arena: &TopologyArena, delta: Delta) {
        let now = Self::capture(arena);
        let mut failures = Vec::new();

        let checks: [(&str, usize, usize, i64); 7] = [
            ("faces", self.faces, now.faces, delta.faces),
            ("vertices", self.vertices, now.vertices, delta.vertices),
            ("edges", self.edges, now.edges, delta.edges),
            ("half_edges", self.half_edges, now.half_edges, delta.half_edges),
            ("loops", self.loops, now.loops, delta.loops),
            ("shells", self.shells, now.shells, delta.shells),
            ("bodies", self.bodies, now.bodies, delta.bodies),
        ];

        for (name, before, after, expected_delta) in &checks {
            let actual_delta = *after as i64 - *before as i64;
            if actual_delta != *expected_delta {
                failures.push(format!(
                    "  {name}: {before} → {after} (delta={actual_delta:+}, expected={expected_delta:+})"
                ));
            }
        }

        if !failures.is_empty() {
            panic!(
                "Entity count delta mismatch ({} of 7 fields wrong):\n{}",
                failures.len(),
                failures.join("\n")
            );
        }
    }

    /// Assert that the entity counts are unchanged.
    pub fn assert_unchanged(&self, arena: &TopologyArena) {
        self.assert_delta(arena, Delta::default());
    }
}
