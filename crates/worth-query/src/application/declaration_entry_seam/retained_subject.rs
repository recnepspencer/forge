use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryAsyncDeclarationClause,
    WorthQueryDeclarationAspectPublication, WorthQueryDeclarationBridgeAuthorityAspectSummary,
    WorthQueryDeclarationEntryContributionCompositionError,
    WorthQueryDeclarationEntryContributionCompositionFailureClass,
    WorthQueryDeclarationEntryRetainedSubjectInput, WorthQueryDeclarationInput,
    WorthQueryDeclarationRelationalAuthorityAspectSummary,
    WorthQueryDeclarationSignalAuthorityAspectSummary, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext,
};

use super::contribution::WorthQueryDeclarationEntryRetainedSubjectStrength;
use super::inspection::{
    envelope_bridge_summary, envelope_relational_summary, envelope_signal_summary,
    normalize_retained_subject,
};

pub(crate) struct ReadinessReconciliation {
    pub(crate) declaration_digest: Option<String>,
    pub(crate) subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength,
    pub(crate) retained_posture: Option<ReadinessRetainedPosture>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ReadinessRetainedPosture {
    pub(crate) temporal_declaration_active: bool,
    pub(crate) async_declaration_active: bool,
    pub(crate) async_resource_clauses: Vec<WorthQueryAsyncDeclarationClause>,
    pub(crate) envelope_aspect_publication: WorthQueryDeclarationAspectPublication,
    pub(crate) relational_authority_summary: WorthQueryDeclarationRelationalAuthorityAspectSummary,
    pub(crate) bridge_authority_summary: WorthQueryDeclarationBridgeAuthorityAspectSummary,
    pub(crate) signal_authority_summary: WorthQueryDeclarationSignalAuthorityAspectSummary,
}

pub(crate) fn readiness_reconciliation_context<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    retained_subject: Option<WorthQueryDeclarationEntryRetainedSubjectInput<D, I>>,
) -> Result<ReadinessReconciliation, WorthQueryDeclarationEntryContributionCompositionError<D, I>> {
    let Some(retained_subject) = retained_subject else {
        return Ok(ReadinessReconciliation {
            declaration_digest: None,
            subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Envelope,
            retained_posture: None,
        });
    };
    let normalized = normalize_retained_subject(retained_subject);
    if normalized.envelope.handle_identity_digest() != handle.handle_identity_digest()
        || normalized.envelope.operating_context_identity_digest()
            != handle.operating_context_identity_digest()
    {
        return Err(WorthQueryDeclarationEntryContributionCompositionError::new(
            normalized.envelope.declaration_family_key(),
            WorthQueryDeclarationEntryContributionCompositionFailureClass::RetainedSubjectMismatch,
            Vec::new(),
            "declaration-entry readiness requires retained seam subjects from the same admitted handle and world",
        ));
    }

    Ok(ReadinessReconciliation {
        declaration_digest: Some(normalized.envelope.declaration_digest().to_string()),
        subject_strength: normalized.subject_strength,
        retained_posture: Some(ReadinessRetainedPosture {
            temporal_declaration_active: !normalized
                .envelope
                .foundational_evidence()
                .subject()
                .canonical_declaration()
                .temporal_clauses()
                .is_empty(),
            async_declaration_active: !normalized
                .envelope
                .foundational_evidence()
                .subject()
                .canonical_declaration()
                .async_resource_clauses()
                .is_empty(),
            async_resource_clauses: normalized
                .envelope
                .foundational_evidence()
                .subject()
                .canonical_declaration()
                .async_resource_clauses()
                .to_vec(),
            envelope_aspect_publication: normalized.envelope.aspect_publication().clone(),
            relational_authority_summary: normalized
                .relational
                .as_ref()
                .map(|posture| posture.aspect_summary().clone())
                .unwrap_or_else(|| envelope_relational_summary(&normalized.envelope)),
            bridge_authority_summary: normalized
                .bridge
                .as_ref()
                .map(|posture| posture.aspect_summary().clone())
                .unwrap_or_else(|| envelope_bridge_summary(&normalized.envelope)),
            signal_authority_summary: normalized
                .signal
                .as_ref()
                .map(|posture| posture.aspect_summary().clone())
                .unwrap_or_else(|| envelope_signal_summary(&normalized.envelope)),
        }),
    })
}
