use worth_spatial::facade::user_response::WorthUserResponseSource;

use super::contract_subject::{
    admitted_overlap_receipt, policy_required_overlap_receipt,
    unsupported_surface_support_response, user_response,
};

#[test]
fn user_response_evidence_digest_comes_from_source_receipts() {
    let admitted_receipt = admitted_overlap_receipt("user-response-evidence-admitted");
    let admitted = user_response(WorthUserResponseSource::from_overlap_receipt(
        &admitted_receipt,
    ));
    assert_eq!(admitted.evidence().digest(), admitted_receipt.fact_digest());
    assert_eq!(
        admitted.stage_identity().upstream_receipt(),
        admitted_receipt.fact_digest()
    );

    let policy_receipt = policy_required_overlap_receipt("user-response-evidence-policy");
    let policy = user_response(WorthUserResponseSource::from_overlap_receipt(
        &policy_receipt,
    ));
    assert_eq!(
        policy.evidence().digest(),
        policy_receipt.policy_required_exits()[0].consumed_fact_digest()
    );
    assert_eq!(
        policy.stage_identity().upstream_receipt(),
        policy_receipt.fact_digest()
    );
}

#[test]
fn unsupported_user_response_consumes_surface_support_posture_receipt() {
    let response = unsupported_surface_support_response("user-response-support-posture");
    assert!(response
        .evidence()
        .digest()
        .contains("stage=surface support"));
    assert_eq!(
        response.stage_identity().upstream_receipt(),
        response.evidence().source_identity()
    );
}
