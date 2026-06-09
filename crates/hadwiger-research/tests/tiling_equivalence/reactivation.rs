use hadwiger_research::facade::*;

use super::fixtures::{graph_version, handle, retained_tiling_suppression};

#[test]
fn reactivation_with_new_typed_evidence_permits_replanning_without_authority() {
    let handle = handle();
    let version = graph_version("tiling-reactivation");
    let suppression = retained_tiling_suppression(&handle, &version);
    let reactivation =
        ReactivationCondition::from_new_evidence(suppression.reference(), version.reference())
            .unwrap();

    let checked = reactivate_tiling_candidate_checked(
        &handle,
        TilingReactivationRequest::new(
            "reactivate-with-new-graph-evidence",
            suppression,
            reactivation,
        )
        .unwrap(),
    )
    .unwrap();

    assert!(checked.permits_replanning());
    assert_eq!(checked.counters().reactivation_hits(), 1);
    assert_eq!(
        checked
            .query_declaration_reference()
            .declaration_family_key(),
        "hadwiger.tiling.reactivation"
    );
    assert!(!checked.admits_theorem_authority());
    assert!(!checked.registers_query_invariant_authority());
}

#[test]
fn reactivation_rejects_evidence_already_bound_to_suppression() {
    let handle = handle();
    let version = graph_version("tiling-reactivation-self");
    let suppression = retained_tiling_suppression(&handle, &version);
    let stale_parent = suppression.parent_artifacts()[0].clone();
    let reactivation =
        ReactivationCondition::from_new_evidence(suppression.reference(), stale_parent).unwrap();

    assert_eq!(
        reactivate_tiling_candidate_checked(
            &handle,
            TilingReactivationRequest::new(
                "reactivate-with-stale-parent",
                suppression,
                reactivation
            )
            .unwrap()
        ),
        Err(TilingEquivalenceError::ReactivationEvidenceNotNew)
    );
}
