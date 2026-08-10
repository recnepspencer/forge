use crate::facade::foundation::{
    promote_preflight_bundle_to_live, LivePolicyCounters, LocalityPredicateContract,
    StreamConsumerShape,
};
use crate::harness::live_certification::{LiveFailureClass, LiveRejectionBundle};
use crate::harness::profiles::CertificationProfile;
use crate::live::{
    admit_region_scoped_live_plan, execute_region_scoped_live_change,
    lower_region_scoped_execution_to_stream_contract,
};

use super::change_scenarios::{
    detail_in_region_change, detail_region_widening_change, detail_without_locality_change,
    duplicate_region_slice_change, partition_coarse_fallback_change, single_field_partition_change,
    two_field_partition_change,
};

pub(in crate::harness::region_live_certification) fn unsupported_locality_family_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::cdc_collection_preflight();
    let error = promote_preflight_bundle_to_live(&preflight)
        .expect_err("cdc collection should remain unsupported before locality admission");
    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::UnsupportedLocalityFamily,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_promotion_error(&error),
    }
}

pub(in crate::harness::region_live_certification) fn unsupported_locality_predicate_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let error =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect_err("ordered collection region scope should be rejected");
    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::UnsupportedLocalityPredicate,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_region_scoped_error(&error),
    }
}

pub(in crate::harness::region_live_certification) fn unsupported_stream_consumer_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail region admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &detail_in_region_change())
        .expect("detail region execution should succeed");
    let error = lower_region_scoped_execution_to_stream_contract(
        &plan,
        &execution,
        StreamConsumerShape::CdcCollectionPatch,
    )
    .expect_err("detail execution should reject cdc collection consumer shape");
    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::UnsupportedStreamConsumerContract,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_region_scoped_error(&error),
    }
}

pub(in crate::harness::region_live_certification) fn raw_partition_leakage_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
            .expect("collection partition admission should succeed");
    let error = execute_region_scoped_live_change(&plan, &partition_coarse_fallback_change())
        .expect_err("coarse fallback should not leak as accepted partition event");
    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::RawPartitionEventLeakageForbidden,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_region_scoped_error(&error),
    }
}

pub(in crate::harness::region_live_certification) fn raw_stream_member_leakage_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    unsupported_stream_consumer_rejection_bundle(profile)
        .with_failure_class(LiveFailureClass::RawStreamMemberLeakageForbidden)
}

pub(in crate::harness::region_live_certification) fn raw_stream_member_forbidden_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
            .expect("collection partition admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &single_field_partition_change())
        .expect("single-field in-partition change should execute");
    let error = lower_region_scoped_execution_to_stream_contract(
        &plan,
        &execution,
        StreamConsumerShape::DetailCurrentState,
    )
    .expect_err("collection execution should reject detail-current-state stream lowering");
    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::RawStreamMemberForbidden,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_region_scoped_error(&error),
    }
}

pub(in crate::harness::region_live_certification) fn forbidden_locality_widening_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
            .expect("collection partition admission should succeed");
    let error = execute_region_scoped_live_change(&plan, &partition_coarse_fallback_change())
        .expect_err("coarse fallback should be a typed widening denial");
    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::ForbiddenLocalityWidening,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_region_scoped_error(&error),
    }
}

pub(in crate::harness::region_live_certification) fn bridge_slice_incompatibility_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail region admission should succeed");
    let error = execute_region_scoped_live_change(&plan, &detail_without_locality_change())
        .expect_err("missing locality slices should be bridge incompatibility");
    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::BridgeSliceIncompatibilityDenied,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_region_scoped_error(&error),
    }
}

pub(in crate::harness::region_live_certification) fn forbidden_broad_success_lane_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail region admission should succeed");
    let error = execute_region_scoped_live_change(&plan, &duplicate_region_slice_change())
        .expect_err("duplicate exact slices should exceed the locality breadth budget");
    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::ForbiddenBroadSuccessLane,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_region_scoped_error(&error),
    }
}

pub(in crate::harness::region_live_certification) fn forbidden_stream_width_overflow_success_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
            .expect("collection partition admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &two_field_partition_change())
        .expect("two-field in-partition change should execute before stream lowering");
    let error = lower_region_scoped_execution_to_stream_contract(
        &plan,
        &execution,
        StreamConsumerShape::CdcCollectionPatch,
    )
    .expect_err("two-field collection patch should overflow the stream member width budget");
    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::ForbiddenStreamWindowOverflowSuccess,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_region_scoped_error(&error),
    }
}

pub(in crate::harness::region_live_certification) fn forbidden_stream_window_overflow_success_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail region admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &detail_region_widening_change())
        .expect("detail region widening should execute before stream lowering");
    let error = lower_region_scoped_execution_to_stream_contract(
        &plan,
        &execution,
        StreamConsumerShape::DetailCurrentState,
    )
    .expect_err("widened detail stream lowering should overflow the stream window budget");
    LiveRejectionBundle {
        profile,
        failure_class: LiveFailureClass::ForbiddenStreamWindowOverflowSuccess,
        failure_digest: format!("{error:?}"),
        counter_snapshot: LivePolicyCounters::from_region_scoped_error(&error),
    }
}

trait FailureClassExt {
    fn with_failure_class(self, failure_class: LiveFailureClass) -> Self;
}

impl FailureClassExt for LiveRejectionBundle {
    fn with_failure_class(mut self, failure_class: LiveFailureClass) -> Self {
        self.failure_class = failure_class;
        self
    }
}
