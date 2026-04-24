use super::*;
use crate::live::LiveQueryFamily;
use crate::view_shape_live::LiveViewShapeFamily;

#[test]
fn runtime_family_certification_closes_with_admitted_and_hostile_family_coverage() {
    let artifacts = runtime_certification_artifacts_for(LiveQueryFamily::Detail, None);
    let hostile = hostile_support_failure_for(&artifacts);

    let admitted_row = QuerySubscriptionFamilyCoverageRow::admitted(
        artifacts.declaration.family(),
        &artifacts.support_report,
        &artifacts.parity_explanation,
        &artifacts.lifecycle_bundle,
        &artifacts.admitted_bundle,
        QuerySubscriptionLifecycleCoverageClass::LifecycleCloseout,
    )
    .unwrap();
    let hostile_row = QuerySubscriptionFamilyCoverageRow::hostile(
        artifacts.declaration.family(),
        &artifacts.support_report,
        &artifacts.parity_explanation,
        &artifacts.lifecycle_bundle,
        &hostile.denied_bundle,
        &hostile.failure,
        QuerySubscriptionLifecycleCoverageClass::ActiveLifecycle,
    )
    .unwrap();
    let matrix = build_query_subscription_family_coverage_matrix(vec![admitted_row, hostile_row]);
    let handle = build_certified_family_coverage_handle(
        &matrix,
        artifacts.declaration.family(),
        CoverageResolutionPosture::IndexedCoverageSet,
    )
    .unwrap();
    let support_report_digest = artifacts.support_report.report_digest().to_string();
    let bridge_parity_digest = artifacts
        .parity_explanation
        .explanation_digest()
        .to_string();
    let diagnostic_bundle_digest = artifacts.admitted_bundle.bundle_digest().to_string();
    let lifecycle_certification_digest = artifacts
        .lifecycle_bundle
        .certification_bundle_digest()
        .to_string();
    let scope = build_query_subscription_runtime_certification_scope(
        artifacts.support_report,
        artifacts.parity_explanation,
        artifacts.admitted_bundle,
        artifacts.lifecycle_bundle,
        handle,
    )
    .unwrap();

    let (bundle, receipt) = certify_query_subscription_runtime_family(scope).unwrap();

    assert_eq!(
        receipt.coverage_resolution_posture(),
        &CoverageResolutionPosture::IndexedCoverageSet
    );
    assert_eq!(receipt.family_coverage_index_lookup_count(), 1);
    assert_eq!(receipt.covered_row_width().admitted_row_count(), 1);
    assert_eq!(receipt.covered_row_width().hostile_row_count(), 1);
    assert_eq!(receipt.uncovered_variation_width(), 0);
    assert_eq!(bundle.counters().certified_family_count(), 1);
    assert_eq!(bundle.counters().hostile_row_coverage_count(), 1);
    assert_eq!(bundle.support_report_digest(), support_report_digest);
    assert_eq!(bundle.bridge_parity_digest(), bridge_parity_digest);
    assert_eq!(bundle.diagnostic_bundle_digest(), diagnostic_bundle_digest);
    assert_eq!(
        bundle.lifecycle_certification_digest(),
        lifecycle_certification_digest
    );
}

#[test]
fn runtime_family_certification_denies_without_hostile_family_coverage() {
    let artifacts = runtime_certification_artifacts_for(LiveQueryFamily::Detail, None);
    let admitted_row = QuerySubscriptionFamilyCoverageRow::admitted(
        artifacts.declaration.family(),
        &artifacts.support_report,
        &artifacts.parity_explanation,
        &artifacts.lifecycle_bundle,
        &artifacts.admitted_bundle,
        QuerySubscriptionLifecycleCoverageClass::LifecycleCloseout,
    )
    .unwrap();
    let matrix = build_query_subscription_family_coverage_matrix(vec![admitted_row]);
    let handle = build_certified_family_coverage_handle(
        &matrix,
        artifacts.declaration.family(),
        CoverageResolutionPosture::IndexedCoverageSet,
    )
    .unwrap();
    let scope = build_query_subscription_runtime_certification_scope(
        artifacts.support_report,
        artifacts.parity_explanation,
        artifacts.admitted_bundle,
        artifacts.lifecycle_bundle,
        handle,
    )
    .unwrap();

    let error = certify_query_subscription_runtime_family(scope).unwrap_err();

    assert_eq!(
        error.error_kind(),
        &QuerySubscriptionRuntimeCertificationErrorKind::MissingHostileCoverage
    );
    assert_eq!(error.counters().uncovered_family_denial_count(), 1);
}

#[test]
fn runtime_family_coverage_handle_rejects_denied_matrix_scan_posture() {
    let artifacts = runtime_certification_artifacts_for(LiveQueryFamily::Detail, None);
    let admitted_row = QuerySubscriptionFamilyCoverageRow::admitted(
        artifacts.declaration.family(),
        &artifacts.support_report,
        &artifacts.parity_explanation,
        &artifacts.lifecycle_bundle,
        &artifacts.admitted_bundle,
        QuerySubscriptionLifecycleCoverageClass::LifecycleCloseout,
    )
    .unwrap();
    let matrix = build_query_subscription_family_coverage_matrix(vec![admitted_row]);

    let error = build_certified_family_coverage_handle(
        &matrix,
        artifacts.declaration.family(),
        CoverageResolutionPosture::MatrixScanDenied,
    )
    .unwrap_err();

    assert_eq!(
        error.error_kind(),
        &QuerySubscriptionRuntimeCertificationErrorKind::CoverageResolutionDenied
    );
}

#[test]
fn runtime_certification_scope_rejects_cross_family_coverage_handles() {
    let detail = runtime_certification_artifacts_for(LiveQueryFamily::Detail, None);
    let collection = runtime_certification_artifacts_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let collection_hostile = hostile_support_failure_for(&collection);
    let collection_matrix = build_query_subscription_family_coverage_matrix(vec![
        QuerySubscriptionFamilyCoverageRow::admitted(
            collection.declaration.family(),
            &collection.support_report,
            &collection.parity_explanation,
            &collection.lifecycle_bundle,
            &collection.admitted_bundle,
            QuerySubscriptionLifecycleCoverageClass::LifecycleCloseout,
        )
        .unwrap(),
        QuerySubscriptionFamilyCoverageRow::hostile(
            collection.declaration.family(),
            &collection.support_report,
            &collection.parity_explanation,
            &collection.lifecycle_bundle,
            &collection_hostile.denied_bundle,
            &collection_hostile.failure,
            QuerySubscriptionLifecycleCoverageClass::ActiveLifecycle,
        )
        .unwrap(),
    ]);
    let collection_handle = build_certified_family_coverage_handle(
        &collection_matrix,
        collection.declaration.family(),
        CoverageResolutionPosture::IndexedCoverageSet,
    )
    .unwrap();

    let error = build_query_subscription_runtime_certification_scope(
        detail.support_report,
        detail.parity_explanation,
        detail.admitted_bundle,
        detail.lifecycle_bundle,
        collection_handle,
    )
    .unwrap_err();

    assert_eq!(
        error.error_kind(),
        &QuerySubscriptionRuntimeCertificationErrorKind::ScopeFamilyMismatch
    );
}

#[test]
fn runtime_family_certification_rejects_non_runtime_support_subjects() {
    let artifacts = runtime_certification_artifacts_for(LiveQueryFamily::Detail, None);
    let hostile = hostile_support_failure_for(&artifacts);
    let declaration_support = report_query_subscription_support(
        QuerySubscriptionSupportSubject::declaration(&artifacts.declaration),
        QuerySubscriptionSupportEvidence::declaration(&artifacts.declaration),
    )
    .unwrap()
    .0;

    let error = QuerySubscriptionFamilyCoverageRow::admitted(
        artifacts.declaration.family(),
        &declaration_support,
        &artifacts.parity_explanation,
        &artifacts.lifecycle_bundle,
        &artifacts.admitted_bundle,
        QuerySubscriptionLifecycleCoverageClass::LifecycleCloseout,
    )
    .unwrap_err();

    assert_eq!(
        error.error_kind(),
        &QuerySubscriptionRuntimeCertificationErrorKind::CertificationSupportClassDenied
    );

    let hostile_error = QuerySubscriptionFamilyCoverageRow::hostile(
        artifacts.declaration.family(),
        &declaration_support,
        &artifacts.parity_explanation,
        &artifacts.lifecycle_bundle,
        &hostile.denied_bundle,
        &hostile.failure,
        QuerySubscriptionLifecycleCoverageClass::ActiveLifecycle,
    )
    .unwrap_err();

    assert_eq!(
        hostile_error.error_kind(),
        &QuerySubscriptionRuntimeCertificationErrorKind::CertificationSupportClassDenied
    );
}

#[test]
fn runtime_family_certification_preserves_matrix_scan_debt_posture() {
    let artifacts = runtime_certification_artifacts_for(LiveQueryFamily::Detail, None);
    let hostile = hostile_support_failure_for(&artifacts);

    let admitted_row = QuerySubscriptionFamilyCoverageRow::admitted(
        artifacts.declaration.family(),
        &artifacts.support_report,
        &artifacts.parity_explanation,
        &artifacts.lifecycle_bundle,
        &artifacts.admitted_bundle,
        QuerySubscriptionLifecycleCoverageClass::LifecycleCloseout,
    )
    .unwrap();
    let hostile_row = QuerySubscriptionFamilyCoverageRow::hostile(
        artifacts.declaration.family(),
        &artifacts.support_report,
        &artifacts.parity_explanation,
        &artifacts.lifecycle_bundle,
        &hostile.denied_bundle,
        &hostile.failure,
        QuerySubscriptionLifecycleCoverageClass::ActiveLifecycle,
    )
    .unwrap();
    let matrix = build_query_subscription_family_coverage_matrix(vec![admitted_row, hostile_row]);
    let handle = build_certified_family_coverage_handle(
        &matrix,
        artifacts.declaration.family(),
        CoverageResolutionPosture::MatrixScanDebtExplicit,
    )
    .unwrap();
    let scope = build_query_subscription_runtime_certification_scope(
        artifacts.support_report,
        artifacts.parity_explanation,
        artifacts.admitted_bundle,
        artifacts.lifecycle_bundle,
        handle,
    )
    .unwrap();

    let (bundle, receipt) = certify_query_subscription_runtime_family(scope).unwrap();

    assert_eq!(
        receipt.coverage_resolution_posture(),
        &CoverageResolutionPosture::MatrixScanDebtExplicit
    );
    assert_eq!(receipt.family_coverage_index_lookup_count(), 0);
    assert_eq!(bundle.counters().family_coverage_index_lookup_count(), 0);
    assert_eq!(
        bundle.counters().family_coverage_matrix_scan_debt_count(),
        1
    );
}

#[test]
fn hostile_family_coverage_rows_reject_denied_bundles_from_foreign_proof_chains() {
    let artifacts = runtime_certification_artifacts_for(LiveQueryFamily::Detail, None);
    let foreign = runtime_certification_artifacts_for_source(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::Direct,
    );
    let foreign_hostile = hostile_support_failure_for(&foreign);

    let error = QuerySubscriptionFamilyCoverageRow::hostile(
        artifacts.declaration.family(),
        &artifacts.support_report,
        &artifacts.parity_explanation,
        &artifacts.lifecycle_bundle,
        &foreign_hostile.denied_bundle,
        &foreign_hostile.failure,
        QuerySubscriptionLifecycleCoverageClass::ActiveLifecycle,
    )
    .unwrap_err();

    assert_eq!(
        error.error_kind(),
        &QuerySubscriptionRuntimeCertificationErrorKind::ScopeSourceMismatch
    );
}

struct RuntimeCertificationArtifacts {
    selection: QuerySubscriptionFamilySelection,
    declaration: QuerySubscriptionDeclarationArtifact,
    lowering: BridgeSubscriptionLoweringPlan,
    admission: QuerySubscriptionAdmissionArtifact,
    support_report: QuerySubscriptionSupportReport,
    parity_explanation: QuerySubscriptionBridgeParityExplanation,
    lifecycle_bundle: SubscriptionLifecycleCertificationBundle,
    admitted_bundle: QuerySubscriptionAdmittedDiagnosticBundle,
}

struct HostileCoverageArtifacts {
    denied_bundle: QuerySubscriptionDeniedDiagnosticBundle,
    failure: QuerySubscriptionDiagnosticFailure,
}

fn runtime_certification_artifacts_for(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
) -> RuntimeCertificationArtifacts {
    runtime_certification_artifacts_for_source(
        live_family,
        view_family,
        QuerySubscriptionConstructionSource::FacadeLive,
    )
}

fn runtime_certification_artifacts_for_source(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    source: QuerySubscriptionConstructionSource,
) -> RuntimeCertificationArtifacts {
    let live = LiveQueryAdmissionArtifact::for_test(live_family, view_family, source);
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
    let witness =
        build_query_subscription_manual_bridge_witness(&declaration, &lowering, &activation)
            .unwrap();
    let parity_explanation =
        explain_query_subscription_bridge_parity(&declaration, &lowering, &activation, witness)
            .unwrap()
            .0;
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
        SubscriptionConsumerAttachmentRequest::admitted("runtime-certification", "cursor"),
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
    let delta = QuerySubscriptionMaintenanceDelta::admitted(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        attachment.lane_digest().clone(),
        "affected-scope",
        MaintenanceDeltaWidth::measured(1),
    );
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
        None,
        SubscriptionLifecyclePreviewCertification::None,
        &closeout,
    )
    .unwrap();
    let support_report = report_query_subscription_support(
        QuerySubscriptionSupportSubject::active_lifecycle(&declaration, &active_admission),
        QuerySubscriptionSupportEvidence::admission(&declaration, &admission).unwrap(),
    )
    .unwrap()
    .0;
    let admitted_trace = trace_admitted_query_subscription_diagnostics(
        &selection,
        &declaration,
        &lowering,
        &admission,
        &support_report,
        &lifecycle_bundle,
        None,
        None,
        Some(&closeout),
    )
    .unwrap();
    let admitted_bundle = bundle_admitted_query_subscription_diagnostics(
        admitted_trace,
        &selection,
        &declaration,
        &lowering,
        &admission,
        &support_report,
        &lifecycle_bundle,
        None,
        None,
        Some(&closeout),
    )
    .unwrap()
    .0;

    RuntimeCertificationArtifacts {
        selection,
        declaration,
        lowering,
        admission,
        support_report,
        parity_explanation,
        lifecycle_bundle,
        admitted_bundle,
    }
}

fn hostile_support_failure_for(
    artifacts: &RuntimeCertificationArtifacts,
) -> HostileCoverageArtifacts {
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
        None,
        failure.clone(),
    )
    .unwrap();
    let denied_bundle = bundle_denied_query_subscription_diagnostics(
        denied_trace,
        &selection_context,
        Some(&artifacts.declaration),
        Some(&artifacts.lowering),
        Some(&artifacts.admission),
        None,
        failure.clone(),
    )
    .unwrap()
    .0;

    HostileCoverageArtifacts {
        denied_bundle,
        failure,
    }
}
