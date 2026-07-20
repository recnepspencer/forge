use super::materialization::{
    materialize_admission_summary, materialize_admission_support_report,
    materialize_admission_trace_artifact, materialize_continuity_explanation_bundle,
    materialize_continuity_summary, materialize_explanation_explanation_bundle,
    materialize_explanation_trace_artifact, materialize_invariant_capability_support_report,
    materialize_support_traceability_support_report,
    materialize_support_traceability_trace_artifact, materialize_workflow_support_report,
};
use super::payloads::{
    WorthQueryAdmissionContributionPayload, WorthQueryAdmissionContributionPosture,
    WorthQueryContinuityContributionPayload, WorthQueryContinuityContributionPosture,
    WorthQueryExplanationContributionPayload, WorthQueryExplanationContributionPosture,
    WorthQueryInvariantCapabilityContributionPayload,
    WorthQueryInvariantCapabilityContributionPosture, WorthQuerySupportContributionPayload,
    WorthQuerySupportContributionPosture, WorthQueryWorkflowContributionPayload,
    WorthQueryWorkflowContributionPosture,
};
use super::targets::{
    WorthQueryAdmittedPlanBoundContributionTarget, WorthQueryDeclarationBoundContributionTarget,
    WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use super::test_support::{
    admitted_plan_target, declaration_target, lower_runtime_target, ready_payload,
};
use worth_foundational::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalDiagnosticDeliveryClass,
    FoundationalDiagnosticOutcomeKind, FoundationalProfileSet, FoundationalProfileSetInput,
    RetentionDeliveryProfile, SupportPostureProfile,
};

#[test]
fn admission_support_report_materializes_foundational_rows_and_fresh_provenance() {
    let report_ready = declaration_ready_contribution(
        "intent-a",
        WorthQueryAdmissionContributionPayload::new(
            WorthQueryAdmissionContributionPosture::Advisory,
            "spatial.arbitration.requires_clarification",
            "multiple candidates remain admissible",
        ),
    );

    let report = materialize_admission_support_report(
        report_ready,
        forensic_support_profile(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .expect("support report should materialize");

    assert_eq!(
        report.report().outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Advisory
    );
    assert_eq!(report.report().decision_rows().count(), 1);
    assert_eq!(report.report().support_rows().count(), 2);
    assert_eq!(report.report().provenance_ready_rows().count(), 1);
    assert_eq!(
        report.provenance().freshness_posture(),
        worth_foundational::FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained
    );
    assert!(report.provenance().strategy_basis().is_none());
    assert_eq!(
        report
            .profile_progression()
            .materialized()
            .payload()
            .materialized()
            .diagnostic_richness(),
        DiagnosticRichnessProfile::Forensic
    );

    let summary_ready = declaration_ready_contribution(
        "intent-a",
        WorthQueryAdmissionContributionPayload::new(
            WorthQueryAdmissionContributionPosture::Advisory,
            "spatial.arbitration.requires_clarification",
            "multiple candidates remain admissible",
        ),
    );
    let summary = materialize_admission_summary(summary_ready, forensic_support_profile())
        .expect("summary should materialize");
    assert_eq!(
        summary.outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Advisory
    );
    assert_eq!(
        summary.provenance().freshness_posture(),
        worth_foundational::FoundationalBoundaryEvidenceFreshnessPosture::ReducedRetained
    );
    assert_eq!(
        summary
            .profile_progression()
            .materialized()
            .payload()
            .materialized()
            .diagnostic_richness(),
        DiagnosticRichnessProfile::Standard
    );
    assert_eq!(summary.required_row_count(), 2);
    assert_eq!(summary.standard_row_count(), 1);
    assert_eq!(summary.forensic_row_count(), 1);

    let trace_ready = declaration_ready_contribution(
        "intent-a",
        WorthQueryAdmissionContributionPayload::new(
            WorthQueryAdmissionContributionPosture::Advisory,
            "spatial.arbitration.requires_clarification",
            "multiple candidates remain admissible",
        ),
    );
    let trace = materialize_admission_trace_artifact(trace_ready, forensic_support_profile())
        .expect("trace artifact should materialize");
    assert_eq!(
        trace.outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Advisory
    );
    assert_eq!(trace.required_rows().len(), 2);
    assert_eq!(trace.standard_rows().len(), 1);
    assert_eq!(trace.forensic_rows().len(), 1);
    assert_eq!(
        trace.provenance().freshness_posture(),
        worth_foundational::FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained
    );
}

#[test]
fn explanation_bundle_narrows_support_posture_to_internal_only() {
    let bundle_ready = declaration_ready_contribution(
        "intent-b",
        WorthQueryExplanationContributionPayload::new(
            WorthQueryExplanationContributionPosture::ExplainsFallback,
            "spatial.reorient.fallback",
            "used canonical perpendicular fallback",
        ),
    );

    let bundle = materialize_explanation_explanation_bundle(
        bundle_ready,
        forensic_support_profile(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .expect("explanation bundle should materialize");

    assert_eq!(
        bundle
            .profile_progression()
            .admitted()
            .payload()
            .admitted()
            .support_posture(),
        SupportPostureProfile::InternalOnly
    );
    assert_eq!(
        bundle
            .profile_progression()
            .materialized()
            .payload()
            .materialized()
            .support_posture(),
        SupportPostureProfile::InternalOnly
    );

    let trace_ready = declaration_ready_contribution(
        "intent-b",
        WorthQueryExplanationContributionPayload::new(
            WorthQueryExplanationContributionPosture::ExplainsFallback,
            "spatial.reorient.fallback",
            "used canonical perpendicular fallback",
        ),
    );
    let trace = materialize_explanation_trace_artifact(trace_ready, forensic_support_profile())
        .expect("trace artifact should materialize");
    assert_eq!(
        trace
            .profile_progression()
            .materialized()
            .payload()
            .materialized()
            .support_posture(),
        SupportPostureProfile::InternalOnly
    );
    assert_eq!(trace.required_rows().len(), 2);
    assert_eq!(trace.standard_rows().len(), 1);
    assert_eq!(trace.forensic_rows().len(), 1);
}

#[test]
fn lower_runtime_support_report_preserves_requested_richness_and_current_provenance() {
    let ready = lower_runtime_ready_contribution(
        "boundary-a",
        WorthQuerySupportContributionPayload::new(
            WorthQuerySupportContributionPosture::DeclarationTraceability,
            "runtime.boundary.traceability",
            "lower runtime support derives from reconstructed boundary evidence",
        ),
    );

    let report = materialize_support_traceability_support_report(
        ready,
        forensic_support_profile(),
        FoundationalDiagnosticDeliveryClass::ReconstructableFromReplay,
    )
    .expect("support report should materialize");

    assert_eq!(
        report
            .profile_progression()
            .materialized()
            .payload()
            .materialized()
            .diagnostic_richness(),
        DiagnosticRichnessProfile::Forensic
    );
    assert_eq!(
        report.provenance().locality(),
        worth_foundational::FoundationalBoundaryEvidenceLocality::ReplayDerived
    );
    assert_eq!(
        report.provenance().freshness_posture(),
        worth_foundational::FoundationalBoundaryEvidenceFreshnessPosture::ReconstructedFromReplay
    );
    assert!(report.provenance().strategy_basis().is_some());
    assert!(report
        .report()
        .support_rows()
        .all(|row| row.outcome_kind() == FoundationalDiagnosticOutcomeKind::Accepted));
    assert!(report.report().support_rows().all(|row| matches!(
        row.evidence_posture(),
        worth_foundational::FoundationalDiagnosticSupportEvidencePosture::Present(
            worth_foundational::FoundationalDiagnosticEvidencePosture::Summarized
        )
    )));

    let trace = materialize_support_traceability_trace_artifact(
        lower_runtime_ready_contribution(
            "boundary-a",
            WorthQuerySupportContributionPayload::new(
                WorthQuerySupportContributionPosture::DeclarationTraceability,
                "runtime.boundary.traceability",
                "lower runtime support derives from reconstructed boundary evidence",
            ),
        ),
        forensic_support_profile(),
    )
    .expect("trace artifact should materialize");
    assert_eq!(
        trace.provenance().freshness_posture(),
        worth_foundational::FoundationalBoundaryEvidenceFreshnessPosture::ReconstructedFromReplay
    );
}

#[test]
fn remaining_categories_materialize_real_descriptive_outputs() {
    let invariant_ready = declaration_ready_contribution(
        "intent-c",
        WorthQueryInvariantCapabilityContributionPayload::new(
            WorthQueryInvariantCapabilityContributionPosture::CapabilityGap,
            "graph.face_inner_loop_insertion",
            "topology substrate is not available",
        ),
    );
    let workflow_ready = admitted_plan_ready_contribution(
        "plan-a",
        WorthQueryWorkflowContributionPayload::new(
            WorthQueryWorkflowContributionPosture::ConfirmationRequired,
            "preview.confirmation.required",
            "destructive promotion requires confirmation",
        ),
    );
    let continuity_bundle_ready = admitted_plan_ready_contribution(
        "plan-b",
        WorthQueryContinuityContributionPayload::new(
            WorthQueryContinuityContributionPosture::Split,
            "continuity.identity.split",
            "edge split produces two descendant identities",
        ),
    );

    let invariant_report = materialize_invariant_capability_support_report(
        invariant_ready,
        standard_profile(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .expect("invariant report");
    let workflow_report = materialize_workflow_support_report(
        workflow_ready,
        standard_profile(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .expect("workflow report");
    let continuity_bundle = materialize_continuity_explanation_bundle(
        continuity_bundle_ready,
        standard_profile(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .expect("continuity bundle");
    let continuity_summary_ready = admitted_plan_ready_contribution(
        "plan-b",
        WorthQueryContinuityContributionPayload::new(
            WorthQueryContinuityContributionPosture::Split,
            "continuity.identity.split",
            "edge split produces two descendant identities",
        ),
    );
    let continuity_summary =
        materialize_continuity_summary(continuity_summary_ready, standard_profile())
            .expect("continuity summary");

    assert_eq!(
        invariant_report.report().outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Unsupported
    );
    assert_eq!(
        workflow_report.report().outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Advisory
    );
    assert!(workflow_report.provenance().strategy_basis().is_some());
    assert_eq!(
        continuity_bundle.bundle().outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Advisory
    );
    assert!(continuity_bundle.provenance().strategy_basis().is_some());
    assert_eq!(
        continuity_summary.outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Advisory
    );
}

fn declaration_ready_contribution<P>(
    target_digest: &str,
    payload: P,
) -> super::WorthQueryMaterializationReadyDomainCapabilityContribution<
    P,
    WorthQueryDeclarationBoundContributionTarget,
>
where
    P: super::WorthQueryDomainCapabilityPayload,
    (P, WorthQueryDeclarationBoundContributionTarget):
        super::proof_integration::AllowedContributionBinding<
            P,
            WorthQueryDeclarationBoundContributionTarget,
        >,
{
    ready_payload(declaration_target(target_digest), payload)
}

fn admitted_plan_ready_contribution<P>(
    target_digest: &str,
    payload: P,
) -> super::WorthQueryMaterializationReadyDomainCapabilityContribution<
    P,
    WorthQueryAdmittedPlanBoundContributionTarget,
>
where
    P: super::WorthQueryDomainCapabilityPayload,
    (P, WorthQueryAdmittedPlanBoundContributionTarget):
        super::proof_integration::AllowedContributionBinding<
            P,
            WorthQueryAdmittedPlanBoundContributionTarget,
        >,
{
    ready_payload(admitted_plan_target(target_digest), payload)
}

fn lower_runtime_ready_contribution<P>(
    target_digest: &str,
    payload: P,
) -> super::WorthQueryMaterializationReadyDomainCapabilityContribution<
    P,
    WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
>
where
    P: super::WorthQueryDomainCapabilityPayload,
    (P, WorthQueryLowerRuntimeBoundaryBoundContributionTarget):
        super::proof_integration::AllowedContributionBinding<
            P,
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
{
    ready_payload(lower_runtime_target(target_digest), payload)
}

fn forensic_support_profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Forensic,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::Uncertified,
    })
    .expect("valid profile")
}

fn standard_profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Standard,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::Uncertified,
    })
    .expect("valid profile")
}
