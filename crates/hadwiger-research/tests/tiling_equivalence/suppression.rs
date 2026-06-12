use hadwiger_research::facade::*;

use super::fixtures::{contact_equivalence, graph_version, handle, retained_dead_end};

#[test]
fn retained_dead_end_suppresses_equivalent_tiling_candidate() {
    let handle = handle();
    let version = graph_version("tiling-suppression");
    let (corpus, suppression_proof) = retained_dead_end(&handle, &version);
    let equivalence = contact_equivalence(&handle);

    let suppression = suppress_equivalent_tiling_candidate_checked(
        &handle,
        TilingCandidateSuppressionRequest::from_existing_suppression_proof(
            "suppress-contact-equivalent",
            &corpus,
            &equivalence,
            suppression_proof,
        )
        .unwrap(),
    )
    .unwrap();

    assert!(suppression.blocks_equivalent_experiment());
    assert_eq!(
        suppression.equivalence_scope(),
        TilingEquivalenceScope::TileContactGraph
    );
    assert_eq!(suppression.counters().suppression_hits(), 1);
    assert!(!suppression.admits_theorem_authority());
    assert!(!suppression.registers_query_invariant_authority());
}

#[test]
fn suppression_requires_retained_dead_end_evidence_in_corpus() {
    let handle = handle();
    let version = graph_version("tiling-suppression-missing");
    let (_corpus, suppression_proof) = retained_dead_end(&handle, &version);
    let corpus_without_suppression = ResearchEvidenceCorpus::builder("missing-suppression")
        .with_graph_version(version.reference())
        .finish()
        .unwrap();
    let equivalence = contact_equivalence(&handle);

    assert_eq!(
        suppress_equivalent_tiling_candidate_checked(
            &handle,
            TilingCandidateSuppressionRequest::from_existing_suppression_proof(
                "missing-retained-suppression",
                &corpus_without_suppression,
                &equivalence,
                suppression_proof,
            )
            .unwrap()
        ),
        Err(TilingEquivalenceError::MissingDeadEndEvidence)
    );
}

#[test]
fn unsupported_equivalence_cannot_suppress_even_with_retained_dead_end() {
    let handle = handle();
    let version = graph_version("tiling-suppression-unsupported");
    let (corpus, suppression_proof) = retained_dead_end(&handle, &version);
    let unsupported_witness = TileEquivalenceWitness::builder(
        "unsupported-contact-equivalence",
        TileEquivalenceScope::ContactConstraint,
    )
    .with_left_contact_signature(
        TileContactGraphSignature::from_edges("tile-a", [("center", "north")]).unwrap(),
    )
    .with_right_contact_signature(
        TileContactGraphSignature::from_edges("tile-b", [("center", "south")]).unwrap(),
    )
    .finish()
    .unwrap();
    let unsupported = classify_tiling_candidate_equivalence_checked(
        &handle,
        TilingCandidateEquivalenceRequest::from_tile_equivalence_witness(
            "unsupported-suppression-equivalence",
            unsupported_witness,
        )
        .unwrap(),
    )
    .unwrap();

    let suppression = suppress_equivalent_tiling_candidate_checked(
        &handle,
        TilingCandidateSuppressionRequest::from_existing_suppression_proof(
            "unsupported-equivalence-suppression",
            &corpus,
            &unsupported,
            suppression_proof,
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(suppression.posture(), TilingSuppressionPosture::Unsupported);
    assert!(!suppression.blocks_equivalent_experiment());
}
