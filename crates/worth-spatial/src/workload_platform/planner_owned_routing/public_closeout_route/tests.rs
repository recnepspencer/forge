use crate::workload_platform::evidence_lookup_public_closeout::{
    current_evidence_lookup_public_closeout, EvidenceLookupPublicCloseoutErrorKind,
};

use super::current::{
    current_evidence_lookup_public_closeout_route_input,
    current_evidence_lookup_public_closeout_with_selected_route_support,
};
use super::input::SelectedEvidenceLookupPublicCloseoutRouteSupport;

#[test]
fn spatial_public_closeout_route_explanation_consumes_planner_route_products_without_evidence_rescan(
) {
    let route_input =
        current_evidence_lookup_public_closeout_route_input().expect("public closeout route input");
    let packet = route_input.route_packet();
    let closeout = current_evidence_lookup_public_closeout().expect("public closeout");
    let seed = closeout.milestone_twelve_seed();

    assert_eq!(
        seed.selected_route_family_identity(),
        packet.route_family_identity()
    );
    assert_eq!(
        seed.selected_compiled_product_identity_digest(),
        packet.compiled_product_identity_digest()
    );
    assert_eq!(
        seed.selected_equivalence_family_identity(),
        packet.selected_equivalence_family_identity()
    );
    assert_eq!(
        seed.selected_reuse_basis_identity_digest(),
        packet.selected_reuse_basis_identity_digest()
    );
    assert_eq!(packet.lowering_raw_row_revisit_count(), 0);
    assert_eq!(packet.lowering_right_receipt_revisit_count(), 0);
    assert_eq!(packet.lowering_caller_owned_revisit_count(), 0);
}

#[test]
fn spatial_closeout_denial_localizes_family_or_support_mismatch() {
    let route_input =
        current_evidence_lookup_public_closeout_route_input().expect("public closeout route input");
    let support = route_input.selected_route_support().clone();

    let family_error = current_evidence_lookup_public_closeout_with_selected_route_support(
        SelectedEvidenceLookupPublicCloseoutRouteSupport::new(
            support.route_family_identity().to_string(),
            support.stage_receipt_family_identity().to_string(),
            support.selected_lookup_plan_digest().to_string(),
            support.lookup_execution_receipt_digest().to_string(),
            support.lookup_product_output_digest().to_string(),
            support.compiled_product_identity_digest().to_string(),
            support.equivalence_policy_identity_digest().to_string(),
            "foreign-selected-family".to_string(),
            support.selected_reuse_basis_identity_digest().to_string(),
        ),
    )
    .expect_err("family mismatch must deny");
    assert_eq!(
        family_error.kind(),
        EvidenceLookupPublicCloseoutErrorKind::MismatchedSelectedRouteFamily
    );

    let support_error = current_evidence_lookup_public_closeout_with_selected_route_support(
        SelectedEvidenceLookupPublicCloseoutRouteSupport::new(
            support.route_family_identity().to_string(),
            support.stage_receipt_family_identity().to_string(),
            support.selected_lookup_plan_digest().to_string(),
            support.lookup_execution_receipt_digest().to_string(),
            support.lookup_product_output_digest().to_string(),
            support.compiled_product_identity_digest().to_string(),
            support.equivalence_policy_identity_digest().to_string(),
            support.selected_equivalence_family_identity().to_string(),
            "foreign-selected-reuse-basis".to_string(),
        ),
    )
    .expect_err("support mismatch must deny");
    assert_eq!(
        support_error.kind(),
        EvidenceLookupPublicCloseoutErrorKind::MismatchedSelectedRouteSupport
    );
}
