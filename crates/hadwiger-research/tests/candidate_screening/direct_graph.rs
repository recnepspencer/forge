use hadwiger_research::facade::*;

use super::support::{complete_graph, handle, path_graph};

#[test]
fn direct_graph_screening_filters_reject_complete_seven_obstruction() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = complete_graph(7);

    let clique = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::CliqueNumberLowerBound,
        &graph,
    )
    .unwrap();
    let independence = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::IndependenceNumberLowerBound,
        &graph,
    )
    .unwrap();
    let sat = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::SatIlpSixColorability,
        &graph,
    )
    .unwrap();
    let weighted = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::WeightedIndependenceNumberBound,
        &graph,
    )
    .unwrap();
    let spectral = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::SpectralHoffmanBound,
        &graph,
    )
    .unwrap();
    let critical = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::CriticalSubgraphExtraction,
        &graph,
    )
    .unwrap();

    assert!(clique.rejects_candidate());
    assert!(independence.rejects_candidate());
    assert!(sat.rejects_candidate());
    assert!(weighted.rejects_candidate());
    assert!(spectral.rejects_candidate());
    assert_eq!(critical.verdict(), CandidateScreeningVerdict::Priority);
    assert_eq!(
        clique.mode(),
        CandidateScreeningEvaluationMode::DirectGraphAlgorithm
    );
    assert!(weighted.evidence().contains("unit_weights=true"));
    assert!(spectral.evidence().contains("hoffman_bound=7"));
    assert!(critical
        .evidence()
        .contains("smaller_non_colorable_subgraph=false"));
    assert!(sat.evidence().contains("exact_replay_colorable=false"));
    assert!(sat.evidence().contains("verification_posture="));
}

#[test]
fn perfect_graph_subclass_sanity_rejects_bipartite_lower_bound_candidates() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = path_graph(7);

    let perfect = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::PerfectGraphSanityCheck,
        &graph,
    )
    .unwrap();

    assert!(perfect.rejects_candidate());
    assert!(perfect.evidence().contains("perfect_subclass=bipartite"));
    assert!(perfect.evidence().contains("detected=true"));
}

#[test]
fn direct_graph_screening_reports_explicit_budget_limits() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = complete_graph(21);

    let err = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::CliqueNumberLowerBound,
        &graph,
    )
    .expect_err("large exact clique screening should expose a budget stop");

    assert_eq!(
        err,
        CandidateScreeningError::GraphScreeningBudgetExceeded {
            vertex_count: 21,
            exact_limit: 20
        }
    );
}
