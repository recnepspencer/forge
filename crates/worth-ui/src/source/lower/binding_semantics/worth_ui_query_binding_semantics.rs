use crate::capability::FrozenViewBindingEntry;
use crate::source::{
    WorthUiBindingDiagnostic, WorthUiBindingDiagnosticCode, WorthUiBoundQueryViewSemantics,
};

use super::worth_ui_binding_semantics_context::WorthUiBindingSemanticsContext;

pub(super) fn bind_query_view_semantics(
    module_id: &crate::source::WorthUiSourceModuleId,
    entry: &FrozenViewBindingEntry,
    semantic_locus: &str,
    provenance: &crate::source::WorthUiArtifactInputProvenance,
    context: &mut WorthUiBindingSemanticsContext<'_>,
) -> Result<WorthUiBoundQueryViewSemantics, WorthUiBindingDiagnostic> {
    context.record_query_owned_semantic_check();
    let descriptor = entry.descriptor();

    if descriptor.has_local_pseudo_query_claim() {
        return Err(WorthUiBindingDiagnostic::new(
            WorthUiBindingDiagnosticCode::LocalPseudoQueryClaimRejected,
            module_id.clone(),
            descriptor.id().as_str(),
            semantic_locus,
            provenance.clone(),
        ));
    }

    let query_capability = descriptor.query_capability().cloned().ok_or_else(|| {
        WorthUiBindingDiagnostic::new(
            WorthUiBindingDiagnosticCode::MissingQueryCapabilityPosture,
            module_id.clone(),
            descriptor.id().as_str(),
            semantic_locus,
            provenance.clone(),
        )
    })?;
    let query_composition_profile_digest = descriptor
        .query_composition_profile_digest()
        .ok_or_else(|| {
            WorthUiBindingDiagnostic::new(
                WorthUiBindingDiagnosticCode::MissingQueryCompositionSupportProfile,
                module_id.clone(),
                descriptor.id().as_str(),
                semantic_locus,
                provenance.clone(),
            )
        })?
        .to_owned();
    let view_shape = descriptor.view_shape().cloned().ok_or_else(|| {
        WorthUiBindingDiagnostic::new(
            WorthUiBindingDiagnosticCode::MissingQueryViewShape,
            module_id.clone(),
            descriptor.id().as_str(),
            semantic_locus,
            provenance.clone(),
        )
    })?;
    let result_shape = descriptor.result_shape().cloned().ok_or_else(|| {
        WorthUiBindingDiagnostic::new(
            WorthUiBindingDiagnosticCode::MissingQueryResultShape,
            module_id.clone(),
            descriptor.id().as_str(),
            semantic_locus,
            provenance.clone(),
        )
    })?;
    let basis_posture = descriptor.basis_posture().cloned().ok_or_else(|| {
        WorthUiBindingDiagnostic::new(
            WorthUiBindingDiagnosticCode::MissingQueryBasisPosture,
            module_id.clone(),
            descriptor.id().as_str(),
            semantic_locus,
            provenance.clone(),
        )
    })?;
    let live_compatibility = descriptor.live_compatibility().cloned().ok_or_else(|| {
        WorthUiBindingDiagnostic::new(
            WorthUiBindingDiagnosticCode::MissingQueryLiveCompatibility,
            module_id.clone(),
            descriptor.id().as_str(),
            semantic_locus,
            provenance.clone(),
        )
    })?;
    let denial_presentation = descriptor.denial_presentation().cloned().ok_or_else(|| {
        WorthUiBindingDiagnostic::new(
            WorthUiBindingDiagnosticCode::MissingQueryDenialPresentation,
            module_id.clone(),
            descriptor.id().as_str(),
            semantic_locus,
            provenance.clone(),
        )
    })?;

    Ok(WorthUiBoundQueryViewSemantics::new(
        query_capability,
        query_composition_profile_digest,
        view_shape,
        result_shape,
        basis_posture,
        live_compatibility,
        denial_presentation,
    ))
}
