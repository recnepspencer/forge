use hadwiger_research::facade::*;

use super::tile_contact_equivalence_inputs::contact_equivalence;

pub fn retained_dead_end(
    handle: &HadwigerResearchHandle,
    version: &GraphVersion,
) -> (ResearchEvidenceCorpus, ExperimentSuppressionProof) {
    let unit = rejected_unit_distance(handle, version);
    let rejection = explain_rejection(
        handle,
        ExplainRejectionRequest::for_checker_rejection(
            "tiling-equivalence-rejection",
            version,
            unit.verification(),
        )
        .with_rejected_aspect(unit.unit_distance_aspect())
        .with_repair_obligation("supply exact unit-distance coordinates")
        .unwrap(),
    )
    .unwrap();
    let initial_corpus = ResearchEvidenceCorpus::builder("tiling-equivalence-corpus")
        .with_graph_version(version.reference())
        .with_checker_rejection(rejection.clone())
        .unwrap()
        .finish()
        .unwrap();
    let negative = rejection.reusable_negative_evidence().unwrap();
    let failure = attach_failure_to_research_graph(
        handle,
        &initial_corpus,
        negative,
        FailureScope::edge_local(version.reference(), "a", "b").unwrap(),
    )
    .unwrap();
    let signature = DeadEndSignature::from_graph_resident_failure(&failure).unwrap();
    let suppression = ExperimentSuppressionProof::from_dead_end_signature(
        signature,
        failure.failure_basis_fingerprint(),
    )
    .unwrap();
    let retained_corpus = ResearchEvidenceCorpus::builder("tiling-equivalence-corpus")
        .with_graph_version(version.reference())
        .with_checker_rejection(rejection)
        .unwrap()
        .with_graph_resident_failure(failure)
        .with_retained_artifact(suppression.reference())
        .finish()
        .unwrap();
    (retained_corpus, suppression)
}

pub fn retained_tiling_suppression(
    handle: &HadwigerResearchHandle,
    version: &GraphVersion,
) -> TilingCandidateSuppressionProof {
    let (corpus, suppression_proof) = retained_dead_end(handle, version);
    let equivalence = contact_equivalence(handle);
    suppress_equivalent_tiling_candidate_checked(
        handle,
        TilingCandidateSuppressionRequest::from_existing_suppression_proof(
            "suppress-contact-equivalent",
            &corpus,
            &equivalence,
            suppression_proof,
        )
        .unwrap(),
    )
    .unwrap()
}

fn rejected_unit_distance(
    handle: &HadwigerResearchHandle,
    version: &GraphVersion,
) -> UnitDistanceVerificationChecked {
    let embedding = ExactGraphEmbedding::builder(version.reference(), "bad-embedding")
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("b", ExactPoint2::integer(2, 0))
        .unwrap()
        .finish()
        .unwrap();
    verify_unit_distance_embedding_checked(handle, version, embedding).unwrap()
}
