use super::super::super::admission::QuerySubscriptionAdmissionArtifact;
use super::super::super::bridge_lowering::BridgeSubscriptionLoweringPlan;
use super::super::super::certification::SubscriptionLifecycleCertificationBundle;
use super::super::super::closeout::SubscriptionLifecycleCloseout;
use super::super::super::continuation::SubscriptionContinuationReport;
use super::super::super::declaration::QuerySubscriptionDeclarationArtifact;
use super::super::super::preview_isolation::PreviewSubscriptionIsolationArtifact;
use super::super::super::support::QuerySubscriptionSupportReport;
use super::super::stage::QuerySubscriptionDiagnosticStage;
use super::super::trace::QuerySubscriptionDiagnosticTrace;
use super::failure::{
    QuerySubscriptionDiagnosticBundleError, QuerySubscriptionDiagnosticBundleErrorKind,
};
use super::trace_source::{validate_optional_trace_stage_source, validate_trace_stage_source};

pub(super) fn validate_admitted_trace_terminal_stage(
    trace: &QuerySubscriptionDiagnosticTrace,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if matches!(
        trace.terminal_stage(),
        QuerySubscriptionDiagnosticStage::Certification
            | QuerySubscriptionDiagnosticStage::Continuation
            | QuerySubscriptionDiagnosticStage::PreviewIsolation
            | QuerySubscriptionDiagnosticStage::LifecycleCloseout
    ) {
        Ok(())
    } else {
        Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
            "admitted diagnostic bundle assembly requires a certification-stage trace that may extend through continuation, preview, or closeout evidence",
            &[format!(
                "trace_terminal_stage:{}",
                trace.terminal_stage().as_str()
            )],
        ))
    }
}

pub(super) fn validate_admitted_trace_sources(
    trace: &QuerySubscriptionDiagnosticTrace,
    selection: &super::super::super::selection::QuerySubscriptionFamilySelection,
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    admission: &QuerySubscriptionAdmissionArtifact,
    support: &QuerySubscriptionSupportReport,
    lifecycle: &SubscriptionLifecycleCertificationBundle,
    continuation: Option<&SubscriptionContinuationReport>,
    preview: Option<&PreviewSubscriptionIsolationArtifact>,
    lifecycle_closeout: Option<&SubscriptionLifecycleCloseout>,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    validate_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::FamilySelection,
        selection.equivalence_basis().evidence_identity(),
        "admitted diagnostic bundle assembly requires family-selection trace evidence for the supplied canonical family selection",
        QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
    )?;
    validate_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::Declaration,
        declaration.declaration_identity(),
        "admitted diagnostic bundle assembly requires declaration trace evidence for the supplied canonical declaration artifact",
        QuerySubscriptionDiagnosticBundleErrorKind::DeclarationSourceMismatch,
    )?;
    validate_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
        lowering.bridge_declaration_identity(),
        "admitted diagnostic bundle assembly requires bridge-lowering trace evidence for the supplied bridge declaration artifact",
        QuerySubscriptionDiagnosticBundleErrorKind::BridgeLoweringSourceMismatch,
    )?;
    validate_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
        admission.evidence_identity(),
        "admitted diagnostic bundle assembly requires runtime-admission trace evidence for the supplied admission artifact",
        QuerySubscriptionDiagnosticBundleErrorKind::AdmissionSourceMismatch,
    )?;
    validate_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::SupportReporting,
        support.report_identity(),
        "admitted diagnostic bundle assembly requires support-reporting trace evidence for the supplied support report",
        QuerySubscriptionDiagnosticBundleErrorKind::SupportSourceMismatch,
    )?;
    validate_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::Certification,
        lifecycle.certification_bundle_identity(),
        "admitted diagnostic bundle assembly requires lifecycle-certification trace evidence for the supplied certification bundle",
        QuerySubscriptionDiagnosticBundleErrorKind::LifecycleSourceMismatch,
    )?;
    validate_optional_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::Continuation,
        continuation.map(|value| value.evidence_identity()),
        "admitted diagnostic bundle assembly may only carry continuation trace evidence when the supplied continuation artifact is present",
        QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
    )?;
    validate_optional_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::PreviewIsolation,
        preview.map(|value| value.isolation_identity()),
        "admitted diagnostic bundle assembly may only carry preview-isolation trace evidence when the supplied preview artifact is present",
        QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
    )?;
    validate_optional_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::LifecycleCloseout,
        lifecycle_closeout.map(|value| value.evidence_identity()),
        "admitted diagnostic bundle assembly may only carry lifecycle-closeout trace evidence when the supplied closeout artifact is present",
        QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
    )?;
    Ok(())
}
