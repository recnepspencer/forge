use super::super::super::admission::QuerySubscriptionAdmissionArtifact;
use super::super::super::bridge_lowering::BridgeSubscriptionLoweringPlan;
use super::super::super::certification::SubscriptionLifecycleCertificationBundle;
use super::super::super::declaration::QuerySubscriptionDeclarationArtifact;
use super::super::super::evidence_identities::typed_identity_drift;
use super::super::super::support::QuerySubscriptionSupportReport;
use super::super::context::QuerySubscriptionDiagnosticSelectionContext;
use super::failure::{
    QuerySubscriptionDiagnosticBundleError, QuerySubscriptionDiagnosticBundleErrorKind,
};

pub(super) fn validate_selection_and_declaration(
    selection: &QuerySubscriptionDiagnosticSelectionContext,
    declaration: &QuerySubscriptionDeclarationArtifact,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if selection.selection().map(|value| value.family()) != Some(declaration.family()) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::DeclarationSourceMismatch,
            "diagnostic bundle assembly requires declaration and family selection to preserve the same query subscription family",
            &[
                format!("selection_family:{}", selection.query_family_label()),
                format!("declaration_family:{}", declaration.family().as_str()),
            ],
        ));
    }
    Ok(())
}

pub(super) fn validate_declaration_and_lowering(
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if typed_identity_drift(
        lowering.query_declaration_identity(),
        declaration.declaration_identity(),
    ) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::BridgeLoweringSourceMismatch,
            "diagnostic bundle assembly requires bridge lowering to bind the same declaration artifact",
            &[
                format!("declaration:{}", declaration.declaration_projection().label()),
                format!("lowering:{}", lowering.query_declaration_projection().label()),
            ],
        ));
    }
    Ok(())
}

pub(super) fn validate_declaration_and_admission(
    declaration: &QuerySubscriptionDeclarationArtifact,
    admission: &QuerySubscriptionAdmissionArtifact,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if typed_identity_drift(
        admission.query_declaration_identity(),
        declaration.declaration_identity(),
    ) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::AdmissionSourceMismatch,
            "diagnostic bundle assembly requires admission and declaration to preserve the same canonical declaration digest",
            &[
                format!("declaration:{}", declaration.declaration_projection().label()),
                format!("admission:{}", admission.query_declaration_projection().label()),
            ],
        ));
    }
    Ok(())
}

pub(super) fn validate_declaration_and_support(
    declaration: &QuerySubscriptionDeclarationArtifact,
    support: &QuerySubscriptionSupportReport,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if typed_identity_drift(
        support.support_subject().declaration_identity(),
        declaration.declaration_identity(),
    ) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::SupportSourceMismatch,
            "diagnostic bundle assembly requires support reporting to bind the same declaration artifact",
            &[
                format!("declaration:{}", declaration.declaration_projection().label()),
                format!(
                    "support_declaration:{}",
                    support.support_subject().declaration_projection().label()
                ),
            ],
        ));
    }
    Ok(())
}

pub(super) fn validate_admitted_sources(
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    lifecycle: &SubscriptionLifecycleCertificationBundle,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if typed_identity_drift(
        lifecycle.subscription_declaration_identity(),
        declaration.declaration_identity(),
    ) || typed_identity_drift(
        lifecycle.bridge_declaration_identity(),
        lowering.bridge_declaration_identity(),
    ) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::LifecycleSourceMismatch,
            "diagnostic bundle assembly requires lifecycle certification to preserve declaration and bridge lowering identity",
            &[
                format!("declaration:{}", declaration.declaration_projection().label()),
                format!(
                    "lifecycle_declaration:{}",
                    lifecycle.subscription_declaration_projection().label()
                ),
                format!("bridge:{}", lowering.bridge_declaration_projection().label()),
                format!(
                    "lifecycle_bridge:{}",
                    lifecycle.bridge_declaration_projection().label()
                ),
            ],
        ));
    }
    Ok(())
}
