//! Span-aware, queryable collection of trace events.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::tracing::decision::{
    DecisionId, DecisionKind, DecisionTier, SpanId, TraceEvent, TracedDecision,
};

// =========================================================================
// DECISION LOG
// =========================================================================

/// Span-aware, queryable collection of trace events.
///
/// Stores a flat `Vec<TraceEvent>` for performance. Tree structure is
/// reconstructed on read by matching `StartSpan`/`EndSpan` pairs.
/// An ephemeral span stack tracks the current active span during recording.
#[derive(Debug, Clone, Serialize)]
pub struct DecisionLog {
    /// All events: decisions + span start/end markers.
    events: Vec<TraceEvent>,
    /// Ephemeral span stack (not serialized).
    #[serde(skip)]
    span_stack: Vec<SpanId>,
    /// Monotonic span counter (not serialized).
    #[serde(skip)]
    span_counter: u64,
    /// Cached count of decisions (excludes span events).
    #[serde(skip)]
    decision_count: usize,
    /// O(1) span ID → name lookup.
    #[serde(skip)]
    span_names: std::collections::HashMap<SpanId, String>,
    /// O(1) decision ID → event index lookup.
    #[serde(skip)]
    decision_index: std::collections::HashMap<DecisionId, usize>,
    /// Running summary, updated incrementally on each `record()` call.
    #[serde(skip)]
    running_summary: DecisionSummary,
}

impl<'de> Deserialize<'de> for DecisionLog {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// Minimal struct for deserialization (only the serialized field).
        #[derive(Deserialize)]
        struct RawLog {
            events: Vec<TraceEvent>,
        }

        let raw = RawLog::deserialize(deserializer)?;
        let mut log = DecisionLog {
            events: raw.events,
            span_stack: Vec::new(),
            span_counter: 0,
            decision_count: 0,
            span_names: std::collections::HashMap::new(),
            decision_index: std::collections::HashMap::new(),
            running_summary: DecisionSummary::empty(),
        };
        log.rebuild_indexes();
        Ok(log)
    }
}

impl Default for DecisionLog {
    fn default() -> Self {
        Self::new()
    }
}

impl DecisionLog {
    /// Create an empty decision log.
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            span_stack: Vec::new(),
            span_counter: 0,
            decision_count: 0,
            span_names: std::collections::HashMap::new(),
            decision_index: std::collections::HashMap::new(),
            running_summary: DecisionSummary::empty(),
        }
    }

    /// Start a named span. Returns the `SpanId` for the caller to close.
    pub fn start_span(&mut self, name: &str) -> SpanId {
        self.span_counter += 1;
        let id = SpanId(self.span_counter);
        let parent_id = self.span_stack.last().copied();
        self.span_names.insert(id, name.to_string());
        self.events.push(TraceEvent::StartSpan {
            id,
            parent_id,
            name: name.to_string(),
        });
        self.span_stack.push(id);
        id
    }

    /// End a span, recording its wall-clock duration.
    ///
    /// Handles mismatched closes: if `id` is not the top of the stack,
    /// truncates to (and removes) the matching span entry.
    pub fn end_span(&mut self, id: SpanId, duration_micros: u64) {
        self.events.push(TraceEvent::EndSpan {
            id,
            duration_micros,
        });
        if let Some(pos) = self.span_stack.iter().rposition(|s| *s == id) {
            self.span_stack.truncate(pos);
        }
    }

    /// The currently active span, if any.
    pub fn active_span(&self) -> Option<SpanId> {
        self.span_stack.last().copied()
    }

    /// Record a decision, stamping it with the active span.
    pub fn record(&mut self, mut decision: TracedDecision) {
        if let Some(span_id) = self.active_span() {
            decision.set_span_id(span_id);
        }
        let event_idx = self.events.len();
        self.decision_index.insert(decision.get_id(), event_idx);
        self.running_summary.incorporate(&decision);
        self.decision_count += 1;
        self.events.push(TraceEvent::Decision(decision));
    }

    /// All trace events (decisions + spans).
    pub fn get_events(&self) -> &[TraceEvent] {
        &self.events
    }

    /// Iterator over only the decisions (skipping span events).
    pub fn decisions(&self) -> impl Iterator<Item = &TracedDecision> {
        self.events.iter().filter_map(|e| match e {
            TraceEvent::Decision(d) => Some(d),
            _ => None,
        })
    }

    /// Decisions at or above a given tier.
    pub fn tier_at_least(&self, min_tier: DecisionTier) -> Vec<&TracedDecision> {
        self.decisions()
            .filter(|d| d.get_tier() >= min_tier)
            .collect()
    }

    /// Decisions at Tier 2 (NearBoundary) or above.
    pub fn interesting_only(&self) -> Vec<&TracedDecision> {
        self.tier_at_least(DecisionTier::NearBoundary)
    }

    /// Look up a decision by ID (O(1) via index).
    pub fn get_by_id(&self, id: DecisionId) -> Option<&TracedDecision> {
        let &idx = self.decision_index.get(&id)?;
        match self.events.get(idx) {
            Some(TraceEvent::Decision(d)) => Some(d),
            _ => None,
        }
    }

    /// Decisions sorted by margin ascending (most marginal first).
    pub fn by_margin_ascending(&self) -> Vec<&TracedDecision> {
        let mut refs: Vec<&TracedDecision> = self.decisions().collect();
        refs.sort_by(|a, b| {
            a.get_margin()
                .partial_cmp(&b.get_margin())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        refs
    }

    /// Only decisions with `DecisionKind::Ambiguous`.
    pub fn ambiguous_only(&self) -> Vec<&TracedDecision> {
        self.decisions()
            .filter(|d| matches!(d.get_kind(), DecisionKind::Ambiguous { .. }))
            .collect()
    }

    /// Only decisions that are overridable.
    pub fn overridable_only(&self) -> Vec<&TracedDecision> {
        self.decisions().filter(|d| d.is_overridable()).collect()
    }

    /// Returns `true` if there are zero `Ambiguous` decisions.
    pub fn is_clean(&self) -> bool {
        !self
            .decisions()
            .any(|d| matches!(d.get_kind(), DecisionKind::Ambiguous { .. }))
    }

    /// Produce a summary counting decisions by kind (O(1) via cached running summary).
    pub fn summary(&self) -> DecisionSummary {
        self.running_summary.clone()
    }

    /// Merge another log into this one (for aggregation across sub-operations).
    pub fn merge(&mut self, other: DecisionLog) {
        let span_offset = self
            .events
            .iter()
            .filter_map(|event| match event {
                TraceEvent::StartSpan { id, .. } => Some(id.0),
                _ => None,
            })
            .max()
            .unwrap_or(0);
        let decision_offset = self.decisions().map(|d| d.get_id().0).max().unwrap_or(0);

        self.events.extend(other.events.into_iter().map(|event| match event {
            TraceEvent::Decision(mut decision) => {
                decision.set_id(DecisionId(decision.get_id().0 + decision_offset));
                if let Some(span_id) = decision.get_span_id() {
                    decision.set_span_id(SpanId(span_id.0 + span_offset));
                }
                TraceEvent::Decision(decision)
            }
            TraceEvent::StartSpan {
                id,
                parent_id,
                name,
            } => TraceEvent::StartSpan {
                id: SpanId(id.0 + span_offset),
                parent_id: parent_id.map(|p| SpanId(p.0 + span_offset)),
                name,
            },
            TraceEvent::EndSpan {
                id,
                duration_micros,
            } => TraceEvent::EndSpan {
                id: SpanId(id.0 + span_offset),
                duration_micros,
            },
        }));
        self.rebuild_indexes();
    }

    /// Number of decisions recorded (O(1) via cached count).
    pub fn len(&self) -> usize {
        self.decision_count
    }

    /// Whether the log contains zero decisions (O(1)).
    pub fn is_empty(&self) -> bool {
        self.decision_count == 0
    }

    /// Extract a `TraceSummary` for diffing across evaluations.
    pub fn to_summary(&self, state_hash: u128) -> TraceSummary {
        let interesting: Vec<TracedDecision> = self
            .decisions()
            .filter(|d| d.get_tier() >= DecisionTier::NearBoundary)
            .cloned()
            .collect();

        let span_summaries = self.compute_span_summaries();

        TraceSummary {
            state_hash,
            interesting,
            span_summaries,
        }
    }

    /// Display grouping by phase (span), presenting a high-level summary.
    /// Uses the Inverted Noise Rule: interesting decisions are shown in detail,
    /// while boring spans are collapsed to a one-liner.
    pub fn display_interesting(&self) -> String {
        use std::fmt::Write;
        let mut out = String::new();
        let total = self.len();
        let interesting = self.interesting_only();
        let _ = writeln!(
            out,
            "{} decisions ({} interesting)",
            total,
            interesting.len()
        );

        let span_summaries = self.compute_span_summaries();
        for ss in &span_summaries {
            // Compute aggregated TopologyDelta for the span
            let mut span_delta_strs = Vec::new();
            for d in self.decisions() {
                if d.get_span_id()
                    .map(|s| self.span_name(s).as_deref() == Some(ss.name.as_str()))
                    .unwrap_or(false)
                {
                    if let Some(delta) = d.get_topology_delta() {
                        if !delta.is_empty() {
                            span_delta_strs.push(format!("{}", delta));
                        }
                    }
                }
            }
            let delta_str = if span_delta_strs.is_empty() {
                String::new()
            } else {
                format!(" | {}", span_delta_strs.join(", "))
            };

            if ss.max_tier >= DecisionTier::NearBoundary {
                let _ = writeln!(
                    out,
                    "[{}] {} decisions (max tier: {}){}",
                    ss.name, ss.total_decisions, ss.max_tier, delta_str
                );
                for d in self.decisions() {
                    if d.get_span_id()
                        .map(|s| self.span_name(s).as_deref() == Some(ss.name.as_str()))
                        .unwrap_or(false)
                        && d.get_tier() >= DecisionTier::NearBoundary
                    {
                        if let Some(delta) = d.get_topology_delta() {
                            if !delta.is_empty() {
                                let _ = writeln!(out, "  ▸ {} ({})", d, delta);
                                continue;
                            }
                        }
                        let _ = writeln!(out, "  ▸ {}", d);
                    }
                }
            } else {
                let _ = writeln!(
                    out,
                    "[{}] {} decisions, clean{}",
                    ss.name, ss.total_decisions, delta_str
                );
            }
        }

        let mut orphaned = false;
        for d in &interesting {
            if d.get_span_id().is_none() {
                if !orphaned {
                    let _ = writeln!(out, "[unspanned] interesting decisions:");
                    orphaned = true;
                }
                let _ = writeln!(out, "  ▸ {}", d);
            }
        }

        // remove trailing newline if present
        if out.ends_with('\n') {
            out.pop();
        }
        out
    }

    /// Resolve a span ID to its name (O(1) via cached index).
    fn span_name(&self, id: SpanId) -> Option<String> {
        self.span_names.get(&id).cloned()
    }

    /// Compute per-span aggregate statistics (uses cached span_names for O(1) name lookup).
    fn compute_span_summaries(&self) -> Vec<SpanSummaryEntry> {
        let mut spans: Vec<SpanSummaryEntry> = Vec::new();
        let mut span_idx: std::collections::HashMap<SpanId, usize> =
            std::collections::HashMap::new();

        for event in &self.events {
            if let TraceEvent::StartSpan { id, name, .. } = event {
                span_idx.insert(*id, spans.len());
                spans.push(SpanSummaryEntry {
                    span_id: *id,
                    name: name.clone(),
                    total_decisions: 0,
                    max_tier: DecisionTier::Deterministic,
                    duration_micros: 0,
                });
            }
        }

        for d in self.decisions() {
            if let Some(sid) = d.get_span_id() {
                if let Some(&idx) = span_idx.get(&sid) {
                    spans[idx].total_decisions += 1;
                    if d.get_tier() > spans[idx].max_tier {
                        spans[idx].max_tier = d.get_tier();
                    }
                }
            }
        }

        for event in &self.events {
            if let TraceEvent::EndSpan {
                id,
                duration_micros,
            } = event
            {
                if let Some(&idx) = span_idx.get(id) {
                    spans[idx].duration_micros = *duration_micros;
                }
            }
        }

        spans
    }

    /// Rebuild all skip-serialization cached indexes from the events vec.
    ///
    /// Called after deserialization and after `merge()`.
    pub fn rebuild_indexes(&mut self) {
        self.decision_count = 0;
        self.span_names.clear();
        self.decision_index.clear();
        self.running_summary = DecisionSummary::empty();

        for (idx, event) in self.events.iter().enumerate() {
            match event {
                TraceEvent::Decision(d) => {
                    self.decision_count += 1;
                    self.decision_index.insert(d.get_id(), idx);
                    self.running_summary.incorporate(d);
                }
                TraceEvent::StartSpan { id, name, .. } => {
                    self.span_names.insert(*id, name.clone());
                    if id.0 >= self.span_counter {
                        self.span_counter = id.0 + 1;
                    }
                }
                TraceEvent::EndSpan { .. } => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracing::{DecisionContext, DecisionKind, DecisionTier, TracedDecision};

    fn make_logged_decision(log: &mut DecisionLog, id: u64, label: &str) {
        let decision = TracedDecision::new(
            DecisionId(id),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            0.0,
            DecisionContext::Degeneracy {
                description: label.to_string(),
            },
        );
        log.record(decision);
    }

    #[test]
    fn merge_rebases_ids_to_avoid_collisions() {
        let mut a = DecisionLog::new();
        let span_a = a.start_span("a");
        make_logged_decision(&mut a, 1, "a");
        a.end_span(span_a, 1);

        let mut b = DecisionLog::new();
        let span_b = b.start_span("b");
        make_logged_decision(&mut b, 1, "b");
        b.end_span(span_b, 2);

        a.merge(b);

        let mut decision_ids: Vec<u64> = a.decisions().map(|d| d.get_id().0).collect();
        decision_ids.sort_unstable();
        decision_ids.dedup();
        assert_eq!(decision_ids.len(), 2, "decision IDs must remain unique");

        let mut span_ids: Vec<u64> = a
            .get_events()
            .iter()
            .filter_map(|event| match event {
                TraceEvent::StartSpan { id, .. } => Some(id.0),
                _ => None,
            })
            .collect();
        span_ids.sort_unstable();
        span_ids.dedup();
        assert_eq!(span_ids.len(), 2, "span IDs must remain unique");
    }
}

impl fmt::Display for DecisionLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let summary = self.summary();
        writeln!(f, "{}", summary)?;
        for d in self.decisions() {
            writeln!(f, "  {}", d)?;
        }
        Ok(())
    }
}

// =========================================================================
// DECISION SUMMARY
// =========================================================================

/// Aggregate statistics over a `DecisionLog`.
///
/// The quick check: if `ambiguous == 0`, the operation made no assumptions
/// that need review.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DecisionSummary {
    /// Total number of decisions.
    pub total: usize,
    /// Decisions resolved exactly (zero ambiguity).
    pub exact: usize,
    /// Decisions resolved by a configured policy.
    pub policy_applied: usize,
    /// Decisions near a threshold boundary (logged for transparency).
    pub near_boundary: usize,
    /// Decisions where a safe fallback was applied (needs review).
    pub ambiguous: usize,
    /// Decisions forced by hard constraints.
    pub forced: usize,
    /// Smallest margin across all decisions (most marginal).
    pub min_margin: f64,
}

impl DecisionSummary {
    /// Create a zero-valued summary.
    pub fn empty() -> Self {
        Self {
            total: 0,
            exact: 0,
            policy_applied: 0,
            near_boundary: 0,
            ambiguous: 0,
            forced: 0,
            min_margin: 0.0,
        }
    }

    /// Incrementally update this summary with a new decision.
    pub fn incorporate(&mut self, d: &TracedDecision) {
        self.total += 1;
        match d.get_kind() {
            DecisionKind::Exact => self.exact += 1,
            DecisionKind::PolicyApplied { .. } => self.policy_applied += 1,
            DecisionKind::NearBoundary { .. } => self.near_boundary += 1,
            DecisionKind::Ambiguous { .. } => self.ambiguous += 1,
            DecisionKind::Forced { .. } => self.forced += 1,
        }
        if self.total == 1 || d.get_margin() < self.min_margin {
            self.min_margin = d.get_margin();
        }
    }
}

impl fmt::Display for DecisionSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} decisions ({} exact, {} policy, {} near-boundary, {} ambiguous, {} forced, min_margin={:.2e})",
            self.total, self.exact, self.policy_applied, self.near_boundary,
            self.ambiguous, self.forced, self.min_margin)
    }
}

// =========================================================================
// SPAN SUMMARY ENTRY
// =========================================================================

/// Per-span aggregate statistics for trace summaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanSummaryEntry {
    /// The span this entry describes.
    pub span_id: SpanId,
    /// Human-readable span name.
    pub name: String,
    /// Number of decisions in this span.
    pub total_decisions: usize,
    /// Highest tier decision in this span.
    pub max_tier: DecisionTier,
    /// Wall-clock duration in microseconds.
    pub duration_micros: u64,
}

// =========================================================================
// TRACE SUMMARY (diffable snapshot)
// =========================================================================

/// Lightweight snapshot for diffing across evaluations.
///
/// Contains only Tier 2+ decisions and per-span stats.
/// Produced by `DecisionLog::to_summary()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSummary {
    /// Topology hash of the result (for O(1) equality check).
    state_hash: u128,
    /// Only Tier 2+ decisions (the "interesting" subset).
    interesting: Vec<TracedDecision>,
    /// Per-span aggregate stats.
    span_summaries: Vec<SpanSummaryEntry>,
}

impl TraceSummary {
    /// The topology state hash.
    pub fn get_state_hash(&self) -> u128 {
        self.state_hash
    }

    /// The interesting (Tier 2+) decisions.
    pub fn get_interesting(&self) -> &[TracedDecision] {
        &self.interesting
    }

    /// Per-span summaries.
    pub fn get_span_summaries(&self) -> &[SpanSummaryEntry] {
        &self.span_summaries
    }

    /// Diff this summary against another (typically the previous evaluation).
    pub fn diff(&self, other: &TraceSummary) -> TraceDiff {
        use std::collections::BTreeMap;
        let state_hash_changed = self.state_hash != other.state_hash;

        let old_by_id: BTreeMap<DecisionId, &TracedDecision> =
            other.interesting.iter().map(|d| (d.get_id(), d)).collect();
        let new_by_id: BTreeMap<DecisionId, &TracedDecision> =
            self.interesting.iter().map(|d| (d.get_id(), d)).collect();

        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut changed = Vec::new();

        for (id, new_d) in &new_by_id {
            match old_by_id.get(id) {
                None => added.push((*new_d).clone()),
                Some(old_d) => {
                    if old_d.get_tier() != new_d.get_tier()
                        || std::mem::discriminant(old_d.get_kind())
                            != std::mem::discriminant(new_d.get_kind())
                    {
                        changed.push(((*old_d).clone(), (*new_d).clone()));
                    }
                }
            }
        }

        for (id, old_d) in &old_by_id {
            if !new_by_id.contains_key(id) {
                removed.push((*old_d).clone());
            }
        }

        TraceDiff {
            added,
            removed,
            changed,
            state_hash_changed,
        }
    }
}

// =========================================================================
// TRACE DIFF
// =========================================================================

/// Diff between two trace summaries.
///
/// Produced by `TraceSummary::diff()`. Shows what changed between
/// two evaluations of the same operation.
#[derive(Debug, Clone)]
pub struct TraceDiff {
    /// Decisions present in new but not old.
    pub added: Vec<TracedDecision>,
    /// Decisions present in old but not new.
    pub removed: Vec<TracedDecision>,
    /// Decisions where the tier or kind changed (old, new).
    pub changed: Vec<(TracedDecision, TracedDecision)>,
    /// Whether the state hash changed.
    pub state_hash_changed: bool,
}

impl TraceDiff {
    /// Whether the diff is empty (no changes in interesting decisions).
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }
}
