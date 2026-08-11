use crate::facade::foundation::{
    promote_preflight_bundle_to_live, LiveChangeOrdinal, LivePolicyCounters,
    MilestoneFiveLiveAdapter, PatchWidthResolution, RefreshAdmissionClass,
};

use super::super::super::profiles::CertificationProfile;
use super::super::model::{LiveFailureClass, LiveRejectionBundle};
use super::changes::ordered_collection_patch_change;

pub(in crate::harness::live_certification) fn width_overflow_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

    let assessment = live.evaluate_delivery_width(33);
    match assessment.resolution() {
        PatchWidthResolution::Reject => LiveRejectionBundle {
            profile,
            failure_class: LiveFailureClass::ForbiddenWidthBudgetOverflowBehavior,
            failure_digest: format!(
                "width-budget-exceeded:{}:{}",
                assessment.budget_limit(),
                assessment.measured_width()
            ),
            counter_snapshot: LivePolicyCounters::from_width_assessment(&assessment),
        },
        other => panic!("expected width rejection, got {other:?}"),
    }
}

pub(in crate::harness::live_certification) fn forbidden_refresh_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

    let lane = MilestoneFiveLiveAdapter::forbidden_refresh_rejection_lane(
        &live,
        RefreshAdmissionClass::WidthOverflow,
    )
    .expect("detail family should reject refresh admission");

    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::ForbiddenRefreshEscapeHatch,
        failure_digest: lane.failure_digest().to_string(),
        counter_snapshot: lane.counters().clone(),
    }
}

pub(in crate::harness::live_certification) fn forbidden_coalescing_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

    let lane = MilestoneFiveLiveAdapter::forbidden_coalescing_rejection_lane(&live, 3)
        .expect("detail family should reject coalescing");

    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::ForbiddenCoalescingClass,
        failure_digest: lane.failure_digest().to_string(),
        counter_snapshot: lane.counters().clone(),
    }
}

pub(in crate::harness::live_certification) fn non_monotonic_sequence_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let advanced_live = live
        .advance_progress(
            LiveChangeOrdinal::from_value(1),
            crate::harness::fixtures::resolved_bases::runtime_basis(
                &crate::harness::fixtures::validated_bundles::runtime_detail_bundle(),
                &crate::harness::fixtures::resolved_bases::alternate_snapshot_identity(),
            ),
        )
        .expect("first ordinal advance should succeed");

    let lane = MilestoneFiveLiveAdapter::non_monotonic_progress_rejection_lane(
        &advanced_live,
        LiveChangeOrdinal::from_value(1),
        crate::harness::fixtures::resolved_bases::runtime_basis(
            &crate::harness::fixtures::validated_bundles::runtime_detail_bundle(),
            &crate::harness::fixtures::resolved_bases::relational_snapshot_identity(3, 1),
        ),
    )
    .expect("backward progress should be rejected");

    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::NonMonotonicChangeSequence,
        failure_digest: lane.failure_digest().to_string(),
        counter_snapshot: lane.counters().clone(),
    }
}

pub(in crate::harness::live_certification) fn change_sequence_gap_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

    let lane = MilestoneFiveLiveAdapter::non_monotonic_progress_rejection_lane(
        &live,
        LiveChangeOrdinal::from_value(2),
        preflight.basis().clone(),
    )
    .expect("ordinal gap should be rejected");

    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::GapfulChangeSequence,
        failure_digest: lane.failure_digest().to_string(),
        counter_snapshot: lane.counters().clone(),
    }
}

pub(in crate::harness::live_certification) fn invalid_live_promotion_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::store_detail_preflight();
    let error = promote_preflight_bundle_to_live(&preflight)
        .expect_err("store-backed preflight should be rejected for live promotion");

    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::InvalidLiveBasisPromotion,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_promotion_error(&error),
    }
}

pub(in crate::harness::live_certification) fn unsupported_patch_family_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let error = live
        .detail_live_outcome(&ordered_collection_patch_change())
        .expect_err("detail patch family should reject ordered collection live plan");

    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::UnsupportedPatchFamily,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_unsupported_patch_family(),
    }
}

pub(in crate::harness::live_certification) fn unsupported_live_family_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::cdc_collection_preflight();
    let error = promote_preflight_bundle_to_live(&preflight)
        .expect_err("cdc-shaped collection preflight should be rejected for live promotion");

    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::UnsupportedLiveFamily,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_promotion_error(&error),
    }
}

pub(in crate::harness::live_certification) fn raw_cdc_leakage_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::cdc_collection_preflight();
    let error = promote_preflight_bundle_to_live(&preflight)
        .expect_err("cdc-shaped collection preflight should not leak into live promotion");

    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::RawCdcLeakageForbidden,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_promotion_error(&error),
    }
}
