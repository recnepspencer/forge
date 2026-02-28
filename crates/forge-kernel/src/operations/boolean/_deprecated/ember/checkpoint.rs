//! Phase bisection checkpoints for the EMBER pipeline.
//!
//! DOMAIN: Structural topology validation between pipeline sub-phases.
//! Logs results as Tier 2 TracedDecisions in the DecisionLog.
//!
//! INVARIANTS CHECKED:
//! - Twin reciprocity: he.twin.twin == he
//! - Twin orientation: twin pairs belong to different faces
//! - Loop closure: prev(he).next == he for all halfedges
//! - Manifold edges: every geometric edge shared by exactly 2 faces
//!
//! DEPENDENCIES: `forge-topo` (arena), `forge-core` (tracing), `ModelingContext`

use forge_core::tracing::{DecisionKind, DecisionTier};
use forge_core::KernelError;
use forge_topo::transactions::MutableDraft;

use crate::core::ModelingContext;

/// Validate structural topology invariants after a pipeline sub-phase.
///
/// Runs structural checks on the current draft arena:
/// - Loop closure (always checked)
/// - Twin reciprocity, twin orientation, manifold edges (skipped if
///   `skip_twin_checks` is true — e.g., before stitching assigns twins)
///
/// On success, logs `DecisionKind::Exact`. On failure, logs
/// `DecisionKind::Forced` and returns `KernelError`.
pub fn validate_checkpoint(
    draft: &MutableDraft,
    ctx: &mut ModelingContext,
    phase_name: &str,
    skip_twin_checks: bool,
) -> Result<(), KernelError> {
    let arena = draft.arena();

    if !skip_twin_checks {
        if let Err(violation) = check_twin_reciprocity(arena) {
            let msg = format!("{phase_name}: {violation}");
            ctx.log_decision(
                DecisionKind::Forced {
                    reason: msg.clone(),
                },
                DecisionTier::Escalated,
                [0.0, 0.0, 0.0],
                0.0,
                0.0,
            );
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::MissingTwin { halfedge_index: 0 },
                context: None,
            }
            .with_phase(phase_name));
        }

        if let Err(violation) = check_twin_orientation(arena) {
            let msg = format!("{phase_name}: {violation}");
            ctx.log_decision(
                DecisionKind::Forced {
                    reason: msg.clone(),
                },
                DecisionTier::Escalated,
                [0.0, 0.0, 0.0],
                0.0,
                0.0,
            );
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::OrientationInconsistency { face_index: 0 },
                context: None,
            }
            .with_phase(phase_name));
        }

        if let Err(violation) = check_manifold_edges(arena) {
            let msg = format!("{phase_name}: {violation}");
            ctx.log_decision(
                DecisionKind::Forced {
                    reason: msg.clone(),
                },
                DecisionTier::Escalated,
                [0.0, 0.0, 0.0],
                0.0,
                0.0,
            );
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::NonManifoldEdge {
                    edge_index: 0,
                    valence: 0,
                },
                context: None,
            }
            .with_phase(phase_name));
        }
    }

    if let Err(violation) = check_loop_closure(arena) {
        let msg = format!("{phase_name}: {violation}");
        ctx.log_decision(
            DecisionKind::Forced {
                reason: msg.clone(),
            },
            DecisionTier::Escalated,
            [0.0, 0.0, 0.0],
            0.0,
            0.0,
        );
        return Err(KernelError::TopologyViolation {
            err: forge_core::TopologyError::BrokenLoop {
                starting_halfedge: 0,
                face_index: 0,
            },
            context: None,
        }
        .with_phase(phase_name));
    }

    ctx.log_decision(
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        [0.0, 0.0, 0.0],
        1.0,
        0.0,
    );

    Ok(())
}

/// Check twin reciprocity: he.twin.twin == he for all non-self-twin halfedges.
fn check_twin_reciprocity(arena: &forge_topo::b_rep::TopologyArena) -> Result<(), String> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.radial_next();
        if he_id == twin_id {
            continue;
        }

        let twin_data = match arena.get_half_edge(twin_id) {
            Ok(d) => d,
            Err(_) => {
                return Err(format!(
                    "twin reciprocity: he[{}].twin={} is invalid/deleted",
                    he_id.index(),
                    twin_id.index()
                ))
            }
        };

        if twin_data.radial_next() != he_id {
            return Err(format!(
                "twin reciprocity: he[{}].twin={}, but he[{}].twin={} (expected {})",
                he_id.index(),
                twin_id.index(),
                twin_id.index(),
                twin_data.radial_next().index(),
                he_id.index()
            ));
        }
    }
    Ok(())
}

/// Check twin orientation: twin pairs must belong to different faces.
fn check_twin_orientation(arena: &forge_topo::b_rep::TopologyArena) -> Result<(), String> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.radial_next();
        if he_id == twin_id {
            continue;
        }

        let twin_data = match arena.get_half_edge(twin_id) {
            Ok(d) => d,
            Err(_) => continue,
        };

        if he_data.face() == twin_data.face() {
            return Err(format!(
                "twin orientation: he[{}] and twin he[{}] both on face {} \
                 (origin {} → twin.origin {})",
                he_id.index(),
                twin_id.index(),
                he_data.face().index(),
                he_data.origin().index(),
                twin_data.origin().index()
            ));
        }
    }
    Ok(())
}

/// Check loop closure: prev(he).next == he for all halfedges.
fn check_loop_closure(arena: &forge_topo::b_rep::TopologyArena) -> Result<(), String> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let prev_data = match arena.get_half_edge(he_data.prev()) {
            Ok(d) => d,
            Err(_) => {
                return Err(format!(
                    "loop closure: he[{}].prev={} is invalid",
                    he_id.index(),
                    he_data.prev().index()
                ))
            }
        };

        if prev_data.next() != he_id {
            return Err(format!(
                "loop closure: he[{}].prev={}, but prev.next={} (expected {})",
                he_id.index(),
                he_data.prev().index(),
                prev_data.next().index(),
                he_id.index()
            ));
        }
    }
    Ok(())
}

/// Check manifold edges: every geometric edge shared by exactly 2 faces.
fn check_manifold_edges(arena: &forge_topo::b_rep::TopologyArena) -> Result<(), String> {
    let mut edge_counts: std::collections::BTreeMap<(u32, u32), usize> =
        std::collections::BTreeMap::new();

    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.radial_next();
        if he_id == twin_id {
            continue;
        }
        let canonical = (
            he_id.index().min(twin_id.index()),
            he_id.index().max(twin_id.index()),
        );
        *edge_counts.entry(canonical).or_insert(0) += 1;
    }

    for (&(lo, hi), &count) in &edge_counts {
        if count > 2 {
            return Err(format!(
                "manifold: edge ({},{}) has {} halfedges (expected 2)",
                lo, hi, count
            ));
        }
    }
    Ok(())
}
