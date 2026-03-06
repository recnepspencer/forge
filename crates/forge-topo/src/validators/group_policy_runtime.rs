//! Runtime validation policy — computed per-draft from config + TopologyContext.
//!
//! DOMAIN: O(1) bitmask-based dispatch layer. Built once at draft creation,
//! queried on every `execute()` call to decide which invariant groups run.
//!
//! Separation: `GroupPolicyConfig` (forge-kernel) is user-facing config.
//! `GroupPolicyRuntime` (this file) is the per-draft computed result.

use forge_core::{
    CertificationStage, Closure, InvariantGroup, TopologyContext, TopologyKind,
    ValidationCheckpoint, ValidatorCost, APPLICABLE_BY_KIND, CLOSED_SHEET_EXTRA,
    DEFER_SEMANTIC_TIER, DEFER_UNCERTIFIED,
};

/// Computed per-draft validation policy.
///
/// All fields are bitmasks or fixed-size arrays — O(1) everything.
/// Built once via `resolve()` at draft creation, then queried by
/// `execute()` and `commit()`.
#[derive(Debug, Clone)]
pub struct GroupPolicyRuntime {
    /// Groups that never run (not applicable for this topology kind + user force_skip).
    skip_mask: u32,
    /// Union of all deferred groups (for fast "is this deferred?" check).
    deferred_mask: u32,
    /// Per-checkpoint: which groups should run at this checkpoint.
    /// Indexed by `[ValidationCheckpoint as u8]`.
    run_at: [u32; ValidationCheckpoint::COUNT],
    /// Per-checkpoint cost ceiling.
    /// Indexed by `[ValidationCheckpoint as u8]`.
    max_cost: [ValidatorCost; ValidationCheckpoint::COUNT],
}

impl GroupPolicyRuntime {
    /// Build from config masks + topology context.
    ///
    /// # Arguments
    /// - `force_skip_mask`: bitmask of groups the user explicitly wants to skip
    /// - `force_per_op_mask`: bitmask of groups the user explicitly wants per-op (un-defers)
    /// - `max_cost_by_checkpoint`: per-checkpoint cost ceilings
    /// - `ctx`: topology context describing the body being mutated
    pub fn resolve(
        force_skip_mask: u32,
        force_per_op_mask: u32,
        max_cost_by_checkpoint: [ValidatorCost; ValidationCheckpoint::COUNT],
        ctx: &TopologyContext,
    ) -> Self {
        // 1. Kind-based applicability
        let mut applicable = APPLICABLE_BY_KIND[ctx.kind as usize];
        if ctx.kind == TopologyKind::Sheet && ctx.closure == Closure::Closed {
            applicable |= CLOSED_SHEET_EXTRA;
        }

        // 2. Skip = not applicable + user force_skip
        let skip_mask = !applicable | force_skip_mask;

        // 3. Deferred = tier defaults + stage defaults - user force_per_op
        let mut deferred_mask = DEFER_SEMANTIC_TIER;
        if ctx.stage == CertificationStage::Uncertified {
            deferred_mask |= DEFER_UNCERTIFIED;
        }
        deferred_mask &= !force_per_op_mask; // un-defer user-forced groups
        deferred_mask &= !skip_mask; // don't defer already-skipped groups

        // 4. Per-checkpoint run masks
        let non_deferred = !skip_mask & !deferred_mask;
        let deferred_runnable = deferred_mask & !skip_mask;

        let mut run_at = [0u32; ValidationCheckpoint::COUNT];
        // PerOp: only non-deferred groups
        run_at[ValidationCheckpoint::PerOp as usize] = non_deferred;
        // All other checkpoints: non-deferred + deferred
        for i in 1..ValidationCheckpoint::COUNT {
            run_at[i] = non_deferred | deferred_runnable;
        }

        Self {
            skip_mask,
            deferred_mask,
            run_at,
            max_cost: max_cost_by_checkpoint,
        }
    }

    /// Should this group run at this checkpoint?
    ///
    /// O(1) — one array index + one bitwise AND. Called from `execute()` hot path.
    #[inline]
    pub fn should_run(&self, group: InvariantGroup, checkpoint: ValidationCheckpoint) -> bool {
        self.run_at[checkpoint as usize] & group.mask() != 0
    }

    /// Cost ceiling for this checkpoint.
    #[inline]
    pub fn max_cost_at(&self, checkpoint: ValidationCheckpoint) -> ValidatorCost {
        self.max_cost[checkpoint as usize]
    }

    /// The skip mask (for diagnostics/tracing).
    pub fn skip_mask(&self) -> u32 {
        self.skip_mask
    }

    /// The deferred mask (for diagnostics/tracing).
    pub fn deferred_mask(&self) -> u32 {
        self.deferred_mask
    }

    /// Snapshot the per-checkpoint cost ceiling array.
    ///
    /// Used by `into_mutation_with()` to preserve the caller's cost
    /// settings when re-resolving the policy from shell metadata.
    pub fn max_cost_snapshot(&self) -> [ValidatorCost; ValidationCheckpoint::COUNT] {
        self.max_cost
    }
}

// ── Option A: Model-derived context ────────────────────────────────────

use crate::b_rep::{ShellKind, TopologyArena};

/// Derive a topology context **hint** from declared `ShellKind` metadata.
///
/// Reads the `ShellKind` stored on each shell in the arena and computes
/// the **widest** (most permissive) `TopologyContext`. "Widest" means:
/// if any shell is `Solid`, the context is `Solid`; if any is `Sheet`,
/// elevate to at least `Sheet`; otherwise `Wire`.
///
/// # Not structural analysis
///
/// This reads **declared metadata**, not graph structure. Operators that
/// change a shell's topological character (e.g., sealing an open sheet
/// into a closed solid) MUST update `ShellData::set_kind()`.
///
/// Structural correctness is verified separately at commit time by
/// [`verify_shell_kind_matches_structure`] (debug builds only).
///
/// # Complexity
///
/// O(shells) — typically 1–4 shells per body.
pub fn topology_context_from_shell_metadata(arena: &TopologyArena) -> TopologyContext {
    let mut has_solid = false;
    let mut has_sheet = false;
    let mut has_wire = false;
    let mut all_closed = true;

    let mut shell_count = 0u32;

    for (_shell_id, shell_data) in arena.iter_shells() {
        shell_count += 1;
        match shell_data.kind() {
            ShellKind::Solid(_) => {
                has_solid = true;
                // Solid shells are closed by definition
            }
            ShellKind::Sheet => {
                has_sheet = true;
                all_closed = false; // sheets have boundary edges
            }
            ShellKind::Wire => {
                has_wire = true;
                all_closed = false;
            }
        }
    }

    // Empty arena → default to Solid (most conservative = runs all validators)
    if shell_count == 0 {
        return TopologyContext::SOLID;
    }

    // Widest kind wins
    let kind = if has_solid {
        TopologyKind::Solid
    } else if has_sheet {
        TopologyKind::Sheet
    } else if has_wire {
        TopologyKind::Wire
    } else {
        TopologyKind::Solid // unreachable, but safe default
    };

    let closure = if all_closed {
        Closure::Closed
    } else {
        Closure::Open
    };

    TopologyContext {
        kind,
        closure,
        manifoldness: forge_core::Manifoldness::Manifold, // default; NMT detection is separate
        stage: CertificationStage::Uncertified,
    }
}

/// **Debug-only**: verify that declared `ShellKind` matches structural reality.
///
/// Walks all half-edges in the arena (O(E)) to definitively classify shells:
/// 1. `Solid` shells must have NO boundary edges anywhere.
/// 2. `Wire` shells must have NO faces.
/// 3. `Sheet` shells that have NO boundary edges trigger a debug warning
///    (they are actually watertight solids and missed a validation promotion).
///
/// Fires in CI/dev (debug builds), not production.
///
/// # Panics
///
/// Panics with a descriptive message if a shell's declared kind
/// contradicts its structural properties.
#[cfg(debug_assertions)]
pub fn verify_shell_kind_matches_structure(arena: &TopologyArena) {
    use crate::queries::traverse::is_boundary_edge;
    use std::collections::HashSet;

    // First check: Wires cannot have faces
    for (face_id, face_data) in arena.iter_faces() {
        let shell_id = face_data.shell();
        if let Ok(shell_data) = arena.get_shell(shell_id) {
            debug_assert!(
                !matches!(shell_data.kind(), ShellKind::Wire),
                "Face {:?} belongs to Shell {:?} which claims to be a Wire. Wires cannot have faces.",
                face_id, shell_id
            );
        }
    }

    // Second check: track which shells actually have boundary edges O(E) scan
    let mut shells_with_boundaries = HashSet::new();

    for (he_id, _he_data) in arena.iter_half_edges() {
        if let Ok(is_bound) = is_boundary_edge(arena, he_id) {
            if is_bound {
                // Find what shell this belongs to
                if let Ok(he) = arena.get_half_edge(he_id) {
                    if let Ok(face) = arena.get_face(he.face()) {
                        shells_with_boundaries.insert(face.shell());
                    }
                }
            }
        }
    }

    // Third check: Verify declared goals against reality
    for (shell_id, shell_data) in arena.iter_shells() {
        let has_boundaries = shells_with_boundaries.contains(&shell_id);

        match shell_data.kind() {
            ShellKind::Solid(_) => {
                // Solid shells must have NO boundary edges anywhere.
                debug_assert!(
                    !has_boundaries,
                    "Shell {:?} declared Solid but contains boundary edges! \
                     Operator must either seal the hole or set_kind(Sheet).",
                    shell_id
                );
            }
            ShellKind::Wire => {
                // Caught by the face iter above
            }
            ShellKind::Sheet => {
                // Sheet is expected to have boundaries. If it doesn't, it's actually watertight!
                // NOTE: We only check this if there are actually faces in the shell
                // (an empty shell might be marked sheet temporarily).
                debug_assert!(
                    has_boundaries || arena.face_count() == 0,
                    "Shell {:?} declared Sheet but is completely WATERTIGHT! \
                     Operator forgot to promote it with set_kind(Solid), missing out on volume validation.",
                    shell_id
                );
            }
        }
    }
}

impl Default for GroupPolicyRuntime {
    /// Default: everything runs everywhere, Expensive allowed.
    /// This is the debug-friendly "check everything" mode.
    fn default() -> Self {
        Self::resolve(
            0, // no skips
            0, // no force-per-op overrides
            [ValidatorCost::Expensive; ValidationCheckpoint::COUNT],
            &TopologyContext::SOLID,
        )
    }
}

#[cfg(test)]
#[path = "group_policy_runtime_tests.rs"]
mod group_policy_runtime_tests;
