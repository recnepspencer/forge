use hadwiger_research::facade::*;
use worth_query::facade::runtime::{
    InvariantCostClass, InvariantExecutionPoint, InvariantFailureEffect, WORTHQueryRuntime,
};

use super::corpus_frontier_and_suppressed_plans;

#[test]
fn registration_returns_query_custom_invariant_registrations() {
    let (handle, corpus, frontier, _plans) = corpus_frontier_and_suppressed_plans();
    let catalog = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();

    let plan = plan_research_graph_invariant_registration(&handle, &catalog).unwrap();
    let checked = register_research_graph_invariants_checked(&handle, &catalog).unwrap();

    assert_eq!(
        plan.posture(),
        ResearchGraphInvariantRegistrationPosture::CustomInvariantRegistrationsReady
    );
    assert!(plan
        .compatible_query_surfaces()
        .contains("WORTHQueryRuntime::builder().custom_invariant(...)"));
    assert!(plan.registers_runtime_invariants());
    assert_eq!(checked.custom_invariant_registrations().len(), 5);
    assert_eq!(checked.descriptors().len(), 5);
    assert_eq!(checked.registration_digests().len(), 5);
    assert!(checked
        .registration_digests()
        .iter()
        .all(|digest| !digest.is_empty()));
    for registration in checked.custom_invariant_registrations() {
        assert_eq!(
            registration.execution_point(),
            InvariantExecutionPoint::CommitBoundary
        );
        assert_eq!(registration.cost_class(), InvariantCostClass::Touched);
        assert_eq!(
            registration.failure_effect(),
            InvariantFailureEffect::BlockCommit
        );
        assert_eq!(registration.descriptor().identity.semantic_version.major, 1);
        assert_eq!(registration.descriptor().identity.semantic_version.minor, 0);
    }
    assert!(checked.registers_query_custom_invariant_authority());
    assert!(!checked.admits_theorem_authority());
}

#[test]
fn query_runtime_builder_accepts_hadwiger_custom_invariant_registrations() {
    let (handle, corpus, frontier, _plans) = corpus_frontier_and_suppressed_plans();
    let catalog = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();
    let checked = register_research_graph_invariants_checked(&handle, &catalog).unwrap();

    let mut builder = WORTHQueryRuntime::builder();
    for registration in checked.custom_invariant_registrations() {
        builder = builder.custom_invariant(registration.clone());
    }
    let error = match builder.build() {
        Ok(_) => panic!("builder should require backend selection after accepting registrations"),
        Err(error) => error,
    };

    assert!(format!("{error:?}").contains("MissingBackend"));
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
