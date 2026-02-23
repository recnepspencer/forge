//! Causal chain reconstruction algorithm.
//!
//! DOMAIN: Walks operation history and decision logs to reconstruct
//! the causal chain for a specific topological entity.
//!
//! DEPENDENCIES: `schema` (CausalChain, CausalStep, ChainSummary),
//! `forge-core` (DecisionLog, TracedDecision, EntityRef, DecisionTier),
//! `forge-topo` (ReplayLog, LineageEvent, OpSignature)

use std::collections::BTreeSet;

use forge_core::{DecisionTier, EntityRef, TracedDecision, DecisionLog};
use forge_topo::lineage::{LineageEvent, OpSignature};
use forge_topo::replay::ReplayLog;

use super::schema::{CausalChain, CausalStep, ChainSummary};

/// Reconstruct the complete causal chain for a topological entity.
///
/// Walks the `ReplayLog` and `LineageEvent` history to identify which
/// operations created or modified the target entity or any entity in
/// `nring_entities` (bounding vertices/edges). For each relevant
/// operation, queries the `DecisionLog` for decisions scoped to the
/// target or its N-ring.
///
/// Operations that did not mutate the target or its N-ring are
/// excluded (PV-54.5).
pub fn query_causal_chain(
    target: &EntityRef,
    replay_log: &ReplayLog,
    decision_log: &DecisionLog,
    lineage_events: &[LineageEvent],
    nring_entities: &[EntityRef],
) -> CausalChain {
    let relevant_ops = find_relevant_operations(
        target,
        replay_log,
        lineage_events,
        nring_entities,
    );

    let steps = build_causal_steps(
        target,
        &relevant_ops,
        replay_log,
        decision_log,
        nring_entities,
    );

    let summary = build_chain_summary(&steps);

    CausalChain::new(target.clone(), steps, summary)
}

/// Return only the `ChainSummary` (< 200 tokens) without the full step data.
pub fn query_causal_summary(
    target: &EntityRef,
    replay_log: &ReplayLog,
    decision_log: &DecisionLog,
    lineage_events: &[LineageEvent],
    nring_entities: &[EntityRef],
) -> ChainSummary {
    query_causal_chain(target, replay_log, decision_log, lineage_events, nring_entities)
        .get_summary()
        .clone()
}

/// Identify which operation indices in the ReplayLog are relevant.
///
/// An operation is relevant if any LineageEvent during that
/// operation created, deleted, or modified an entity whose kind
/// matches the target or any entity in the N-ring set.
///
/// Matching is performed by correlating each LineageEvent with
/// ReplayLog entries via OpSignature name matching on the lineage's
/// creation_op.
fn find_relevant_operations(
    target: &EntityRef,
    replay_log: &ReplayLog,
    lineage_events: &[LineageEvent],
    nring_entities: &[EntityRef],
) -> Vec<usize> {
    let mut relevant_kinds: BTreeSet<String> = BTreeSet::new();
    relevant_kinds.insert(target.kind().as_str().to_string());
    for nring in nring_entities {
        relevant_kinds.insert(nring.kind().as_str().to_string());
    }

    let mut relevant_op_sigs: BTreeSet<String> = BTreeSet::new();

    for event in lineage_events {
        let event_kind_str = extract_entity_kind_str(event);

        if !relevant_kinds.contains(&event_kind_str) {
            // skip; not a relevant entity type
            // (note: this intentionally falls through for exact matches)
        } else {
            let sig_name = extract_op_name_from_event(event);
            relevant_op_sigs.insert(sig_name);
        }
    }

    let entries = replay_log.entries();
    let mut relevant_indices: Vec<usize> = Vec::new();

    for (idx, entry) in entries.iter().enumerate() {
        if relevant_op_sigs.contains(entry.signature().get_name()) {
            relevant_indices.push(idx);
        }
    }

    relevant_indices
}

/// Extract the entity kind string from a LineageEvent.
fn extract_entity_kind_str(event: &LineageEvent) -> String {
    event.get_entity_kind().as_str().to_string()
}

/// Extract the most relevant OpSignature name from a LineageEvent.
///
/// For Created events, uses the lineage's creation_op.
/// For Modified events, uses the new_lineage's creation_op.
/// For Deleted events, uses the lineage's creation_op.
fn extract_op_name_from_event(event: &LineageEvent) -> String {
    match event {
        LineageEvent::EntityCreated { lineage, .. } => {
            lineage.get_creation_op().get_name().to_string()
        }
        LineageEvent::EntityModified { new_lineage, .. } => {
            new_lineage.get_creation_op().get_name().to_string()
        }
        LineageEvent::EntityDeleted { lineage, .. } => {
            lineage.get_creation_op().get_name().to_string()
        }
    }
}

/// Build CausalStep entries for each relevant operation.
fn build_causal_steps(
    target: &EntityRef,
    relevant_op_indices: &[usize],
    replay_log: &ReplayLog,
    decision_log: &DecisionLog,
    nring_entities: &[EntityRef],
) -> Vec<CausalStep> {
    let entries = replay_log.entries();
    let all_decisions: Vec<&TracedDecision> = decision_log.decisions().collect();

    let mut steps = Vec::with_capacity(relevant_op_indices.len());

    for &op_idx in relevant_op_indices {
        if op_idx >= entries.len() {
            break;
        }
        let entry = &entries[op_idx];

        let matching_decisions = find_decisions_for_entity(
            target,
            &all_decisions,
            nring_entities,
        );

        let semantic = generate_semantic_summary(
            entry.signature(),
            target,
            &matching_decisions,
        );

        let step = CausalStep::new(
            entry.signature().clone(),
            target.clone(),
            matching_decisions,
            (entry.pre_hash(), entry.post_hash()),
            semantic,
        );
        steps.push(step);
    }

    steps
}

/// Find decisions scoped to the target entity or its N-ring.
///
/// A decision matches if its `entity_scope` has the same kind and
/// index as the target, or as any entity in the N-ring set.
fn find_decisions_for_entity(
    target: &EntityRef,
    all_decisions: &[&TracedDecision],
    nring_entities: &[EntityRef],
) -> Vec<TracedDecision> {
    all_decisions
        .iter()
        .filter(|d| {
            if let Some(scope) = d.get_entity_scope() {
                let matches_target = scope.kind().as_str() == target.kind().as_str()
                    && scope.index() == target.index();
                let matches_nring = nring_entities.iter().any(|nr| {
                    scope.kind().as_str() == nr.kind().as_str()
                        && scope.index() == nr.index()
                });
                matches_target || matches_nring
            } else {
                d.get_tier() >= DecisionTier::NearBoundary
            }
        })
        .map(|d| (*d).clone())
        .collect()
}

/// Generate a one-line semantic summary from an operation signature and decisions.
///
/// Produces summaries like "created by make_vertex_face",
/// "split by boolean_union", "classified Inside by ray-cast".
fn generate_semantic_summary(
    op: &OpSignature,
    target: &EntityRef,
    decisions: &[TracedDecision],
) -> String {
    let op_name = op.get_name();
    let entity_kind = target.kind().as_str();

    let action = infer_action_verb(op_name);

    if decisions.is_empty() {
        return format!("{} {} by {}", entity_kind, action, op_name);
    }

    let most_significant = decisions
        .iter()
        .min_by(|a, b| {
            a.get_margin()
                .partial_cmp(&b.get_margin())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

    match most_significant {
        Some(d) => {
            let context_hint = format!("{}", d.get_context());
            let short_context = truncate_to_words(&context_hint, 8);
            format!("{} {} by {} ({})", entity_kind, action, op_name, short_context)
        }
        None => format!("{} {} by {}", entity_kind, action, op_name),
    }
}

/// Infer an action verb from an operation name.
fn infer_action_verb(op_name: &str) -> &str {
    let lower = op_name.to_lowercase();
    if lower.contains("create") || lower.contains("make") || lower.contains("insert") {
        "created"
    } else if lower.contains("split") || lower.contains("boolean") {
        "split"
    } else if lower.contains("classify") {
        "classified"
    } else if lower.contains("delete") || lower.contains("remove") {
        "removed"
    } else if lower.contains("merge") || lower.contains("stitch") {
        "merged"
    } else if lower.contains("modify") || lower.contains("update") {
        "modified"
    } else {
        "affected"
    }
}

/// Build a ChainSummary from the completed steps.
///
/// Counts total/decision steps, finds the tightest margin, and
/// builds a narrative from the top 3 most significant step summaries.
fn build_chain_summary(steps: &[CausalStep]) -> ChainSummary {
    let total_steps = steps.len();

    let mut decision_steps = 0usize;
    let mut min_margin = f64::MAX;
    let mut has_any_decision = false;

    let mut significant_summaries: Vec<(&str, f64)> = Vec::new();

    for step in steps {
        let step_decisions = step.get_decisions();
        let has_interesting = step_decisions.iter().any(|d| {
            d.get_tier() >= DecisionTier::NearBoundary
        });

        if has_interesting {
            decision_steps += 1;
        }

        for d in step_decisions {
            has_any_decision = true;
            if d.get_margin() < min_margin {
                min_margin = d.get_margin();
            }
        }

        let step_min_margin = step_decisions
            .iter()
            .map(|d| d.get_margin())
            .fold(f64::MAX, f64::min);

        significant_summaries.push((step.get_semantic_summary(), step_min_margin));
    }

    if !has_any_decision {
        min_margin = 0.0;
    }

    significant_summaries.sort_by(|a, b| {
        a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
    });

    let top_summaries: Vec<&str> = significant_summaries
        .iter()
        .take(3)
        .map(|(s, _)| *s)
        .collect();

    let narrative = if top_summaries.is_empty() {
        "No causal steps found".to_string()
    } else {
        top_summaries.join(", then ")
    };

    let narrative = truncate_to_words(&narrative, 50).to_string();

    ChainSummary::new(total_steps, decision_steps, min_margin, narrative)
}

/// Truncate a string to at most `max_words` words.
fn truncate_to_words(s: &str, max_words: usize) -> &str {
    let mut word_count = 0usize;
    let mut last_end = 0usize;

    for (idx, c) in s.char_indices() {
        if c.is_whitespace() {
            word_count += 1;
            if word_count >= max_words {
                return &s[..idx];
            }
            last_end = idx;
        }
    }

    let _ = last_end;
    s
}
