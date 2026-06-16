use super::{
    MilestoneNineTwoCertificationMatrix, MilestoneNineTwoCertificationRow,
    MilestoneNineTwoFailureClass, MilestoneNineTwoPerturbationClass,
    MilestoneNineTwoRejectionBundle, MilestoneNineTwoRejectionRow,
    SubscriptionLifecycleCertificationBundle, MILESTONE_NINE_TWO_REQUIRED_COMPILE_FAIL_TARGETS,
};
use crate::harness::certification::{digest_parts, HostileExpectation, ParityAnchor};
use crate::live::LiveQueryFamily;
use crate::subscription::{
    admit_active_subscription_lane, admit_preview_subscription_isolation, admit_query_subscription,
    admit_subscription_continuation_evidence, advance_subscription_acknowledgement,
    apply_active_subscription_continuation, attach_subscription_consumer,
    build_active_delivery_work_packet, certify_query_subscription_scale_slope,
    certify_subscription_lifecycle, close_subscription_lifecycle, declare_query_subscription,
    deny_preview_authoritative_sharing, deny_raw_bridge_invalidation_delivery,
    deny_raw_cdc_delivery_fallback, discard_preview_subscription, emit_query_delivery_batch,
    join_active_subscription_lane, lower_query_subscription_maintenance_delta,
    lower_query_subscription_to_bridge, lower_subscription_continuation_report,
    measure_preview_subscription_residue, open_active_subscription_lane,
    open_query_delivery_window, prepare_subscription_activation, promote_preview_subscription,
    select_query_subscription_family, ActiveAllocationScopeWidth,
    ActiveDeliveryAffectedAttachmentWidth, ActiveDeliveryAffectedLaneWidth,
    ActiveDeliveryContinuationWidth, ActiveDeliveryDensityPosture,
    ActiveDeliveryPreviewResidueWidth, ActiveFanoutWidth, ActiveRegistryLookupWidth,
    ActiveSubscriptionAllocationPolicy, ActiveSubscriptionAllocationPosture,
    ActiveSubscriptionRuntime, ActiveSubscriptionWorkBudget, ConsumerDeliveryPacingWidth,
    ContinuationRemapWidth, DeliveryBackpressurePolicy, DeliveryWindowWidth,
    LiveQueryAdmissionArtifact, MaintenanceDeltaWidth, PatchGroupWidth, PreviewResidueWidth,
    QueryDeliveryWindowBudget, QuerySubscriptionAdmissionBudget,
    QuerySubscriptionBridgeLoweringBudget, QuerySubscriptionConstructionSource,
    QuerySubscriptionMaintenanceDelta, QuerySubscriptionMaintenanceDeltaKind,
    QuerySubscriptionScaleCounterSnapshot, QuerySubscriptionScaleFixtureSize,
    QuerySubscriptionSliceBudget, QuerySubscriptionWorkBudget, SubscriptionActivationInput,
    SubscriptionConsumerAttachmentBudget, SubscriptionConsumerAttachmentRequest,
    SubscriptionContinuationClass, SubscriptionLifecycleCertificationContext,
    SubscriptionLifecycleCloseRequest, SubscriptionLifecyclePreviewCertification,
};
use crate::view_shape_live::LiveViewShapeFamily;

fn continuation_harness_identity(label: &str) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("identity_family"),
        "subscription_continuation_harness_identity_v1",
    )
    .field_shape(crate::ForgeQueryEvidenceTag::new("label"), label)
    .seal()
}

pub(super) fn canonical_rows() -> Vec<MilestoneNineTwoCertificationRow> {
    vec![
        row(
            "detail-active-lifecycle-delivery-ack",
            MilestoneNineTwoPerturbationClass::DetailLifecycleDelivery,
            HostileExpectation::EquivalentToControl,
            ParityAnchor::Control,
            lifecycle_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
                "manager_id",
                1,
                0,
            ),
            lifecycle_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
                "manager_id",
                1,
                0,
            ),
            lifecycle_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
                "manager_id",
                1,
                0,
            ),
        ),
        row(
            "equivalent-subscription-sharing-fanout",
            MilestoneNineTwoPerturbationClass::EquivalentSharingFanout,
            HostileExpectation::EquivalentToControl,
            ParityAnchor::Control,
            sharing_lane("consumer-a", "consumer-b"),
            sharing_lane("consumer-a", "consumer-b"),
            sharing_lane("consumer-a", "consumer-b"),
        ),
        row(
            "grouped-membership-query-shaped-delivery",
            MilestoneNineTwoPerturbationClass::GroupedMembershipDelivery,
            HostileExpectation::DistinctFromControl,
            ParityAnchor::Hostile,
            lifecycle_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionMaintenanceDeltaKind::CollectionMembershipDelta,
                "employee:engineering-to-design",
                2,
                0,
            ),
            lifecycle_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::KanbanGrouped),
                QuerySubscriptionMaintenanceDeltaKind::GroupedMembershipDelta,
                "employee:engineering-to-design",
                2,
                0,
            ),
            lifecycle_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::KanbanGrouped),
                QuerySubscriptionMaintenanceDeltaKind::GroupedMembershipDelta,
                "employee:engineering-to-design",
                2,
                0,
            ),
        ),
        row(
            "identity-continuation-remap-delivery",
            MilestoneNineTwoPerturbationClass::IdentityContinuationRemap,
            HostileExpectation::DistinctFromControl,
            ParityAnchor::Hostile,
            lifecycle_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
                "employee:name",
                1,
                0,
            ),
            lifecycle_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionMaintenanceDeltaKind::ContinuationDelta,
                "employee:old-to-new",
                1,
                1,
            ),
            lifecycle_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionMaintenanceDeltaKind::ContinuationDelta,
                "employee:old-to-new",
                1,
                1,
            ),
        ),
        row(
            "preview-discard-zero-authoritative-residue",
            MilestoneNineTwoPerturbationClass::PreviewDiscardIsolation,
            HostileExpectation::EquivalentToControl,
            ParityAnchor::Control,
            preview_discard_lane(),
            preview_discard_lane(),
            preview_discard_lane(),
        ),
        row(
            "preview-promotion-boundary-handoff",
            MilestoneNineTwoPerturbationClass::PreviewPromotionBoundary,
            HostileExpectation::DistinctFromControl,
            ParityAnchor::Hostile,
            preview_discard_lane(),
            preview_promotion_lane(),
            preview_promotion_lane(),
        ),
        row(
            "performance-receipt-posture-sensitive",
            MilestoneNineTwoPerturbationClass::PerformanceReceiptPostureSensitive,
            HostileExpectation::DistinctFromControl,
            ParityAnchor::Hostile,
            lifecycle_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
                "manager_id",
                1,
                0,
            ),
            lifecycle_lane_with_delivery_profile(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
                "manager_id",
                1,
                0,
                ActiveDeliveryDensityPosture::BurstCoalesced,
                2,
                ActiveSubscriptionAllocationPosture::HeapAllocationDebtExplicit,
            ),
            lifecycle_lane_with_delivery_profile(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
                "manager_id",
                1,
                0,
                ActiveDeliveryDensityPosture::BurstCoalesced,
                2,
                ActiveSubscriptionAllocationPosture::HeapAllocationDebtExplicit,
            ),
        ),
        row(
            "scale-slope-width-bounded-lifecycle",
            MilestoneNineTwoPerturbationClass::ScaleSlopeWidthBounded,
            HostileExpectation::DistinctFromControl,
            ParityAnchor::Hostile,
            lifecycle_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
                "manager_id",
                1,
                0,
            ),
            lifecycle_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
                "manager_id",
                3,
                2,
            ),
            lifecycle_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
                "manager_id",
                3,
                2,
            ),
        ),
    ]
}

pub(super) fn rejection_rows() -> Vec<MilestoneNineTwoRejectionRow> {
    vec![
        rejection_row(
            "masked-sharing-denies-before-join",
            MilestoneNineTwoPerturbationClass::MaskedSharingDenied,
            masked_sharing_rejection(),
        ),
        rejection_row(
            "raw-cdc-delivery-denied-before-batch",
            MilestoneNineTwoPerturbationClass::RawCdcDeliveryDenied,
            raw_cdc_rejection(),
        ),
        rejection_row(
            "raw-bridge-invalidation-denied-before-batch",
            MilestoneNineTwoPerturbationClass::RawBridgeInvalidationDenied,
            raw_bridge_rejection(),
        ),
        rejection_row(
            "preview-authoritative-sharing-denied",
            MilestoneNineTwoPerturbationClass::PreviewAuthoritativeSharingDenied,
            preview_sharing_rejection(),
        ),
        rejection_row(
            "preview-discard-authoritative-residue-denied",
            MilestoneNineTwoPerturbationClass::PreviewDiscardResidueDenied,
            preview_residue_rejection(),
        ),
        rejection_row(
            "dense-refresh-denied-before-work-packet",
            MilestoneNineTwoPerturbationClass::DenseRefreshDenied,
            dense_refresh_rejection(),
        ),
        rejection_row(
            "store-backed-restart-denied-before-lane",
            MilestoneNineTwoPerturbationClass::StoreBackedRestartDenied,
            store_backed_restart_rejection(),
        ),
    ]
}

fn row(
    row_name: &'static str,
    perturbation_class: MilestoneNineTwoPerturbationClass,
    hostile_expectation: HostileExpectation,
    parity_anchor: ParityAnchor,
    control_lane: SubscriptionLifecycleCertificationBundle,
    hostile_lane: SubscriptionLifecycleCertificationBundle,
    parity_lane: SubscriptionLifecycleCertificationBundle,
) -> MilestoneNineTwoCertificationRow {
    MilestoneNineTwoCertificationRow {
        row_name,
        perturbation_class,
        hostile_expectation,
        parity_anchor,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}

fn rejection_row(
    row_name: &'static str,
    perturbation_class: MilestoneNineTwoPerturbationClass,
    hostile_lane: MilestoneNineTwoRejectionBundle,
) -> MilestoneNineTwoRejectionRow {
    MilestoneNineTwoRejectionRow {
        row_name,
        perturbation_class,
        control_lane: lifecycle_lane(
            LiveQueryFamily::Detail,
            None,
            QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
            "control",
            1,
            0,
        ),
        hostile_lane,
        parity_lane: lifecycle_lane(
            LiveQueryFamily::Detail,
            None,
            QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
            "control",
            1,
            0,
        ),
    }
}

pub(super) fn bundle_digest_parts(matrix: &MilestoneNineTwoCertificationMatrix) -> Vec<String> {
    matrix
        .rows
        .iter()
        .flat_map(|row| {
            [
                format!(
                    "{}:control:{}",
                    row.row_name,
                    row.control_lane.lifecycle_signature()
                ),
                format!(
                    "{}:hostile:{}",
                    row.row_name,
                    row.hostile_lane.lifecycle_signature()
                ),
                format!(
                    "{}:parity:{}",
                    row.row_name,
                    row.parity_lane.lifecycle_signature()
                ),
            ]
        })
        .chain(matrix.rejection_rows.iter().flat_map(|row| {
            [
                format!(
                    "{}:control:{}",
                    row.row_name,
                    row.control_lane.lifecycle_signature()
                ),
                format!(
                    "{}:hostile:{}",
                    row.row_name, row.hostile_lane.failure_digest
                ),
                format!(
                    "{}:parity:{}",
                    row.row_name,
                    row.parity_lane.lifecycle_signature()
                ),
            ]
        }))
        .collect()
}

pub(super) fn coverage_digest_parts(matrix: &MilestoneNineTwoCertificationMatrix) -> Vec<String> {
    matrix
        .rows
        .iter()
        .map(|row| {
            format!(
                "canonical:{}:{:?}:{:?}:{:?}",
                row.row_name, row.perturbation_class, row.hostile_expectation, row.parity_anchor
            )
        })
        .chain(
            matrix
                .rejection_rows
                .iter()
                .map(|row| format!("rejection:{}:{:?}", row.row_name, row.perturbation_class)),
        )
        .collect()
}

fn lifecycle_lane(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    delta_kind: QuerySubscriptionMaintenanceDeltaKind,
    affected_scope: &str,
    patch_width: u64,
    continuation_width: u64,
) -> SubscriptionLifecycleCertificationBundle {
    lifecycle_lane_with_delivery_profile(
        live_family,
        view_family,
        delta_kind,
        affected_scope,
        patch_width,
        continuation_width,
        ActiveDeliveryDensityPosture::SparseDelta,
        1,
        ActiveSubscriptionAllocationPosture::PatchScratch,
    )
}

fn shipped_bundle(
    bundle: crate::subscription::SubscriptionLifecycleCertificationBundle,
) -> SubscriptionLifecycleCertificationBundle {
    SubscriptionLifecycleCertificationBundle {
        query_digest: bundle.query_scope_projection().label().to_string(),
        subscription_family_digest: bundle.subscription_family_projection().label().to_string(),
        subscription_declaration_digest: bundle
            .subscription_declaration_projection()
            .label()
            .to_string(),
        subscription_equivalence_digest: bundle
            .subscription_equivalence_projection()
            .label()
            .to_string(),
        active_lane_digest: bundle.active_lane_projection().label().to_string(),
        active_lane_handle_digest: bundle.active_lane_handle_projection().label().to_string(),
        active_lane_lookup_class_digest: bundle
            .active_lane_lookup_class_projection()
            .label()
            .to_string(),
        subscription_budget_digest: bundle.subscription_budget_projection().label().to_string(),
        subscription_performance_receipt_digest: bundle
            .subscription_performance_receipt_projection()
            .label()
            .to_string(),
        consumer_attachment_digest: bundle.consumer_attachment_projection().label().to_string(),
        acknowledgement_frontier_digest: bundle
            .acknowledgement_frontier_projection()
            .label()
            .to_string(),
        delivery_window_digest: bundle.delivery_window_projection().label().to_string(),
        maintenance_delta_digest: bundle.maintenance_delta_projection().label().to_string(),
        active_delivery_work_packet_digest: bundle
            .active_delivery_work_packet_projection()
            .label()
            .to_string(),
        active_delivery_density_posture_digest: bundle
            .active_delivery_density_posture_projection()
            .label()
            .to_string(),
        allocation_posture_digest: bundle.allocation_posture_projection().label().to_string(),
        delivery_batch_digest: bundle.delivery_batch_projection().label().to_string(),
        patch_group_digest: bundle.patch_group_projection().label().to_string(),
        delivery_receipt_digest: bundle.delivery_receipt_projection().label().to_string(),
        continuation_digest: bundle.continuation_projection().label().to_string(),
        preview_isolation_digest: bundle.preview_isolation_projection().label().to_string(),
        preview_residue_digest: bundle.preview_residue_projection().label().to_string(),
        policy_digest: bundle.policy_projection().label().to_string(),
        tenant_basis_digest: bundle.tenant_basis_projection().label().to_string(),
        relationship_proof_digest: bundle.relationship_proof_projection().label().to_string(),
        view_shape_digest: bundle.view_shape_projection().label().to_string(),
        basis_digest: bundle.basis_posture_projection().label().to_string(),
        bridge_declaration_digest: bundle.bridge_declaration_projection().label().to_string(),
        signal_strategy_digest: bundle.signal_strategy_projection().label().to_string(),
        failure_digest: "none".to_string(),
        lifecycle_denial_digest: "none".to_string(),
        counter_snapshot: bundle.counter_snapshot_projection().label().to_string(),
        counter_evidence: Vec::new(),
        subscription_lifecycle_scale_slope_digest: bundle
            .subscription_lifecycle_scale_slope_projection()
            .label()
            .to_string(),
        compile_fail_boundary_digest: compile_fail_boundary_digest(),
        support_matrix_digest: bundle.support_matrix_projection().label().to_string(),
    }
}

#[allow(clippy::too_many_arguments)]
fn lifecycle_lane_with_delivery_profile(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    delta_kind: QuerySubscriptionMaintenanceDeltaKind,
    affected_scope: &str,
    patch_width: u64,
    continuation_width: u64,
    density_posture: ActiveDeliveryDensityPosture,
    allocation_scope_width: u64,
    allocation_posture: ActiveSubscriptionAllocationPosture,
) -> SubscriptionLifecycleCertificationBundle {
    let live = LiveQueryAdmissionArtifact::for_test(
        live_family,
        view_family,
        QuerySubscriptionConstructionSource::FacadeLive,
    );
    let selection = select_query_subscription_family(live.clone(), work_budget()).unwrap();
    let context = SubscriptionLifecycleCertificationContext::from_live_selection(&live, &selection);
    let declaration = declare_query_subscription(selection, slice_budget()).unwrap();
    let lowering = lower_query_subscription_to_bridge(declaration, lowering_budget()).unwrap();
    let admission_artifact = admit_query_subscription(lowering, admission_budget()).unwrap();
    let activation = prepare_subscription_activation(admission_artifact.clone());
    let scale_report = scale_slope_report(&activation, patch_width, continuation_width);
    let mut runtime = ActiveSubscriptionRuntime::new();
    let active_admission =
        admit_active_subscription_lane(activation.clone(), active_budget()).unwrap();
    let handle = open_active_subscription_lane(&mut runtime, active_admission.clone()).unwrap();
    let attachment = attach_subscription_consumer(
        &mut runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted("employee-dashboard", "cursor"),
        attachment_budget(),
    )
    .unwrap();
    let window = open_query_delivery_window(&mut runtime, &attachment, delivery_budget()).unwrap();
    let (window, delta, continuation_report, extra_counters) = if continuation_width > 0 {
        let evidence = admit_subscription_continuation_evidence(
            attachment.lane_digest().clone(),
            SubscriptionContinuationClass::IdentityRemap,
            continuation_harness_identity("employee:old"),
            continuation_harness_identity("employee:new"),
            continuation_harness_identity("basis:current"),
            continuation_harness_identity("identity-authority"),
            ContinuationRemapWidth::measured(continuation_width),
        )
        .unwrap();
        let (window, report) =
            apply_active_subscription_continuation(&mut runtime, window, evidence).unwrap();
        let (delta, counters) = lower_subscription_continuation_report(&report);
        (window, delta, Some(report), vec![counters.counter_projection().label().to_string()])
    } else {
        let delta = QuerySubscriptionMaintenanceDelta::admitted_with_scope_label(
            delta_kind,
            attachment.lane_digest().clone(),
            affected_scope,
            MaintenanceDeltaWidth::measured(patch_width),
        );
        (window, delta, None, Vec::new())
    };
    let (delta, lowering_report, lowering_counters) =
        lower_query_subscription_maintenance_delta(delta).unwrap();
    finish_delivery_lane(
        &mut runtime,
        context,
        admission_artifact,
        activation,
        scale_report,
        active_admission,
        handle,
        attachment,
        window,
        delta,
        lowering_report,
        patch_width,
        continuation_width,
        continuation_report,
        &[
            extra_counters,
            vec![lowering_counters.counter_projection().label().to_string()],
            scale_axis_evidence(patch_width, continuation_width),
        ]
        .concat(),
        density_posture,
        allocation_scope_width,
        allocation_posture,
    )
}

#[allow(clippy::too_many_arguments)]
fn finish_delivery_lane(
    runtime: &mut ActiveSubscriptionRuntime,
    context: SubscriptionLifecycleCertificationContext,
    admission_artifact: crate::subscription::QuerySubscriptionAdmissionArtifact,
    activation: SubscriptionActivationInput,
    scale_report: crate::subscription::QuerySubscriptionScaleSlopeReport,
    active_admission: crate::subscription::ActiveSubscriptionLaneAdmission,
    active_lane_handle: crate::subscription::ActiveSubscriptionLaneHandle,
    attachment: crate::subscription::SubscriptionConsumerAttachment,
    window: crate::subscription::QueryDeliveryWindow,
    delta: QuerySubscriptionMaintenanceDelta,
    lowering_report: crate::subscription::QueryMaintenanceDeltaLoweringReport,
    patch_width: u64,
    continuation_width: u64,
    continuation_report: Option<crate::subscription::SubscriptionContinuationReport>,
    extra_counter_digests: &[String],
    density_posture: ActiveDeliveryDensityPosture,
    allocation_scope_width: u64,
    allocation_posture: ActiveSubscriptionAllocationPosture,
) -> SubscriptionLifecycleCertificationBundle {
    let evidence = deliver_to_attachment(
        runtime,
        attachment.clone(),
        window,
        delta.clone(),
        lowering_report.clone(),
        density_posture,
        1,
        1,
        patch_width,
        continuation_width,
        0,
        allocation_scope_width,
        allocation_posture,
    );
    let lifecycle_closeout = close_subscription_lifecycle(
        runtime,
        &active_lane_handle,
        SubscriptionLifecycleCloseRequest::TerminateConsumer(
            evidence.acknowledged_attachment.clone(),
        ),
    )
    .unwrap();
    let shipped = certify_subscription_lifecycle(
        context,
        &admission_artifact,
        &activation,
        &scale_report,
        &active_admission,
        &active_lane_handle,
        &attachment,
        evidence.delivery_batch.delivery_window_identity(),
        &delta,
        &lowering_report,
        &evidence.work_packet,
        &evidence.delivery_batch,
        &evidence.acknowledged_attachment,
        continuation_report.as_ref(),
        SubscriptionLifecyclePreviewCertification::None,
        &lifecycle_closeout,
    )
    .unwrap();
    let mut bundle = shipped_bundle(shipped);
    let mut counter_parts = bundle.counter_evidence.clone();
    counter_parts.push(format!("packet:{}", evidence.work_packet_counter_digest));
    counter_parts.push(format!("batch:{}", evidence.batch_counter_digest));
    counter_parts.push(format!("ack:{}", evidence.ack_counter_digest));
    counter_parts.push(format!(
        "closeout:{}",
        lifecycle_closeout.counters().counter_projection().label()
    ));
    counter_parts.extend(extra_counter_digests.iter().cloned());
    bundle.counter_snapshot = digest_parts(&counter_parts);
    bundle.counter_evidence = counter_parts;
    bundle
}

#[derive(Debug, Eq, PartialEq)]
struct DeliveryEvidence {
    performance_receipt_digest: String,
    active_delivery_work_packet_digest: String,
    density_posture_digest: String,
    maintenance_delta_digest: String,
    delivery_batch_digest: String,
    delivery_window_digest: String,
    patch_group_digest: String,
    delivery_receipt_digest: String,
    acknowledgement_frontier_digest: String,
    work_packet: crate::subscription::ActiveDeliveryWorkPacket,
    delivery_batch: crate::subscription::QueryDeliveryBatch,
    acknowledged_attachment: crate::subscription::SubscriptionConsumerAttachment,
    work_packet_counter_digest: String,
    batch_counter_digest: String,
    ack_counter_digest: String,
}

#[allow(clippy::too_many_arguments)]
fn deliver_to_attachment(
    runtime: &mut ActiveSubscriptionRuntime,
    attachment: crate::subscription::SubscriptionConsumerAttachment,
    window: crate::subscription::QueryDeliveryWindow,
    delta: QuerySubscriptionMaintenanceDelta,
    lowering_report: crate::subscription::QueryMaintenanceDeltaLoweringReport,
    density_posture: ActiveDeliveryDensityPosture,
    affected_lane_width: u64,
    affected_attachment_width: u64,
    patch_width: u64,
    continuation_width: u64,
    preview_residue_width: u64,
    allocation_scope_width: u64,
    allocation_posture: ActiveSubscriptionAllocationPosture,
) -> DeliveryEvidence {
    let work_packet = build_active_delivery_work_packet(
        runtime,
        &attachment,
        delta,
        lowering_report,
        density_posture,
        ActiveDeliveryAffectedLaneWidth::measured(affected_lane_width),
        ActiveDeliveryAffectedAttachmentWidth::measured(affected_attachment_width),
        PatchGroupWidth::measured(patch_width),
        ActiveDeliveryContinuationWidth::measured(continuation_width),
        ActiveDeliveryPreviewResidueWidth::measured(preview_residue_width),
        ActiveAllocationScopeWidth::measured(allocation_scope_width),
        allocation_posture,
    )
    .unwrap();
    let work_packet_counter_digest = runtime.counters().counter_projection().label().to_string();
    let performance_receipt_digest = work_packet
        .performance_receipt()
        .performance_receipt_projection()
        .label()
        .to_string();
    let active_delivery_work_packet_digest = work_packet.work_packet_projection().label().to_string();
    let density_posture_digest =
        digest_parts(&[work_packet.density_posture().as_str().to_string()]);
    let maintenance_delta_digest = work_packet
        .maintenance_delta()
        .maintenance_delta_projection()
        .label()
        .to_string();
    let batch = emit_query_delivery_batch(runtime, window, work_packet.clone()).unwrap();
    let batch_counter_digest = batch.counters().counter_projection().label().to_string();
    let delivery_batch_digest = batch.delivery_batch_projection().label().to_string();
    let delivery_window_digest = batch.delivery_window_projection().label().to_string();
    let patch_group_digest = batch.patch_group().patch_group_projection().label().to_string();
    let delivery_receipt_digest = batch.receipt().receipt_projection().label().to_string();
    let acknowledgement =
        advance_subscription_acknowledgement(runtime, attachment, batch.receipt().clone()).unwrap();
    let ack_counter_digest = runtime.counters().counter_projection().label().to_string();
    let acknowledgement_frontier_digest = acknowledgement
        .acknowledgement_frontier()
        .frontier_projection()
        .label()
        .to_string();

    DeliveryEvidence {
        performance_receipt_digest,
        active_delivery_work_packet_digest,
        density_posture_digest,
        maintenance_delta_digest,
        delivery_batch_digest,
        delivery_window_digest,
        patch_group_digest,
        delivery_receipt_digest,
        acknowledgement_frontier_digest,
        work_packet,
        delivery_batch: batch,
        acknowledged_attachment: acknowledgement,
        work_packet_counter_digest,
        batch_counter_digest,
        ack_counter_digest,
    }
}

fn sharing_lane(
    first_consumer: &str,
    second_consumer: &str,
) -> SubscriptionLifecycleCertificationBundle {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let activation = activation_for(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::FacadeLive,
    );
    let admission = admit_active_subscription_lane(activation.clone(), active_budget()).unwrap();
    let handle = open_active_subscription_lane(&mut runtime, admission).unwrap();
    let open_counter_digest = runtime.counters().counter_projection().label().to_string();
    let join_admission = admit_active_subscription_lane(activation, active_budget()).unwrap();
    let joined = join_active_subscription_lane(&mut runtime, &handle, join_admission).unwrap();
    let join_counter_digest = runtime.counters().counter_projection().label().to_string();
    let first = attach_subscription_consumer(
        &mut runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted(first_consumer, "cursor-a"),
        attachment_budget(),
    )
    .unwrap();
    let first_attachment_counter_digest = runtime.counters().counter_projection().label().to_string();
    let second = attach_subscription_consumer(
        &mut runtime,
        &joined,
        SubscriptionConsumerAttachmentRequest::admitted(second_consumer, "cursor-b"),
        attachment_budget(),
    )
    .unwrap();
    let second_attachment_counter_digest = runtime.counters().counter_projection().label().to_string();
    let first_window = open_query_delivery_window(&mut runtime, &first, delivery_budget()).unwrap();
    let first_window_counter_digest = runtime.counters().counter_projection().label().to_string();
    let second_window =
        open_query_delivery_window(&mut runtime, &second, delivery_budget()).unwrap();
    let second_window_counter_digest = runtime.counters().counter_projection().label().to_string();
    let first_delta = QuerySubscriptionMaintenanceDelta::admitted_with_scope_label(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        first.lane_digest().clone(),
        "shared-manager",
        MaintenanceDeltaWidth::measured(1),
    );
    let (first_delta, first_lowering, first_lowering_counters) =
        lower_query_subscription_maintenance_delta(first_delta).unwrap();
    let second_delta = QuerySubscriptionMaintenanceDelta::admitted_with_scope_label(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        second.lane_digest().clone(),
        "shared-manager",
        MaintenanceDeltaWidth::measured(1),
    );
    let (second_delta, second_lowering, second_lowering_counters) =
        lower_query_subscription_maintenance_delta(second_delta).unwrap();
    let first_digest = first.attachment_projection().label().to_string();
    let second_digest = second.attachment_projection().label().to_string();
    let first_evidence = deliver_to_attachment(
        &mut runtime,
        first,
        first_window,
        first_delta,
        first_lowering,
        ActiveDeliveryDensityPosture::SparseDelta,
        1,
        2,
        1,
        0,
        0,
        1,
        ActiveSubscriptionAllocationPosture::PatchScratch,
    );
    let second_evidence = deliver_to_attachment(
        &mut runtime,
        second,
        second_window,
        second_delta,
        second_lowering,
        ActiveDeliveryDensityPosture::SparseDelta,
        1,
        2,
        1,
        0,
        0,
        1,
        ActiveSubscriptionAllocationPosture::PatchScratch,
    );
    let mut bundle = lifecycle_lane(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        "shared-manager",
        1,
        0,
    );
    bundle.active_lane_digest = handle.lane_projection().label().to_string();
    bundle.active_lane_handle_digest = digest_parts(&[
        format!("opened:{}", handle.lane_projection().label()),
        format!("joined:{}", joined.lane_projection().label()),
        format!("same_lane:{}", handle.lane_digest() == joined.lane_digest()),
    ]);
    bundle.consumer_attachment_digest = digest_parts(&[first_digest, second_digest]);
    bundle.subscription_performance_receipt_digest = digest_parts(&[
        first_evidence.performance_receipt_digest,
        second_evidence.performance_receipt_digest,
    ]);
    bundle.acknowledgement_frontier_digest = digest_parts(&[
        first_evidence.acknowledgement_frontier_digest,
        second_evidence.acknowledgement_frontier_digest,
    ]);
    bundle.delivery_window_digest = digest_parts(&[
        first_evidence.delivery_window_digest,
        second_evidence.delivery_window_digest,
    ]);
    bundle.maintenance_delta_digest = digest_parts(&[
        first_evidence.maintenance_delta_digest,
        second_evidence.maintenance_delta_digest,
    ]);
    bundle.active_delivery_work_packet_digest = digest_parts(&[
        first_evidence.active_delivery_work_packet_digest,
        second_evidence.active_delivery_work_packet_digest,
    ]);
    bundle.delivery_batch_digest = digest_parts(&[
        first_evidence.delivery_batch_digest,
        second_evidence.delivery_batch_digest,
    ]);
    bundle.patch_group_digest = digest_parts(&[
        first_evidence.patch_group_digest,
        second_evidence.patch_group_digest,
    ]);
    bundle.delivery_receipt_digest = digest_parts(&[
        first_evidence.delivery_receipt_digest,
        second_evidence.delivery_receipt_digest,
    ]);
    let sharing_counter_evidence = vec![
        format!("open:{open_counter_digest}"),
        format!("join:{join_counter_digest}"),
        format!("first_attach:{first_attachment_counter_digest}"),
        format!("second_attach:{second_attachment_counter_digest}"),
        format!("first_window:{first_window_counter_digest}"),
        format!("second_window:{second_window_counter_digest}"),
        format!("first_lowering:{}", first_lowering_counters.counter_projection().label()),
        format!("second_lowering:{}", second_lowering_counters.counter_projection().label()),
        format!("first_packet:{}", first_evidence.work_packet_counter_digest),
        format!(
            "second_packet:{}",
            second_evidence.work_packet_counter_digest
        ),
        format!("first_batch:{}", first_evidence.batch_counter_digest),
        format!("second_batch:{}", second_evidence.batch_counter_digest),
        format!("first_ack:{}", first_evidence.ack_counter_digest),
        format!("second_ack:{}", second_evidence.ack_counter_digest),
        bundle.counter_snapshot,
        format!("shared_lane:{}", joined.lane_projection().label()),
        "same_lane:true".to_string(),
        "consumer_local_delivery_count:2".to_string(),
    ];
    bundle.counter_snapshot = digest_parts(&sharing_counter_evidence);
    bundle.counter_evidence = sharing_counter_evidence;
    bundle
}

fn preview_discard_lane() -> SubscriptionLifecycleCertificationBundle {
    let live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::FacadeLive,
    );
    let selection = select_query_subscription_family(live.clone(), work_budget()).unwrap();
    let context = SubscriptionLifecycleCertificationContext::from_live_selection(&live, &selection);
    let declaration = declare_query_subscription(selection, slice_budget()).unwrap();
    let lowering = lower_query_subscription_to_bridge(declaration, lowering_budget()).unwrap();
    let admission = admit_query_subscription(lowering, admission_budget()).unwrap();
    let activation = prepare_subscription_activation(admission.clone());
    let scale_report = scale_slope_report(&activation, 1, 0);
    let mut runtime = ActiveSubscriptionRuntime::new();
    let active_admission =
        admit_active_subscription_lane(activation.clone(), active_budget()).unwrap();
    let handle = open_active_subscription_lane(&mut runtime, active_admission.clone()).unwrap();
    let attachment = attach_subscription_consumer(
        &mut runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted("preview-consumer", "cursor"),
        attachment_budget(),
    )
    .unwrap();
    let window = open_query_delivery_window(&mut runtime, &attachment, delivery_budget()).unwrap();
    let delta = QuerySubscriptionMaintenanceDelta::admitted_with_scope_label(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        attachment.lane_digest().clone(),
        "preview",
        MaintenanceDeltaWidth::measured(1),
    );
    let (delta, lowering_report, lowering_counters) =
        lower_query_subscription_maintenance_delta(delta).unwrap();
    let evidence = deliver_to_attachment(
        &mut runtime,
        attachment.clone(),
        window,
        delta.clone(),
        lowering_report.clone(),
        ActiveDeliveryDensityPosture::SparseDelta,
        1,
        1,
        1,
        0,
        0,
        1,
        ActiveSubscriptionAllocationPosture::PatchScratch,
    );
    let residue_report = measure_preview_subscription_residue(
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(1),
    );
    let isolation = admit_preview_subscription_isolation(
        &evidence.acknowledged_attachment,
        "preview-epoch",
        PreviewResidueWidth::measured(2),
    )
    .unwrap();
    let discard_closeout =
        discard_preview_subscription(isolation.clone(), residue_report.clone()).unwrap();
    let lifecycle_closeout = close_subscription_lifecycle(
        &mut runtime,
        &handle,
        SubscriptionLifecycleCloseRequest::PreviewDiscard(discard_closeout.clone()),
    )
    .unwrap();
    let mut bundle = shipped_bundle(
        certify_subscription_lifecycle(
            context,
            &admission,
            &activation,
            &scale_report,
            &active_admission,
            &handle,
            &attachment,
            evidence.delivery_batch.delivery_window_identity(),
            &delta,
            &lowering_report,
            &evidence.work_packet,
            &evidence.delivery_batch,
            &evidence.acknowledged_attachment,
            None,
            SubscriptionLifecyclePreviewCertification::Discard {
                isolation: &isolation,
                residue_report: &residue_report,
                discard_closeout: &discard_closeout,
            },
            &lifecycle_closeout,
        )
        .unwrap(),
    );
    let mut counter_evidence = bundle.counter_evidence.clone();
    counter_evidence.push(format!("lowering:{}", lowering_counters.counter_projection().label()));
    counter_evidence.extend([
        "authoritative_routing_residue:0".to_string(),
        "authoritative_checkpoint_residue:0".to_string(),
        "authoritative_replay_residue:0".to_string(),
        "authoritative_diagnostics_residue:0".to_string(),
        "authoritative_writeback_residue:0".to_string(),
    ]);
    bundle.counter_snapshot = digest_parts(&counter_evidence);
    bundle.counter_evidence = counter_evidence;
    bundle
}

fn preview_promotion_lane() -> SubscriptionLifecycleCertificationBundle {
    let live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::FacadeLive,
    );
    let selection = select_query_subscription_family(live.clone(), work_budget()).unwrap();
    let context = SubscriptionLifecycleCertificationContext::from_live_selection(&live, &selection);
    let declaration = declare_query_subscription(selection, slice_budget()).unwrap();
    let lowering = lower_query_subscription_to_bridge(declaration, lowering_budget()).unwrap();
    let admission = admit_query_subscription(lowering, admission_budget()).unwrap();
    let activation = prepare_subscription_activation(admission.clone());
    let scale_report = scale_slope_report(&activation, 1, 0);
    let mut runtime = ActiveSubscriptionRuntime::new();
    let active_admission =
        admit_active_subscription_lane(activation.clone(), active_budget()).unwrap();
    let handle = open_active_subscription_lane(&mut runtime, active_admission.clone()).unwrap();
    let attachment = attach_subscription_consumer(
        &mut runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted("preview-consumer", "cursor"),
        attachment_budget(),
    )
    .unwrap();
    let window = open_query_delivery_window(&mut runtime, &attachment, delivery_budget()).unwrap();
    let delta = QuerySubscriptionMaintenanceDelta::admitted_with_scope_label(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        attachment.lane_digest().clone(),
        "preview-promotion",
        MaintenanceDeltaWidth::measured(1),
    );
    let (delta, lowering_report, lowering_counters) =
        lower_query_subscription_maintenance_delta(delta).unwrap();
    let evidence = deliver_to_attachment(
        &mut runtime,
        attachment.clone(),
        window,
        delta.clone(),
        lowering_report.clone(),
        ActiveDeliveryDensityPosture::SparseDelta,
        1,
        1,
        1,
        0,
        0,
        1,
        ActiveSubscriptionAllocationPosture::PatchScratch,
    );
    let residue_report = measure_preview_subscription_residue(
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(0),
    );
    let isolation = admit_preview_subscription_isolation(
        &evidence.acknowledged_attachment,
        "preview-epoch-promotion",
        PreviewResidueWidth::measured(1),
    )
    .unwrap();
    let authoritative = active_attachment(&mut runtime).0;
    let handoff = promote_preview_subscription(
        isolation.clone(),
        &residue_report,
        &authoritative,
        "authority",
    )
    .unwrap();
    let lifecycle_closeout = close_subscription_lifecycle(
        &mut runtime,
        &handle,
        SubscriptionLifecycleCloseRequest::PreviewPromotion(handoff.clone()),
    )
    .unwrap();
    let mut bundle = shipped_bundle(
        certify_subscription_lifecycle(
            context,
            &admission,
            &activation,
            &scale_report,
            &active_admission,
            &handle,
            &attachment,
            evidence.delivery_batch.delivery_window_identity(),
            &delta,
            &lowering_report,
            &evidence.work_packet,
            &evidence.delivery_batch,
            &evidence.acknowledged_attachment,
            None,
            SubscriptionLifecyclePreviewCertification::Promotion {
                isolation: &isolation,
                residue_report: &residue_report,
                promotion_handoff: &handoff,
            },
            &lifecycle_closeout,
        )
        .unwrap(),
    );
    let mut counter_evidence = bundle.counter_evidence.clone();
    counter_evidence.push(format!("lowering:{}", lowering_counters.counter_projection().label()));
    bundle.counter_snapshot = digest_parts(&counter_evidence);
    bundle.counter_evidence = counter_evidence;
    bundle
}

fn masked_sharing_rejection() -> MilestoneNineTwoRejectionBundle {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let source = activation_with_context("policy-unmasked");
    let foreign = activation_with_context("policy-masked");
    let admission = admit_active_subscription_lane(source, active_budget()).unwrap();
    let open_handle = open_active_subscription_lane(&mut runtime, admission).unwrap();
    let foreign_admission = admit_active_subscription_lane(foreign, active_budget()).unwrap();
    let error =
        join_active_subscription_lane(&mut runtime, &open_handle, foreign_admission).unwrap_err();
    rejection(
        MilestoneNineTwoFailureClass::ActiveLifecycleDenied,
        error.denial_kind().as_str(),
        error.source_projection().label(),
        error.counters().counter_projection().label().to_string(),
    )
}

fn raw_cdc_rejection() -> MilestoneNineTwoRejectionBundle {
    let error = deny_raw_cdc_delivery_fallback("raw-cdc").unwrap_err();
    rejection(
        MilestoneNineTwoFailureClass::DeliveryDenied,
        error.denial_kind().as_str(),
        error.source_projection().label(),
        error.counters().counter_projection().label().to_string(),
    )
}

fn raw_bridge_rejection() -> MilestoneNineTwoRejectionBundle {
    let error = deny_raw_bridge_invalidation_delivery("raw-bridge").unwrap_err();
    rejection(
        MilestoneNineTwoFailureClass::DeliveryDenied,
        error.denial_kind().as_str(),
        error.source_projection().label(),
        error.counters().counter_projection().label().to_string(),
    )
}

fn preview_sharing_rejection() -> MilestoneNineTwoRejectionBundle {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let (handle, attachment) = active_attachment(&mut runtime);
    let isolation = admit_preview_subscription_isolation(
        &attachment,
        "preview-epoch",
        PreviewResidueWidth::measured(1),
    )
    .unwrap();
    let error = deny_preview_authoritative_sharing(&isolation, &handle).unwrap_err();
    rejection(
        MilestoneNineTwoFailureClass::PreviewIsolationDenied,
        error.denial_kind().as_str(),
        error.source_projection().label(),
        error.counters().counter_projection().label().to_string(),
    )
}

fn preview_residue_rejection() -> MilestoneNineTwoRejectionBundle {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let (_, attachment) = active_attachment(&mut runtime);
    let isolation = admit_preview_subscription_isolation(
        &attachment,
        "preview-epoch",
        PreviewResidueWidth::measured(2),
    )
    .unwrap();
    let residue = measure_preview_subscription_residue(
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(0),
    );
    let error = discard_preview_subscription(isolation, residue).unwrap_err();
    rejection(
        MilestoneNineTwoFailureClass::PreviewIsolationDenied,
        error.denial_kind().as_str(),
        error.source_projection().label(),
        error.counters().counter_projection().label().to_string(),
    )
}

fn dense_refresh_rejection() -> MilestoneNineTwoRejectionBundle {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let (_, attachment) = active_attachment(&mut runtime);
    let delta = QuerySubscriptionMaintenanceDelta::admitted_with_scope_label(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        attachment.lane_digest().clone(),
        "dense",
        MaintenanceDeltaWidth::measured(1),
    );
    let (delta, lowering_report, _) = lower_query_subscription_maintenance_delta(delta).unwrap();
    let error = build_active_delivery_work_packet(
        &mut runtime,
        &attachment,
        delta,
        lowering_report,
        ActiveDeliveryDensityPosture::DenseRefreshDenied,
        ActiveDeliveryAffectedLaneWidth::measured(1),
        ActiveDeliveryAffectedAttachmentWidth::measured(1),
        PatchGroupWidth::measured(1),
        ActiveDeliveryContinuationWidth::measured(0),
        ActiveDeliveryPreviewResidueWidth::measured(0),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::PatchScratch,
    )
    .unwrap_err();
    rejection(
        MilestoneNineTwoFailureClass::DeliveryDenied,
        error.denial_kind().as_str(),
        error.source_projection().label(),
        error.counters().counter_projection().label().to_string(),
    )
}

fn store_backed_restart_rejection() -> MilestoneNineTwoRejectionBundle {
    let activation = activation_for(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::FacadeLive,
    );
    let error = admit_active_subscription_lane(
        activation,
        active_budget().with_store_backed_restart_request(),
    )
    .unwrap_err();
    rejection(
        MilestoneNineTwoFailureClass::ActiveLifecycleDenied,
        error.denial_kind().as_str(),
        error.source_projection().label(),
        error.counters().counter_projection().label().to_string(),
    )
}

fn rejection(
    failure_class: MilestoneNineTwoFailureClass,
    failure_kind: &str,
    source_digest: &str,
    counter_snapshot: String,
) -> MilestoneNineTwoRejectionBundle {
    let failure_digest = digest_parts(&[
        format!("failure_class:{failure_class:?}"),
        format!("failure_kind:{failure_kind}"),
        format!("source:{source_digest}"),
        format!("counters:{counter_snapshot}"),
    ]);
    MilestoneNineTwoRejectionBundle {
        failure_class,
        failure_kind: failure_kind.to_string(),
        lifecycle_denial_digest: digest_parts(&[
            source_digest.to_string(),
            failure_kind.to_string(),
            counter_snapshot.clone(),
        ]),
        failure_digest,
        counter_snapshot,
    }
}

fn activation_for(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    construction_source: QuerySubscriptionConstructionSource,
) -> SubscriptionActivationInput {
    let live = LiveQueryAdmissionArtifact::for_test(live_family, view_family, construction_source);
    activation_from_live(live)
}

fn activation_with_context(policy: &str) -> SubscriptionActivationInput {
    let live = LiveQueryAdmissionArtifact::for_test_with_context(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::FacadeLive,
        crate::subscription::QuerySubscriptionBasisPosture::CurrentHead,
        crate::subscription::QuerySubscriptionFutureSelection::ordinary(),
        Some(policy.to_string()),
        Some("tenant".to_string()),
        Some("relationship-proof".to_string()),
        crate::subscription::QuerySubscriptionRelationshipProofPosture::Admitted,
    );
    activation_from_live(live)
}

fn activation_from_live(live: LiveQueryAdmissionArtifact) -> SubscriptionActivationInput {
    let selection = select_query_subscription_family(live, work_budget()).unwrap();
    let declaration = declare_query_subscription(selection, slice_budget()).unwrap();
    let lowering = lower_query_subscription_to_bridge(declaration, lowering_budget()).unwrap();
    let admission = admit_query_subscription(lowering, admission_budget()).unwrap();
    prepare_subscription_activation(admission)
}

fn active_attachment(
    runtime: &mut ActiveSubscriptionRuntime,
) -> (
    crate::subscription::ActiveSubscriptionLaneHandle,
    crate::subscription::SubscriptionConsumerAttachment,
) {
    let activation = activation_for(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::FacadeLive,
    );
    let admission = admit_active_subscription_lane(activation, active_budget()).unwrap();
    let handle = open_active_subscription_lane(runtime, admission).unwrap();
    let attachment = attach_subscription_consumer(
        runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted("preview-consumer", "cursor"),
        attachment_budget(),
    )
    .unwrap();
    (handle, attachment)
}

fn scale_slope_report(
    activation: &SubscriptionActivationInput,
    patch_width: u64,
    continuation_width: u64,
) -> crate::subscription::QuerySubscriptionScaleSlopeReport {
    let row_factor = 10 + patch_width + continuation_width;
    certify_query_subscription_scale_slope(
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Small,
            row_factor,
            activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Medium,
            row_factor * 10,
            activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Large,
            row_factor * 100,
            activation,
        ),
    )
    .unwrap()
}

fn scale_axis_evidence(patch_width: u64, continuation_width: u64) -> Vec<String> {
    vec![
        "scale_axis:unrelated_row_count".to_string(),
        "scale_axis:active_lane_count".to_string(),
        "scale_axis:consumers_per_lane".to_string(),
        format!("scale_axis:patch_width:{patch_width}"),
        "scale_axis:group_count".to_string(),
        "scale_axis:delivery_window_width:3".to_string(),
        format!("scale_axis:continuation_remap_width:{continuation_width}"),
        "scale_axis:preview_residue_width:0".to_string(),
        "scale_axis:allocation_scope_width:1".to_string(),
    ]
}

fn compile_fail_boundary_digest() -> String {
    let mut parts = MILESTONE_NINE_TWO_REQUIRED_COMPILE_FAIL_TARGETS
        .iter()
        .flat_map(|target| {
            [
                format!("target:{target}"),
                format!(
                    "stderr:{}",
                    target.trim_end_matches(".rs").to_string() + ".stderr"
                ),
            ]
        })
        .collect::<Vec<_>>();
    parts.sort();
    digest_parts(&parts)
}

fn work_budget() -> QuerySubscriptionWorkBudget {
    QuerySubscriptionWorkBudget::scratch_buffer_only(8, 8, 8, 64, 1)
}

fn slice_budget() -> QuerySubscriptionSliceBudget {
    QuerySubscriptionSliceBudget::scratch_buffer_only(8, 8, 8, 8, 8, 8, 8, 8)
}

fn lowering_budget() -> QuerySubscriptionBridgeLoweringBudget {
    QuerySubscriptionBridgeLoweringBudget::admitted(1, 8, 8, 1, 1)
}

fn admission_budget() -> QuerySubscriptionAdmissionBudget {
    QuerySubscriptionAdmissionBudget::admitted(1, 1, 1, 1, 1)
}

fn active_budget() -> ActiveSubscriptionWorkBudget {
    ActiveSubscriptionWorkBudget::admitted(
        ActiveRegistryLookupWidth::measured(1),
        ActiveFanoutWidth::measured(2),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPolicy::LifecycleArena,
    )
}

fn attachment_budget() -> SubscriptionConsumerAttachmentBudget {
    SubscriptionConsumerAttachmentBudget::admitted(
        ActiveFanoutWidth::measured(2),
        ConsumerDeliveryPacingWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        DeliveryBackpressurePolicy::RetainWithinWindow,
    )
}

fn delivery_budget() -> QueryDeliveryWindowBudget {
    QueryDeliveryWindowBudget::admitted(
        DeliveryWindowWidth::measured(3),
        PatchGroupWidth::measured(3),
        MaintenanceDeltaWidth::measured(3),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    )
}
