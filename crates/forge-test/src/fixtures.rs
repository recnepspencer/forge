//! Reusable test fixtures and topology builders.
//!
//! DOMAIN: Test infrastructure for all Forge crates.
//! INVARIANTS: Fixtures must produce deterministic, valid topologies.
//! DEPENDENCIES: forge-topo (state, handles)
//!
//! # Usage
//!
//! Import fixtures in any crate's tests:
//! ```ignore
//! use forge_test::fixtures;
//! let state = fixtures::build_empty_state();
//! ```

use forge_topo::state::TopologyState;

/// Build an empty topology state (epoch 0, no geometry).
pub fn build_empty_state() -> TopologyState {
    TopologyState::empty()
}

/// Build a committed state at epoch 1 (one mutation cycle, no actual geometry yet).
///
/// Useful when tests need a non-zero epoch as a starting point.
pub fn build_epoch_one_state() -> TopologyState {
    let state = TopologyState::empty();
    let draft = state.begin_mutation();
    draft.commit().expect("empty commit should not fail")
}

/// Topology counts for a known polyhedron (for Euler formula validation).
#[derive(Debug, Clone, Copy)]
pub struct PolyhedronCounts {
    /// Number of vertices
    pub vertices: usize,
    /// Number of edges
    pub edges: usize,
    /// Number of faces
    pub faces: usize,
}

/// Counts for a tetrahedron (V=4, E=6, F=4).
pub const TETRAHEDRON: PolyhedronCounts = PolyhedronCounts {
    vertices: 4,
    edges: 6,
    faces: 4,
};

/// Counts for a cube / hexahedron (V=8, E=12, F=6).
pub const CUBE: PolyhedronCounts = PolyhedronCounts {
    vertices: 8,
    edges: 12,
    faces: 6,
};

/// Counts for an octahedron (V=6, E=12, F=8).
pub const OCTAHEDRON: PolyhedronCounts = PolyhedronCounts {
    vertices: 6,
    edges: 12,
    faces: 8,
};

/// Counts for an icosahedron (V=12, E=30, F=20).
pub const ICOSAHEDRON: PolyhedronCounts = PolyhedronCounts {
    vertices: 12,
    edges: 30,
    faces: 20,
};

impl PolyhedronCounts {
    /// Verify the Euler formula holds: V - E + F = 2 (for genus-0 solids).
    pub fn satisfies_euler_formula(&self) -> bool {
        (self.vertices as isize) - (self.edges as isize) + (self.faces as isize) == 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_state_has_epoch_zero() {
        let state = build_empty_state();
        assert_eq!(state.epoch(), 0);
    }

    #[test]
    fn epoch_one_state_has_epoch_one() {
        let state = build_epoch_one_state();
        assert_eq!(state.epoch(), 1);
    }

    #[test]
    fn tetrahedron_satisfies_euler() {
        assert!(TETRAHEDRON.satisfies_euler_formula());
    }

    #[test]
    fn cube_satisfies_euler() {
        assert!(CUBE.satisfies_euler_formula());
    }

    #[test]
    fn octahedron_satisfies_euler() {
        assert!(OCTAHEDRON.satisfies_euler_formula());
    }

    #[test]
    fn icosahedron_satisfies_euler() {
        assert!(ICOSAHEDRON.satisfies_euler_formula());
    }
}
