use super::*;
use crate::live::LiveQueryFamily;
use crate::view_shape_live::LiveViewShapeFamily;

#[test]
fn support_matrix_is_family_aware_for_all_admitted_subscription_families() {
    for (live_family, view_family, expected_family) in [
        (
            LiveQueryFamily::Detail,
            None,
            QuerySubscriptionFamily::DetailExact,
        ),
        (
            LiveQueryFamily::Detail,
            Some(LiveViewShapeFamily::InspectorDetailFocused),
            QuerySubscriptionFamily::InspectorDetailExact,
        ),
        (
            LiveQueryFamily::OrderedCollection,
            Some(LiveViewShapeFamily::Table),
            QuerySubscriptionFamily::CollectionMembership,
        ),
        (
            LiveQueryFamily::OrderedCollection,
            Some(LiveViewShapeFamily::KanbanGrouped),
            QuerySubscriptionFamily::GroupedCollectionMembership,
        ),
        (
            LiveQueryFamily::BoundedMaterialization,
            None,
            QuerySubscriptionFamily::BoundedMaterialization,
        ),
    ] {
        let declaration = declaration_for(live_family, view_family);
        let subject = QuerySubscriptionSupportSubject::declaration(&declaration);
        let evidence = QuerySubscriptionSupportEvidence::declaration(&declaration);

        let (report, lookup_receipt) =
            report_query_subscription_support(subject, evidence).unwrap();

        assert_eq!(report.support_subject().family(), &expected_family);
        assert_eq!(report.support_matrix().family(), &expected_family);
        assert_eq!(
            report.support_posture(),
            &QuerySubscriptionSupportPosture::RuntimeBackedCertified
        );
        assert_eq!(
            report
                .support_matrix()
                .capability_digest()
                .as_str()
                .is_empty(),
            false
        );
        assert_eq!(
            report
                .support_matrix()
                .row_for_class(QuerySubscriptionSupportClass::Declaration)
                .unwrap()
                .posture(),
            &QuerySubscriptionSupportPosture::RuntimeBackedCertified
        );
        assert_eq!(
            report
                .support_matrix()
                .row_for_class(QuerySubscriptionSupportClass::Activation)
                .unwrap()
                .posture(),
            &QuerySubscriptionSupportPosture::UncertifiedDenied
        );
        assert_eq!(
            report
                .support_matrix()
                .row_for_class(QuerySubscriptionSupportClass::ActiveLifecycle)
                .unwrap()
                .posture(),
            &QuerySubscriptionSupportPosture::UncertifiedDenied
        );
        assert_eq!(
            report
                .support_matrix()
                .row_for_class(QuerySubscriptionSupportClass::Continuation)
                .unwrap()
                .posture(),
            &QuerySubscriptionSupportPosture::UncertifiedDenied
        );
        assert_eq!(
            report
                .support_matrix()
                .row_for_class(QuerySubscriptionSupportClass::PreviewCloseout)
                .unwrap()
                .posture(),
            &QuerySubscriptionSupportPosture::UncertifiedDenied
        );
        assert_eq!(
            report
                .support_matrix()
                .row_for_class(QuerySubscriptionSupportClass::DurableReplay)
                .unwrap()
                .posture(),
            &QuerySubscriptionSupportPosture::RuntimeBackedDeferred
        );
        assert_eq!(
            report
                .support_matrix()
                .row_for_class(QuerySubscriptionSupportClass::StoreBackedRestart)
                .unwrap()
                .posture(),
            &QuerySubscriptionSupportPosture::RuntimeBackedDeferred
        );
        assert_eq!(
            lookup_receipt.resolution_posture(),
            &SupportResolutionPosture::IndexedFamilyLookup
        );
        assert_eq!(lookup_receipt.consumed_lookup_width(), 1);
        assert_eq!(lookup_receipt.remaining_lookup_width(), 6);
        assert_eq!(report.counters().support_report_request_count(), 1);
        assert_eq!(report.counters().supported_family_count(), 1);
        assert_eq!(report.counters().deferred_family_count(), 0);
        assert_eq!(report.counters().denied_family_count(), 0);
        assert_eq!(report.counters().uncertified_family_denial_count(), 0);
        assert_eq!(report.counters().support_matrix_emission_count(), 1);
        assert_eq!(report.counters().support_family_index_lookup_count(), 1);
        assert_eq!(report.counters().support_matrix_scan_debt_count(), 0);
        assert_eq!(report.lookup_receipt_digest(), lookup_receipt.digest());
        assert!(!report.report_digest().is_empty());
        assert!(!report.counter_snapshot().is_empty());
    }
}

#[test]
fn durable_and_store_backed_subjects_remain_explicitly_deferred() {
    let declaration = declaration_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let admission = admission_for(&declaration);
    let admission_evidence =
        QuerySubscriptionSupportEvidence::admission(&declaration, &admission).unwrap();

    for subject in [
        QuerySubscriptionSupportSubject::durable_replay(&declaration),
        QuerySubscriptionSupportSubject::store_backed_restart(&declaration),
    ] {
        let (report, lookup_receipt) =
            report_query_subscription_support(subject, admission_evidence.clone()).unwrap();

        assert_eq!(
            report.support_posture(),
            &QuerySubscriptionSupportPosture::RuntimeBackedDeferred
        );
        assert_eq!(report.counters().supported_family_count(), 0);
        assert_eq!(report.counters().deferred_family_count(), 1);
        assert_eq!(report.counters().denied_family_count(), 0);
        assert_eq!(
            lookup_receipt.resolution_posture(),
            &SupportResolutionPosture::IndexedFamilyLookup
        );
    }
}

#[test]
fn active_lifecycle_subject_reports_certified_support_with_indexed_lookup_receipt() {
    let declaration = declaration_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::KanbanGrouped),
    );
    let admission = admission_for(&declaration);
    let activation = prepare_subscription_activation(admission.clone());
    let active_admission = admit_active_subscription_lane(
        activation,
        ActiveSubscriptionWorkBudget::admitted(
            ActiveRegistryLookupWidth::measured(1),
            ActiveFanoutWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPolicy::LifecycleArena,
        ),
    )
    .unwrap();
    let subject =
        QuerySubscriptionSupportSubject::active_lifecycle(&declaration, &admission, &active_admission);
    let evidence = QuerySubscriptionSupportEvidence::admission(&declaration, &admission).unwrap();

    let (report, lookup_receipt) = report_query_subscription_support(subject, evidence).unwrap();

    assert_eq!(
        report.support_subject().support_class(),
        &QuerySubscriptionSupportClass::ActiveLifecycle
    );
    assert_eq!(
        report.support_posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::Activation)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::ActiveLifecycle)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::Continuation)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::UncertifiedDenied
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::PreviewCloseout)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::UncertifiedDenied
    );
    assert_eq!(
        lookup_receipt.resolution_posture(),
        &SupportResolutionPosture::IndexedFamilyLookup
    );
    assert_eq!(report.counters().supported_family_count(), 1);
}

#[test]
fn support_report_denies_mismatched_declaration_subject_sources() {
    let detail_declaration = declaration_for(LiveQueryFamily::Detail, None);
    let grouped_declaration = declaration_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::KanbanGrouped),
    );
    let detail_admission = admission_for(&detail_declaration);
    let mismatched_subject = QuerySubscriptionSupportSubject::declaration(&grouped_declaration);

    let error = report_query_subscription_support(
        mismatched_subject,
        QuerySubscriptionSupportEvidence::admission(&detail_declaration, &detail_admission)
            .unwrap(),
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionSupportReportDenialKind::DeclarationSourceMismatch
    );
    assert!(error.message().contains("same declaration artifact"));
    assert!(!error.failure_digest().is_empty());
}

#[test]
fn support_report_denies_mismatched_admission_subject_sources() {
    let table_declaration = declaration_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let table_admission = admission_for(&table_declaration);
    let alternate_lowering =
        lower_query_subscription_to_bridge(table_declaration.clone(), roomy_lowering_budget())
            .unwrap();
    let alternate_admission = admit_query_subscription(
        alternate_lowering,
        QuerySubscriptionAdmissionBudget::admitted(2, 1, 1, 1, 1),
    )
    .unwrap();
    let activation = prepare_subscription_activation(alternate_admission);
    let subject = QuerySubscriptionSupportSubject::activation(&table_declaration, &activation);

    let error = report_query_subscription_support(
        subject,
        QuerySubscriptionSupportEvidence::admission(&table_declaration, &table_admission).unwrap(),
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionSupportReportDenialKind::AdmissionSourceMismatch
    );
    assert!(error.message().contains("same admission artifact"));
    assert!(!error.failure_digest().is_empty());
}

#[test]
fn support_evidence_rejects_mismatched_declaration_and_admission_sources_early() {
    let detail_declaration = declaration_for(LiveQueryFamily::Detail, None);
    let table_declaration = declaration_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let table_admission = admission_for(&table_declaration);

    let error = QuerySubscriptionSupportEvidence::admission(&detail_declaration, &table_admission)
        .unwrap_err();

    assert!(error
        .message()
        .contains("same canonical query subscription family"));
    assert!(!error.failure_digest().is_empty());
}

#[test]
fn declaration_support_can_be_reported_without_admission_evidence() {
    let declaration = declaration_for(LiveQueryFamily::Detail, None);
    let subject = QuerySubscriptionSupportSubject::declaration(&declaration);

    let (report, lookup_receipt) = report_query_subscription_support(
        subject,
        QuerySubscriptionSupportEvidence::declaration(&declaration),
    )
    .unwrap();

    assert_eq!(
        report.support_subject().support_class(),
        &QuerySubscriptionSupportClass::Declaration
    );
    assert_eq!(
        report.support_posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        lookup_receipt.resolution_posture(),
        &SupportResolutionPosture::IndexedFamilyLookup
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::Activation)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::UncertifiedDenied
    );
}

#[test]
fn activation_support_denies_when_only_declaration_evidence_is_available() {
    let declaration = declaration_for(LiveQueryFamily::Detail, None);
    let admission = admission_for(&declaration);
    let activation = prepare_subscription_activation(admission);
    let subject = QuerySubscriptionSupportSubject::activation(&declaration, &activation);

    let error = report_query_subscription_support(
        subject,
        QuerySubscriptionSupportEvidence::declaration(&declaration),
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionSupportReportDenialKind::AdmissionEvidenceRequired
    );
    assert!(error.message().contains("requires admission evidence"));
    assert!(!error.failure_digest().is_empty());
}

#[test]
fn activation_subject_only_certifies_activation_and_keeps_later_runtime_rows_uncertified() {
    let declaration = declaration_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let admission = admission_for(&declaration);
    let activation = prepare_subscription_activation(admission.clone());
    let subject = QuerySubscriptionSupportSubject::activation(&declaration, &activation);

    let (report, _) = report_query_subscription_support(
        subject,
        QuerySubscriptionSupportEvidence::admission(&declaration, &admission).unwrap(),
    )
    .unwrap();

    assert_eq!(
        report.support_posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::Activation)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::ActiveLifecycle)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::UncertifiedDenied
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::Continuation)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::UncertifiedDenied
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::PreviewCloseout)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::UncertifiedDenied
    );
}

#[test]
fn continuation_subject_only_certifies_through_continuation() {
    let declaration = declaration_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let (admission, continuation) = continuation_report_for(&declaration);
    let subject =
        QuerySubscriptionSupportSubject::continuation(&declaration, &admission, &continuation);

    let (report, _) = report_query_subscription_support(
        subject,
        QuerySubscriptionSupportEvidence::admission(&declaration, &admission).unwrap(),
    )
    .unwrap();

    assert_eq!(
        report.support_posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::Activation)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::ActiveLifecycle)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::Continuation)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::PreviewCloseout)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::UncertifiedDenied
    );
}

#[test]
fn preview_closeout_subject_certifies_preview_closeout() {
    let declaration = declaration_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let (admission, closeout) = preview_closeout_for(&declaration);
    let subject =
        QuerySubscriptionSupportSubject::preview_closeout(&declaration, &admission, &closeout);

    let (report, _) = report_query_subscription_support(
        subject,
        QuerySubscriptionSupportEvidence::admission(&declaration, &admission).unwrap(),
    )
    .unwrap();

    assert_eq!(
        report.support_posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::Activation)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::ActiveLifecycle)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::Continuation)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
    assert_eq!(
        report
            .support_matrix()
            .row_for_class(QuerySubscriptionSupportClass::PreviewCloseout)
            .unwrap()
            .posture(),
        &QuerySubscriptionSupportPosture::RuntimeBackedCertified
    );
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
        DeliveryWindowWidth::measured(2),
        PatchGroupWidth::measured(2),
        MaintenanceDeltaWidth::measured(2),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    )
}

fn declaration_for(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
) -> QuerySubscriptionDeclarationArtifact {
    let live = LiveQueryAdmissionArtifact::for_test(
        live_family,
        view_family,
        QuerySubscriptionConstructionSource::Direct,
    );
    let selection = select_query_subscription_family(live, roomy_budget()).unwrap();
    declare_query_subscription(selection, roomy_slice_budget()).unwrap()
}

fn admission_for(
    declaration: &QuerySubscriptionDeclarationArtifact,
) -> QuerySubscriptionAdmissionArtifact {
    let lowering =
        lower_query_subscription_to_bridge(declaration.clone(), roomy_lowering_budget()).unwrap();
    admit_query_subscription(
        lowering,
        QuerySubscriptionAdmissionBudget::admitted(1, 1, 1, 1, 1),
    )
    .unwrap()
}

fn continuation_report_for(
    declaration: &QuerySubscriptionDeclarationArtifact,
) -> (
    QuerySubscriptionAdmissionArtifact,
    SubscriptionContinuationReport,
) {
    let admission = admission_for(declaration);
    let activation = prepare_subscription_activation(admission.clone());
    let active_admission = admit_active_subscription_lane(activation, active_budget()).unwrap();
    let mut runtime = ActiveSubscriptionRuntime::new();
    let handle = open_active_subscription_lane(&mut runtime, active_admission).unwrap();
    let attachment = attach_subscription_consumer(
        &mut runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted("consumer-a", "cursor-a"),
        attachment_budget(),
    )
    .unwrap();
    let window = open_query_delivery_window(&mut runtime, &attachment, delivery_budget()).unwrap();
    let evidence = admit_subscription_continuation_evidence(
        attachment.lane_digest().clone(),
        SubscriptionContinuationClass::IdentityRemap,
        "employee:old",
        "employee:new",
        "basis:current",
        "identity-evolution-authority",
        ContinuationRemapWidth::measured(1),
    )
    .unwrap();
    let (_, report) =
        apply_active_subscription_continuation(&mut runtime, window, evidence).unwrap();
    (admission, report)
}

fn preview_closeout_for(
    declaration: &QuerySubscriptionDeclarationArtifact,
) -> (
    QuerySubscriptionAdmissionArtifact,
    SubscriptionLifecycleCloseout,
) {
    let admission = admission_for(declaration);
    let activation = prepare_subscription_activation(admission.clone());
    let active_admission = admit_active_subscription_lane(activation, active_budget()).unwrap();
    let mut runtime = ActiveSubscriptionRuntime::new();
    let handle = open_active_subscription_lane(&mut runtime, active_admission).unwrap();
    let attachment = attach_subscription_consumer(
        &mut runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted("preview-a", "cursor-a"),
        attachment_budget(),
    )
    .unwrap();
    let isolation = admit_preview_subscription_isolation(
        &attachment,
        "preview-epoch-a",
        PreviewResidueWidth::measured(1),
    )
    .unwrap();
    let residue = measure_preview_subscription_residue(
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(0),
    );
    let preview_closeout = discard_preview_subscription(isolation, residue).unwrap();
    let closeout = close_subscription_lifecycle(
        &mut runtime,
        &handle,
        SubscriptionLifecycleCloseRequest::PreviewDiscard(preview_closeout),
    )
    .unwrap();
    (admission, closeout)
}
