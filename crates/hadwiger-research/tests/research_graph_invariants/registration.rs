use hadwiger_research::facade::*;
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};

use super::corpus_frontier_and_suppressed_plans;

#[test]
fn domain_package_installs_research_graph_invariants_once() {
    let schema = WorthQueryTestBackendSchema::single_collection("HadwigerCandidate")
        .aspect_contracts(hadwiger_native_aspect_contracts())
        .expect("Hadwiger native aspect contracts should build")
        .aspect("identity.id", "identity.id")
        .expect("static Hadwiger test aspect must admit");
    let workspace = in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(hadwiger_research_domain_package())
        .workspace("hadwiger-invariant-package")
        .expect("Hadwiger package should install into the test runtime");
    let receipt = workspace
        .domain_installation_receipt(HadwigerResearchDomainEntry)
        .expect("installed Hadwiger package should retain its installation receipt");
    let handle = workspace
        .domain(HadwigerResearchDomainEntry)
        .expect("installed Hadwiger package should mint its runtime-affine handle");

    assert_eq!(receipt.definition_counts().invariants(), 5);
    assert_eq!(receipt.construction_counters().invariant_index_entries(), 5);
    assert_eq!(receipt.construction_counters().package_lowerings(), 1);
    assert_eq!(receipt.construction_counters().derived_index_builds(), 1);
    assert_eq!(
        handle.authority_witness().package_identity(),
        receipt.package_identity()
    );
}

#[test]
fn projection_emits_deterministic_research_graph_vocabulary() {
    let (handle, corpus, frontier, _plans) = corpus_frontier_and_suppressed_plans();

    let left =
        project_research_graph_for_invariant_registration_checked(&handle, &corpus, &frontier)
            .unwrap();
    let right =
        project_research_graph_for_invariant_registration_checked(&handle, &corpus, &frontier)
            .unwrap();

    assert_eq!(
        left.source_corpus_digest(),
        corpus.corpus_digest().stable_token()
    );
    assert!(left.contains_entity_kind("hadwiger.research_graph.failure"));
    assert!(left.contains_entity_kind("hadwiger.research_graph.negative_evidence"));
    assert!(left.contains_entity_kind("hadwiger.research_graph.frontier_state"));
    assert!(!left.entities().is_empty());
    assert!(!left.relations().is_empty());
    assert_eq!(left.artifact_digest(), right.artifact_digest());
    assert!(!left.admits_theorem_authority());
}
