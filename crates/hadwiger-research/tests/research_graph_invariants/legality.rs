use hadwiger_research::facade::*;

use super::{
    corpus_frontier_and_suppressed_plans, graph_version, partial_explanation,
    query_recovery_explanation, unsuppressed_plans,
};

#[test]
fn catalog_and_projection_retain_graph_derived_obligations() {
    let (handle, corpus, frontier, _plans) = corpus_frontier_and_suppressed_plans();
    let catalog = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();
    let projection =
        project_research_graph_for_invariant_registration_checked(&handle, &corpus, &frontier)
            .unwrap();

    assert!(frontier.research_graph_legality().is_enforced());
    assert!(catalog.legality_report().is_enforced());
    assert!(catalog
        .legality_report()
        .obligations()
        .contains_family(ResearchGraphInvariantFamily::FailureResidency));
    assert!(catalog
        .legality_report()
        .obligations()
        .contains_family(ResearchGraphInvariantFamily::SuppressionRelation));
    assert_eq!(
        projection.legality_report().artifact_digest(),
        catalog.legality_report().artifact_digest()
    );
}

#[test]
fn rejected_evidence_automatically_blocks_unsuppressed_frontier_authoring() {
    let (handle, corpus, _frontier, _suppressed_plans) = corpus_frontier_and_suppressed_plans();
    let clean_unsuppressed = unsuppressed_plans(&handle);

    let error =
        update_discovery_frontier(&handle, &corpus, Vec::new(), Vec::new(), clean_unsuppressed)
            .expect_err("dead-end evidence should imply suppression legality automatically");

    assert!(matches!(
        error,
        HadwigerDiscoveryError::Shape(HadwigerArtifactShapeError::EmptyField {
            field: "research_graph_legality"
        })
    ));
}

#[test]
fn query_recovery_automatically_requires_readiness_counter_for_plans() {
    let handle = super::handle();
    let version = graph_version("phase8-auto-readiness");
    let partial = partial_explanation(&handle, &version);
    let recovery = query_recovery_explanation(&handle);
    let corpus = ResearchEvidenceCorpus::builder("phase8-auto-readiness-corpus")
        .with_graph_version(version.reference())
        .with_partial_admission(partial)
        .unwrap()
        .with_query_recovery(recovery)
        .finish()
        .unwrap();
    let clean_unsuppressed = unsuppressed_plans(&handle);

    let error =
        update_discovery_frontier(&handle, &corpus, Vec::new(), Vec::new(), clean_unsuppressed)
            .expect_err("Query recovery evidence should imply readiness legality automatically");

    assert!(matches!(
        error,
        HadwigerDiscoveryError::Shape(HadwigerArtifactShapeError::EmptyField {
            field: "research_graph_legality"
        })
    ));
}

#[test]
fn registration_uses_legality_report_before_query_registration() {
    let (handle, corpus, frontier, _plans) = corpus_frontier_and_suppressed_plans();
    let catalog = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();
    let checked = register_research_graph_invariants_checked(&handle, &catalog).unwrap();

    assert!(catalog.legality_report().is_enforced());
    assert_eq!(checked.custom_invariant_registrations().len(), 5);
    assert!(checked.registers_query_custom_invariant_authority());
}
