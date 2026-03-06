//! Explicit topology classification enums.
//!
//! DOMAIN: Canonical edge/vertex topology class labels used by
//! operators and validators to avoid ad-hoc structural checks.

use serde::{Deserialize, Serialize};

/// Explicit radial classification of an edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeRadialClass {
    /// Radial ring length = 1 (`radial_next == self`).
    Boundary,
    /// Radial ring length = 2.
    Manifold,
    /// Radial ring length >= 3.
    NonManifold,
}

/// Explicit disk classification of a vertex.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VertexDiskClass {
    /// Exactly one disk entry (`primary_disk` only).
    Single,
    /// More than one disk entry (`primary_disk` + extras).
    Multi { count: usize },
}
