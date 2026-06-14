use super::*;
use crate::live::LiveQueryFamily;
use crate::subscription::posture::QuerySubscriptionBasisPosture;
use crate::view_shape_live::LiveViewShapeFamily;

#[test]
fn admitted_diagnostic_bundle_carries_offline_semantic_labels_and_canonical_digests() {
    let artifacts = runtime_artifacts_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
        0,
    );
    let support_subject = QuerySubscriptionSupportSubject::active_lifecycle(
        &artifacts.declaration,
        &artifacts.admission,
        &artifacts.active_admission,
    );
    let support_evidence =
        QuerySubscriptionSupportEvidence::admission(&artifacts.declaration, &artifacts.admission)
            .unwrap();
    let (support_report, _) =
        report_query_subscription_support(support_subject, support_evidence).unwrap();
    let trace = trace_admitted_query_subscription_diagnostics(
        &artifacts.selection,
        &artifacts.declaration,
        &artifacts.lowering,
        &artifacts.admission,
        &support_report,
        &artifacts.lifecycle_bundle,
        artifacts.continuation_report.as_ref(),
        None,
        Some(&artifacts.closeout),
    )
    .unwrap();

    let (bundle, receipt) = bundle_admitted_query_subscription_diagnostics(
        trace,
        &artifacts.selection,
        &artifacts.declaration,
        &artifacts.lowering,
        &artifacts.admission,
        &support_report,
        &artifacts.lifecycle_bundle,
        artifacts.continuation_report.as_ref(),
        None,
        Some(&artifacts.closeout),
    )
    .unwrap();

    assert_eq!(
        bundle.semantic_labels().query_family_label(),
        artifacts.selection.family().as_str()
    );
    assert_eq!(
        bundle.semantic_labels().declaration_family_label(),
        artifacts.declaration.family().as_str()
    );
    assert_eq!(
        bundle.semantic_labels().bridge_family_label(),
        artifacts.lowering.bridge_family().as_str()
    );
    assert_eq!(
        bundle.semantic_labels().basis_posture_label(),
        artifacts.lowering.basis_request().request_kind().as_str()
    );
    assert_eq!(
        bundle.semantic_labels().signal_strategy_class_label(),
        artifacts
            .lowering
            .signal_strategy_request()
            .request_kind()
            .as_str()
    );
    assert_eq!(
        bundle.semantic_labels().support_posture_label(),
        support_report.support_posture().as_str()
    );
    assert_eq!(
        bundle.semantic_labels().denial_or_coverage_class_label(),
        "runtime_lifecycle_certified"
    );
    assert_eq!(
        bundle.support_report_digest(),
        support_report.report_digest()
    );
    assert_eq!(
        bundle.lifecycle_certification_digest(),
        artifacts.lifecycle_bundle.certification_bundle_for_reporting()
    );
    assert_eq!(
        bundle.lifecycle_closeout_digest(),
        Some(artifacts.closeout.closeout_for_reporting())
    );
    assert_eq!(
        receipt.bundle_assembly_posture(),
        &BundleAssemblyPosture::ComposedFromCanonicalArtifacts
    );
    assert_eq!(receipt.stage_rederivation_count(), 0);
    assert_eq!(bundle.counters().diagnostic_bundle_emission_count(), 1);
    assert_eq!(bundle.counters().denied_bundle_emission_count(), 0);
    assert!(!bundle.bundle_digest().is_empty());
}

#[test]
fn denied_diagnostic_bundle_localizes_declaration_failure_and_omits_later_stages() {
    let live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::KanbanGrouped),
        QuerySubscriptionConstructionSource::Direct,
    );
    let selection = select_query_subscription_family(live, roomy_budget()).unwrap();
    let declaration_error = declare_query_subscription(
        selection.clone(),
        roomy_slice_budget().with_masked_slice_request_detected(),
    )
    .unwrap_err();
    let failure = QuerySubscriptionDiagnosticFailure::from_declaration_denial(&declaration_error);
    let selection_context = QuerySubscriptionDiagnosticSelectionContext::from_selection(&selection);
    let trace = trace_denied_query_subscription_diagnostics(
        &selection_context,
        None,
        None,
        None,
        None,
        failure.clone(),
    )
    .unwrap();

    let (bundle, receipt) = bundle_denied_query_subscription_diagnostics(
        trace,
        &selection_context,
        None,
        None,
        None,
        None,
        failure,
    )
    .unwrap();

    assert_eq!(
        bundle.failure().stage(),
        &QuerySubscriptionDiagnosticStage::Declaration
    );
    assert_eq!(
        bundle.semantic_labels().query_family_label(),
        selection.family().as_str()
    );
    assert_eq!(bundle.support_report_digest(), None);
    assert_eq!(
        bundle.omitted_stages(),
        &[
            QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
            QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
            QuerySubscriptionDiagnosticStage::SupportReporting,
            QuerySubscriptionDiagnosticStage::Certification,
        ]
    );
    assert_eq!(receipt.bundle_width().failure_evidence_count(), 1);
    assert_eq!(bundle.counters().denied_bundle_emission_count(), 1);
    assert!(!bundle.bundle_digest().is_empty());
}

#[test]
fn admitted_bundle_rejects_trace_with_missing_terminal_certification_stage() {
    let artifacts = runtime_artifacts_for(LiveQueryFamily::Detail, None, 0);
    let support_subject = QuerySubscriptionSupportSubject::active_lifecycle(
        &artifacts.declaration,
        &artifacts.admission,
        &artifacts.active_admission,
    );
    let support_evidence =
        QuerySubscriptionSupportEvidence::admission(&artifacts.declaration, &artifacts.admission)
            .unwrap();
    let (support_report, _) =
        report_query_subscription_support(support_subject, support_evidence).unwrap();
    let failure = QuerySubscriptionDiagnosticFailure::from_support_report_error(
        &report_query_subscription_support(
            QuerySubscriptionSupportSubject::activation(
                &artifacts.declaration,
                &prepare_subscription_activation(artifacts.admission.clone()),
            ),
            QuerySubscriptionSupportEvidence::declaration(&artifacts.declaration),
        )
        .unwrap_err(),
    );
    let selection_context =
        QuerySubscriptionDiagnosticSelectionContext::from_selection(&artifacts.selection);
    let denied_trace = trace_denied_query_subscription_diagnostics(
        &selection_context,
        Some(&artifacts.declaration),
        Some(&artifacts.lowering),
        Some(&artifacts.admission),
        Some(&support_report),
        failure,
    )
    .unwrap();

    let error = bundle_admitted_query_subscription_diagnostics(
        denied_trace,
        &artifacts.selection,
        &artifacts.declaration,
        &artifacts.lowering,
        &artifacts.admission,
        &support_report,
        &artifacts.lifecycle_bundle,
        None,
        None,
        Some(&artifacts.closeout),
    )
    .unwrap_err();

    assert_eq!(
        error.error_kind(),
        &QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage
    );
    assert_eq!(error.counters().diagnostic_missing_stage_denial_count(), 1);
}

#[test]
fn admitted_bundle_rejects_trace_with_unclaimed_optional_continuation_stage() {
    let artifacts = runtime_artifacts_for(LiveQueryFamily::Detail, None, 1);
    let support_subject = QuerySubscriptionSupportSubject::continuation(
        &artifacts.declaration,
        &artifacts.admission,
        artifacts.continuation_report.as_ref().unwrap(),
    );
    let support_evidence =
        QuerySubscriptionSupportEvidence::admission(&artifacts.declaration, &artifacts.admission)
            .unwrap();
    let (support_report, _) =
        report_query_subscription_support(support_subject, support_evidence).unwrap();
    let trace = trace_admitted_query_subscription_diagnostics(
        &artifacts.selection,
        &artifacts.declaration,
        &artifacts.lowering,
        &artifacts.admission,
        &support_report,
        &artifacts.lifecycle_bundle,
        artifacts.continuation_report.as_ref(),
        None,
        Some(&artifacts.closeout),
    )
    .unwrap();

    let error = bundle_admitted_query_subscription_diagnostics(
        trace,
        &artifacts.selection,
        &artifacts.declaration,
        &artifacts.lowering,
        &artifacts.admission,
        &support_report,
        &artifacts.lifecycle_bundle,
        None,
        None,
        Some(&artifacts.closeout),
    )
    .unwrap_err();

    assert_eq!(
        error.error_kind(),
        &QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage
    );
    assert!(error.message().contains("continuation trace evidence"));
}

#[test]
fn lifecycle_instance_churn_changes_trace_without_changing_family_semantic_labels() {
    let base = runtime_artifacts_for(LiveQueryFamily::Detail, None, 0);
    let continued = runtime_artifacts_for(LiveQueryFamily::Detail, None, 1);

    let base_support = report_query_subscription_support(
        QuerySubscriptionSupportSubject::active_lifecycle(
            &base.declaration,
            &base.admission,
            &base.active_admission,
        ),
        QuerySubscriptionSupportEvidence::admission(&base.declaration, &base.admission).unwrap(),
    )
    .unwrap()
    .0;
    let continued_support = report_query_subscription_support(
        QuerySubscriptionSupportSubject::continuation(
            &continued.declaration,
            &continued.admission,
            continued.continuation_report.as_ref().unwrap(),
        ),
        QuerySubscriptionSupportEvidence::admission(&continued.declaration, &continued.admission)
            .unwrap(),
    )
    .unwrap()
    .0;

    let base_bundle = bundle_admitted_query_subscription_diagnostics(
        trace_admitted_query_subscription_diagnostics(
            &base.selection,
            &base.declaration,
            &base.lowering,
            &base.admission,
            &base_support,
            &base.lifecycle_bundle,
            None,
            None,
            Some(&base.closeout),
        )
        .unwrap(),
        &base.selection,
        &base.declaration,
        &base.lowering,
        &base.admission,
        &base_support,
        &base.lifecycle_bundle,
        None,
        None,
        Some(&base.closeout),
    )
    .unwrap()
    .0;
    let continued_bundle = bundle_admitted_query_subscription_diagnostics(
        trace_admitted_query_subscription_diagnostics(
            &continued.selection,
            &continued.declaration,
            &continued.lowering,
            &continued.admission,
            &continued_support,
            &continued.lifecycle_bundle,
            continued.continuation_report.as_ref(),
            None,
            Some(&continued.closeout),
        )
        .unwrap(),
        &continued.selection,
        &continued.declaration,
        &continued.lowering,
        &continued.admission,
        &continued_support,
        &continued.lifecycle_bundle,
        continued.continuation_report.as_ref(),
        None,
        Some(&continued.closeout),
    )
    .unwrap()
    .0;

    assert_eq!(
        base_bundle.semantic_labels().query_family_label(),
        continued_bundle.semantic_labels().query_family_label()
    );
    assert_eq!(
        base_bundle.semantic_labels().declaration_family_label(),
        continued_bundle
            .semantic_labels()
            .declaration_family_label()
    );
    assert_ne!(
        base_bundle.trace().trace_digest(),
        continued_bundle.trace().trace_digest()
    );
    assert_ne!(
        base_bundle.bundle_digest(),
        continued_bundle.bundle_digest()
    );
}

#[test]
fn selection_denied_diagnostic_bundle_localizes_family_selection_failure_and_omits_later_stages() {
    let live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::Detail,
        Some(LiveViewShapeFamily::KanbanGrouped),
        QuerySubscriptionConstructionSource::Direct,
    );
    let selection_error =
        select_query_subscription_family(live.clone(), roomy_budget()).unwrap_err();
    let failure = QuerySubscriptionDiagnosticFailure::from_family_selection_error(&selection_error);
    let selection_context =
        QuerySubscriptionDiagnosticSelectionContext::from_selection_denial(&live, &selection_error);
    let trace = trace_denied_query_subscription_diagnostics(
        &selection_context,
        None,
        None,
        None,
        None,
        failure.clone(),
    )
    .unwrap();

    let (bundle, receipt) = bundle_denied_query_subscription_diagnostics(
        trace,
        &selection_context,
        None,
        None,
        None,
        None,
        failure,
    )
    .unwrap();

    assert_eq!(
        bundle.failure().stage(),
        &QuerySubscriptionDiagnosticStage::ViewMismatch
    );
    assert_eq!(
        bundle.semantic_labels().query_family_label(),
        "selection_unresolved:detail:kanban_grouped"
    );
    assert_eq!(
        bundle.semantic_labels().declaration_family_label(),
        "not_declared:selection_unresolved:detail:kanban_grouped"
    );
    assert_eq!(
        bundle.semantic_labels().basis_posture_label(),
        "current_head"
    );
    assert_eq!(bundle.support_report_digest(), None);
    assert_eq!(
        bundle.omitted_stages(),
        &[
            QuerySubscriptionDiagnosticStage::Declaration,
            QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
            QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
            QuerySubscriptionDiagnosticStage::SupportReporting,
            QuerySubscriptionDiagnosticStage::Certification,
        ]
    );
    assert_eq!(receipt.stage_rederivation_count(), 0);
    assert_eq!(bundle.counters().denied_bundle_emission_count(), 1);
}

#[test]
fn denied_bundle_rejects_trace_that_claims_runtime_admission_without_admission_artifact() {
    let artifacts = runtime_artifacts_for(LiveQueryFamily::Detail, None, 0);
    let failure = QuerySubscriptionDiagnosticFailure::from_support_report_error(
        &report_query_subscription_support(
            QuerySubscriptionSupportSubject::activation(
                &artifacts.declaration,
                &prepare_subscription_activation(artifacts.admission.clone()),
            ),
            QuerySubscriptionSupportEvidence::declaration(&artifacts.declaration),
        )
        .unwrap_err(),
    );
    let selection_context =
        QuerySubscriptionDiagnosticSelectionContext::from_selection(&artifacts.selection);
    let trace = trace_denied_query_subscription_diagnostics(
        &selection_context,
        Some(&artifacts.declaration),
        Some(&artifacts.lowering),
        Some(&artifacts.admission),
        None,
        failure.clone(),
    )
    .unwrap();

    let error = bundle_denied_query_subscription_diagnostics(
        trace,
        &selection_context,
        Some(&artifacts.declaration),
        Some(&artifacts.lowering),
        None,
        None,
        failure,
    )
    .unwrap_err();

    assert_eq!(
        error.error_kind(),
        &QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage
    );
    assert!(error
        .message()
        .contains("trace to carry every stage that the assembled artifacts claim"));
}

#[test]
fn selection_denied_trace_rejects_mismatched_failure_source() {
    let first_live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::Detail,
        Some(LiveViewShapeFamily::KanbanGrouped),
        QuerySubscriptionConstructionSource::Direct,
    );
    let second_live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::BoundedMaterialization,
        Some(LiveViewShapeFamily::InspectorDetailFocused),
        QuerySubscriptionConstructionSource::Direct,
    );
    let first_error =
        select_query_subscription_family(first_live.clone(), roomy_budget()).unwrap_err();
    let second_error =
        select_query_subscription_family(second_live.clone(), roomy_budget()).unwrap_err();
    let selection_context = QuerySubscriptionDiagnosticSelectionContext::from_selection_denial(
        &first_live,
        &first_error,
    );
    let failure = QuerySubscriptionDiagnosticFailure::from_family_selection_error(&second_error);

    let error = trace_denied_query_subscription_diagnostics(
        &selection_context,
        None,
        None,
        None,
        None,
        failure,
    )
    .unwrap_err();

    assert_eq!(
        error.error_kind(),
        &QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch
    );
    assert_eq!(error.counters().diagnostic_missing_stage_denial_count(), 1);
}

#[test]
fn denied_trace_preserves_runtime_backed_admission_stage_before_support_failure() {
    let artifacts = runtime_artifacts_for(LiveQueryFamily::Detail, None, 0);
    let failure = QuerySubscriptionDiagnosticFailure::from_support_report_error(
        &report_query_subscription_support(
            QuerySubscriptionSupportSubject::activation(
                &artifacts.declaration,
                &prepare_subscription_activation(artifacts.admission.clone()),
            ),
            QuerySubscriptionSupportEvidence::declaration(&artifacts.declaration),
        )
        .unwrap_err(),
    );
    let selection_context =
        QuerySubscriptionDiagnosticSelectionContext::from_selection(&artifacts.selection);

    let trace = trace_denied_query_subscription_diagnostics(
        &selection_context,
        Some(&artifacts.declaration),
        Some(&artifacts.lowering),
        Some(&artifacts.admission),
        None,
        failure,
    )
    .unwrap();

    assert_eq!(
        trace
            .stage_traces()
            .iter()
            .map(|trace| (trace.stage(), trace.outcome()))
            .collect::<Vec<_>>(),
        vec![
            (
                &QuerySubscriptionDiagnosticStage::FamilySelection,
                &QuerySubscriptionDiagnosticOutcome::Admitted,
            ),
            (
                &QuerySubscriptionDiagnosticStage::Declaration,
                &QuerySubscriptionDiagnosticOutcome::Admitted,
            ),
            (
                &QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
                &QuerySubscriptionDiagnosticOutcome::Admitted,
            ),
            (
                &QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
                &QuerySubscriptionDiagnosticOutcome::Admitted,
            ),
            (
                &QuerySubscriptionDiagnosticStage::SupportReporting,
                &QuerySubscriptionDiagnosticOutcome::Denied,
            ),
        ]
    );
}

#[test]
fn admitted_diagnostic_bundle_preserves_canonical_basis_posture_labels() {
    let artifacts = runtime_artifacts_for_with_basis(
        LiveQueryFamily::Detail,
        None,
        0,
        QuerySubscriptionBasisPosture::RuntimeHistoricalSnapshot,
    );
    let support_subject = QuerySubscriptionSupportSubject::active_lifecycle(
        &artifacts.declaration,
        &artifacts.admission,
        &artifacts.active_admission,
    );
    let support_evidence =
        QuerySubscriptionSupportEvidence::admission(&artifacts.declaration, &artifacts.admission)
            .unwrap();
    let (support_report, _) =
        report_query_subscription_support(support_subject, support_evidence).unwrap();
    let trace = trace_admitted_query_subscription_diagnostics(
        &artifacts.selection,
        &artifacts.declaration,
        &artifacts.lowering,
        &artifacts.admission,
        &support_report,
        &artifacts.lifecycle_bundle,
        artifacts.continuation_report.as_ref(),
        None,
        Some(&artifacts.closeout),
    )
    .unwrap();

    let (bundle, _) = bundle_admitted_query_subscription_diagnostics(
        trace,
        &artifacts.selection,
        &artifacts.declaration,
        &artifacts.lowering,
        &artifacts.admission,
        &support_report,
        &artifacts.lifecycle_bundle,
        artifacts.continuation_report.as_ref(),
        None,
        Some(&artifacts.closeout),
    )
    .unwrap();

    assert_eq!(
        bundle.semantic_labels().basis_posture_label(),
        QuerySubscriptionBasisPosture::RuntimeHistoricalSnapshot.as_str()
    );
}

struct RuntimeArtifacts {
    selection: QuerySubscriptionFamilySelection,
    declaration: QuerySubscriptionDeclarationArtifact,
    lowering: BridgeSubscriptionLoweringPlan,
    admission: QuerySubscriptionAdmissionArtifact,
    active_admission: ActiveSubscriptionLaneAdmission,
    lifecycle_bundle: SubscriptionLifecycleCertificationBundle,
    continuation_report: Option<SubscriptionContinuationReport>,
    closeout: SubscriptionLifecycleCloseout,
}

fn runtime_artifacts_for(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    continuation_width: u64,
) -> RuntimeArtifacts {
    runtime_artifacts_for_with_basis(
        live_family,
        view_family,
        continuation_width,
        QuerySubscriptionBasisPosture::CurrentHead,
    )
}

fn runtime_artifacts_for_with_basis(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    continuation_width: u64,
    basis_posture: QuerySubscriptionBasisPosture,
) -> RuntimeArtifacts {
    let live = LiveQueryAdmissionArtifact::for_test_with_basis(
        live_family,
        view_family,
        QuerySubscriptionConstructionSource::FacadeLive,
        basis_posture,
    );
    let selection = select_query_subscription_family(live.clone(), roomy_budget()).unwrap();
    let context = SubscriptionLifecycleCertificationContext::from_live_selection(&live, &selection);
    let declaration = declare_query_subscription(selection.clone(), roomy_slice_budget()).unwrap();
    let lowering =
        lower_query_subscription_to_bridge(declaration.clone(), roomy_lowering_budget()).unwrap();
    let admission = admit_query_subscription(
        lowering.clone(),
        QuerySubscriptionAdmissionBudget::admitted(1, 1, 1, 1, 1),
    )
    .unwrap();
    let activation = prepare_subscription_activation(admission.clone());
    let scale_report = certify_query_subscription_scale_slope(
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Small,
            10 + continuation_width,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Medium,
            (10 + continuation_width) * 10,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Large,
            (10 + continuation_width) * 100,
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
        SubscriptionConsumerAttachmentRequest::admitted("diagnostic-bundle", "cursor"),
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

    let (delta, continuation_report, window) = if continuation_width > 0 {
        let evidence = admit_subscription_continuation_evidence(
            attachment.lane_digest().clone(),
            SubscriptionContinuationClass::IdentityRemap,
            "employee:old",
            "employee:new",
            "basis:current",
            "identity-authority",
            ContinuationRemapWidth::measured(continuation_width),
        )
        .unwrap();
        let (continued_window, report) =
            apply_active_subscription_continuation(&mut runtime, window, evidence).unwrap();
        let (delta, _) = lower_subscription_continuation_report(&report);
        (delta, Some(report), continued_window)
    } else {
        (
            QuerySubscriptionMaintenanceDelta::admitted(
                QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
                attachment.lane_digest().clone(),
                "affected-scope",
                MaintenanceDeltaWidth::measured(1),
            ),
            None,
            window,
        )
    };

    let delivery_window_digest = window.delivery_window_digest().to_string();
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
        ActiveDeliveryContinuationWidth::measured(continuation_width),
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
    let lifecycle_bundle = certify_subscription_lifecycle(
        context,
        &admission,
        &activation,
        &scale_report,
        &active_admission,
        &handle,
        &attachment,
        delivery_window_digest,
        &delta,
        &lowering_report,
        &work_packet,
        &delivery_batch,
        &acknowledged_attachment,
        continuation_report.as_ref(),
        SubscriptionLifecyclePreviewCertification::None,
        &closeout,
    )
    .unwrap();

    RuntimeArtifacts {
        selection,
        declaration,
        lowering,
        admission,
        active_admission,
        lifecycle_bundle,
        continuation_report,
        closeout,
    }
}
