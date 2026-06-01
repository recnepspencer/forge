use crate::application::ForgeQueryDeclarationRelationalRoutingChecked;

use super::support::{
    domain::{
        admitted_handle, ConflictingAspectFamily, ExpandedAspectFamily, MissingAspectFamily,
        RoutingInput, RuntimeFamily,
    },
    proof::checked_from_progressed,
};

#[test]
fn routed_relational_artifacts_expose_authority_scoped_aspect_state() {
    let routing = admitted_handle("primary")
        .declare_review_progress_describe_plan_receipt_envelope_and_route_relational_truth(
            RoutingInput::<RuntimeFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("relational routing should succeed"));

    assert_eq!(
        routing.aspect_contract().required(),
        &[
            "selection.active_face".to_string(),
            "selection.neighborhood.local_topology".to_string()
        ]
    );
    assert_eq!(
        routing.aspect_coverage_basis(),
        crate::application::ForgeQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage
    );
    assert_eq!(
        routing.aspect_fit(),
        crate::application::ForgeQueryDeclarationAspectFit::CompatibleSuperset
    );
}

#[test]
fn relational_routing_denies_missing_and_conflicting_authority_slices() {
    match checked_from_progressed(
        &admitted_handle("primary"),
        RoutingInput::<MissingAspectFamily>::new("edge:42"),
    ) {
        ForgeQueryDeclarationRelationalRoutingChecked::Denied(denial) => assert_eq!(
            denial.cause(),
            crate::application::ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalAspectGap
        ),
        _ => panic!("missing relational aspect coverage should deny"),
    }

    match checked_from_progressed(
        &admitted_handle("primary"),
        RoutingInput::<ConflictingAspectFamily>::new("edge:42"),
    ) {
        ForgeQueryDeclarationRelationalRoutingChecked::Denied(denial) => assert_eq!(
            denial.cause(),
            crate::application::ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalAspectGap
        ),
        _ => panic!("conflicting relational aspect coverage should deny"),
    }
}

#[test]
fn relational_routing_digest_changes_with_authority_aspect_truth() {
    let base = admitted_handle("primary")
        .declare_review_progress_describe_plan_receipt_envelope_and_route_relational_truth(
            RoutingInput::<RuntimeFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("base relational routing should succeed"));
    let expanded = admitted_handle("primary")
        .declare_review_progress_describe_plan_receipt_envelope_and_route_relational_truth(
            RoutingInput::<ExpandedAspectFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("expanded relational routing should succeed"));

    assert_ne!(
        base.relational_routing_digest(),
        expanded.relational_routing_digest()
    );
}

#[test]
fn relational_support_rows_expose_aspect_gap_mismatch() {
    let support =
        admitted_handle("primary").relational_truth_support::<RoutingInput<MissingAspectFamily>>();
    let row = &support.rows()[0];

    assert_eq!(
        row.aspect_mismatch(),
        Some(crate::application::ForgeQueryDeclarationAuthorityAspectMismatch::AuthorityAspectGap)
    );
}
