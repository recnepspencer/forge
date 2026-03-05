//! Runtime validation policy — computed per-draft from config + TopologyContext.
//!
//! DOMAIN: O(1) bitmask-based dispatch layer. Built once at draft creation,
//! queried on every `execute()` call to decide which invariant groups run.
//!
//! Separation: `GroupPolicyConfig` (forge-kernel) is user-facing config.
//! `GroupPolicyRuntime` (this file) is the per-draft computed result.

use forge_core::{
    InvariantGroup, TopologyContext, TopologyKind, Closure, CertificationStage,
    ValidatorCost, ValidationCheckpoint,
    APPLICABLE_BY_KIND, CLOSED_SHEET_EXTRA, DEFER_SEMANTIC_TIER, DEFER_UNCERTIFIED,
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
    pub fn skip_mask(&self) -> u32 { self.skip_mask }

    /// The deferred mask (for diagnostics/tracing).
    pub fn deferred_mask(&self) -> u32 { self.deferred_mask }
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
mod tests {
    use super::*;

    #[test]
    fn solid_certified_runs_everything_at_commit() {
        let ctx = TopologyContext {
            stage: CertificationStage::Certified,
            ..TopologyContext::SOLID
        };
        let rt = GroupPolicyRuntime::resolve(0, 0,
            [ValidatorCost::Expensive; ValidationCheckpoint::COUNT], &ctx);

        for &group in InvariantGroup::ALL {
            assert!(
                rt.should_run(group, ValidationCheckpoint::PostCommit),
                "Solid+Certified should run {:?} at PostCommit", group,
            );
        }
    }

    #[test]
    fn solid_defers_semantic_tier_from_per_op() {
        let rt = GroupPolicyRuntime::resolve(0, 0,
            [ValidatorCost::Expensive; ValidationCheckpoint::COUNT],
            &TopologyContext::SOLID);

        // Semantic tier should be deferred (not per-op)
        assert!(!rt.should_run(InvariantGroup::EulerFormula, ValidationCheckpoint::PerOp));
        assert!(!rt.should_run(InvariantGroup::ShellClosure, ValidationCheckpoint::PerOp));

        // But runs at PostCommit
        assert!(rt.should_run(InvariantGroup::EulerFormula, ValidationCheckpoint::PostCommit));
        assert!(rt.should_run(InvariantGroup::ShellClosure, ValidationCheckpoint::PostCommit));

        // Topology tier still runs per-op
        assert!(rt.should_run(InvariantGroup::PointerCoherence, ValidationCheckpoint::PerOp));
        assert!(rt.should_run(InvariantGroup::CacheCoherence, ValidationCheckpoint::PerOp));
    }

    #[test]
    fn wire_skips_face_based_groups() {
        let rt = GroupPolicyRuntime::resolve(0, 0,
            [ValidatorCost::Expensive; ValidationCheckpoint::COUNT],
            &TopologyContext::WIRE);

        // Wire should skip face-based groups even at PostCommit
        assert!(!rt.should_run(InvariantGroup::LoopIntegrity, ValidationCheckpoint::PostCommit));
        assert!(!rt.should_run(InvariantGroup::RadialEdge, ValidationCheckpoint::PostCommit));
        assert!(!rt.should_run(InvariantGroup::ShellClosure, ValidationCheckpoint::PostCommit));
        assert!(!rt.should_run(InvariantGroup::VertexDisk, ValidationCheckpoint::PostCommit));
        assert!(!rt.should_run(InvariantGroup::EulerFormula, ValidationCheckpoint::PostCommit));

        // Wire should still run pointer + ownership + cache
        assert!(rt.should_run(InvariantGroup::PointerCoherence, ValidationCheckpoint::PerOp));
        assert!(rt.should_run(InvariantGroup::Ownership, ValidationCheckpoint::PerOp));
        assert!(rt.should_run(InvariantGroup::CacheCoherence, ValidationCheckpoint::PerOp));
    }

    #[test]
    fn open_sheet_skips_shell_closure() {
        let rt = GroupPolicyRuntime::resolve(0, 0,
            [ValidatorCost::Expensive; ValidationCheckpoint::COUNT],
            &TopologyContext::SHEET_OPEN);

        assert!(!rt.should_run(InvariantGroup::ShellClosure, ValidationCheckpoint::PostCommit));
    }

    #[test]
    fn force_per_op_overrides_deferral() {
        let force_per_op = InvariantGroup::EulerFormula.mask();
        let rt = GroupPolicyRuntime::resolve(0, force_per_op,
            [ValidatorCost::Expensive; ValidationCheckpoint::COUNT],
            &TopologyContext::SOLID);

        // EulerFormula should now run at PerOp despite being Semantic tier
        assert!(rt.should_run(InvariantGroup::EulerFormula, ValidationCheckpoint::PerOp));
    }

    #[test]
    fn force_skip_overrides_applicability() {
        let force_skip = InvariantGroup::PointerCoherence.mask();
        let rt = GroupPolicyRuntime::resolve(force_skip, 0,
            [ValidatorCost::Expensive; ValidationCheckpoint::COUNT],
            &TopologyContext::SOLID);

        assert!(!rt.should_run(InvariantGroup::PointerCoherence, ValidationCheckpoint::PerOp));
        assert!(!rt.should_run(InvariantGroup::PointerCoherence, ValidationCheckpoint::PostCommit));
    }

    #[test]
    fn should_run_is_o1() {
        // Just verify it doesn't panic on all combinations
        let rt = GroupPolicyRuntime::default();
        for &group in InvariantGroup::ALL {
            let _ = rt.should_run(group, ValidationCheckpoint::PerOp);
            let _ = rt.should_run(group, ValidationCheckpoint::PostCommit);
            let _ = rt.should_run(group, ValidationCheckpoint::PostBoolean);
            let _ = rt.should_run(group, ValidationCheckpoint::PostFeature);
            let _ = rt.should_run(group, ValidationCheckpoint::PostImport);
            let _ = rt.should_run(group, ValidationCheckpoint::OnDemand);
        }
    }
}
