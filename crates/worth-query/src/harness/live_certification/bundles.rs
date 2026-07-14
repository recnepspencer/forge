use crate::facade::foundation::{
    promote_preflight_bundle_to_live, replay_live_sequence, BridgeChangeSummary, BridgeFieldDelta,
    BridgeRelationDelta, LiveChangeOrdinal, LivePatchPayload, LivePolicyCounters, LiveReplayBundle,
    LiveReplayRun, LiveReplayStepInput, MilestoneFiveLiveAdapter, PatchWidthResolution,
    RefreshAdmissionClass,
};

use super::super::certification::{ParityAnchor, RejectionCertificationRow};
use super::super::profiles::CertificationProfile;
use super::model::{
    LiveBundleFamily, LiveCertificationBundle, LiveCertificationRow, LiveFailureClass,
    LiveHostileExpectation, LiveOutcomeKind, LivePerturbationClass, LiveRejectionBundle,
    LiveRejectionRow,
};

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

pub(super) fn detail_patch_bundle(profile: CertificationProfile) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let change = detail_patch_change();
    let lane = MilestoneFiveLiveAdapter::detail_patch_lane(&live, &change)
        .expect("detail patch lane should build");

    bundle_from_lane(profile, &lane)
}

pub(super) fn detail_suppression_bundle(profile: CertificationProfile) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "profile",
        "display_name",
        Some("Esther"),
        Some("Ess"),
    ));
    let lane = MilestoneFiveLiveAdapter::suppression_lane(&live, &change)
        .expect("suppression lane should build");

    bundle_from_lane(profile, &lane)
}

pub(super) fn ordered_collection_patch_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let change = ordered_collection_patch_change();
    let lane = MilestoneFiveLiveAdapter::ordered_collection_patch_lane(&live, &change)
        .expect("ordered collection lane should build");

    bundle_from_lane(profile, &lane)
}

pub(super) fn bounded_materialization_patch_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("bounded preflight should promote");
    let change = bounded_materialization_patch_change();
    let lane = MilestoneFiveLiveAdapter::bounded_materialization_patch_lane(&live, &change)
        .expect("bounded materialization lane should build");

    bundle_from_lane(profile, &lane)
}

pub(super) fn detail_replay_end_state_control_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::alternate_basis_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let lane = MilestoneFiveLiveAdapter::detail_patch_lane(&live, &detail_patch_change())
        .expect("detail patch lane should build");

    bundle_from_lane(profile, &lane)
}

pub(super) fn detail_replay_bundle(profile: CertificationProfile) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let run = replay_live_sequence(
        &live,
        &[LiveReplayStepInput::new(
            detail_patch_change(),
            LiveChangeOrdinal::from_value(1),
            crate::harness::fixtures::resolved_bases::runtime_basis(
                &crate::harness::fixtures::validated_bundles::runtime_detail_bundle(),
                &crate::harness::fixtures::resolved_bases::alternate_snapshot_identity(),
            ),
        )],
    )
    .expect("detail replay sequence should succeed");

    bundle_from_replay_run(profile, &run)
}

pub(super) fn ordered_collection_replay_end_state_control_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::alternate_basis_ordered_collection_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let lane = MilestoneFiveLiveAdapter::ordered_collection_patch_lane(
        &live,
        &ordered_collection_patch_change(),
    )
    .expect("ordered collection patch lane should build");

    bundle_from_lane(profile, &lane)
}

pub(super) fn ordered_collection_replay_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let run = replay_live_sequence(
        &live,
        &[LiveReplayStepInput::new(
            ordered_collection_patch_change(),
            LiveChangeOrdinal::from_value(1),
            crate::harness::fixtures::resolved_bases::runtime_basis(
                &crate::harness::fixtures::validated_bundles::ordered_collection_without_traversal_bundle(),
                &crate::harness::fixtures::resolved_bases::alternate_snapshot_identity(),
            ),
        )],
    )
    .expect("ordered collection replay sequence should succeed");

    bundle_from_replay_run(profile, &run)
}

pub(super) fn bounded_materialization_replay_end_state_control_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::alternate_basis_bounded_materialization_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("bounded preflight should promote");
    let lane = MilestoneFiveLiveAdapter::bounded_materialization_patch_lane(
        &live,
        &bounded_materialization_patch_change(),
    )
    .expect("bounded materialization patch lane should build");

    bundle_from_lane(profile, &lane)
}

pub(super) fn bounded_materialization_replay_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("bounded preflight should promote");
    let run = replay_live_sequence(
        &live,
        &[LiveReplayStepInput::new(
            bounded_materialization_patch_change(),
            LiveChangeOrdinal::from_value(1),
            crate::harness::fixtures::resolved_bases::runtime_basis(
                &crate::harness::fixtures::validated_bundles::ordered_collection_bundle(),
                &crate::harness::fixtures::resolved_bases::alternate_snapshot_identity(),
            ),
        )],
    )
    .expect("bounded materialization replay sequence should succeed");

    bundle_from_replay_run(profile, &run)
}

pub(super) fn bounded_materialization_refresh_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("bounded preflight should promote");

    let lane = MilestoneFiveLiveAdapter::refresh_fallback_lane(
        &live,
        RefreshAdmissionClass::WidthOverflow,
    )
    .expect("refresh fallback lane should build");

    bundle_from_lane(profile, &lane)
}

pub(super) fn coalesced_delivery_bundle(profile: CertificationProfile) -> LiveCertificationBundle {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");

    let lane = MilestoneFiveLiveAdapter::coalesced_delivery_lane(&live, 3)
        .expect("coalesced delivery lane should build");

    bundle_from_lane(profile, &lane)
}

pub(super) fn progress_advance_bundle(profile: CertificationProfile) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

    let lane = MilestoneFiveLiveAdapter::progress_advance_lane(
        &live,
        LiveChangeOrdinal::from_value(1),
        preflight.basis().clone(),
    )
    .expect("progress advance lane should build");

    bundle_from_lane(profile, &lane)
}

pub(super) fn width_overflow_rejection_bundle(
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

pub(super) fn forbidden_refresh_rejection_bundle(
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

pub(super) fn forbidden_coalescing_rejection_bundle(
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

pub(super) fn non_monotonic_sequence_rejection_bundle(
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

pub(super) fn change_sequence_gap_rejection_bundle(
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

pub(super) fn invalid_live_promotion_rejection_bundle(
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

pub(super) fn unsupported_patch_family_rejection_bundle(
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

pub(super) fn unsupported_live_family_rejection_bundle(
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

pub(super) fn raw_cdc_leakage_rejection_bundle(
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

fn bundle_from_lane(
    profile: CertificationProfile,
    lane: &crate::facade::certification::LiveCertificationLane,
) -> LiveCertificationBundle {
    LiveCertificationBundle {
        profile,
        query_digest: lane.execution().replay_bundle().query_digest().to_string(),
        result_digest: lane.execution().replay_bundle().result_digest().to_string(),
        delivery_digest: lane
            .execution()
            .replay_bundle()
            .delivery_digest()
            .to_string(),
        replay_digest: lane.execution().replay_bundle().replay_digest().to_string(),
        replay_step_delivery_digests: Vec::new(),
        family: bundle_family(lane.execution().patch_envelope().family()),
        outcome_kind: outcome_kind_from_payload(lane.execution().patch_envelope().payload()),
        outcome_digest: lane.execution().report().outcome_digest().to_string(),
        basis_digest: lane.execution().replay_bundle().basis_digest().to_string(),
        subscription_digest: lane
            .execution()
            .replay_bundle()
            .subscription_digest()
            .to_string(),
        counter_snapshot: lane.execution().replay_bundle().counter_snapshot().clone(),
    }
}

fn bundle_from_replay_run(
    profile: CertificationProfile,
    run: &LiveReplayRun,
) -> LiveCertificationBundle {
    let final_bundle = run
        .bundles()
        .last()
        .expect("replay run should emit at least one bundle");
    let mut counter_snapshot = LivePolicyCounters::default();
    for bundle in run.bundles() {
        counter_snapshot.absorb(bundle.counter_snapshot());
    }

    LiveCertificationBundle {
        profile,
        query_digest: final_bundle.query_digest().to_string(),
        result_digest: final_bundle.result_digest().to_string(),
        delivery_digest: final_bundle.delivery_digest().to_string(),
        replay_digest: final_bundle.replay_digest().to_string(),
        replay_step_delivery_digests: run
            .bundles()
            .iter()
            .map(|bundle| bundle.delivery_digest().to_string())
            .collect(),
        family: bundle_family(final_bundle.patch_envelope().family()),
        outcome_kind: replay_payload_kind(final_bundle),
        outcome_digest: final_bundle.delivery_digest().to_string(),
        basis_digest: final_bundle.basis_digest().to_string(),
        subscription_digest: final_bundle.subscription_digest().to_string(),
        counter_snapshot,
    }
}

fn replay_payload_kind(bundle: &LiveReplayBundle) -> LiveOutcomeKind {
    outcome_kind_from_payload(bundle.patch_envelope().payload())
}

fn bundle_family(family: &crate::facade::foundation::LiveQueryFamily) -> LiveBundleFamily {
    match family {
        crate::facade::foundation::LiveQueryFamily::Detail => LiveBundleFamily::Detail,
        crate::facade::foundation::LiveQueryFamily::OrderedCollection => {
            LiveBundleFamily::OrderedCollection
        }
        crate::facade::foundation::LiveQueryFamily::BoundedMaterialization => {
            LiveBundleFamily::BoundedMaterialization
        }
    }
}

fn outcome_kind_from_payload(payload: &LivePatchPayload) -> LiveOutcomeKind {
    match payload {
        LivePatchPayload::Detail(_)
        | LivePatchPayload::OrderedCollection(_)
        | LivePatchPayload::BoundedMaterialization(_) => LiveOutcomeKind::Patch,
        LivePatchPayload::Suppressed(_) => LiveOutcomeKind::Suppressed,
        LivePatchPayload::Refresh(_) => LiveOutcomeKind::Refresh,
        LivePatchPayload::Coalesced(_) => LiveOutcomeKind::CoalescedDelivery,
        LivePatchPayload::ProgressAdvance { .. } => LiveOutcomeKind::ProgressAdvance,
    }
}

fn detail_patch_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "identity",
        "id",
        Some("user-1"),
        Some("user-2"),
    ))
}

fn ordered_collection_patch_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "profile",
        "display_name",
        Some("Avery"),
        Some("Zoey"),
    ))
}

fn bounded_materialization_patch_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_relation_delta(BridgeRelationDelta::new("manager"))
        .with_materialization_scope_transition(false, true)
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Old Manager"),
            Some("New Manager"),
        ))
}
