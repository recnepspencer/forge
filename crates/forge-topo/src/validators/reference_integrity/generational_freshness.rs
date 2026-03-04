//! Generational ID freshness validator.
//!
//! INVARIANT: Every handle stored inside an entity (e.g. `he.next()`,
//! `he.face()`, `face.outer_loop()`) must have a generation that matches
//! the current slot generation in the arena. A mismatch means the handle
//! is stale — it was valid before an entity at that index was
//! removed and the slot was reused with a bumped generation.
//!
//! This complements `validate_no_dangling_half_edge_refs` which checks
//! whether a handle resolves at all. This validator specifically targets
//! the silent reuse case: same index, wrong generation.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

use super::vf;

/// Check that a handle's generation matches the slot's actual generation.
///
/// Returns `Ok(())` if the handle refers to a live entity at the correct generation.
/// Returns `Err` with a `generational_id_freshness` violation if:
/// - The index is in bounds but the slot's generation doesn't match (stale handle)
/// - The index is out of bounds (dangling handle — also caught by dangling_refs)
macro_rules! check_generation {
    ($arena:expr, $entity_gen_fn:ident, $handle:expr, $owner_label:expr, $owner_idx:expr, $field:expr) => {
        {
            let idx = $handle.index() as usize;
            match $arena.$entity_gen_fn(idx) {
                Some(slot_gen) if slot_gen != $handle.generation() => {
                    return Err(vf("generational_id_freshness", format!(
                        "{} {} .{} references index {} with generation {} but slot has generation {} (stale handle)",
                        $owner_label, $owner_idx, $field,
                        $handle.index(), $handle.generation(), slot_gen
                    )));
                }
                None => {
                    // Index out of bounds or slot is empty — covered by dangling_refs,
                    // but we still report it for completeness.
                    // so we don't report it here.
                }
                _ => {} // Matches!
            }
        }
    };
}

pub(crate) fn validate_generational_id_freshness(arena: &TopologyArena) -> Result<(), KernelError> {
    // ── HalfEdge handles ─────────────────────────────────────────────
    for (he_id, he_data) in arena.iter_half_edges() {
        check_generation!(arena, vertex_generation,    he_data.origin(),      "HE", he_id.index(), "origin");
        check_generation!(arena, face_generation,      he_data.face(),        "HE", he_id.index(), "face");
        check_generation!(arena, edge_generation,      he_data.edge(),        "HE", he_id.index(), "edge");
        check_generation!(arena, half_edge_generation, he_data.next(),        "HE", he_id.index(), "next");
        check_generation!(arena, half_edge_generation, he_data.prev(),        "HE", he_id.index(), "prev");
        check_generation!(arena, half_edge_generation, he_data.radial_next(), "HE", he_id.index(), "radial_next");
    }

    // ── Vertex handles ───────────────────────────────────────────────
    for (vertex_id, vertex_data) in arena.iter_vertices() {
        check_generation!(arena, half_edge_generation, vertex_data.outgoing(), "Vertex", vertex_id.index(), "outgoing");
    }

    // ── Face handles ─────────────────────────────────────────────────
    for (face_id, face_data) in arena.iter_faces() {
        check_generation!(arena, loop_generation,  face_data.outer_loop(), "Face", face_id.index(), "outer_loop");
        for (i, &il) in face_data.inner_loops().iter().enumerate() {
            check_generation!(arena, loop_generation, il, "Face", face_id.index(), &format!("inner_loops[{}]", i));
        }
        check_generation!(arena, shell_generation, face_data.shell(),      "Face", face_id.index(), "shell");
    }

    // ── Loop handles ─────────────────────────────────────────────────
    for (loop_id, loop_data) in arena.iter_loops() {
        check_generation!(arena, half_edge_generation, loop_data.half_edge(), "Loop", loop_id.index(), "half_edge");
        check_generation!(arena, face_generation,      loop_data.face(),      "Loop", loop_id.index(), "face");
    }

    // ── Edge handles ─────────────────────────────────────────────────
    for (edge_id, edge_data) in arena.iter_edges() {
        check_generation!(arena, half_edge_generation, edge_data.half_edge(), "Edge", edge_id.index(), "half_edge");
    }

    // ── Shell handles ────────────────────────────────────────────────
    for (shell_id, shell_data) in arena.iter_shells() {
        check_generation!(arena, face_generation, shell_data.representative_face(), "Shell", shell_id.index(), "representative_face");
        check_generation!(arena, region_generation, shell_data.region(), "Shell", shell_id.index(), "region");
    }

    // ── Region handles ───────────────────────────────────────────────
    for (region_id, region_data) in arena.iter_regions() {
        if let Some(outer) = region_data.outer_shell() {
            check_generation!(arena, shell_generation, outer, "Region", region_id.index(), "outer_shell");
        }
        for (i, &is) in region_data.inner_shells().iter().enumerate() {
            check_generation!(arena, shell_generation, is, "Region", region_id.index(), &format!("inner_shells[{}]", i));
        }
        check_generation!(arena, lump_generation, region_data.lump(), "Region", region_id.index(), "lump");
    }

    // ── Lump handles ─────────────────────────────────────────────────
    for (lump_id, lump_data) in arena.iter_lumps() {
        check_generation!(arena, body_generation, lump_data.body(), "Lump", lump_id.index(), "body");
        for (i, &r) in lump_data.regions().iter().enumerate() {
            check_generation!(arena, region_generation, r, "Lump", lump_id.index(), &format!("regions[{}]", i));
        }
    }

    // ── Body handles ─────────────────────────────────────────────────
    for (body_id, body_data) in arena.iter_bodies() {
        for (i, &l) in body_data.lumps().iter().enumerate() {
            check_generation!(arena, lump_generation, l, "Body", body_id.index(), &format!("lumps[{}]", i));
        }
    }

    Ok(())
}
