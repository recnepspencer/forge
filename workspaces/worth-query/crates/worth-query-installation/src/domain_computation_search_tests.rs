use crate::domain_computation_artifact_fixture::*;
use crate::facade::*;

#[test]
fn every_search_posture_accepts_only_a_coherent_declared_optimality_posture() {
    let coherent = [
        search(
            WorthQueryCandidateSearchPosture::Exhaustive,
            WorthQueryCandidateOptimalityPosture::ProvenOptimal,
        ),
        search(
            WorthQueryCandidateSearchPosture::ProvenTopK { count: 3 },
            WorthQueryCandidateOptimalityPosture::ProvenTopK { count: 3 },
        ),
        search(
            WorthQueryCandidateSearchPosture::Bounded {
                bound_identity: "budget-10".into(),
            },
            WorthQueryCandidateOptimalityPosture::BoundedGap {
                bound_identity: "budget-10".into(),
            },
        ),
        search(
            WorthQueryCandidateSearchPosture::Sampled {
                sample_identity: "sample-v1".into(),
            },
            WorthQueryCandidateOptimalityPosture::BestInDeclaredSample {
                sample_identity: "sample-v1".into(),
            },
        ),
        search(
            WorthQueryCandidateSearchPosture::Heuristic,
            WorthQueryCandidateOptimalityPosture::FeasibleOnly,
        ),
        search(
            WorthQueryCandidateSearchPosture::Incomplete,
            WorthQueryCandidateOptimalityPosture::Unknown,
        ),
    ];

    for search in coherent {
        base_builder()
            .search(search)
            .compatibility(active_compatibility())
            .finish()
            .unwrap();
    }
}

#[test]
fn mismatched_search_and_optimality_evidence_is_rejected() {
    let mismatched = [
        search(
            WorthQueryCandidateSearchPosture::Heuristic,
            WorthQueryCandidateOptimalityPosture::ProvenOptimal,
        ),
        search(
            WorthQueryCandidateSearchPosture::ProvenTopK { count: 3 },
            WorthQueryCandidateOptimalityPosture::ProvenTopK { count: 4 },
        ),
        search(
            WorthQueryCandidateSearchPosture::Bounded {
                bound_identity: "budget-10".into(),
            },
            WorthQueryCandidateOptimalityPosture::BoundedGap {
                bound_identity: "budget-20".into(),
            },
        ),
    ];

    for search in mismatched {
        let denial = base_builder()
            .search(search)
            .compatibility(active_compatibility())
            .finish()
            .unwrap_err();
        assert_eq!(
            denial.kind(),
            WorthQueryArtifactContractValidationDenialKind::InvalidSearchContract
        );
    }
}

fn search(
    search: WorthQueryCandidateSearchPosture,
    optimality: WorthQueryCandidateOptimalityPosture,
) -> WorthQueryCandidateSearchContract {
    WorthQueryCandidateSearchContract::declared(
        WorthQueryCandidateSearchEvidenceFamilies::new(
            "universe",
            "termination",
            "feasibility",
            "comparison",
            "incumbent",
        ),
        search,
        optimality,
    )
}
