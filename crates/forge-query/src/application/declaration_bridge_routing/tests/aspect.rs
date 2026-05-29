use crate::application::ForgeQueryDeclarationBridgeRoutingChecked;

use super::support::{
    domain::{
        admitted_handle, ConflictingAspectFamily, ExpandedAspectFamily, MissingAspectFamily,
        PreviewSessionFamily, RoutingInput,
    },
    proof::checked_from_progressed,
};

#[test]
fn bridge_routed_artifacts_expose_authority_and_mapping_aspect_state() {
    let routing = admitted_handle("primary")
        .declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(
            RoutingInput::<PreviewSessionFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("bridge routing should succeed"));

    assert_eq!(
        routing.aspect_coverage_basis(),
        crate::application::ForgeQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage
    );
    assert_eq!(
        routing.aspect_fit(),
        crate::application::ForgeQueryDeclarationAspectFit::CompatibleSuperset
    );
    assert_eq!(
        routing.mapping_fit(),
        crate::application::ForgeQueryDeclarationAspectFit::Exact
    );
    assert!(routing.mapped_aspects().present().len() < routing.aspect_coverage().present().len());
}

#[test]
fn bridge_routing_denies_missing_and_conflicting_authority_slices() {
    match checked_from_progressed(
        &admitted_handle("primary"),
        RoutingInput::<MissingAspectFamily>::new("edge:42"),
    ) {
        ForgeQueryDeclarationBridgeRoutingChecked::Denied(denial) => assert_eq!(
            denial.cause(),
            crate::application::ForgeQueryDeclarationBridgeRoutingDenialCause::AuthorityAspectGap
        ),
        _ => panic!("missing bridge aspect coverage should deny"),
    }

    match checked_from_progressed(
        &admitted_handle("primary"),
        RoutingInput::<ConflictingAspectFamily>::new("edge:42"),
    ) {
        ForgeQueryDeclarationBridgeRoutingChecked::Denied(denial) => assert_eq!(
            denial.cause(),
            crate::application::ForgeQueryDeclarationBridgeRoutingDenialCause::AuthorityAspectGap
        ),
        _ => panic!("conflicting bridge aspect coverage should deny"),
    }
}

#[test]
fn bridge_routing_digest_changes_with_mapped_aspect_truth() {
    let base = admitted_handle("primary")
        .declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(
            RoutingInput::<PreviewSessionFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("bridge routing should succeed"));
    let expanded = admitted_handle("primary")
        .declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(
            RoutingInput::<ExpandedAspectFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("expanded bridge routing should succeed"));

    assert_ne!(
        base.bridge_routing_digest(),
        expanded.bridge_routing_digest()
    );
}

#[test]
fn bridge_support_rows_expose_narrower_mapped_slice_than_available_coverage() {
    let support = admitted_handle("primary")
        .bridge_continuation_support::<RoutingInput<ExpandedAspectFamily>>();
    let row = &support.rows()[0];

    assert!(
        row.mapped_aspect_slice().present().len() < row.available_aspect_slice().present().len()
    );
    assert_eq!(
        row.mapping_fit(),
        crate::application::ForgeQueryDeclarationAspectFit::Exact
    );
}
