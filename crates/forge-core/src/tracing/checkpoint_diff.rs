//! Checkpoint diffing — diff `DecisionLog` snapshots between operation steps.
//!
//! DOMAIN: Causal replay infrastructure (P3.1). Identifies exactly when a
//! divergence was introduced by diffing full decision logs (all tiers, not
//! just Tier 2+ like `TraceSummary::diff()`).
//!
//! DEPENDENCIES: `schema` (TracedDecision, DecisionId), `decision_log` (DecisionLog, DecisionSummary)

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::decision_log::{DecisionLog, DecisionSummary};
use super::schema::{DecisionId, TracedDecision};

// =========================================================================
// DECISION CHANGE
// =========================================================================

/// A single decision that changed between two checkpoints.
///
/// Records the before/after state and classifies what kind of change
/// occurred (kind, tier, margin, or combination).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionChange {
    /// The decision ID that changed.
    id: DecisionId,
    /// The decision state before the change.
    before: TracedDecision,
    /// The decision state after the change.
    after: TracedDecision,
    /// Whether the decision kind discriminant changed.
    kind_changed: bool,
    /// Whether the decision tier changed.
    tier_changed: bool,
    /// Absolute difference in margin (after.margin - before.margin).
    margin_delta: f64,
}

impl DecisionChange {
    /// The decision ID.
    pub fn get_id(&self) -> DecisionId {
        self.id
    }

    /// The before state.
    pub fn get_before(&self) -> &TracedDecision {
        &self.before
    }

    /// The after state.
    pub fn get_after(&self) -> &TracedDecision {
        &self.after
    }

    /// Whether the decision kind discriminant changed.
    pub fn is_kind_changed(&self) -> bool {
        self.kind_changed
    }

    /// Whether the decision tier changed.
    pub fn is_tier_changed(&self) -> bool {
        self.tier_changed
    }

    /// Absolute difference in margin.
    pub fn get_margin_delta(&self) -> f64 {
        self.margin_delta
    }
}

impl fmt::Display for DecisionChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.id)?;
        if self.kind_changed {
            write!(f, " kind: {} → {}", self.before.get_kind(), self.after.get_kind())?;
        }
        if self.tier_changed {
            write!(f, " tier: {} → {}", self.before.get_tier(), self.after.get_tier())?;
        }
        if self.margin_delta.abs() > f64::EPSILON {
            write!(f, " margin: {:.2e} → {:.2e}", self.before.get_margin(), self.after.get_margin())?;
        }
        Ok(())
    }
}

// =========================================================================
// DECISION DELTA
// =========================================================================

/// What changed between two `DecisionLog` snapshots.
///
/// Produced by `diff_decision_logs()`. Unlike `TraceDiff` (which only
/// examines Tier 2+ decisions), this captures changes across ALL tiers
/// for complete causal traceability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionDelta {
    /// Decisions present in `after` but not `before`.
    added: Vec<TracedDecision>,
    /// Decisions present in `before` but not `after`.
    removed: Vec<TracedDecision>,
    /// Decisions present in both but with changed kind, tier, or margin.
    changed: Vec<DecisionChange>,
    /// Summary of the `before` log.
    summary_before: DecisionSummary,
    /// Summary of the `after` log.
    summary_after: DecisionSummary,
}

impl DecisionDelta {
    /// Decisions added in the `after` snapshot.
    pub fn get_added(&self) -> &[TracedDecision] {
        &self.added
    }

    /// Decisions removed from the `before` snapshot.
    pub fn get_removed(&self) -> &[TracedDecision] {
        &self.removed
    }

    /// Decisions that changed between snapshots.
    pub fn get_changed(&self) -> &[DecisionChange] {
        &self.changed
    }

    /// Summary of the `before` log.
    pub fn get_summary_before(&self) -> &DecisionSummary {
        &self.summary_before
    }

    /// Summary of the `after` log.
    pub fn get_summary_after(&self) -> &DecisionSummary {
        &self.summary_after
    }

    /// Whether the delta is empty (no additions, removals, or changes).
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }

    /// Total number of differences (added + removed + changed).
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len() + self.changed.len()
    }
}

impl fmt::Display for DecisionDelta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "DecisionDelta: no changes");
        }
        writeln!(f, "DecisionDelta: {} added, {} removed, {} changed",
            self.added.len(), self.removed.len(), self.changed.len())?;
        for d in &self.added {
            writeln!(f, "  + {}", d)?;
        }
        for d in &self.removed {
            writeln!(f, "  - {}", d)?;
        }
        for c in &self.changed {
            writeln!(f, "  ~ {}", c)?;
        }
        Ok(())
    }
}

// =========================================================================
// diff_decision_logs
// =========================================================================

/// Diff two `DecisionLog` snapshots to produce a `DecisionDelta`.
///
/// Compares all decisions (not just Tier 2+) by `DecisionId`. A decision
/// is considered "changed" if its kind discriminant, tier, or margin differ.
pub fn diff_decision_logs(before: &DecisionLog, after: &DecisionLog) -> DecisionDelta {
    let before_by_id: BTreeMap<DecisionId, &TracedDecision> =
        before.decisions().map(|d| (d.get_id(), d)).collect();
    let after_by_id: BTreeMap<DecisionId, &TracedDecision> =
        after.decisions().map(|d| (d.get_id(), d)).collect();

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut changed = Vec::new();

    for (&id, &after_d) in &after_by_id {
        match before_by_id.get(&id) {
            None => added.push(after_d.clone()),
            Some(&before_d) => {
                let kind_changed = std::mem::discriminant(before_d.get_kind())
                    != std::mem::discriminant(after_d.get_kind());
                let tier_changed = before_d.get_tier() != after_d.get_tier();
                let margin_delta = after_d.get_margin() - before_d.get_margin();
                let margin_changed = margin_delta.abs() > f64::EPSILON;

                if kind_changed || tier_changed || margin_changed {
                    changed.push(DecisionChange {
                        id,
                        before: before_d.clone(),
                        after: after_d.clone(),
                        kind_changed,
                        tier_changed,
                        margin_delta,
                    });
                }
            }
        }
    }

    for (&id, &before_d) in &before_by_id {
        if !after_by_id.contains_key(&id) {
            removed.push(before_d.clone());
        }
    }

    DecisionDelta {
        added,
        removed,
        changed,
        summary_before: before.summary(),
        summary_after: after.summary(),
    }
}

// =========================================================================
// CHECKPOINT LOG
// =========================================================================

/// Step-by-step `DecisionLog` snapshot manager.
///
/// After each operation step in a chain, call `snapshot()` to store a
/// clone of the current `DecisionLog`. Then use `delta_at()` or
/// `delta_between()` for temporal queries.
///
/// ```
/// use forge_core::tracing::checkpoint_diff::CheckpointLog;
/// use forge_core::tracing::decision_log::DecisionLog;
///
/// let mut checkpoint_log = CheckpointLog::new();
/// let log = DecisionLog::new();
/// checkpoint_log.snapshot(&log);
/// assert_eq!(checkpoint_log.step_count(), 1);
/// ```
#[derive(Debug, Clone, Default)]
pub struct CheckpointLog {
    /// Ordered snapshots, one per step.
    snapshots: Vec<DecisionLog>,
}

impl CheckpointLog {
    /// Create an empty checkpoint log.
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
        }
    }

    /// Snapshot the current `DecisionLog` state.
    ///
    /// Clones the log and appends it as the next step.
    pub fn snapshot(&mut self, log: &DecisionLog) {
        self.snapshots.push(log.clone());
    }

    /// Number of captured snapshots.
    pub fn step_count(&self) -> usize {
        self.snapshots.len()
    }

    /// Retrieve the snapshot at a given step (0-indexed).
    pub fn get_snapshot(&self, step: usize) -> Option<&DecisionLog> {
        self.snapshots.get(step)
    }

    /// Diff between step and its predecessor (step - 1).
    ///
    /// Returns `None` for step 0 (no predecessor) or out-of-bounds steps.
    pub fn delta_at(&self, step: usize) -> Option<DecisionDelta> {
        if step == 0 {
            return None;
        }
        self.delta_between(step - 1, step)
    }

    /// Diff between two arbitrary steps.
    ///
    /// Returns `None` if either step is out of bounds.
    pub fn delta_between(&self, step_a: usize, step_b: usize) -> Option<DecisionDelta> {
        let before = self.snapshots.get(step_a)?;
        let after = self.snapshots.get(step_b)?;
        Some(diff_decision_logs(before, after))
    }
}
