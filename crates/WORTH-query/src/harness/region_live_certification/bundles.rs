use crate::facade::{
    admit_region_scoped_live_plan, execute_live_change, execute_region_scoped_live_change,
    lower_region_scoped_execution_to_stream_contract, promote_preflight_bundle_to_live,
    BridgeChangeSummary, BridgeFieldDelta, LiveExecutionEnvelope, LivePolicyCounters,
    LocalityPredicateContract, RegionScopedLiveExecutionEnvelope, StreamConsumerShape,
};
use crate::harness::certification::{ParityAnchor, RejectionCertificationRow};
use crate::harness::live_certification::{
    LiveBundleFamily, LiveCertificationBundle, LiveCertificationRow, LiveFailureClass,
    LiveHostileExpectation, LiveOutcomeKind, LivePerturbationClass, LiveRejectionBundle,
    LiveRejectionRow,
};
use crate::harness::profiles::CertificationProfile;

pub(super) fn canonical_row(
    row_name: &'static str,
    perturbation_class: LivePerturbationClass,
    hostile_expectation: LiveHostileExpectation,
    control_lane: LiveCertificationBundle,
    hostile_lane: LiveCertificationBundle,
    parity_lane: LiveCertificationBundle,
) -> LiveCertificationRow {
    LiveCertificationRow {
        row_name,
        perturbation_class,
        hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}

pub(super) fn rejection_row(
    row_name: &'static str,
    perturbation_class: LivePerturbationClass,
    control_lane: LiveCertificationBundle,
    hostile_lane: LiveRejectionBundle,
    parity_lane: LiveCertificationBundle,
) -> RejectionCertificationRow<LivePerturbationClass, LiveCertificationBundle, LiveRejectionBundle>
{
    LiveRejectionRow {
        row_name,
        perturbation_class,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}

pub(super) fn detail_region_convergence_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail region admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &detail_in_region_change())
        .expect("in-region detail change should execute");
    bundle_from_region_execution(profile, &execution)
}

pub(super) fn off_region_suppression_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail region admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &detail_off_region_change())
        .expect("off-region detail change should suppress");
    bundle_from_region_execution(profile, &execution)
}

pub(super) fn detail_region_widening_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail region admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &detail_region_widening_change())
        .expect("detail region widening should execute");
    bundle_from_region_execution(profile, &execution)
}

pub(super) fn ordered_collection_partition_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
            .expect("collection partition admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &single_field_partition_change())
        .expect("single-field in-partition change should execute");
    bundle_from_region_execution(profile, &execution)
}

pub(super) fn bounded_materialization_region_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("bounded preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("bounded materialization should admit region scope");
    let execution = execute_region_scoped_live_change(&plan, &bounded_in_region_change())
        .expect("bounded in-region change should execute");
    bundle_from_region_execution(profile, &execution)
}

pub(super) fn broad_control_bundle(profile: CertificationProfile) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let execution = execute_live_change(&live, &detail_in_region_change())
        .expect("broad live control should execute");
    bundle_from_live_execution(profile, &execution)
}

pub(super) fn stream_contract_bundle(profile: CertificationProfile) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail region admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &detail_in_region_change())
        .expect("in-region detail change should execute");
    let contract = lower_region_scoped_execution_to_stream_contract(
        &plan,
        &execution,
        StreamConsumerShape::DetailCurrentState,
    )
    .expect("detail stream lowering should succeed");
    bundle_from_stream_contract(profile, &execution, &contract)
}

pub(super) fn cdc_stream_contract_bundle(profile: CertificationProfile) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
            .expect("collection partition admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &single_field_partition_change())
        .expect("single-field in-partition change should execute");
    let contract = lower_region_scoped_execution_to_stream_contract(
        &plan,
        &execution,
        StreamConsumerShape::CdcCollectionPatch,
    )
    .expect("single-field collection patch should lower into the CDC stream contract");
    bundle_from_stream_contract(profile, &execution, &contract)
}

pub(super) fn locality_breadth_budget_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    detail_region_convergence_bundle(profile)
}

pub(super) fn stream_member_width_budget_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    cdc_stream_contract_bundle(profile)
}

pub(super) fn locality_work_avoided_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    off_region_suppression_bundle(profile)
}

pub(super) fn unsupported_locality_family_rejection_bundle(
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

pub(super) fn unsupported_locality_predicate_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
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

pub(super) fn unsupported_stream_consumer_rejection_bundle(
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

pub(super) fn raw_partition_leakage_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
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

pub(super) fn raw_stream_member_leakage_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    unsupported_stream_consumer_rejection_bundle(profile)
        .with_failure_class(LiveFailureClass::RawStreamMemberLeakageForbidden)
}

pub(super) fn raw_stream_member_forbidden_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
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

pub(super) fn forbidden_locality_widening_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
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

pub(super) fn bridge_slice_incompatibility_rejection_bundle(
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

pub(super) fn forbidden_broad_success_lane_rejection_bundle(
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

pub(super) fn forbidden_stream_width_overflow_success_rejection_bundle(
    profile: CertificationProfile,
) -> LiveRejectionBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
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

pub(super) fn forbidden_stream_window_overflow_success_rejection_bundle(
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

fn bundle_from_live_execution(
    profile: CertificationProfile,
    execution: &LiveExecutionEnvelope,
) -> LiveCertificationBundle {
    LiveCertificationBundle {
        profile,
        query_digest: execution.replay_bundle().query_digest().to_string(),
        result_digest: execution.replay_bundle().result_digest().to_string(),
        delivery_digest: execution.replay_bundle().delivery_digest().to_string(),
        replay_digest: execution.replay_bundle().replay_digest().to_string(),
        replay_step_delivery_digests: Vec::new(),
        family: bundle_family(execution.patch_envelope().family()),
        outcome_kind: outcome_kind_from_payload(execution.patch_envelope().payload()),
        outcome_digest: execution.report().outcome_digest().to_string(),
        basis_digest: execution.replay_bundle().basis_digest().to_string(),
        subscription_digest: execution.replay_bundle().subscription_digest().to_string(),
        counter_snapshot: execution.replay_bundle().counter_snapshot().clone(),
    }
}

fn bundle_from_region_execution(
    profile: CertificationProfile,
    execution: &RegionScopedLiveExecutionEnvelope,
) -> LiveCertificationBundle {
    let replay_record = execution.region_scoped_replay_bundle().replay_record();
    let (outcome_kind, outcome_digest) = match execution.patch_envelope().payload() {
        crate::facade::LivePatchPayload::Detail(_)
        | crate::facade::LivePatchPayload::OrderedCollection(_)
        | crate::facade::LivePatchPayload::BoundedMaterialization(_) => (
            LiveOutcomeKind::Patch,
            execution.patch_envelope().delivery_digest().to_string(),
        ),
        crate::facade::LivePatchPayload::Suppressed(_) => (
            LiveOutcomeKind::Suppressed,
            execution.patch_envelope().delivery_digest().to_string(),
        ),
        crate::facade::LivePatchPayload::Refresh(_) => (
            LiveOutcomeKind::Refresh,
            execution.patch_envelope().delivery_digest().to_string(),
        ),
        crate::facade::LivePatchPayload::ProgressAdvance { .. } => (
            LiveOutcomeKind::ProgressAdvance,
            execution.patch_envelope().delivery_digest().to_string(),
        ),
        crate::facade::LivePatchPayload::Coalesced(_) => (
            LiveOutcomeKind::CoalescedDelivery,
            execution.patch_envelope().delivery_digest().to_string(),
        ),
    };
    LiveCertificationBundle {
        profile,
        query_digest: replay_record.query_digest().to_string(),
        result_digest: execution.replay_bundle().result_digest().to_string(),
        delivery_digest: replay_record.delivery_digest().to_string(),
        replay_digest: replay_record.replay_digest().to_string(),
        replay_step_delivery_digests: Vec::new(),
        family: bundle_family(execution.patch_envelope().family()),
        outcome_kind,
        outcome_digest,
        basis_digest: execution.replay_bundle().basis_digest().to_string(),
        subscription_digest: execution.replay_bundle().subscription_digest().to_string(),
        counter_snapshot: execution
            .region_scoped_replay_bundle()
            .counter_snapshot()
            .clone(),
    }
}

fn bundle_from_stream_contract(
    profile: CertificationProfile,
    execution: &RegionScopedLiveExecutionEnvelope,
    contract: &crate::facade::StreamLoweredDeliveryContract,
) -> LiveCertificationBundle {
    LiveCertificationBundle {
        profile,
        query_digest: contract
            .query_delivery_contract()
            .query_digest()
            .to_string(),
        result_digest: execution.replay_bundle().result_digest().to_string(),
        delivery_digest: contract
            .query_delivery_contract()
            .delivery_digest()
            .to_string(),
        replay_digest: contract.replay_record().replay_digest().to_string(),
        replay_step_delivery_digests: Vec::new(),
        family: bundle_family(contract.query_delivery_contract().family()),
        outcome_kind: LiveOutcomeKind::StreamLoweredDelivery,
        outcome_digest: contract.stream_contract_digest().to_string(),
        basis_digest: execution.replay_bundle().basis_digest().to_string(),
        subscription_digest: execution.replay_bundle().subscription_digest().to_string(),
        counter_snapshot: contract.counter_snapshot().clone(),
    }
}

fn bundle_family(family: &crate::facade::LiveQueryFamily) -> LiveBundleFamily {
    match family {
        crate::facade::LiveQueryFamily::Detail => LiveBundleFamily::Detail,
        crate::facade::LiveQueryFamily::OrderedCollection => LiveBundleFamily::OrderedCollection,
        crate::facade::LiveQueryFamily::BoundedMaterialization => {
            LiveBundleFamily::BoundedMaterialization
        }
    }
}

fn outcome_kind_from_payload(payload: &crate::facade::LivePatchPayload) -> LiveOutcomeKind {
    match payload {
        crate::facade::LivePatchPayload::Detail(_)
        | crate::facade::LivePatchPayload::OrderedCollection(_)
        | crate::facade::LivePatchPayload::BoundedMaterialization(_) => LiveOutcomeKind::Patch,
        crate::facade::LivePatchPayload::Suppressed(_) => LiveOutcomeKind::Suppressed,
        crate::facade::LivePatchPayload::Refresh(_) => LiveOutcomeKind::Refresh,
        crate::facade::LivePatchPayload::ProgressAdvance { .. } => LiveOutcomeKind::ProgressAdvance,
        crate::facade::LivePatchPayload::Coalesced(_) => LiveOutcomeKind::CoalescedDelivery,
    }
}

fn detail_in_region_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-a")
}

fn detail_off_region_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-b")
}

fn detail_region_widening_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-a")
        .with_region_slice("assembly-b")
}

fn detail_without_locality_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "identity",
        "id",
        Some("user-1"),
        Some("user-2"),
    ))
}

fn partition_coarse_fallback_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Esther"),
            Some("Ess"),
        ))
        .with_coarse_fallback_slice("tenant-a")
}

fn duplicate_region_slice_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-a")
        .with_region_slice("assembly-a")
}

fn single_field_partition_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Esther"),
            Some("Ess"),
        ))
        .with_partition_slice("tenant-a")
}

fn two_field_partition_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Esther"),
            Some("Ess"),
        ))
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_partition_slice("tenant-a")
}

fn bounded_in_region_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Esther"),
            Some("Ess"),
        ))
        .with_relation_delta(crate::facade::BridgeRelationDelta::new("manager"))
        .with_materialization_scope_transition(false, true)
        .with_region_slice("assembly-a")
}
