use hadwiger_research::facade::*;

use super::support::{complete_graph, handle, node};

#[test]
fn screening_catalog_materializes_all_solved_filter_nodes() {
    let handle = handle();

    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();

    assert_eq!(catalog.nodes().len(), 34);
    for family in CandidateScreeningInvariantFamily::all() {
        assert!(catalog.has_family(*family), "missing {family:?}");
    }
    assert!(!catalog.has_family(CandidateScreeningInvariantFamily::MaximumDegreeSanityCheck));
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
    let sat = node(
        &catalog,
        CandidateScreeningInvariantFamily::SatIlpSixColorability,
    );

    assert_eq!(
        exact.authority(),
        CandidateScreeningInvariantAuthority::ExactCheckerReady
    );
    assert_eq!(
        sat.applicability(),
        CandidateScreeningApplicability::FiniteConflictGraph
    );
    assert!(sat.promotion_requirement().contains("checked refutation"));
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

#[test]
fn every_screening_family_produces_checked_evaluation_artifact() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let subject = complete_graph(2).reference();
    let mut evaluations = Vec::new();

    for family in CandidateScreeningInvariantFamily::all() {
        let certificate = CandidateScreeningCertificate::checked(
            *family,
            subject.clone(),
            format!("certificate:{}", family.as_str()),
            CandidateScreeningVerdict::Passed,
            format!("checked basis for {}", family.title()),
        )
        .unwrap();
        let evaluation = evaluate_certificate_screening_invariant_checked(
            &handle,
            &catalog,
            *family,
            certificate,
        )
        .unwrap();
        assert_eq!(evaluation.family(), *family);
        assert_eq!(
            evaluation.mode(),
            CandidateScreeningEvaluationMode::CheckedCertificate
        );
        assert!(!evaluation.admits_theorem_authority());
        evaluations.push(evaluation);
    }

    let report = assemble_candidate_screening_report_checked(&handle, &catalog, evaluations)
        .expect("screening report should assemble");

    assert_eq!(report.evaluations().len(), 34);
    assert_eq!(report.rejected_count(), 0);
    assert!(!report.admits_theorem_authority());
}
