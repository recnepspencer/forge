use forge_foundational::facade::CanonicalDerivedDigest;

use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::{
    contribution_reconciliation::{
        reconcile_contribution_evidence,
        ForgeQueryDeclarationEntryContributionReconciliationContext,
    },
    digest::derive_inspection_digest,
    inspection_artifact::{
        ForgeQueryDeclarationEntryInspection, ForgeQueryDeclarationEntryInspectionError,
    },
    inventory::forge_query_declaration_entry_crossing_inventory,
    row::ForgeQueryDeclarationEntryCrossingSurface,
    subject::{normalized_subject, ForgeQueryDeclarationEntryInspectionInput},
};

pub(crate) fn forge_query_declaration_entry_inspection_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    subject: ForgeQueryDeclarationEntryInspectionInput<D, I>,
) -> Result<
    ForgeQueryDeclarationEntryInspection<D, I>,
    ForgeQueryDeclarationEntryInspectionError<D, I>,
> {
    let (subject, contribution_evidence, contribution_scope) = normalized_subject(subject);
    let readiness =
        super::support::forge_query_declaration_entry_readiness_report_with_context::<D, C, I>(
            handle,
            contribution_evidence.as_ref(),
            Some(subject.envelope.declaration_digest()),
            subject.subject_strength,
            &contribution_scope,
        )
        .map_err(ForgeQueryDeclarationEntryInspectionError::ContributionComposition)?;
    let inventory = forge_query_declaration_entry_crossing_inventory::<D, C, I>(handle);
    if subject.envelope.handle_identity_digest() != handle.handle_identity_digest()
        || subject.envelope.operating_context_identity_digest()
            != handle.operating_context_identity_digest()
    {
        return Err(ForgeQueryDeclarationEntryInspectionError::new(
            subject.envelope.declaration_family_key(),
            "declaration-entry inspection requires retained seam artifacts from the same admitted handle and world",
        ));
    }
    let contribution_composition = reconcile_contribution_evidence::<D, I>(
        ForgeQueryDeclarationEntryContributionReconciliationContext {
            declaration_family_key: subject.envelope.declaration_family_key(),
            declaration_digest: Some(subject.envelope.declaration_digest()),
            subject_strength: subject.subject_strength,
            admitted_plan_digest: contribution_scope.admitted_plan_digest(),
            lower_runtime_boundary_digest: contribution_scope.lower_runtime_boundary_digest(),
        },
        contribution_evidence.as_ref(),
    )
    .map_err(ForgeQueryDeclarationEntryInspectionError::ContributionComposition)?;

    let matching_row_digests = inventory
        .rows()
        .iter()
        .filter(|row| row_matches_subject(row, &subject))
        .map(|row| row.row_digest().to_string())
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

    Ok(ForgeQueryDeclarationEntryInspection {
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

fn row_matches_subject<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    row: &crate::application::ForgeQueryDeclarationEntryCrossingRow,
    subject: &super::subject::NormalizedInspectionSubject<D, I>,
) -> bool {
    matches!(
        row.surface(),
        ForgeQueryDeclarationEntryCrossingSurface::Envelope
    ) || (subject.relational.is_some()
        && matches!(
            row.surface(),
            ForgeQueryDeclarationEntryCrossingSurface::RelationalTruthRouting
        ))
        || (subject.bridge.is_some()
            && matches!(
                row.surface(),
                ForgeQueryDeclarationEntryCrossingSurface::BridgeContinuationRouting
            ))
        || subject.signal.as_ref().is_some_and(|signal| {
            matches!(
                row.surface(),
                ForgeQueryDeclarationEntryCrossingSurface::SignalCompatibility
            ) && row.signal_execution_family() == Some(signal.execution_family)
                && signal
                    .basis_families
                    .iter()
                    .all(|basis| row.basis_families().contains(basis))
        })
}

fn canonical_digest_token(digest: &CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
