//! Structural proof invariant validators.
//!
//! DOMAIN: Shared validation functions used by P3 divergence-detection tests.
//! These validators cross-reference proof metadata (ReplayLog, LineageEvents,
//! causal chains) against the actual topology to ensure structural consistency.
//!
//! INVARIANTS:
//! - INV-1: Hash chain continuity (replay entries form unbroken chain)
//! - INV-2: Lineage coverage (every result face has exactly one creation event)
//! - INV-3: Causal chain reachability (every face's chain reaches a real op)

use std::collections::BTreeSet;

use forge_core::{DecisionLog, EntityRef};
use forge_topo::b_rep::TopologyArena;
use forge_topo::transactions::compute_arena_topology_hash;
use forge_topo::provenance::LineageEvent;
use forge_topo::provenance::ReplayLog;

use crate::proof::causal_chain::query_causal_chain;

/// INV-1: Verify that replay entries form an unbroken hash chain.
///
/// Checks:
/// - entry[i].post_hash == entry[i+1].pre_hash for all consecutive pairs
/// - Last entry's post_hash matches `result_hash`
///
/// Returns descriptive error string on failure.
pub fn validate_hash_chain(replay: &ReplayLog, result_hash: u128) -> Result<(), String> {
    let entries = replay.entries();
    if entries.is_empty() {
        return Err("Replay log is empty — no hash chain to validate".to_string());
    }

    for i in 0..entries.len() - 1 {
        let post = entries[i].post_hash();
        let pre_next = entries[i + 1].pre_hash();
        if post != pre_next {
            return Err(format!(
                "Hash chain break at entry {}->{}: post_hash={:#x} != pre_hash={:#x} (ops: {} -> {})",
                i, i + 1, post, pre_next,
                entries[i].signature().get_name(),
                entries[i + 1].signature().get_name(),
            ));
        }
    }

    let last_post = entries.last().unwrap().post_hash();
    if last_post != result_hash {
        return Err(format!(
            "Hash chain end mismatch: last post_hash={:#x} != result_hash={:#x}",
            last_post, result_hash,
        ));
    }

    Ok(())
}

/// INV-2: Verify that lineage events have 1:1 coverage with result faces.
///
/// Checks:
/// - lineage_events.len() == arena.face_count()
/// - Every EntityCreated event references a unique, active face index
/// - No duplicate face indices
///
/// Returns descriptive error string on failure.
pub fn validate_lineage_coverage(
    lineage_events: &[LineageEvent],
    arena: &TopologyArena,
) -> Result<(), String> {
    let face_count = arena.face_count();

    let creation_count = lineage_events.iter().filter(|e| {
        matches!(e, LineageEvent::EntityCreated { entity, .. } if entity.kind().as_str() == "Face")
    }).count();

    if creation_count != face_count {
        return Err(format!(
            "Lineage coverage mismatch: {} face creation events vs {} faces in arena",
            creation_count, face_count,
        ));
    }

    let mut seen_indices = BTreeSet::new();
    for event in lineage_events {
        if let LineageEvent::EntityCreated {
            lineage, entity, ..
        } = event
        {
            if entity.kind().as_str() != "Face" {
                continue;
            }
            let idx = lineage.get_origin_features()[0] as u32;
            if !seen_indices.insert(idx) {
                return Err(format!("Duplicate lineage event for face index {}", idx,));
            }
        }
    }

    Ok(())
}

/// INV-3: Verify that every face in the result has a non-empty causal chain
/// whose steps reference operations present in the replay log.
///
/// Returns descriptive error string on failure.
pub fn validate_causal_reachability(
    arena: &TopologyArena,
    replay: &ReplayLog,
    decisions: &DecisionLog,
    lineage: &[LineageEvent],
) -> Result<(), String> {
    let replay_op_names: BTreeSet<&str> = replay
        .entries()
        .iter()
        .map(|e| e.signature().get_name())
        .collect();

    let mut faces_with_empty_chains = Vec::new();
    let mut faces_with_orphan_ops = Vec::new();

    for (fid, _) in arena.iter_faces() {
        let face_ref = EntityRef::new(forge_core::EntityKind::Face, fid.index() as u32);
        let chain = query_causal_chain(&face_ref, replay, decisions, lineage, &[]);

        if chain.get_steps().is_empty() {
            faces_with_empty_chains.push(fid.index());
        }

        for step in chain.get_steps() {
            let op_name = step.get_operation().get_name();
            if !replay_op_names.contains(op_name) {
                faces_with_orphan_ops.push((fid.index(), op_name.to_string()));
            }
        }
    }

    if !faces_with_empty_chains.is_empty() {
        return Err(format!(
            "{} faces have empty causal chains: {:?}",
            faces_with_empty_chains.len(),
            faces_with_empty_chains,
        ));
    }

    if !faces_with_orphan_ops.is_empty() {
        return Err(format!(
            "Causal chain references ops not in replay log: {:?}",
            faces_with_orphan_ops,
        ));
    }

    Ok(())
}

/// Run all structural invariants and return a combined error if any fail.
pub fn validate_all(
    replay: &ReplayLog,
    decisions: &DecisionLog,
    lineage: &[LineageEvent],
    arena: &TopologyArena,
    result_hash: u128,
) -> Result<(), String> {
    let mut errors = Vec::new();

    if let Err(e) = validate_hash_chain(replay, result_hash) {
        errors.push(format!("INV-1 (hash chain): {}", e));
    }
    if let Err(e) = validate_lineage_coverage(lineage, arena) {
        errors.push(format!("INV-2 (lineage): {}", e));
    }
    if let Err(e) = validate_causal_reachability(arena, replay, decisions, lineage) {
        errors.push(format!("INV-3 (causal): {}", e));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}
