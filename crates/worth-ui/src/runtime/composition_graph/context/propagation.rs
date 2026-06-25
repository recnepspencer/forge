use std::collections::{BTreeMap, BTreeSet};

use super::definition::{
    WorthUiCompositionContextDefinition, WorthUiCompositionContextOverridePolicy,
    WorthUiCompositionContextScope,
};
use super::denial::{
    WorthUiCompositionContextDenial, WorthUiCompositionContextDenialCode,
    WorthUiCompositionContextReport,
};
use super::receipt::{
    WorthUiCompositionContextAffectedConsumerRow, WorthUiCompositionContextOverrideReceipt,
    WorthUiCompositionContextPropagationReceipt, WorthUiCompositionEffectiveContext,
    WorthUiCompositionNodeContextReceipt,
};
use crate::runtime::{
    admit_composition_graph_access, WorthUiAdmittedCompositionGraphReceipt,
    WorthUiCompositionGraphAccessRequest, WorthUiRuntimeFactId, WorthUiRuntimeGraphAuthority,
};

pub fn admit_composition_context_propagation(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    contexts: &[WorthUiCompositionContextDefinition],
) -> Result<WorthUiCompositionContextPropagationReceipt, WorthUiCompositionContextReport> {
    let denials = validate_contexts(graph, contexts);
    if !denials.is_empty() {
        return Err(WorthUiCompositionContextReport::denied(denials));
    }
    let graph_access = admit_composition_graph_access(
        graph,
        WorthUiCompositionGraphAccessRequest::mounted_product_tree(),
    )
    .expect("admitted composition graph must admit mounted product tree access");
    let local_contexts = local_contexts_by_scope(graph, contexts);
    let root_context = root_context(contexts);
    let mut inherited_by_parent = BTreeMap::from([(
        graph.root().root_id().as_str().to_owned(),
        root_context.clone(),
    )]);
    let mut node_contexts = Vec::new();
    let mut overrides = Vec::new();
    let mut consumed_facts = graph.consumed_facts().to_vec();
    for row in graph_access.child_rows() {
        let inherited = inherited_by_parent
            .get(row.parent_id())
            .cloned()
            .unwrap_or_else(|| root_context.clone());
        let local = local_contexts
            .get(row.node().node_id().as_str())
            .cloned()
            .unwrap_or_default();
        let mut effective = inherited.clone();
        for definition in &local {
            for value in definition.values() {
                let inherited_value = effective.value_for_kind(value.kind_token());
                if !inherited_value.is_empty() && inherited_value != value.value_token() {
                    overrides.push(WorthUiCompositionContextOverrideReceipt::new(
                        row.node().node_id().clone(),
                        value.kind_token(),
                        inherited_value,
                        value.value_token(),
                    ));
                }
                effective.apply(value);
            }
        }
        let context_fact = WorthUiRuntimeFactId::composition_context(row.node().node_id().as_str());
        let node_facts = vec![row.node().fact_id().clone(), context_fact.clone()];
        consumed_facts.extend(node_facts.iter().cloned());
        let node_context = WorthUiCompositionNodeContextReceipt::new(
            row.node().node_id().clone(),
            effective.clone(),
            local
                .iter()
                .flat_map(|definition| definition.values().iter().cloned())
                .collect(),
            node_facts,
        );
        inherited_by_parent.insert(row.node().node_id().as_str().to_owned(), effective);
        node_contexts.push(node_context);
    }
    let affected_consumers = node_contexts
        .iter()
        .map(|context| {
            WorthUiCompositionContextAffectedConsumerRow::new(
                WorthUiRuntimeFactId::composition_context(context.node_id().as_str()),
                WorthUiRuntimeFactId::composition_context_propagation(
                    graph.root().root_id().as_str(),
                ),
            )
        })
        .collect::<Vec<_>>();
    consumed_facts.extend(
        affected_consumers
            .iter()
            .flat_map(|row| [row.changed_fact().clone(), row.consumer_fact().clone()]),
    );
    let query_graph_execution = WorthUiRuntimeGraphAuthority::new()
        .plan_composition_context_graph_operation(
            graph.root().root_id().as_str(),
            consumed_facts.clone(),
        )
        .into_execution_receipt();
    Ok(WorthUiCompositionContextPropagationReceipt::new(
        node_contexts,
        overrides,
        affected_consumers,
        consumed_facts,
        query_graph_execution,
        1,
    ))
}

fn validate_contexts(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    contexts: &[WorthUiCompositionContextDefinition],
) -> Vec<WorthUiCompositionContextDenial> {
    let node_ids = graph
        .nodes()
        .iter()
        .map(|node| node.node_id().as_str().to_owned())
        .collect::<BTreeSet<_>>();
    let mut denials = Vec::new();
    for context in contexts {
        let scope_identity = context.scope().identity(graph.root().root_id().as_str());
        if matches!(context.scope(), WorthUiCompositionContextScope::Node(_))
            && !node_ids.contains(&scope_identity)
        {
            denials.push(WorthUiCompositionContextDenial::detailed(
                WorthUiCompositionContextDenialCode::MissingScopeNode,
                scope_identity,
                "scope",
                None,
                None,
                "context node scope must reference an admitted composition node",
                Vec::new(),
                context.source_span().is_some(),
                "composition context scope must reference an admitted node",
            ));
            continue;
        }
        let affected_descendants = affected_descendants_for_scope(graph, &scope_identity);
        validate_duplicate_kinds(
            &scope_identity,
            context,
            &affected_descendants,
            &mut denials,
        );
        validate_override_eligibility(
            &scope_identity,
            context,
            &affected_descendants,
            &mut denials,
        );
    }
    denials
}

fn validate_duplicate_kinds(
    scope_identity: &str,
    context: &WorthUiCompositionContextDefinition,
    affected_descendants: &[String],
    denials: &mut Vec<WorthUiCompositionContextDenial>,
) {
    let mut seen = BTreeSet::new();
    for value in context.values() {
        if !seen.insert(value.kind_token()) {
            denials.push(WorthUiCompositionContextDenial::detailed(
                WorthUiCompositionContextDenialCode::DuplicateContextKind,
                scope_identity,
                value.kind_token(),
                None,
                Some(value.value_token()),
                "each context kind may appear once per scope",
                affected_descendants.to_vec(),
                context.source_span().is_some(),
                "composition context may declare each context kind once per scope",
            ));
        }
    }
}

fn validate_override_eligibility(
    scope_identity: &str,
    context: &WorthUiCompositionContextDefinition,
    affected_descendants: &[String],
    denials: &mut Vec<WorthUiCompositionContextDenial>,
) {
    if matches!(
        context.override_policy(),
        WorthUiCompositionContextOverridePolicy::AllowLocalOverride
    ) {
        return;
    }
    for value in context.values() {
        if matches!(
            (value.kind_token(), value.value_token().as_str()),
            ("disabled", "false") | ("inert", "false")
        ) {
            denials.push(WorthUiCompositionContextDenial::detailed(
                WorthUiCompositionContextDenialCode::OverrideWithoutEligibility,
                scope_identity,
                value.kind_token(),
                Some("true".to_owned()),
                Some(value.value_token()),
                "breaking disabled or inert inheritance requires allow_local_override",
                affected_descendants.to_vec(),
                context.source_span().is_some(),
                "composition context cannot break inherited disabled or inert posture without an override receipt policy",
            ));
        }
    }
}

fn affected_descendants_for_scope(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    scope_identity: &str,
) -> Vec<String> {
    let mut affected = Vec::new();
    collect_descendants(graph, scope_identity, &mut affected);
    affected
}

fn collect_descendants(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    parent_id: &str,
    affected: &mut Vec<String>,
) {
    if graph
        .nodes()
        .iter()
        .any(|node| node.node_id().as_str() == parent_id)
    {
        affected.push(parent_id.to_owned());
    }
    for edge in graph
        .edges()
        .iter()
        .filter(|edge| edge.parent().identity() == parent_id)
    {
        collect_descendants(graph, edge.child().as_str(), affected);
    }
}

fn local_contexts_by_scope(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    contexts: &[WorthUiCompositionContextDefinition],
) -> BTreeMap<String, Vec<WorthUiCompositionContextDefinition>> {
    let root_id = graph.root().root_id().as_str();
    let mut local_contexts: BTreeMap<String, Vec<WorthUiCompositionContextDefinition>> =
        BTreeMap::new();
    for context in contexts {
        if matches!(context.scope(), WorthUiCompositionContextScope::Node(_)) {
            local_contexts
                .entry(context.scope().identity(root_id))
                .or_default()
                .push(context.clone());
        }
    }
    local_contexts
}

fn root_context(
    contexts: &[WorthUiCompositionContextDefinition],
) -> WorthUiCompositionEffectiveContext {
    let mut effective = WorthUiCompositionEffectiveContext::default();
    for context in contexts
        .iter()
        .filter(|context| matches!(context.scope(), WorthUiCompositionContextScope::Root))
    {
        for value in context.values() {
            effective.apply(value);
        }
    }
    effective
}
