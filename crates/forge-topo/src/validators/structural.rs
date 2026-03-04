//! Structural topology validation (commit-time invariant checking).
//!
//! DOMAIN: Pure connectivity checks that require no geometry data.
//!
//! This module acts as a dispatcher, calling category-specific validators
//! defined in the subdirectories of `topology/validators`.

use crate::b_rep::TopologyArena;
use crate::validators::validate::ValidationLevel;
use forge_core::KernelError;

use super::loop_wiring;
use super::radial_edge;
use super::vertex_disk;
use super::shell_closure;
use super::euler_genus;
use super::reference_integrity;
use super::cache_index;

/// Validate structural topology of an arena with the specified strictness.
///
/// Called automatically by `MutableDraft::commit()`. Runs structural checks
/// (pointer coherence, loop closure, hierarchy, Euler formula) and
/// shell-kind-aware manifold enforcement (Solid shells require valence ≤ 2).
pub fn validate_topology(arena: &TopologyArena, level: ValidationLevel) -> Result<(), KernelError> {
    if level == ValidationLevel::None {
        return Ok(());
    }

    // ── Tier 1a: Core pointer coherence and local invariants ─────────
    radial_edge::validate_radial_rings(arena)?;
    radial_edge::validate_radial_edge_consistency(arena)?;
    loop_wiring::validate_prev_consistency(arena)?;
    loop_wiring::validate_vertex_continuity(arena)?;
    vertex_disk::validate_vertex_outgoing(arena)?;

    // Batch 1: Pure pointer checks (always run)
    reference_integrity::validate_no_dangling_half_edge_refs(arena)?;
    reference_integrity::validate_generational_id_freshness(arena)?;
    reference_integrity::validate_bidirectional_links(arena)?;
    reference_integrity::validate_face_has_at_least_one_loop(arena)?;
    loop_wiring::validate_loop_minimum_cardinality(arena)?;
    loop_wiring::validate_no_duplicate_coedges_in_loop(arena)?;
    radial_edge::validate_radial_cycle_uniqueness(arena)?;

    if level == ValidationLevel::Intermediate || level == ValidationLevel::Full {
        // ── Tier 1b: Loop structure + hierarchy ─────────────────────
        loop_wiring::validate_loops(arena)?;
        reference_integrity::validate_hierarchy(arena)?;

        // Batch 1 (continued): membership completeness
        loop_wiring::validate_face_loop_membership_complete(arena)?;

        // Batch 2: Ownership and loop domain invariants
        reference_integrity::validate_single_owner_per_loop(arena)?;
        reference_integrity::validate_inner_outer_loop_consistency(arena)?;
        loop_wiring::validate_edge_endpoints_match_loop_vertices(arena)?;
        reference_integrity::validate_no_orphan_half_edges(arena)?;
        reference_integrity::validate_acyclic_containment(arena)?;
    }

    if level == ValidationLevel::Full {
        // ── Tier 2: Global structural integrity ─────────────────────
        euler_genus::validate_euler(arena)?;
        shell_closure::validate_shell_consistency(arena)?;
        shell_closure::validate_manifold_edges(arena)?;
        shell_closure::validate_orientation_consistency(arena)?;
        cache_index::validate_index_coherence(arena)?;

        // Batch 3: Face/shell adjacency and radial splices
        shell_closure::validate_face_adjacency_consistency(arena)?;
        shell_closure::validate_no_broken_face_boundary(arena)?;
        shell_closure::validate_boundary_edges_laminar_only(arena)?;
        radial_edge::validate_radial_neighbor_consistency(arena)?;
        radial_edge::validate_no_broken_radial_splices(arena)?;
    }

    Ok(())
}

/// Re-export for external callers that need manifold checks directly.
pub use shell_closure::validate_manifold_edges;


