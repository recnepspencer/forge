use crate::authorized_projection::AuthorizedProjectionFieldPath;
use crate::harness::milestone_nine_certification::phase_three_test_narrowed_artifact;
use forge_foundational::facade::{AspectKey, FieldKey};

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
    let hidden_salary = authorized_field("secret", "salary");
    assert!(!delivery
        .delivered_field_paths()
        .iter()
        .any(|field| field == &hidden_salary));
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
        PolicyPlaceholderMaskingRequest::from_authorized_field_paths(vec![authorized_field(
            "secret", "salary",
        )]),
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

fn authorized_field(aspect: &str, field: &str) -> AuthorizedProjectionFieldPath {
    AuthorizedProjectionFieldPath::from_native_keys(
        AspectKey::new(aspect).expect("aspect key should admit"),
        FieldKey::new(field).expect("field key should admit"),
    )
}
