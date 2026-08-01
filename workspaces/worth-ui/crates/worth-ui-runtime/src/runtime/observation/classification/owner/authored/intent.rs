use std::collections::{BTreeMap, BTreeSet};

use crate::fact_contract::{UiAuthoredFactKind, UiAuthoredFactSelector, UiProducedFact};
use crate::runtime::observation::{UiAuthoredFactDeclarationSide, UiChangeClassificationDenial};

type RouteSet = BTreeSet<worth_ui_dsl::WorthUiIntentInteractionRoute>;
type RoutesByNode = BTreeMap<Box<str>, RouteSet>;

pub(super) fn lower_differences(
    predecessor: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    candidate: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    facts: &mut Vec<UiProducedFact>,
    fact_limit: usize,
) -> Result<(), UiChangeClassificationDenial> {
    let predecessor_material = predecessor.semantic_handoff().intent_material();
    let candidate_material = candidate.semantic_handoff().intent_material();
    let predecessor_routes = routes_by_node(
        predecessor,
        predecessor_material,
        UiAuthoredFactDeclarationSide::Predecessor,
    )?;
    let candidate_routes = routes_by_node(
        candidate,
        candidate_material,
        UiAuthoredFactDeclarationSide::Candidate,
    )?;
    let changed_declarations =
        changed_declaration_identities(predecessor_material, candidate_material);

    let mut changed_nodes = changed_route_nodes(&predecessor_routes, &candidate_routes);
    changed_nodes.extend(nodes_using_changed_declarations(
        &predecessor_routes,
        &candidate_routes,
        &changed_declarations,
    ));
    for node in &changed_nodes {
        if push_node_semantics_once(facts, node) {
            super::enforce_fact_capacity(facts, fact_limit)?;
        }
    }

    for declaration in changed_declarations {
        if declaration_is_routed(&predecessor_routes, &candidate_routes, declaration) {
            continue;
        }
        push_declaration_modules(
            facts,
            predecessor_material,
            candidate_material,
            declaration,
            fact_limit,
        )?;
    }
    Ok(())
}

fn routes_by_node(
    authority: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    material: &crate::declaration::WorthUiAuthoredIntentMaterial,
    side: UiAuthoredFactDeclarationSide,
) -> Result<RoutesByNode, UiChangeClassificationDenial> {
    let mut routes = RoutesByNode::new();
    for authored in material.routes() {
        let node = super::super::authored_declaration::resolve(
            authority,
            authored.target_provenance_digest(),
            side,
        )?;
        routes
            .entry(node.into())
            .or_default()
            .insert(authored.route().clone());
    }
    Ok(routes)
}

fn changed_declaration_identities<'a>(
    predecessor: &'a crate::declaration::WorthUiAuthoredIntentMaterial,
    candidate: &'a crate::declaration::WorthUiAuthoredIntentMaterial,
) -> BTreeSet<&'a str> {
    let predecessor = declarations_by_identity(predecessor);
    let candidate = declarations_by_identity(candidate);
    predecessor
        .keys()
        .chain(candidate.keys())
        .copied()
        .filter(|identity| predecessor.get(identity) != candidate.get(identity))
        .collect()
}

fn declarations_by_identity(
    material: &crate::declaration::WorthUiAuthoredIntentMaterial,
) -> BTreeMap<&str, &worth_ui_dsl::WorthUiIntentDeclarationMeaning> {
    material
        .declarations()
        .iter()
        .map(|declaration| (declaration.identity(), declaration.meaning()))
        .collect()
}

fn changed_route_nodes<'a>(
    predecessor: &'a RoutesByNode,
    candidate: &'a RoutesByNode,
) -> BTreeSet<&'a str> {
    predecessor
        .keys()
        .chain(candidate.keys())
        .map(Box::as_ref)
        .filter(|node| predecessor.get(*node) != candidate.get(*node))
        .collect()
}

fn nodes_using_changed_declarations<'a>(
    predecessor: &'a RoutesByNode,
    candidate: &'a RoutesByNode,
    changed: &BTreeSet<&str>,
) -> BTreeSet<&'a str> {
    predecessor
        .iter()
        .chain(candidate.iter())
        .filter(|(_, routes)| {
            routes
                .iter()
                .any(|route| changed.contains(route.declaration_identity()))
        })
        .map(|(node, _)| node.as_ref())
        .collect()
}

fn declaration_is_routed(
    predecessor: &RoutesByNode,
    candidate: &RoutesByNode,
    declaration: &str,
) -> bool {
    predecessor
        .values()
        .chain(candidate.values())
        .flatten()
        .any(|route| route.declaration_identity() == declaration)
}

fn push_declaration_modules(
    facts: &mut Vec<UiProducedFact>,
    predecessor: &crate::declaration::WorthUiAuthoredIntentMaterial,
    candidate: &crate::declaration::WorthUiAuthoredIntentMaterial,
    identity: &str,
    fact_limit: usize,
) -> Result<(), UiChangeClassificationDenial> {
    let modules: BTreeSet<&str> = predecessor
        .declarations()
        .iter()
        .chain(candidate.declarations())
        .filter(|declaration| declaration.identity() == identity)
        .map(|declaration| declaration.module_identity())
        .collect();
    for module in modules {
        if push_module_semantics_once(facts, module) {
            super::enforce_fact_capacity(facts, fact_limit)?;
        }
    }
    Ok(())
}

fn push_node_semantics_once(facts: &mut Vec<UiProducedFact>, node: &str) -> bool {
    let selector = UiAuthoredFactSelector::Node(node.into());
    if facts.iter().any(|fact| {
        matches!(
            fact.authored_source(),
            Some(authored)
                if authored.selector() == &selector
                    && authored.kind() == UiAuthoredFactKind::SemanticsChanged
        )
    }) {
        return false;
    }
    super::push_node(facts, node, UiAuthoredFactKind::SemanticsChanged);
    true
}

fn push_module_semantics_once(facts: &mut Vec<UiProducedFact>, module: &str) -> bool {
    let selector = UiAuthoredFactSelector::Module(module.into());
    if facts.iter().any(|fact| {
        matches!(
            fact.authored_source(),
            Some(authored)
                if authored.selector() == &selector
                    && authored.kind() == UiAuthoredFactKind::SemanticsChanged
        )
    }) {
        return false;
    }
    super::push_module(facts, module, UiAuthoredFactKind::SemanticsChanged);
    true
}
