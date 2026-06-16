use super::*;
use crate::live::LiveQueryFamily;
use crate::subscription::evidence_identities::lifecycle_absent_preview_isolation_identity;
use crate::view_shape_live::LiveViewShapeFamily;

fn roomy_admission_budget() -> QuerySubscriptionAdmissionBudget {
    QuerySubscriptionAdmissionBudget::admitted(1, 1, 1, 1, 1)
}

fn lowering_for(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
) -> BridgeSubscriptionLoweringPlan {
    let input = LiveQueryAdmissionArtifact::for_test(
        live_family,
        view_family,
        QuerySubscriptionConstructionSource::FacadeLive,
    );
    let selection = select_query_subscription_family(input, roomy_budget()).unwrap();
    let declaration = declare_query_subscription(selection, roomy_slice_budget()).unwrap();
    lower_query_subscription_to_bridge(declaration, roomy_lowering_budget()).unwrap()
}

fn admitted_activation_for(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
) -> (
    QuerySubscriptionAdmissionArtifact,
    SubscriptionActivationInput,
    QuerySubscriptionScaleSlopeReport,
) {
    let lowering = lowering_for(live_family, view_family);
    let admission = admit_query_subscription(lowering, roomy_admission_budget()).unwrap();
    let activation = prepare_subscription_activation(admission.clone());
    let scale_report = certify_query_subscription_scale_slope(
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Small,
            10,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Medium,
            100,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Large,
            1000,
            &activation,
        ),
    )
    .unwrap();

    (admission, activation, scale_report)
}

fn activation_for(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
) -> SubscriptionActivationInput {
    admitted_activation_for(live_family, view_family).1
}

struct LifecycleCertificationArtifacts {
    context: SubscriptionLifecycleCertificationContext,
    admission: QuerySubscriptionAdmissionArtifact,
    activation: SubscriptionActivationInput,
    scale_report: QuerySubscriptionScaleSlopeReport,
    active_admission: ActiveSubscriptionLaneAdmission,
    handle: ActiveSubscriptionLaneHandle,
    attachment: SubscriptionConsumerAttachment,
    delta: QuerySubscriptionMaintenanceDelta,
    lowering_report: QueryMaintenanceDeltaLoweringReport,
    work_packet: ActiveDeliveryWorkPacket,
    delivery_batch: QueryDeliveryBatch,
    acknowledged_attachment: SubscriptionConsumerAttachment,
    continuation_report: Option<SubscriptionContinuationReport>,
    preview: SubscriptionLifecyclePreviewCertificationArtifacts,
    closeout: SubscriptionLifecycleCloseout,
}

enum SubscriptionLifecyclePreviewCertificationArtifacts {
    None,
    Discard {
        isolation: PreviewSubscriptionIsolationArtifact,
        residue_report: PreviewSubscriptionResidueReport,
        discard_closeout: PreviewSubscriptionDiscardCloseout,
    },
    Promotion {
        isolation: PreviewSubscriptionIsolationArtifact,
        residue_report: PreviewSubscriptionResidueReport,
        promotion_handoff: PreviewSubscriptionPromotionHandoff,
    },
}

fn active_lifecycle_certification_for(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    delta_kind: QuerySubscriptionMaintenanceDeltaKind,
    patch_width: u64,
    continuation_width: u64,
) -> LifecycleCertificationArtifacts {
    let live = LiveQueryAdmissionArtifact::for_test(
        live_family,
        view_family,
        QuerySubscriptionConstructionSource::FacadeLive,
    );
    let selection = select_query_subscription_family(live.clone(), roomy_budget()).unwrap();
    let context = SubscriptionLifecycleCertificationContext::from_live_selection(&live, &selection);
    let declaration = declare_query_subscription(selection, roomy_slice_budget()).unwrap();
    let lowering =
        lower_query_subscription_to_bridge(declaration, roomy_lowering_budget()).unwrap();
    let admission = admit_query_subscription(lowering, roomy_admission_budget()).unwrap();
    let activation = prepare_subscription_activation(admission.clone());
    let scale_report = certify_query_subscription_scale_slope(
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Small,
            10 + patch_width + continuation_width,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Medium,
            (10 + patch_width + continuation_width) * 10,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Large,
            (10 + patch_width + continuation_width) * 100,
            &activation,
        ),
    )
    .unwrap();
    let mut runtime = ActiveSubscriptionRuntime::new();
    let active_admission = admit_active_subscription_lane(
        activation.clone(),
        ActiveSubscriptionWorkBudget::admitted(
            ActiveRegistryLookupWidth::measured(1),
            ActiveFanoutWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::LifecycleArena,
        ),
    )
    .unwrap();
    let handle = open_active_subscription_lane(&mut runtime, active_admission.clone()).unwrap();
    let attachment = attach_subscription_consumer(
        &mut runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted("employee-dashboard", "cursor"),
        SubscriptionConsumerAttachmentBudget::admitted(
            ActiveFanoutWidth::measured(1),
            ConsumerDeliveryPacingWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            DeliveryBackpressurePolicy::RetainWithinWindow,
        ),
    )
    .unwrap();
    let window = open_query_delivery_window(
        &mut runtime,
        &attachment,
        QueryDeliveryWindowBudget::admitted(
            DeliveryWindowWidth::measured(3),
            PatchGroupWidth::measured(patch_width.max(1)),
            MaintenanceDeltaWidth::measured(patch_width.max(1)),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
            DeliveryBackpressurePolicy::RetainWithinWindow,
        ),
    )
    .unwrap();
    let (delta, continuation_report) = if continuation_width > 0 {
        let evidence = admit_subscription_continuation_evidence(
            attachment.lane_digest().clone(),
            SubscriptionContinuationClass::IdentityRemap,
            continuation_test_identity("employee:old"),
            continuation_test_identity("employee:new"),
            continuation_test_identity("basis:current"),
            continuation_test_identity("identity-authority"),
            ContinuationRemapWidth::measured(continuation_width),
        )
        .unwrap();
        let (continued_window, report) =
            apply_active_subscription_continuation(&mut runtime, window, evidence).unwrap();
        let (delta, _) = lower_subscription_continuation_report(&report);
        let (delta, lowering_report, _) =
            lower_query_subscription_maintenance_delta(delta).unwrap();
        let work_packet = build_active_delivery_work_packet(
            &mut runtime,
            &attachment,
            delta.clone(),
            lowering_report.clone(),
            ActiveDeliveryDensityPosture::SparseDelta,
            ActiveDeliveryAffectedLaneWidth::measured(1),
            ActiveDeliveryAffectedAttachmentWidth::measured(1),
            PatchGroupWidth::measured(patch_width.max(1)),
            ActiveDeliveryContinuationWidth::measured(continuation_width),
            ActiveDeliveryPreviewResidueWidth::measured(0),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::PatchScratch,
        )
        .unwrap();
        let delivery_batch =
            emit_query_delivery_batch(&mut runtime, continued_window, work_packet.clone()).unwrap();
        let acknowledged_attachment = advance_subscription_acknowledgement(
            &mut runtime,
            attachment.clone(),
            delivery_batch.receipt().clone(),
        )
        .unwrap();
        let closeout = close_subscription_lifecycle(
            &mut runtime,
            &handle,
            SubscriptionLifecycleCloseRequest::TerminateConsumer(acknowledged_attachment.clone()),
        )
        .unwrap();
        return LifecycleCertificationArtifacts {
            context,
            admission,
            activation,
            scale_report,
            active_admission,
            handle,
            attachment,
            delta,
            lowering_report,
            work_packet,
            delivery_batch,
            acknowledged_attachment,
            continuation_report: Some(report),
            preview: SubscriptionLifecyclePreviewCertificationArtifacts::None,
            closeout,
        };
    } else {
        (
            QuerySubscriptionMaintenanceDelta::admitted_with_scope_label(
                delta_kind,
                attachment.lane_digest().clone(),
                "affected-scope",
                MaintenanceDeltaWidth::measured(patch_width.max(1)),
            ),
            None,
        )
    };
    let (delta, lowering_report, _) = lower_query_subscription_maintenance_delta(delta).unwrap();
    let work_packet = build_active_delivery_work_packet(
        &mut runtime,
        &attachment,
        delta.clone(),
        lowering_report.clone(),
        ActiveDeliveryDensityPosture::SparseDelta,
        ActiveDeliveryAffectedLaneWidth::measured(1),
        ActiveDeliveryAffectedAttachmentWidth::measured(1),
        PatchGroupWidth::measured(patch_width.max(1)),
        ActiveDeliveryContinuationWidth::measured(0),
        ActiveDeliveryPreviewResidueWidth::measured(0),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::PatchScratch,
    )
    .unwrap();
    let delivery_batch =
        emit_query_delivery_batch(&mut runtime, window, work_packet.clone()).unwrap();
    let acknowledged_attachment = advance_subscription_acknowledgement(
        &mut runtime,
        attachment.clone(),
        delivery_batch.receipt().clone(),
    )
    .unwrap();
    let closeout = close_subscription_lifecycle(
        &mut runtime,
        &handle,
        SubscriptionLifecycleCloseRequest::TerminateConsumer(acknowledged_attachment.clone()),
    )
    .unwrap();

    LifecycleCertificationArtifacts {
        context,
        admission,
        activation,
        scale_report,
        active_admission,
        handle,
        attachment,
        delta,
        lowering_report,
        work_packet,
        delivery_batch,
        acknowledged_attachment,
        continuation_report,
        preview: SubscriptionLifecyclePreviewCertificationArtifacts::None,
        closeout,
    }
}

fn preview_discard_certification_artifacts() -> LifecycleCertificationArtifacts {
    let live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::FacadeLive,
    );
    let selection = select_query_subscription_family(live.clone(), roomy_budget()).unwrap();
    let context = SubscriptionLifecycleCertificationContext::from_live_selection(&live, &selection);
    let declaration = declare_query_subscription(selection, roomy_slice_budget()).unwrap();
    let lowering =
        lower_query_subscription_to_bridge(declaration, roomy_lowering_budget()).unwrap();
    let admission = admit_query_subscription(lowering, roomy_admission_budget()).unwrap();
    let activation = prepare_subscription_activation(admission.clone());
    let scale_report = certify_query_subscription_scale_slope(
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Small,
            11,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Medium,
            110,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Large,
            1100,
            &activation,
        ),
    )
    .unwrap();
    let mut runtime = ActiveSubscriptionRuntime::new();
    let active_admission = admit_active_subscription_lane(
        activation.clone(),
        ActiveSubscriptionWorkBudget::admitted(
            ActiveRegistryLookupWidth::measured(1),
            ActiveFanoutWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::LifecycleArena,
        ),
    )
    .unwrap();
    let handle = open_active_subscription_lane(&mut runtime, active_admission.clone()).unwrap();
    let attachment = attach_subscription_consumer(
        &mut runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted("preview-dashboard", "cursor"),
        SubscriptionConsumerAttachmentBudget::admitted(
            ActiveFanoutWidth::measured(1),
            ConsumerDeliveryPacingWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            DeliveryBackpressurePolicy::RetainWithinWindow,
        ),
    )
    .unwrap();
    let window = open_query_delivery_window(
        &mut runtime,
        &attachment,
        QueryDeliveryWindowBudget::admitted(
            DeliveryWindowWidth::measured(3),
            PatchGroupWidth::measured(1),
            MaintenanceDeltaWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
            DeliveryBackpressurePolicy::RetainWithinWindow,
        ),
    )
    .unwrap();
    let delta = QuerySubscriptionMaintenanceDelta::admitted_with_scope_label(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        attachment.lane_digest().clone(),
        "preview-field",
        MaintenanceDeltaWidth::measured(1),
    );
    let (delta, lowering_report, _) = lower_query_subscription_maintenance_delta(delta).unwrap();
    let work_packet = build_active_delivery_work_packet(
        &mut runtime,
        &attachment,
        delta.clone(),
        lowering_report.clone(),
        ActiveDeliveryDensityPosture::SparseDelta,
        ActiveDeliveryAffectedLaneWidth::measured(1),
        ActiveDeliveryAffectedAttachmentWidth::measured(1),
        PatchGroupWidth::measured(1),
        ActiveDeliveryContinuationWidth::measured(0),
        ActiveDeliveryPreviewResidueWidth::measured(0),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::PatchScratch,
    )
    .unwrap();
    let delivery_batch =
        emit_query_delivery_batch(&mut runtime, window, work_packet.clone()).unwrap();
    let acknowledged_attachment = advance_subscription_acknowledgement(
        &mut runtime,
        attachment.clone(),
        delivery_batch.receipt().clone(),
    )
    .unwrap();
    let isolation = admit_preview_subscription_isolation(
        &acknowledged_attachment,
        "preview-certification-discard",
        PreviewResidueWidth::measured(2),
    )
    .unwrap();
    let residue_report = measure_preview_subscription_residue(
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(1),
    );
    let discard_closeout =
        discard_preview_subscription(isolation.clone(), residue_report.clone()).unwrap();
    let closeout = close_subscription_lifecycle(
        &mut runtime,
        &handle,
        SubscriptionLifecycleCloseRequest::PreviewDiscard(discard_closeout.clone()),
    )
    .unwrap();
    LifecycleCertificationArtifacts {
        context,
        admission,
        activation,
        scale_report,
        active_admission,
        handle,
        attachment,
        delta,
        lowering_report,
        work_packet,
        delivery_batch,
        acknowledged_attachment,
        continuation_report: None,
        preview: SubscriptionLifecyclePreviewCertificationArtifacts::Discard {
            isolation,
            residue_report,
            discard_closeout,
        },
        closeout,
    }
}

fn preview_promotion_certification_artifacts() -> LifecycleCertificationArtifacts {
    let live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::FacadeLive,
    );
    let selection = select_query_subscription_family(live.clone(), roomy_budget()).unwrap();
    let context = SubscriptionLifecycleCertificationContext::from_live_selection(&live, &selection);
    let declaration = declare_query_subscription(selection, roomy_slice_budget()).unwrap();
    let lowering =
        lower_query_subscription_to_bridge(declaration, roomy_lowering_budget()).unwrap();
    let admission = admit_query_subscription(lowering, roomy_admission_budget()).unwrap();
    let activation = prepare_subscription_activation(admission.clone());
    let scale_report = certify_query_subscription_scale_slope(
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Small,
            11,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Medium,
            110,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Large,
            1100,
            &activation,
        ),
    )
    .unwrap();
    let mut runtime = ActiveSubscriptionRuntime::new();
    let preview_admission = admit_active_subscription_lane(
        activation.clone(),
        ActiveSubscriptionWorkBudget::admitted(
            ActiveRegistryLookupWidth::measured(1),
            ActiveFanoutWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::LifecycleArena,
        ),
    )
    .unwrap();
    let handle = open_active_subscription_lane(&mut runtime, preview_admission.clone()).unwrap();
    let attachment = attach_subscription_consumer(
        &mut runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted("preview-dashboard", "cursor"),
        SubscriptionConsumerAttachmentBudget::admitted(
            ActiveFanoutWidth::measured(1),
            ConsumerDeliveryPacingWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            DeliveryBackpressurePolicy::RetainWithinWindow,
        ),
    )
    .unwrap();
    let window = open_query_delivery_window(
        &mut runtime,
        &attachment,
        QueryDeliveryWindowBudget::admitted(
            DeliveryWindowWidth::measured(3),
            PatchGroupWidth::measured(1),
            MaintenanceDeltaWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
            DeliveryBackpressurePolicy::RetainWithinWindow,
        ),
    )
    .unwrap();
    let delta = QuerySubscriptionMaintenanceDelta::admitted_with_scope_label(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        attachment.lane_digest().clone(),
        "preview-field",
        MaintenanceDeltaWidth::measured(1),
    );
    let (delta, lowering_report, _) = lower_query_subscription_maintenance_delta(delta).unwrap();
    let work_packet = build_active_delivery_work_packet(
        &mut runtime,
        &attachment,
        delta.clone(),
        lowering_report.clone(),
        ActiveDeliveryDensityPosture::SparseDelta,
        ActiveDeliveryAffectedLaneWidth::measured(1),
        ActiveDeliveryAffectedAttachmentWidth::measured(1),
        PatchGroupWidth::measured(1),
        ActiveDeliveryContinuationWidth::measured(0),
        ActiveDeliveryPreviewResidueWidth::measured(0),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::PatchScratch,
    )
    .unwrap();
    let delivery_batch =
        emit_query_delivery_batch(&mut runtime, window, work_packet.clone()).unwrap();
    let acknowledged_attachment = advance_subscription_acknowledgement(
        &mut runtime,
        attachment.clone(),
        delivery_batch.receipt().clone(),
    )
    .unwrap();
    let authoritative_activation = activation_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let authoritative_admission = admit_active_subscription_lane(
        authoritative_activation,
        ActiveSubscriptionWorkBudget::admitted(
            ActiveRegistryLookupWidth::measured(1),
            ActiveFanoutWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::LifecycleArena,
        ),
    )
    .unwrap();
    let authoritative_handle =
        open_active_subscription_lane(&mut runtime, authoritative_admission).unwrap();
    let isolation = admit_preview_subscription_isolation(
        &acknowledged_attachment,
        "preview-certification-promotion",
        PreviewResidueWidth::measured(1),
    )
    .unwrap();
    let residue_report = measure_preview_subscription_residue(
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(0),
    );
    let promotion_handoff = promote_preview_subscription(
        isolation.clone(),
        &residue_report,
        &authoritative_handle,
        "authority",
    )
    .unwrap();
    let closeout = close_subscription_lifecycle(
        &mut runtime,
        &handle,
        SubscriptionLifecycleCloseRequest::PreviewPromotion(promotion_handoff.clone()),
    )
    .unwrap();
    LifecycleCertificationArtifacts {
        context,
        admission,
        activation,
        scale_report,
        active_admission: preview_admission,
        handle,
        attachment,
        delta,
        lowering_report,
        work_packet,
        delivery_batch,
        acknowledged_attachment,
        continuation_report: None,
        preview: SubscriptionLifecyclePreviewCertificationArtifacts::Promotion {
            isolation,
            residue_report,
            promotion_handoff,
        },
        closeout,
    }
}

fn preview_certification<'a>(
    preview: &'a SubscriptionLifecyclePreviewCertificationArtifacts,
) -> SubscriptionLifecyclePreviewCertification<'a> {
    match preview {
        SubscriptionLifecyclePreviewCertificationArtifacts::None => {
            SubscriptionLifecyclePreviewCertification::None
        }
        SubscriptionLifecyclePreviewCertificationArtifacts::Discard {
            isolation,
            residue_report,
            discard_closeout,
        } => SubscriptionLifecyclePreviewCertification::Discard {
            isolation,
            residue_report,
            discard_closeout,
        },
        SubscriptionLifecyclePreviewCertificationArtifacts::Promotion {
            isolation,
            residue_report,
            promotion_handoff,
        } => SubscriptionLifecyclePreviewCertification::Promotion {
            isolation,
            residue_report,
            promotion_handoff,
        },
    }
}

#[test]
fn admitted_activation_emits_query_subscription_certification_bundle() {
    let (admission, activation, scale_report) = admitted_activation_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let admission_digest = admission.admission_projection().label().to_string();
    let activation_digest = activation.activation_projection().label().to_string();
    let scale_slope_digest = scale_report.report_projection().label().to_string();
    let bundle =
        certify_query_subscription_activation(admission, activation, scale_report).unwrap();

    assert!(!bundle.certification_bundle_projection().label().is_empty());
    assert_eq!(bundle.admission_projection().label(), admission_digest.as_str());
    assert_eq!(bundle.activation_projection().label(), activation_digest.as_str());
    assert_eq!(bundle.scale_slope_projection().label(), scale_slope_digest.as_str());
    assert_eq!(bundle.scale_activation_projection().label(), activation_digest.as_str());
    assert_eq!(bundle.scale_admission_projection().label(), admission_digest.as_str());
    assert!(!bundle.support_profile_projection().label().is_empty());
    assert!(!bundle.diagnostics_projection().label().is_empty());
    assert!(!bundle.admission_counter_projection().label().is_empty());
    assert!(!bundle.activation_counter_projection().label().is_empty());
}

#[test]
fn certification_denies_activation_from_different_admission() {
    let (admission, _, scale_report) = admitted_activation_for(LiveQueryFamily::Detail, None);
    let (_, foreign_activation, _) = admitted_activation_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );

    let error = certify_query_subscription_activation(admission, foreign_activation, scale_report)
        .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionCertificationDenialKind::ActivationAdmissionMismatch
    );
    assert!(!error.failure_projection().label().is_empty());
}

#[test]
fn certification_denies_scale_report_from_different_activation() {
    let (admission, activation, _) = admitted_activation_for(LiveQueryFamily::Detail, None);
    let (_, _, foreign_scale_report) = admitted_activation_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );

    let error = certify_query_subscription_activation(admission, activation, foreign_scale_report)
        .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionCertificationDenialKind::ScaleSlopeSourceMismatch
    );
    assert!(!error.failure_projection().label().is_empty());
}

#[test]
fn scale_slope_certification_admits_row_count_only_variation() {
    let (_, activation, _) = admitted_activation_for(LiveQueryFamily::Detail, None);
    let report = certify_query_subscription_scale_slope(
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Small,
            1,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Medium,
            10,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Large,
            100,
            &activation,
        ),
    )
    .unwrap();

    assert_eq!(
        report.activation_projection().label(),
        activation.activation_projection().label().as_str()
    );
    assert_eq!(
        report.admission_projection().label(),
        activation.admission_projection().label().as_str()
    );
    assert_eq!(report.small_row_count(), 1);
    assert_eq!(report.medium_row_count(), 10);
    assert_eq!(report.large_row_count(), 100);
    assert!(!report.structural_counter_projection().label().is_empty());
}

#[test]
fn scale_slope_certification_denies_mixed_activation_sources() {
    let (_, activation, _) = admitted_activation_for(LiveQueryFamily::Detail, None);
    let (_, foreign_activation, _) = admitted_activation_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );

    let error = certify_query_subscription_scale_slope(
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Small,
            1,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Medium,
            10,
            &foreign_activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Large,
            100,
            &activation,
        ),
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionCertificationDenialKind::ScaleSlopeDrift
    );
    assert!(!error.failure_projection().label().is_empty());
}

#[test]
fn scale_slope_certification_denies_zero_row_baseline() {
    let (_, activation, _) = admitted_activation_for(LiveQueryFamily::Detail, None);

    let error = certify_query_subscription_scale_slope(
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Small,
            0,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Medium,
            10,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Large,
            100,
            &activation,
        ),
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionCertificationDenialKind::ScaleSlopeDrift
    );
    assert!(!error.failure_projection().label().is_empty());
}

#[test]
fn scale_slope_certification_denies_structural_counter_drift() {
    let (_, activation, _) = admitted_activation_for(LiveQueryFamily::Detail, None);
    let small = QuerySubscriptionScaleCounterSnapshot::from_activation(
        QuerySubscriptionScaleFixtureSize::Small,
        1,
        &activation,
    );
    let medium = QuerySubscriptionScaleCounterSnapshot::from_activation(
        QuerySubscriptionScaleFixtureSize::Medium,
        10,
        &activation,
    )
    .with_bridge_slice_count_for_test(&activation, activation.counters().bridge_slice_count() + 1);
    let large = QuerySubscriptionScaleCounterSnapshot::from_activation(
        QuerySubscriptionScaleFixtureSize::Large,
        100,
        &activation,
    );

    let error = certify_query_subscription_scale_slope(small, medium, large).unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionCertificationDenialKind::ScaleSlopeDrift
    );
    assert!(!error.failure_projection().label().is_empty());
}

#[test]
fn lifecycle_certification_emits_runtime_backed_bundle() {
    let artifacts = active_lifecycle_certification_for(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        1,
        0,
    );

    let bundle = certify_subscription_lifecycle(
        artifacts.context,
        &artifacts.admission,
        &artifacts.activation,
        &artifacts.scale_report,
        &artifacts.active_admission,
        &artifacts.handle,
        &artifacts.attachment,
        artifacts.delivery_batch.delivery_window_identity(),
        &artifacts.delta,
        &artifacts.lowering_report,
        &artifacts.work_packet,
        &artifacts.delivery_batch,
        &artifacts.acknowledged_attachment,
        artifacts.continuation_report.as_ref(),
        preview_certification(&artifacts.preview),
        &artifacts.closeout,
    )
    .unwrap();

    assert!(!bundle.certification_bundle_projection().label().is_empty());
    assert_eq!(
        bundle.active_lane_projection().label(),
        artifacts.handle.lane_projection().label()
    );
    assert_eq!(
        bundle.delivery_receipt_projection().label(),
        artifacts.delivery_batch.receipt().receipt_projection().label()
    );
    assert_eq!(
        bundle.acknowledgement_frontier_projection().label(),
        artifacts
            .acknowledged_attachment
            .acknowledgement_frontier()
            .frontier_projection().label()
    );
    assert!(artifacts.continuation_report.is_none());
    assert_eq!(
        bundle.preview_isolation_projection().label(),
        lifecycle_absent_preview_isolation_identity().as_str()
    );
    assert!(!bundle.support_matrix_projection().label().is_empty());
    assert!(!bundle.counter_snapshot_projection().label().is_empty());
    assert!(
        !bundle.counter_sequence_identity().as_str().is_empty(),
        "lifecycle certification should bind typed counter sequence identity"
    );
    assert!(
        !bundle.certification_bundle_identity().as_str().is_empty(),
        "certification bundle authority must be typed evidence identity"
    );
}

#[test]
fn lifecycle_certification_binds_continuation_receipt_and_digest() {
    let artifacts = active_lifecycle_certification_for(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        1,
        1,
    );

    let bundle = certify_subscription_lifecycle(
        artifacts.context,
        &artifacts.admission,
        &artifacts.activation,
        &artifacts.scale_report,
        &artifacts.active_admission,
        &artifacts.handle,
        &artifacts.attachment,
        artifacts.delivery_batch.delivery_window_identity(),
        &artifacts.delta,
        &artifacts.lowering_report,
        &artifacts.work_packet,
        &artifacts.delivery_batch,
        &artifacts.acknowledged_attachment,
        artifacts.continuation_report.as_ref(),
        preview_certification(&artifacts.preview),
        &artifacts.closeout,
    )
    .unwrap();

    assert!(artifacts.continuation_report.is_some());
    assert_ne!(bundle.continuation_projection().label(), "none");
    assert!(!bundle
        .subscription_performance_receipt_projection()
        .label()
        .is_empty());
}

#[test]
fn lifecycle_certification_denies_attachment_from_foreign_lane() {
    let control = active_lifecycle_certification_for(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        1,
        0,
    );
    let foreign = active_lifecycle_certification_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
        QuerySubscriptionMaintenanceDeltaKind::CollectionMembershipDelta,
        1,
        0,
    );

    let error = certify_subscription_lifecycle(
        control.context,
        &control.admission,
        &control.activation,
        &control.scale_report,
        &control.active_admission,
        &control.handle,
        &foreign.attachment,
        control.delivery_batch.delivery_window_identity(),
        &control.delta,
        &control.lowering_report,
        &control.work_packet,
        &control.delivery_batch,
        &control.acknowledged_attachment,
        control.continuation_report.as_ref(),
        preview_certification(&control.preview),
        &control.closeout,
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &SubscriptionLifecycleCertificationDenialKind::AttachmentSourceMismatch
    );
    assert!(!error.failure_projection().label().is_empty());
}

#[test]
fn lifecycle_certification_emits_preview_discard_and_support_evidence() {
    let artifacts = preview_discard_certification_artifacts();

    let bundle = certify_subscription_lifecycle(
        artifacts.context,
        &artifacts.admission,
        &artifacts.activation,
        &artifacts.scale_report,
        &artifacts.active_admission,
        &artifacts.handle,
        &artifacts.attachment,
        artifacts.delivery_batch.delivery_window_identity(),
        &artifacts.delta,
        &artifacts.lowering_report,
        &artifacts.work_packet,
        &artifacts.delivery_batch,
        &artifacts.acknowledged_attachment,
        artifacts.continuation_report.as_ref(),
        preview_certification(&artifacts.preview),
        &artifacts.closeout,
    )
    .unwrap();

    assert_ne!(bundle.preview_isolation_projection().label(), "none");
    assert_ne!(bundle.preview_residue_projection().label(), "none");
    assert!(
        !bundle.counter_sequence_identity().as_str().is_empty(),
        "preview discard certification should include typed counter sequence identity"
    );
    assert!(!bundle.support_matrix_projection().label().is_empty());
}

#[test]
fn lifecycle_certification_emits_preview_promotion_boundary_evidence() {
    let artifacts = preview_promotion_certification_artifacts();

    let bundle = certify_subscription_lifecycle(
        artifacts.context,
        &artifacts.admission,
        &artifacts.activation,
        &artifacts.scale_report,
        &artifacts.active_admission,
        &artifacts.handle,
        &artifacts.attachment,
        artifacts.delivery_batch.delivery_window_identity(),
        &artifacts.delta,
        &artifacts.lowering_report,
        &artifacts.work_packet,
        &artifacts.delivery_batch,
        &artifacts.acknowledged_attachment,
        artifacts.continuation_report.as_ref(),
        preview_certification(&artifacts.preview),
        &artifacts.closeout,
    )
    .unwrap();

    assert_ne!(bundle.preview_isolation_projection().label(), "none");
    assert_ne!(bundle.preview_residue_projection().label(), "none");
    assert!(
        !bundle.counter_sequence_identity().as_str().is_empty(),
        "preview promotion certification should include typed counter sequence identity"
    );
}

#[test]
fn lifecycle_certification_denies_preview_promotion_with_foreign_handoff_source() {
    let artifacts = preview_promotion_certification_artifacts();
    let SubscriptionLifecyclePreviewCertificationArtifacts::Promotion {
        isolation,
        residue_report,
        ..
    } = &artifacts.preview
    else {
        panic!("expected preview promotion artifacts");
    };
    let foreign_residue_report = measure_preview_subscription_residue(
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
    );
    let mut foreign_runtime = ActiveSubscriptionRuntime::new();
    let foreign_authoritative_admission = admit_active_subscription_lane(
        activation_for(
            LiveQueryFamily::OrderedCollection,
            Some(LiveViewShapeFamily::Table),
        ),
        ActiveSubscriptionWorkBudget::admitted(
            ActiveRegistryLookupWidth::measured(1),
            ActiveFanoutWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::LifecycleArena,
        ),
    )
    .unwrap();
    let foreign_authoritative_handle =
        open_active_subscription_lane(&mut foreign_runtime, foreign_authoritative_admission)
            .unwrap();
    let foreign_handoff = promote_preview_subscription(
        isolation.clone(),
        &foreign_residue_report,
        &foreign_authoritative_handle,
        "foreign-authority",
    )
    .unwrap();

    let error = certify_subscription_lifecycle(
        artifacts.context,
        &artifacts.admission,
        &artifacts.activation,
        &artifacts.scale_report,
        &artifacts.active_admission,
        &artifacts.handle,
        &artifacts.attachment,
        artifacts.delivery_batch.delivery_window_identity(),
        &artifacts.delta,
        &artifacts.lowering_report,
        &artifacts.work_packet,
        &artifacts.delivery_batch,
        &artifacts.acknowledged_attachment,
        artifacts.continuation_report.as_ref(),
        SubscriptionLifecyclePreviewCertification::Promotion {
            isolation,
            residue_report,
            promotion_handoff: &foreign_handoff,
        },
        &artifacts.closeout,
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &SubscriptionLifecycleCertificationDenialKind::PreviewSourceMismatch
    );
}
