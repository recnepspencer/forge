use crate::harness::certification::{
    digest_parts, HostileExpectation, ParityAnchor, RejectionCertificationRow,
};
use crate::live::LiveQueryFamily;
use crate::subscription::*;
use crate::view_shape_live::LiveViewShapeFamily;

use super::{
    MilestoneNineThreeCertificationBundle, MilestoneNineThreeCertificationMatrix,
    MilestoneNineThreeCertificationRow, MilestoneNineThreeFailureClass,
    MilestoneNineThreePerturbationClass, MilestoneNineThreeRejectionBundle,
    MILESTONE_NINE_THREE_REQUIRED_COMPILE_FAIL_TARGETS,
};

#[derive(Clone, Copy)]
enum LaneScenario {
    ActiveLifecycle,
    Continuation,
    PreviewDiscard,
}

#[derive(Clone)]
struct CertifiedLaneArtifacts {
    selection: QuerySubscriptionFamilySelection,
    declaration: QuerySubscriptionDeclarationArtifact,
    lowering: BridgeSubscriptionLoweringPlan,
    admission: QuerySubscriptionAdmissionArtifact,
    support_report: QuerySubscriptionSupportReport,
    support_lookup_receipt: SupportLookupReceipt,
    witness: QuerySubscriptionManualBridgeWitness,
    parity_explanation: QuerySubscriptionBridgeParityExplanation,
    parity_receipt: BridgeParityReceipt,
    lifecycle_bundle: SubscriptionLifecycleCertificationBundle,
    admitted_trace: QuerySubscriptionDiagnosticTrace,
    admitted_bundle: QuerySubscriptionAdmittedDiagnosticBundle,
    diagnostic_receipt: DiagnosticAssemblyReceipt,
    runtime_bundle: QuerySubscriptionRuntimeCertificationBundle,
    coverage_receipt: CertificationCoverageReceipt,
    continuation_digest: String,
    preview_isolation_digest: String,
}

struct SupportDeniedArtifacts {
    denied_bundle: QuerySubscriptionDeniedDiagnosticBundle,
    failure: QuerySubscriptionDiagnosticFailure,
}

pub fn canonical_rows() -> Vec<MilestoneNineThreeCertificationRow> {
    let detail = lane_for(
        LiveQueryFamily::Detail,
        None,
        LaneScenario::ActiveLifecycle,
        CoverageResolutionPosture::IndexedCoverageSet,
        1,
    );
    let inspector = lane_for(
        LiveQueryFamily::Detail,
        Some(LiveViewShapeFamily::InspectorDetailFocused),
        LaneScenario::ActiveLifecycle,
        CoverageResolutionPosture::IndexedCoverageSet,
        2,
    );
    let ordered = lane_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
        LaneScenario::ActiveLifecycle,
        CoverageResolutionPosture::IndexedCoverageSet,
        3,
    );
    let grouped = lane_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::KanbanGrouped),
        LaneScenario::ActiveLifecycle,
        CoverageResolutionPosture::IndexedCoverageSet,
        4,
    );
    let bounded = lane_for(
        LiveQueryFamily::BoundedMaterialization,
        None,
        LaneScenario::ActiveLifecycle,
        CoverageResolutionPosture::IndexedCoverageSet,
        5,
    );
    let continuation = lane_for(
        LiveQueryFamily::Detail,
        None,
        LaneScenario::Continuation,
        CoverageResolutionPosture::IndexedCoverageSet,
        6,
    );
    let preview = lane_for(
        LiveQueryFamily::Detail,
        None,
        LaneScenario::PreviewDiscard,
        CoverageResolutionPosture::IndexedCoverageSet,
        7,
    );
    let churn_control = lane_for(
        LiveQueryFamily::Detail,
        None,
        LaneScenario::ActiveLifecycle,
        CoverageResolutionPosture::IndexedCoverageSet,
        8,
    );
    let churn_hostile = lane_for(
        LiveQueryFamily::Detail,
        None,
        LaneScenario::ActiveLifecycle,
        CoverageResolutionPosture::IndexedCoverageSet,
        9,
    );
    let debt = lane_for(
        LiveQueryFamily::Detail,
        None,
        LaneScenario::ActiveLifecycle,
        CoverageResolutionPosture::MatrixScanDebtExplicit,
        1,
    );

    vec![
        admitted_row(
            "detail-family-support-and-parity",
            MilestoneNineThreePerturbationClass::DetailFamilySupportAndParity,
            HostileExpectation::EquivalentToControl,
            &detail,
            &detail,
            &detail,
        ),
        admitted_row(
            "inspector-family-support-and-parity",
            MilestoneNineThreePerturbationClass::InspectorFamilySupportAndParity,
            HostileExpectation::DistinctFromControl,
            &detail,
            &inspector,
            &inspector,
        ),
        admitted_row(
            "ordered-collection-family-support-and-parity",
            MilestoneNineThreePerturbationClass::OrderedCollectionFamilySupportAndParity,
            HostileExpectation::EquivalentToControl,
            &ordered,
            &ordered,
            &ordered,
        ),
        admitted_row(
            "grouped-collection-family-support-and-parity",
            MilestoneNineThreePerturbationClass::GroupedCollectionFamilySupportAndParity,
            HostileExpectation::DistinctFromControl,
            &ordered,
            &grouped,
            &grouped,
        ),
        admitted_row(
            "bounded-materialization-family-support-and-parity",
            MilestoneNineThreePerturbationClass::BoundedMaterializationFamilySupportAndParity,
            HostileExpectation::EquivalentToControl,
            &bounded,
            &bounded,
            &bounded,
        ),
        admitted_row(
            "preview-family-lifecycle-certification-bundle",
            MilestoneNineThreePerturbationClass::PreviewFamilyLifecycleCertificationBundle,
            HostileExpectation::DistinctFromControl,
            &detail,
            &preview,
            &preview,
        ),
        admitted_row(
            "continuation-family-support-sync",
            MilestoneNineThreePerturbationClass::ContinuationFamilySupportSync,
            HostileExpectation::DistinctFromControl,
            &detail,
            &continuation,
            &continuation,
        ),
        admitted_row(
            "family-coverage-certification-closure",
            MilestoneNineThreePerturbationClass::FamilyCoverageCertificationClosure,
            HostileExpectation::EquivalentToControl,
            &detail,
            &detail,
            &detail,
        ),
        admitted_row(
            "declaration-family-drift-vs-lifecycle-churn-distinctness",
            MilestoneNineThreePerturbationClass::DeclarationFamilyDriftVsLifecycleChurnDistinctness,
            HostileExpectation::DistinctFromControl,
            &churn_control,
            &churn_hostile,
            &churn_control,
        ),
        admitted_row(
            "basis-policy-viewshape-family-coverage-closure",
            MilestoneNineThreePerturbationClass::BasisPolicyViewshapeFamilyCoverageClosure,
            HostileExpectation::DistinctFromControl,
            &detail,
            &grouped,
            &grouped,
        ),
        admitted_row(
            "support-matrix-scale-honesty",
            MilestoneNineThreePerturbationClass::SupportMatrixScaleHonesty,
            HostileExpectation::DistinctFromControl,
            &detail,
            &debt,
            &detail,
        ),
    ]
}

pub fn rejection_rows() -> Vec<
    RejectionCertificationRow<
        MilestoneNineThreePerturbationClass,
        MilestoneNineThreeCertificationBundle,
        MilestoneNineThreeRejectionBundle,
    >,
> {
    let detail = lane_for(
        LiveQueryFamily::Detail,
        None,
        LaneScenario::ActiveLifecycle,
        CoverageResolutionPosture::IndexedCoverageSet,
        11,
    );
    let collection = lane_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
        LaneScenario::ActiveLifecycle,
        CoverageResolutionPosture::IndexedCoverageSet,
        12,
    );

    vec![
        rejection_row(
            "uncertified-family-support-overclaim-forbidden",
            MilestoneNineThreePerturbationClass::UncertifiedFamilySupportOverclaimForbidden,
            &detail,
            denied_support_failure(&detail),
            &detail,
        ),
        rejection_row(
            "store-backed-restart-support-overclaim-forbidden",
            MilestoneNineThreePerturbationClass::StoreBackedRestartSupportOverclaimForbidden,
            &detail,
            compile_fail_rejection("subscription_support_report_durable_overclaim_forbidden.rs"),
            &detail,
        ),
        rejection_row(
            "durable-replay-support-overclaim-forbidden",
            MilestoneNineThreePerturbationClass::DurableReplaySupportOverclaimForbidden,
            &detail,
            compile_fail_rejection("subscription_support_report_durable_overclaim_forbidden.rs"),
            &detail,
        ),
        rejection_row(
            "bridge-parity-declaration-source-mismatch",
            MilestoneNineThreePerturbationClass::BridgeParityDeclarationSourceMismatch,
            &detail,
            denied_bridge_parity_failure(&detail, &collection),
            &detail,
        ),
        rejection_row(
            "bridge-parity-signal-strategy-source-mismatch",
            MilestoneNineThreePerturbationClass::BridgeParitySignalStrategySourceMismatch,
            &detail,
            compile_fail_rejection(
                "subscription_bridge_parity_mismatched_signal_strategy_forbidden.rs",
            ),
            &detail,
        ),
        rejection_row(
            "diagnostic-bundle-missing-hostile-row-forbidden",
            MilestoneNineThreePerturbationClass::DiagnosticBundleMissingHostileRowForbidden,
            &detail,
            denied_runtime_certification_failure(&detail),
            &detail,
        ),
        rejection_row(
            "runtime-certification-cross-family-row-mix-forbidden",
            MilestoneNineThreePerturbationClass::RuntimeCertificationCrossFamilyRowMixForbidden,
            &detail,
            denied_cross_family_scope_failure(&detail, &collection),
            &detail,
        ),
        rejection_row(
            "generic-family-certification-shortcut-forbidden",
            MilestoneNineThreePerturbationClass::GenericFamilyCertificationShortcutForbidden,
            &detail,
            compile_fail_rejection(
                "subscription_runtime_certification_uncertified_family_forbidden.rs",
            ),
            &detail,
        ),
    ]
}

pub fn bundle_digest_parts(matrix: &MilestoneNineThreeCertificationMatrix) -> Vec<String> {
    let mut parts = vec![matrix.suite_name.to_string()];
    parts.extend(matrix.rows.iter().map(|row| {
        format!(
            "row:{}:{}:{}:{}",
            row.row_name,
            row.control_lane.runtime_certification_bundle_digest,
            row.hostile_lane.runtime_certification_bundle_digest,
            row.parity_lane.runtime_certification_bundle_digest,
        )
    }));
    parts.extend(matrix.rejection_rows.iter().map(|row| {
        format!(
            "reject:{}:{}:{}",
            row.row_name,
            row.hostile_lane.failure_digest,
            row.hostile_lane.compile_fail_boundary_digest
        )
    }));
    parts
}

pub fn coverage_digest_parts(matrix: &MilestoneNineThreeCertificationMatrix) -> Vec<String> {
    let mut parts = vec![matrix.suite_name.to_string()];
    parts.extend(matrix.rows.iter().map(|row| {
        format!(
            "row:{}:{}",
            row.row_name,
            row.control_lane.semantic_signature()
        )
    }));
    parts.extend(matrix.rejection_rows.iter().map(|row| {
        format!(
            "reject:{}:{}",
            row.row_name, row.hostile_lane.failure_digest
        )
    }));
    parts
}

fn admitted_row(
    row_name: &'static str,
    perturbation_class: MilestoneNineThreePerturbationClass,
    hostile_expectation: HostileExpectation,
    control: &CertifiedLaneArtifacts,
    hostile: &CertifiedLaneArtifacts,
    parity: &CertifiedLaneArtifacts,
) -> MilestoneNineThreeCertificationRow {
    MilestoneNineThreeCertificationRow {
        row_name,
        perturbation_class,
        hostile_expectation,
        parity_anchor: if hostile.runtime_bundle.runtime_certification_bundle_digest()
            == parity.runtime_bundle.runtime_certification_bundle_digest()
        {
            ParityAnchor::Hostile
        } else {
            ParityAnchor::Control
        },
        control_lane: certification_bundle(control),
        hostile_lane: certification_bundle(hostile),
        parity_lane: certification_bundle(parity),
    }
}

fn rejection_row(
    row_name: &'static str,
    perturbation_class: MilestoneNineThreePerturbationClass,
    control: &CertifiedLaneArtifacts,
    hostile: MilestoneNineThreeRejectionBundle,
    parity: &CertifiedLaneArtifacts,
) -> RejectionCertificationRow<
    MilestoneNineThreePerturbationClass,
    MilestoneNineThreeCertificationBundle,
    MilestoneNineThreeRejectionBundle,
> {
    RejectionCertificationRow {
        row_name,
        perturbation_class,
        control_lane: certification_bundle(control),
        hostile_lane: hostile,
        parity_lane: certification_bundle(parity),
    }
}

fn certification_bundle(
    artifacts: &CertifiedLaneArtifacts,
) -> MilestoneNineThreeCertificationBundle {
    MilestoneNineThreeCertificationBundle {
        query_family_label: artifacts
            .parity_explanation
            .query_family_label()
            .to_string(),
        declaration_family_label: artifacts
            .parity_explanation
            .declaration_family_label()
            .to_string(),
        bridge_family_label: artifacts
            .parity_explanation
            .bridge_family_label()
            .to_string(),
        support_class_label: artifacts
            .support_report
            .support_subject()
            .support_class()
            .as_str()
            .to_string(),
        support_resolution_posture_label: artifacts
            .support_lookup_receipt
            .resolution_posture()
            .as_str()
            .to_string(),
        coverage_resolution_posture_label: artifacts
            .coverage_receipt
            .coverage_resolution_posture()
            .as_str()
            .to_string(),
        query_digest: artifacts.lifecycle_bundle.query_digest().to_string(),
        subscription_family_digest: artifacts
            .lifecycle_bundle
            .subscription_family_digest()
            .to_string(),
        subscription_declaration_digest: artifacts
            .lifecycle_bundle
            .subscription_declaration_digest()
            .to_string(),
        subscription_equivalence_digest: artifacts
            .lifecycle_bundle
            .subscription_equivalence_digest()
            .to_string(),
        bridge_declaration_digest: artifacts
            .lifecycle_bundle
            .bridge_declaration_digest()
            .to_string(),
        bridge_basis_digest: artifacts.lifecycle_bundle.basis_digest().to_string(),
        signal_strategy_digest: artifacts
            .lifecycle_bundle
            .signal_strategy_digest()
            .to_string(),
        support_report_digest: artifacts.support_report.report_digest().to_string(),
        support_matrix_digest: artifacts
            .support_report
            .support_matrix()
            .digest()
            .to_string(),
        support_lookup_receipt_digest: artifacts.support_lookup_receipt.digest().to_string(),
        manual_bridge_witness_digest: artifacts.witness.witness_digest().to_string(),
        bridge_parity_digest: artifacts
            .parity_explanation
            .explanation_digest()
            .to_string(),
        bridge_parity_receipt_digest: artifacts.parity_receipt.digest().to_string(),
        diagnostic_trace_digest: artifacts.admitted_trace.trace_digest().to_string(),
        admitted_diagnostic_bundle_digest: artifacts.admitted_bundle.bundle_digest().to_string(),
        denied_diagnostic_bundle_digest: "none".to_string(),
        diagnostic_assembly_receipt_digest: artifacts.diagnostic_receipt.digest().to_string(),
        lifecycle_certification_digest: artifacts
            .lifecycle_bundle
            .certification_bundle_digest()
            .to_string(),
        runtime_certification_bundle_digest: artifacts
            .runtime_bundle
            .runtime_certification_bundle_digest()
            .to_string(),
        certification_coverage_receipt_digest: artifacts.coverage_receipt.digest().to_string(),
        continuation_digest: artifacts.continuation_digest.clone(),
        preview_isolation_digest: artifacts.preview_isolation_digest.clone(),
        failure_digest: "none".to_string(),
        counter_snapshot: artifacts.runtime_bundle.counter_snapshot().to_string(),
        compile_fail_boundary_digest: compile_fail_boundary_digest(
            MILESTONE_NINE_THREE_REQUIRED_COMPILE_FAIL_TARGETS,
        ),
    }
}

fn compile_fail_rejection(target: &'static str) -> MilestoneNineThreeRejectionBundle {
    let digest = compile_fail_boundary_digest(&[target]);
    MilestoneNineThreeRejectionBundle {
        failure_class: MilestoneNineThreeFailureClass::CompileFailBoundary,
        failure_kind: "compile_fail_boundary".to_string(),
        failure_digest: digest.clone(),
        denied_bundle_digest: "compile_fail_boundary".to_string(),
        counter_snapshot: digest.clone(),
        compile_fail_boundary_digest: digest,
    }
}

fn denied_support_failure(artifacts: &CertifiedLaneArtifacts) -> MilestoneNineThreeRejectionBundle {
    let denied = denied_support_artifacts(artifacts);
    MilestoneNineThreeRejectionBundle {
        failure_class: MilestoneNineThreeFailureClass::SupportDenied,
        failure_kind: denied.failure.stage().as_str().to_string(),
        failure_digest: denied.failure.failure_digest().to_string(),
        denied_bundle_digest: denied.denied_bundle.bundle_digest().to_string(),
        counter_snapshot: denied.denied_bundle.counter_snapshot().to_string(),
        compile_fail_boundary_digest: compile_fail_boundary_digest(&[
            "subscription_support_report_durable_overclaim_forbidden.rs",
        ]),
    }
}

fn denied_bridge_parity_failure(
    detail: &CertifiedLaneArtifacts,
    foreign: &CertifiedLaneArtifacts,
) -> MilestoneNineThreeRejectionBundle {
    let activation = prepare_subscription_activation(foreign.admission.clone());
    let error = explain_query_subscription_bridge_parity(
        &foreign.declaration,
        &foreign.lowering,
        &activation,
        detail.witness.clone(),
    )
    .unwrap_err();

    MilestoneNineThreeRejectionBundle {
        failure_class: MilestoneNineThreeFailureClass::BridgeParityDenied,
        failure_kind: error.failure().failure_kind().as_str().to_string(),
        failure_digest: error.failure().failure_digest().to_string(),
        denied_bundle_digest: "none".to_string(),
        counter_snapshot: error.counters().digest(),
        compile_fail_boundary_digest: compile_fail_boundary_digest(&[
            "subscription_bridge_parity_mismatched_declaration_forbidden.rs",
        ]),
    }
}

fn denied_runtime_certification_failure(
    artifacts: &CertifiedLaneArtifacts,
) -> MilestoneNineThreeRejectionBundle {
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
        artifacts.support_report.clone(),
        artifacts.parity_explanation.clone(),
        artifacts.admitted_bundle.clone(),
        artifacts.lifecycle_bundle.clone(),
        handle,
    )
    .unwrap();
    let error = certify_query_subscription_runtime_family(scope).unwrap_err();

    MilestoneNineThreeRejectionBundle {
        failure_class: MilestoneNineThreeFailureClass::RuntimeCertificationDenied,
        failure_kind: error.error_kind().as_str().to_string(),
        failure_digest: error.failure_digest().to_string(),
        denied_bundle_digest: "none".to_string(),
        counter_snapshot: error.counters().digest(),
        compile_fail_boundary_digest: compile_fail_boundary_digest(&[
            "subscription_diagnostic_bundle_missing_hostile_coverage_forbidden.rs",
        ]),
    }
}

fn denied_cross_family_scope_failure(
    detail: &CertifiedLaneArtifacts,
    foreign: &CertifiedLaneArtifacts,
) -> MilestoneNineThreeRejectionBundle {
    let hostile = denied_support_artifacts(foreign);
    let matrix = build_query_subscription_family_coverage_matrix(vec![
        QuerySubscriptionFamilyCoverageRow::admitted(
            foreign.declaration.family(),
            &foreign.support_report,
            &foreign.parity_explanation,
            &foreign.lifecycle_bundle,
            &foreign.admitted_bundle,
            QuerySubscriptionLifecycleCoverageClass::LifecycleCloseout,
        )
        .unwrap(),
        QuerySubscriptionFamilyCoverageRow::hostile(
            foreign.declaration.family(),
            &foreign.support_report,
            &foreign.parity_explanation,
            &foreign.lifecycle_bundle,
            &hostile.denied_bundle,
            &hostile.failure,
            QuerySubscriptionLifecycleCoverageClass::ActiveLifecycle,
        )
        .unwrap(),
    ]);
    let handle = build_certified_family_coverage_handle(
        &matrix,
        foreign.declaration.family(),
        CoverageResolutionPosture::IndexedCoverageSet,
    )
    .unwrap();
    let error = build_query_subscription_runtime_certification_scope(
        detail.support_report.clone(),
        detail.parity_explanation.clone(),
        detail.admitted_bundle.clone(),
        detail.lifecycle_bundle.clone(),
        handle,
    )
    .unwrap_err();

    MilestoneNineThreeRejectionBundle {
        failure_class: MilestoneNineThreeFailureClass::RuntimeCertificationDenied,
        failure_kind: error.error_kind().as_str().to_string(),
        failure_digest: error.failure_digest().to_string(),
        denied_bundle_digest: "none".to_string(),
        counter_snapshot: error.counters().digest(),
        compile_fail_boundary_digest: compile_fail_boundary_digest(&[
            "subscription_runtime_certification_uncertified_family_forbidden.rs",
        ]),
    }
}

fn denied_support_artifacts(artifacts: &CertifiedLaneArtifacts) -> SupportDeniedArtifacts {
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

    SupportDeniedArtifacts {
        denied_bundle,
        failure,
    }
}

fn lane_for(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    scenario: LaneScenario,
    coverage_posture: CoverageResolutionPosture,
    admission_width: usize,
) -> CertifiedLaneArtifacts {
    let live = LiveQueryAdmissionArtifact::for_test(
        live_family,
        view_family,
        QuerySubscriptionConstructionSource::FacadeLive,
    );
    let selection = select_query_subscription_family(live.clone(), roomy_budget()).unwrap();
    let context = SubscriptionLifecycleCertificationContext::from_live_selection(&live, &selection);
    let declaration = declare_query_subscription(selection.clone(), roomy_slice_budget()).unwrap();
    let lowering =
        lower_query_subscription_to_bridge(declaration.clone(), roomy_lowering_budget()).unwrap();
    let admission = admit_query_subscription(
        lowering.clone(),
        QuerySubscriptionAdmissionBudget::admitted(admission_width, 8, 1, 1, 1),
    )
    .unwrap();
    let activation = prepare_subscription_activation(admission.clone());
    let witness =
        build_query_subscription_manual_bridge_witness(&declaration, &lowering, &activation)
            .unwrap();
    let (parity_explanation, parity_receipt) = explain_query_subscription_bridge_parity(
        &declaration,
        &lowering,
        &activation,
        witness.clone(),
    )
    .unwrap();
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
    let active_admission = admit_active_subscription_lane(activation, active_budget()).unwrap();
    let handle = open_active_subscription_lane(&mut runtime, active_admission.clone()).unwrap();
    let attachment = attach_subscription_consumer(
        &mut runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted("certification-consumer", "cursor"),
        attachment_budget(),
    )
    .unwrap();
    let mut continuation_report = None;
    let mut preview_isolation = None;
    let mut preview_residue = None;
    let mut preview_discard = None;
    let (window, delta, lowering_report, continuation_width) = match scenario {
        LaneScenario::Continuation => {
            let window =
                open_query_delivery_window(&mut runtime, &attachment, delivery_budget()).unwrap();
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
            let (continued_window, report) =
                apply_active_subscription_continuation(&mut runtime, window, evidence).unwrap();
            let (delta, _) = lower_subscription_continuation_report(&report);
            let (delta, lowering_report, _) =
                lower_query_subscription_maintenance_delta(delta).unwrap();
            continuation_report = Some(report);
            (continued_window, delta, lowering_report, 1)
        }
        LaneScenario::ActiveLifecycle | LaneScenario::PreviewDiscard => {
            let window =
                open_query_delivery_window(&mut runtime, &attachment, delivery_budget()).unwrap();
            let delta = QuerySubscriptionMaintenanceDelta::admitted(
                QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
                attachment.lane_digest().clone(),
                "affected-scope",
                MaintenanceDeltaWidth::measured(1),
            );
            let (delta, lowering_report, _) =
                lower_query_subscription_maintenance_delta(delta).unwrap();
            (window, delta, lowering_report, 0)
        }
    };

    if matches!(scenario, LaneScenario::PreviewDiscard) {
        let isolation = admit_preview_subscription_isolation(
            &attachment,
            "preview-epoch-a",
            PreviewResidueWidth::measured(2),
        )
        .unwrap();
        let residue = measure_preview_subscription_residue(
            PreviewResidueWidth::measured(0),
            PreviewResidueWidth::measured(0),
            PreviewResidueWidth::measured(0),
            PreviewResidueWidth::measured(0),
            PreviewResidueWidth::measured(0),
            PreviewResidueWidth::measured(1),
            PreviewResidueWidth::measured(1),
        );
        let discard = discard_preview_subscription(isolation.clone(), residue.clone()).unwrap();
        preview_isolation = Some(isolation);
        preview_residue = Some(residue);
        preview_discard = Some(discard);
    }

    let delivery_window_digest = window.delivery_window_digest().to_string();
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
    let close_request = match &preview_discard {
        Some(discard) => SubscriptionLifecycleCloseRequest::PreviewDiscard(discard.clone()),
        None => {
            SubscriptionLifecycleCloseRequest::TerminateConsumer(acknowledged_attachment.clone())
        }
    };
    let closeout = close_subscription_lifecycle(&mut runtime, &handle, close_request).unwrap();
    let preview_certification = match (&preview_isolation, &preview_residue, &preview_discard) {
        (Some(isolation), Some(residue), Some(discard)) => {
            SubscriptionLifecyclePreviewCertification::Discard {
                isolation,
                residue_report: residue,
                discard_closeout: discard,
            }
        }
        _ => SubscriptionLifecyclePreviewCertification::None,
    };
    let lifecycle_bundle = certify_subscription_lifecycle(
        context,
        &admission,
        &prepare_subscription_activation(admission.clone()),
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
        preview_certification,
        &closeout,
    )
    .unwrap();
    let support_subject = match scenario {
        LaneScenario::ActiveLifecycle => {
            QuerySubscriptionSupportSubject::active_lifecycle(&declaration, &active_admission)
        }
        LaneScenario::Continuation => QuerySubscriptionSupportSubject::continuation(
            &declaration,
            &admission,
            continuation_report.as_ref().unwrap(),
        ),
        LaneScenario::PreviewDiscard => {
            QuerySubscriptionSupportSubject::preview_closeout(&declaration, &admission, &closeout)
        }
    };
    let (support_report, support_lookup_receipt) = report_query_subscription_support(
        support_subject,
        QuerySubscriptionSupportEvidence::admission(&declaration, &admission).unwrap(),
    )
    .unwrap();
    let admitted_trace = trace_admitted_query_subscription_diagnostics(
        &selection,
        &declaration,
        &lowering,
        &admission,
        &support_report,
        &lifecycle_bundle,
        continuation_report.as_ref(),
        preview_isolation.as_ref(),
        Some(&closeout),
    )
    .unwrap();
    let (admitted_bundle, diagnostic_receipt) = bundle_admitted_query_subscription_diagnostics(
        admitted_trace.clone(),
        &selection,
        &declaration,
        &lowering,
        &admission,
        &support_report,
        &lifecycle_bundle,
        continuation_report.as_ref(),
        preview_isolation.as_ref(),
        Some(&closeout),
    )
    .unwrap();

    let hostile =
        denied_support_artifacts_from_parts(&selection, &declaration, &lowering, &admission);
    let matrix = build_query_subscription_family_coverage_matrix(vec![
        QuerySubscriptionFamilyCoverageRow::admitted(
            declaration.family(),
            &support_report,
            &parity_explanation,
            &lifecycle_bundle,
            &admitted_bundle,
            lifecycle_class_for(scenario),
        )
        .unwrap(),
        QuerySubscriptionFamilyCoverageRow::hostile(
            declaration.family(),
            &support_report,
            &parity_explanation,
            &lifecycle_bundle,
            &hostile.denied_bundle,
            &hostile.failure,
            QuerySubscriptionLifecycleCoverageClass::ActiveLifecycle,
        )
        .unwrap(),
    ]);
    let handle =
        build_certified_family_coverage_handle(&matrix, declaration.family(), coverage_posture)
            .unwrap();
    let scope = build_query_subscription_runtime_certification_scope(
        support_report.clone(),
        parity_explanation.clone(),
        admitted_bundle.clone(),
        lifecycle_bundle.clone(),
        handle,
    )
    .unwrap();
    let (runtime_bundle, coverage_receipt) =
        certify_query_subscription_runtime_family(scope).unwrap();

    CertifiedLaneArtifacts {
        selection,
        declaration,
        lowering,
        admission,
        support_report,
        support_lookup_receipt,
        witness,
        parity_explanation,
        parity_receipt,
        lifecycle_bundle,
        admitted_trace,
        admitted_bundle,
        diagnostic_receipt,
        runtime_bundle,
        coverage_receipt,
        continuation_digest: continuation_report
            .as_ref()
            .map(|value| value.report_digest().to_string())
            .unwrap_or_else(|| "none".to_string()),
        preview_isolation_digest: preview_isolation
            .as_ref()
            .map(|value| value.isolation_digest().to_string())
            .unwrap_or_else(|| "none".to_string()),
    }
}

fn denied_support_artifacts_from_parts(
    selection: &QuerySubscriptionFamilySelection,
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    admission: &QuerySubscriptionAdmissionArtifact,
) -> SupportDeniedArtifacts {
    let failure = QuerySubscriptionDiagnosticFailure::from_support_report_error(
        &report_query_subscription_support(
            QuerySubscriptionSupportSubject::activation(
                declaration,
                &prepare_subscription_activation(admission.clone()),
            ),
            QuerySubscriptionSupportEvidence::declaration(declaration),
        )
        .unwrap_err(),
    );
    let selection_context = QuerySubscriptionDiagnosticSelectionContext::from_selection(selection);
    let denied_trace = trace_denied_query_subscription_diagnostics(
        &selection_context,
        Some(declaration),
        Some(lowering),
        Some(admission),
        None,
        failure.clone(),
    )
    .unwrap();
    let denied_bundle = bundle_denied_query_subscription_diagnostics(
        denied_trace,
        &selection_context,
        Some(declaration),
        Some(lowering),
        Some(admission),
        None,
        failure.clone(),
    )
    .unwrap()
    .0;

    SupportDeniedArtifacts {
        denied_bundle,
        failure,
    }
}

fn lifecycle_class_for(scenario: LaneScenario) -> QuerySubscriptionLifecycleCoverageClass {
    match scenario {
        LaneScenario::ActiveLifecycle => QuerySubscriptionLifecycleCoverageClass::ActiveLifecycle,
        LaneScenario::Continuation => QuerySubscriptionLifecycleCoverageClass::Continuation,
        LaneScenario::PreviewDiscard => QuerySubscriptionLifecycleCoverageClass::PreviewIsolation,
    }
}

fn compile_fail_boundary_digest(targets: &[&str]) -> String {
    digest_parts(
        &targets
            .iter()
            .map(|target| format!("compile_fail:{target}"))
            .collect::<Vec<_>>(),
    )
}

fn roomy_budget() -> QuerySubscriptionWorkBudget {
    QuerySubscriptionWorkBudget::scratch_buffer_only(8, 8, 8, 32, 1)
}

fn roomy_slice_budget() -> QuerySubscriptionSliceBudget {
    QuerySubscriptionSliceBudget::scratch_buffer_only(8, 8, 8, 8, 8, 8, 8, 8)
}

fn roomy_lowering_budget() -> QuerySubscriptionBridgeLoweringBudget {
    QuerySubscriptionBridgeLoweringBudget::admitted(1, 8, 8, 1, 1)
}

fn active_budget() -> ActiveSubscriptionWorkBudget {
    ActiveSubscriptionWorkBudget::admitted(
        ActiveRegistryLookupWidth::measured(1),
        ActiveFanoutWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::LifecycleArena,
    )
}

fn attachment_budget() -> SubscriptionConsumerAttachmentBudget {
    SubscriptionConsumerAttachmentBudget::admitted(
        ActiveFanoutWidth::measured(1),
        ConsumerDeliveryPacingWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        DeliveryBackpressurePolicy::RetainWithinWindow,
    )
}

fn delivery_budget() -> QueryDeliveryWindowBudget {
    QueryDeliveryWindowBudget::admitted(
        DeliveryWindowWidth::measured(3),
        PatchGroupWidth::measured(1),
        MaintenanceDeltaWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    )
}
