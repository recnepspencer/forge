use crate::harness::milestone_nine_certification::phase_three_test_narrowed_artifact;

use super::{
    deny_policy_placeholder_masking, lower_policy_aware_delivery_shape, DeliveryWidthClass,
    PolicyPlaceholderMaskingRequest,
};

#[test]
fn delivery_shape_uses_narrowed_result_shape() {
    let artifact = phase_three_test_narrowed_artifact();
    let delivery = lower_policy_aware_delivery_shape(&artifact, DeliveryWidthClass::ScalarDetail)
        .expect("narrow scalar delivery should admit");

    assert_eq!(
        delivery.narrowed_result_shape_digest(),
        artifact.narrowed_result_shape_digest()
    );
    assert!(!delivery
        .delivered_fields()
        .contains(&"secret.salary".to_string()));
    assert_eq!(delivery.report().delivery_width(), 2);
}

#[test]
fn denied_width_inflation_fails_before_delivery_shape() {
    let artifact = phase_three_test_narrowed_artifact();
    let error =
        lower_policy_aware_delivery_shape(&artifact, DeliveryWidthClass::DeniedWidthInflation)
            .expect_err("denied width class should fail");

    assert_eq!(error.counters().delivery_overexposure_denial_count(), 1);
}

#[test]
fn placeholder_masking_is_a_distinct_typed_denial() {
    let artifact = phase_three_test_narrowed_artifact();
    let placeholder = deny_policy_placeholder_masking(
        &artifact,
        PolicyPlaceholderMaskingRequest::new(vec!["secret.salary".to_string()]),
    )
    .expect_err("masked placeholder delivery should fail distinctly");
    let width =
        lower_policy_aware_delivery_shape(&artifact, DeliveryWidthClass::DeniedWidthInflation)
            .expect_err("width inflation should still be separate");

    assert_eq!(placeholder.counters().placeholder_masking_denial_count(), 1);
    assert_eq!(
        placeholder.counters().delivery_overexposure_denial_count(),
        0
    );
    assert_eq!(width.counters().delivery_overexposure_denial_count(), 1);
    assert_ne!(placeholder.failure_class(), width.failure_class());
}
