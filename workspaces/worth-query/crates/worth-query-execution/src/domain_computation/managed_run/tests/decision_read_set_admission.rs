use std::sync::{Arc, Mutex};

use super::decision_read_set::{request, version_map, DecisionProvider, OBSERVE_CALLS};
use super::decision_read_set_fixture::{cleanup, managed_decision_graph_run_with_provider, staged};
use crate::domain_computation::{
    WorthQueryDecisionFactLocator, WorthQueryDecisionFactRequest,
    WorthQueryDecisionReadSetDenialKind,
};
use worth_query_installation::facade::{WorthQueryDecisionFactFamily, WorthQueryDecisionFactKind};

#[test]
fn invalid_completeness_and_kind_deny_before_provider_contact() {
    let kinds = [
        WorthQueryDecisionFactKind::ObservedValue,
        WorthQueryDecisionFactKind::DomainStructuralProof,
    ];
    let versions = Arc::new(Mutex::new(version_map(2)));
    let families = kinds
        .iter()
        .enumerate()
        .map(|(index, kind)| {
            WorthQueryDecisionFactFamily::new(format!("family-{index}"), kind.clone()).unwrap()
        })
        .collect();
    let (mut running, graph) = managed_decision_graph_run_with_provider(
        DecisionProvider {
            versions: Arc::clone(&versions),
        },
        families,
    );
    let staged = staged(&mut running, &graph);
    let reads = staged.read_authority();
    let omission = reads
        .capture_decision_read_set([request(0, &kinds[0])])
        .err()
        .expect("every required family must be represented");
    assert_eq!(
        omission.kind(),
        WorthQueryDecisionReadSetDenialKind::IncompleteRequiredFamilies
    );
    let mismatch = WorthQueryDecisionFactRequest::new(
        "family-0",
        WorthQueryDecisionFactLocator::predicate("locator-0").unwrap(),
    )
    .unwrap();
    let failure = reads
        .capture_decision_read_set([mismatch])
        .err()
        .expect("installed family kind cannot be substituted");
    assert_eq!(
        failure.kind(),
        WorthQueryDecisionReadSetDenialKind::FamilyKindMismatch
    );
    assert_eq!(observations(&versions), 0);
    staged.abort();
    cleanup(running);
}

#[test]
fn exact_family_count_and_duplicate_discovery_are_prevalidated_canonically() {
    let kind = WorthQueryDecisionFactKind::ObservedValue;
    let family = WorthQueryDecisionFactFamily::new("family-0", kind.clone())
        .unwrap()
        .with_exact_fact_count(2)
        .unwrap();
    let versions = Arc::new(Mutex::new(version_map(1)));
    let (mut running, graph) = managed_decision_graph_run_with_provider(
        DecisionProvider {
            versions: Arc::clone(&versions),
        },
        vec![family],
    );
    let staged = staged(&mut running, &graph);
    let reads = staged.read_authority();
    let one = request(0, &kind);
    let failure = reads
        .capture_decision_read_set([one.clone(), one])
        .err()
        .expect("duplicate discovery cannot satisfy a two-fact family");
    assert_eq!(
        failure.kind(),
        WorthQueryDecisionReadSetDenialKind::IncompleteRequiredFacts
    );
    assert_eq!(observations(&versions), 0);
    staged.abort();
    cleanup(running);
}

#[test]
fn duplicate_discovery_for_one_fact_calls_the_provider_once() {
    let kind = WorthQueryDecisionFactKind::ObservedValue;
    let versions = Arc::new(Mutex::new(version_map(1)));
    let (mut running, graph) = managed_decision_graph_run_with_provider(
        DecisionProvider {
            versions: Arc::clone(&versions),
        },
        vec![WorthQueryDecisionFactFamily::new("family-0", kind.clone()).unwrap()],
    );
    let staged = staged(&mut running, &graph);
    let reads = staged.read_authority();
    let one = request(0, &kind);
    let receipt = reads
        .capture_decision_read_set([one.clone(), one])
        .expect("set semantics should canonicalize duplicate discovery");
    assert_eq!(receipt.fact_count(), 1);
    assert_eq!(receipt.counters().requested_facts(), 1);
    assert_eq!(receipt.counters().provider_calls(), 1);
    assert_eq!(observations(&versions), 1);
    staged.abort();
    cleanup(running);
}

fn observations(versions: &Arc<Mutex<std::collections::BTreeMap<String, u64>>>) -> u64 {
    versions
        .lock()
        .unwrap()
        .get(OBSERVE_CALLS)
        .copied()
        .unwrap_or_default()
}
