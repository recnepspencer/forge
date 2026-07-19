#[cfg(test)]
use worth_foundational::facade::CanonicalDerivedDigest;

#[cfg(test)]
use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

mod artifact;
#[cfg(test)]
mod authority_posture;
#[cfg(test)]
mod subject;

pub use artifact::{
    WorthQueryDeclarationEntryInspection, WorthQueryDeclarationEntryInspectionBridgePosture,
    WorthQueryDeclarationEntryInspectionError,
    WorthQueryDeclarationEntryInspectionRelationalPosture,
    WorthQueryDeclarationEntryInspectionSignalPosture,
};
#[cfg(test)]
pub(crate) use subject::{
    WorthQueryDeclarationEntryInspectionInput, WorthQueryDeclarationEntryRetainedSubjectInput,
};

#[cfg(test)]
pub(crate) use authority_posture::{
    envelope_bridge_summary, envelope_relational_summary, envelope_signal_summary,
};
#[cfg(test)]
pub(crate) use subject::normalize_retained_subject;
#[cfg(test)]
pub(crate) use subject::normalized_subject;

#[cfg(test)]
use super::retained_subject::ReadinessRetainedPosture;
#[cfg(test)]
use super::{digest::derive_inspection_digest, row::WorthQueryDeclarationEntryCrossingSurface};

#[cfg(test)]
pub(crate) fn worth_query_declaration_entry_inspection_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<D, C>,
    subject: WorthQueryDeclarationEntryInspectionInput<D, I>,
) -> Result<
    WorthQueryDeclarationEntryInspection<D, I>,
    WorthQueryDeclarationEntryInspectionError<D, I>,
> {
    let (subject, contribution_evidence, contribution_scope) = normalized_subject(subject);
    let readiness =
        super::support::worth_query_declaration_entry_readiness_report_with_context::<D, C, I>(
            handle,
            contribution_evidence.as_ref(),
            Some(subject.envelope.declaration_digest()),
            subject.subject_strength,
            Some(&ReadinessRetainedPosture {
                temporal_declaration_active: !subject
                    .envelope
                    .foundational_evidence()
                    .subject()
                    .canonical_declaration()
                    .temporal_clauses()
                    .is_empty(),
                async_declaration_active: !subject
                    .envelope
                    .foundational_evidence()
                    .subject()
                    .canonical_declaration()
                    .async_resource_clauses()
                    .is_empty(),
                async_resource_clauses: subject
                    .envelope
                    .foundational_evidence()
                    .subject()
                    .canonical_declaration()
                    .async_resource_clauses()
                    .to_vec(),
                envelope_aspect_publication: subject.envelope.aspect_publication().clone(),
                relational_authority_summary: subject
                    .relational
                    .as_ref()
                    .map(|posture| posture.aspect_summary().clone())
                    .unwrap_or_else(|| envelope_relational_summary(&subject.envelope)),
                bridge_authority_summary: subject
                    .bridge
                    .as_ref()
                    .map(|posture| posture.aspect_summary().clone())
                    .unwrap_or_else(|| envelope_bridge_summary(&subject.envelope)),
                signal_authority_summary: subject
                    .signal
                    .as_ref()
                    .map(|posture| posture.aspect_summary().clone())
                    .unwrap_or_else(|| envelope_signal_summary(&subject.envelope)),
            }),
            &contribution_scope,
        )
        .map_err(WorthQueryDeclarationEntryInspectionError::ContributionComposition)?;
    if subject.envelope.handle_identity_digest() != handle.handle_identity_digest()
        || subject.envelope.operating_context_identity_digest()
            != handle.operating_context_identity_digest()
    {
        return Err(WorthQueryDeclarationEntryInspectionError::new(
            subject.envelope.declaration_family_key(),
            "declaration-entry inspection requires retained seam artifacts from the same admitted handle and world",
        ));
    }
    let contribution_composition = readiness.contribution_composition().cloned();

    let matching_row_digests = readiness
        .rows()
        .iter()
        .filter(|row| row_matches_subject(row.crossing_row(), &subject))
        .map(|row| row.readiness_digest().to_string())
        .collect::<Vec<_>>();
    let mut digest_parts = vec![
        format!("family:{}", subject.envelope.declaration_family_key()),
        format!(
            "envelope:{}",
            canonical_digest_token(subject.envelope.envelope_digest())
        ),
        format!(
            "relational:{}",
            subject
                .relational
                .as_ref()
                .map(|value| value.routing_digest.as_str())
                .unwrap_or("none")
        ),
        format!(
            "bridge:{}",
            subject
                .bridge
                .as_ref()
                .map(|value| value.routing_digest.as_str())
                .unwrap_or("none")
        ),
        format!(
            "signal:{}",
            subject
                .signal
                .as_ref()
                .map(|value| value.compatibility_digest.as_str())
                .unwrap_or("none")
        ),
        format!(
            "envelope_publication:{:?}",
            subject.envelope.aspect_publication()
        ),
        format!("relational_posture:{:?}", subject.relational),
        format!("bridge_posture:{:?}", subject.bridge),
        format!("signal_posture:{:?}", subject.signal),
        format!("rows:{}", matching_row_digests.join("|")),
        format!("readiness:{}", readiness.readiness_digest()),
    ];
    if let Some(composition) = contribution_composition.as_ref() {
        digest_parts.push(format!(
            "contribution:{}",
            composition.contribution_digest()
        ));
    }
    if let Some(plan_digest) = contribution_scope.admitted_plan_digest() {
        digest_parts.push(format!("plan:{plan_digest}"));
    }
    if let Some(lower_runtime_digest) = contribution_scope.lower_runtime_boundary_digest() {
        digest_parts.push(format!("lower_runtime:{lower_runtime_digest}"));
    }
    let inspection_digest = derive_inspection_digest(&digest_parts);

    Ok(WorthQueryDeclarationEntryInspection {
        declaration_family_key: subject.envelope.declaration_family_key(),
        handle_identity_digest: subject.envelope.handle_identity_digest().to_string(),
        operating_context_identity_digest: subject
            .envelope
            .operating_context_identity_digest()
            .to_string(),
        declaration_digest: subject.envelope.declaration_digest().to_string(),
        progression_digest: subject.envelope.progression_digest().map(ToOwned::to_owned),
        route_plan_digest: subject.envelope.route_plan_digest().map(ToOwned::to_owned),
        receipt_digest: Some(canonical_digest_token(subject.envelope.receipt_digest())),
        envelope_digest: canonical_digest_token(subject.envelope.envelope_digest()),
        envelope_class: subject.envelope.class(),
        envelope_aspect_publication: subject.envelope.aspect_publication().clone(),
        evidence_origin: subject.envelope.evidence_origin(),
        route_denial_cause: subject.envelope.route_denial_cause(),
        receipt_denial_cause: subject.envelope.receipt_denial_cause(),
        route_reason: subject
            .envelope
            .explain()
            .route_governing_reason()
            .map(ToOwned::to_owned),
        receipt_reason: subject
            .envelope
            .explain()
            .receipt_governing_reason()
            .to_string(),
        relational_posture: subject.relational,
        bridge_posture: subject.bridge,
        signal_posture: subject.signal,
        contribution_composition,
        matching_row_digests,
        readiness,
        inspection_digest,
    })
}

#[cfg(test)]
fn row_matches_subject<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    row: &crate::application::WorthQueryDeclarationEntryCrossingRow,
    subject: &subject::NormalizedInspectionSubject<D, I>,
) -> bool {
    matches!(
        row.surface(),
        WorthQueryDeclarationEntryCrossingSurface::Envelope
    ) || (subject.relational.is_some()
        && matches!(
            row.surface(),
            WorthQueryDeclarationEntryCrossingSurface::RelationalTruthRouting
        ))
        || (subject.bridge.is_some()
            && matches!(
                row.surface(),
                WorthQueryDeclarationEntryCrossingSurface::BridgeContinuationRouting
            ))
        || subject.signal.as_ref().is_some_and(|signal| {
            matches!(
                row.surface(),
                WorthQueryDeclarationEntryCrossingSurface::SignalCompatibility
            ) && signal.execution_family().is_some_and(|execution_family| {
                row.signal_execution_family() == Some(execution_family)
                    && signal
                        .basis_families
                        .iter()
                        .all(|basis| row.basis_families().contains(basis))
            })
        })
}

#[cfg(test)]
fn canonical_digest_token(digest: &CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
