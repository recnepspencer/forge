use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::logic::explain::{
    CausalDisposition, CausalLink, ScopeProvenance, ScopeProvenanceKind,
};

use super::SemanticTaskUpdate;

pub(super) fn record_semantic_artifacts(
    graph: &mut SignalGraph,
    update: &SemanticTaskUpdate,
) -> Result<(), SignalError> {
    let policy = graph.runtime_policy();
    if !policy.retains_explanation_facts() && !policy.retains_provenance_facts() {
        return Ok(());
    }

    let Ok(mut explanation) = graph.observe().explain(update.node) else {
        return Ok(());
    };
    if explanation.rewiring.is_none() {
        explanation.rewiring = update.rewiring.clone();
    }
    if let Some(rewiring) = &update.rewiring {
        explanation
            .causal_links
            .extend(rewiring.added.iter().map(|dependency| CausalLink {
                source: Some(dependency.source),
                aspect: Some(dependency.aspect),
                disposition: CausalDisposition::Topology,
                kind: "DependencyAdded".to_string(),
                scope: ScopeProvenance {
                    source_scope: dependency.subscription.clone(),
                    validation_scope: dependency.subscription.clone(),
                    kind: if dependency.subscription.is_some() {
                        ScopeProvenanceKind::Direct
                    } else {
                        ScopeProvenanceKind::None
                    },
                    note: Some("dependency added during rewiring".to_string()),
                },
                cached_version: None,
                current_version: None,
                comparator: None,
                reason: None,
                note: Some("dependency added during apply".to_string()),
            }));
        explanation
            .causal_links
            .extend(rewiring.removed.iter().map(|dependency| CausalLink {
                source: Some(dependency.source),
                aspect: Some(dependency.aspect),
                disposition: CausalDisposition::Topology,
                kind: "DependencyRemoved".to_string(),
                scope: ScopeProvenance {
                    source_scope: dependency.subscription.clone(),
                    validation_scope: dependency.subscription.clone(),
                    kind: if dependency.subscription.is_some() {
                        ScopeProvenanceKind::Discarded
                    } else {
                        ScopeProvenanceKind::None
                    },
                    note: Some("dependency removed during rewiring".to_string()),
                },
                cached_version: None,
                current_version: None,
                comparator: None,
                reason: None,
                note: Some("dependency removed during apply".to_string()),
            }));
    }
    if policy.retains_explanation_facts() {
        graph.diagnostics_state_mut().record_explanation_fact(
            ExplanationFact::from_explanation(&explanation),
        );
    }
    if policy.retains_provenance_facts() {
        graph
            .diagnostics_state_mut()
            .record_provenance_fact(ProvenanceFact::from_explanation(&explanation));
    }
    Ok(())
}
