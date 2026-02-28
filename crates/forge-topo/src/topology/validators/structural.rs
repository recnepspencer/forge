//! Structural topology validation (commit-time invariant checking).
//!
//! DOMAIN: Pure connectivity checks that require no geometry data.
//!
//! This module acts as a dispatcher, calling category-specific validators
//! defined in the subdirectories of `topology/validators`.

use crate::arena::TopologyArena;
use crate::topology::validators::validate::ValidationLevel;
use forge_core::KernelError;

use super::loop_wiring;
use super::radial_edge;
use super::vertex_disk;
use super::shell_closure;
use super::euler_genus;
use super::reference_integrity;

/// Validate structural topology of an arena with the specified strictness.
///
/// Called automatically by `MutableDraft::commit()`. Runs structural checks
/// (pointer coherence, loop closure, hierarchy, Euler formula) and
/// shell-kind-aware manifold enforcement (Solid shells require valence ≤ 2).
pub fn validate_topology(arena: &TopologyArena, level: ValidationLevel) -> Result<(), KernelError> {
    if level == ValidationLevel::None {
        return Ok(());
    }

    // Tier 1a: Core pointer coherence and local invariants.
    radial_edge::validate_radial_rings(arena)?;
    radial_edge::validate_radial_edge_consistency(arena)?;
    loop_wiring::validate_prev_consistency(arena)?;
    loop_wiring::validate_vertex_continuity(arena)?;
    vertex_disk::validate_vertex_outgoing(arena)?;

    if level == ValidationLevel::Intermediate || level == ValidationLevel::Full {
        loop_wiring::validate_loops(arena)?;
        reference_integrity::validate_hierarchy(arena)?;
    }

    if level == ValidationLevel::Full {
        euler_genus::validate_euler(arena)?;
        shell_closure::validate_shell_consistency(arena)?;
        shell_closure::validate_manifold_edges(arena)?;
        shell_closure::validate_orientation_consistency(arena)?;
    }

    Ok(())
}

/// Re-export for external callers that need manifold checks directly.
pub use shell_closure::validate_manifold_edges;


