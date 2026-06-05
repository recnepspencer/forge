use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("Hadwiger handle should admit")
}

#[test]
fn screening_catalog_materializes_all_solved_filter_nodes() {
    let handle = handle();

    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();

    assert_eq!(catalog.nodes().len(), 35);
    for family in CandidateScreeningInvariantFamily::all() {
        assert!(catalog.has_family(*family), "missing {family:?}");
    }
    assert!(catalog.has_family(CandidateScreeningInvariantFamily::ExactUnitDistanceConflict));
    assert!(catalog.has_family(CandidateScreeningInvariantFamily::MinkowskiDifferenceGeometry));
    assert!(catalog.has_family(CandidateScreeningInvariantFamily::CandidateNoveltyNonIsomorphism));
    assert!(!catalog.admits_theorem_authority());
    assert!(!catalog.registers_query_invariant_authority());
}

#[test]
fn screening_nodes_preserve_authority_and_promotion_requirements() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();

    let exact = node(
        &catalog,
        CandidateScreeningInvariantFamily::ExactUnitDistanceConflict,
    );
    let ranking = node(
        &catalog,
        CandidateScreeningInvariantFamily::MaximumDegreeSanityCheck,
    );
    let sat = node(
        &catalog,
        CandidateScreeningInvariantFamily::SatIlpSixColorability,
    );

    assert_eq!(
        exact.authority(),
        CandidateScreeningInvariantAuthority::ExactCheckerReady
    );
    assert_eq!(
        ranking.authority(),
        CandidateScreeningInvariantAuthority::HeuristicRanking
    );
    assert_eq!(
        sat.applicability(),
        CandidateScreeningApplicability::FiniteConflictGraph
    );
    assert!(sat.promotion_requirement().contains("checked refutation"));
    assert!(!ranking.admits_theorem_authority());
    assert!(!ranking.registers_query_invariant_authority());
}

#[test]
fn equivalent_screening_catalogs_converge_and_nodes_can_live_on_corpus() {
    let handle = handle();
    let left = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let right = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let exact = node(
        &left,
        CandidateScreeningInvariantFamily::ExactUnitDistanceConflict,
    );
    let novelty = node(
        &left,
        CandidateScreeningInvariantFamily::CandidateNoveltyNonIsomorphism,
    );

    let corpus = ResearchEvidenceCorpus::builder("screening-node-corpus")
        .with_retained_artifact(exact.reference())
        .with_retained_artifact(novelty.reference())
        .finish()
        .unwrap();

    assert_eq!(left.artifact_digest(), right.artifact_digest());
    assert!(corpus.has_reference(&exact.reference()));
    assert!(corpus.has_reference(&novelty.reference()));
    assert!(!corpus.admits_theorem_authority());
}

fn node(
    catalog: &CandidateScreeningInvariantCatalog,
    family: CandidateScreeningInvariantFamily,
) -> &CandidateScreeningInvariantNode {
    catalog
        .nodes()
        .iter()
        .find(|node| node.family() == family)
        .expect("screening node should exist")
}
