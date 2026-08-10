use super::super::super::admission::QuerySubscriptionAdmissionArtifact;
use super::super::super::bridge_lowering::BridgeSubscriptionLoweringPlan;
use super::super::super::declaration::QuerySubscriptionDeclarationArtifact;
use super::super::super::support::QuerySubscriptionSupportReport;
use super::super::context::QuerySubscriptionDiagnosticSelectionContext;
use super::super::stage::QuerySubscriptionDiagnosticStage;
use super::super::trace::QuerySubscriptionDiagnosticTrace;
use super::failure::{
    QuerySubscriptionDiagnosticBundleError, QuerySubscriptionDiagnosticBundleErrorKind,
    QuerySubscriptionDiagnosticFailure,
};
use super::trace_source::{validate_optional_trace_stage_source, validate_trace_stage_source};

pub(super) fn validate_denied_trace_sources(
    trace: &QuerySubscriptionDiagnosticTrace,
    selection_context: &QuerySubscriptionDiagnosticSelectionContext,
    declaration: Option<&QuerySubscriptionDeclarationArtifact>,
    lowering: Option<&BridgeSubscriptionLoweringPlan>,
    admission: Option<&QuerySubscriptionAdmissionArtifact>,
    support: Option<&QuerySubscriptionSupportReport>,
    failure: &QuerySubscriptionDiagnosticFailure,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    let selection_source_identity = selection_context.source_identity();
    let failure_source_identity = failure.source_identity();

    validate_trace_stage_source(
        trace,
        if selection_context.is_selection_denied() {
            *failure.stage()
        } else {
            QuerySubscriptionDiagnosticStage::FamilySelection
        },
        if selection_context.is_selection_denied() {
            failure_source_identity
        } else {
            &selection_source_identity
        },
        "denied diagnostic bundle assembly requires trace family-selection evidence for the supplied selection context",
        QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch,
    )?;

    if let Some(declaration) = declaration {
        validate_trace_stage_source(
            trace,
            QuerySubscriptionDiagnosticStage::Declaration,
            if matches!(
                failure.stage(),
                QuerySubscriptionDiagnosticStage::Declaration
                    | QuerySubscriptionDiagnosticStage::DeliveryIntent
            ) {
                failure_source_identity
            } else {
                declaration.declaration_identity()
            },
            "denied diagnostic bundle assembly requires declaration trace evidence aligned with the supplied declaration or declaration-stage failure",
            QuerySubscriptionDiagnosticBundleErrorKind::DeclarationSourceMismatch,
        )?;
    } else {
        validate_optional_trace_stage_source(
            trace,
            QuerySubscriptionDiagnosticStage::Declaration,
            if matches!(
                failure.stage(),
                QuerySubscriptionDiagnosticStage::Declaration
                    | QuerySubscriptionDiagnosticStage::DeliveryIntent
            ) {
                Some(failure_source_identity)
            } else {
                None
            },
            "diagnostic bundle assembly requires the trace to carry every stage that the assembled artifacts claim",
            QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
        )?;
    }

    if let Some(lowering) = lowering {
        validate_trace_stage_source(
            trace,
            QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
            if matches!(
                failure.stage(),
                QuerySubscriptionDiagnosticStage::BridgeFamilyLowering
                    | QuerySubscriptionDiagnosticStage::BridgeSliceLowering
                    | QuerySubscriptionDiagnosticStage::BasisBinding
            ) {
                failure_source_identity
            } else {
                lowering.bridge_declaration_identity()
            },
            "denied diagnostic bundle assembly requires bridge-lowering trace evidence aligned with the supplied lowering artifact or bridge-stage failure",
            QuerySubscriptionDiagnosticBundleErrorKind::BridgeLoweringSourceMismatch,
        )?;
    } else {
        validate_optional_trace_stage_source(
            trace,
            QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
            if matches!(
                failure.stage(),
                QuerySubscriptionDiagnosticStage::BridgeFamilyLowering
                    | QuerySubscriptionDiagnosticStage::BridgeSliceLowering
                    | QuerySubscriptionDiagnosticStage::BasisBinding
            ) {
                Some(failure_source_identity)
            } else {
                None
            },
            "diagnostic bundle assembly requires the trace to carry every stage that the assembled artifacts claim",
            QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
        )?;
    }

    if let Some(admission) = admission {
        validate_trace_stage_source(
            trace,
            QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
            if matches!(
                failure.stage(),
                QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission
                    | QuerySubscriptionDiagnosticStage::AdmissionBudget
                    | QuerySubscriptionDiagnosticStage::DurableReloadOverclaim
                    | QuerySubscriptionDiagnosticStage::ActiveLifecycleAllocation
                    | QuerySubscriptionDiagnosticStage::ActivationReadiness
            ) {
                failure_source_identity
            } else {
                admission.evidence_identity()
            },
            "denied diagnostic bundle assembly requires runtime-admission trace evidence aligned with the supplied admission artifact or admission-stage failure",
            QuerySubscriptionDiagnosticBundleErrorKind::AdmissionSourceMismatch,
        )?;
    } else {
        validate_optional_trace_stage_source(
            trace,
            QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
            if matches!(
                failure.stage(),
                QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission
                    | QuerySubscriptionDiagnosticStage::AdmissionBudget
                    | QuerySubscriptionDiagnosticStage::DurableReloadOverclaim
                    | QuerySubscriptionDiagnosticStage::ActiveLifecycleAllocation
                    | QuerySubscriptionDiagnosticStage::ActivationReadiness
            ) {
                Some(failure_source_identity)
            } else {
                None
            },
            "diagnostic bundle assembly requires the trace to carry every stage that the assembled artifacts claim",
            QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
        )?;
    }

    validate_optional_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::SupportReporting,
        if *failure.stage() == QuerySubscriptionDiagnosticStage::SupportReporting {
            Some(failure_source_identity)
        } else {
            support.map(|value| value.report_identity())
        },
        "denied diagnostic bundle assembly may only carry support-reporting trace evidence when the supplied support report is present",
        QuerySubscriptionDiagnosticBundleErrorKind::SupportSourceMismatch,
    )?;
    Ok(())
}
