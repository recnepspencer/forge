#![cfg_attr(not(test), allow(dead_code))]

use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationEntryInspection,
    ForgeQueryDeclarationEntryInspectionError, ForgeQueryDeclarationEntryInspectionInput,
    ForgeQueryDeclarationEntryProgressionError, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptChecked, ForgeQueryDeclarationRoutePlanChecked,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPostureKind,
};

use super::summaries::{
    envelope_checked_summary, receipt_checked_summary, route_checked_summary,
    KernelEnvelopeCheckedSummary, KernelReceiptCheckedSummary, KernelRouteCheckedSummary,
};

pub(crate) struct KernelCanonicalQueryWorkflowArtifactSet<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    canonical_entries: Vec<ForgeQueryDeclarationCanonicalEntry>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    route_checked: ForgeQueryDeclarationRoutePlanChecked<D, I>,
    receipt_checked: ForgeQueryDeclarationReceiptChecked<D, I>,
    envelope_checked: ForgeQueryDeclarationEnvelopeChecked<D, I>,
    ordinary_outcome: ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationEnvelope<D, I>>,
    inspection: ForgeQueryDeclarationEntryInspection<D, I>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    KernelCanonicalQueryWorkflowArtifactSet<D, I>
{
    pub(crate) fn canonical_entries(&self) -> &[ForgeQueryDeclarationCanonicalEntry] {
        &self.canonical_entries
    }

    pub(crate) fn progression(&self) -> &ForgeQueryAdmittedDeclarationProgression<D, I> {
        &self.progression
    }

    pub(crate) fn route_checked_summary(&self) -> KernelRouteCheckedSummary {
        route_checked_summary(&self.route_checked)
    }

    pub(crate) fn receipt_checked_summary(&self) -> KernelReceiptCheckedSummary {
        receipt_checked_summary(&self.receipt_checked)
    }

    pub(crate) fn envelope_checked_summary(&self) -> KernelEnvelopeCheckedSummary {
        envelope_checked_summary(&self.envelope_checked)
    }

    pub(crate) fn ordinary_outcome_label(&self) -> &'static str {
        match self.ordinary_outcome {
            ForgeQueryOrdinaryOutcome::Bound(_) => "bound",
            ForgeQueryOrdinaryOutcome::Ambiguous(_) => "ambiguous",
            ForgeQueryOrdinaryOutcome::AspectConflict(_) => "aspect_conflict",
            ForgeQueryOrdinaryOutcome::AuthorityMismatch(_) => "authority_mismatch",
            ForgeQueryOrdinaryOutcome::BasisMismatch(_) => "basis_mismatch",
            ForgeQueryOrdinaryOutcome::Deferred(_) => "deferred",
            ForgeQueryOrdinaryOutcome::Denied(_) => "denied",
            ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(_) => {
                "explicit_narrowing_required"
            }
            ForgeQueryOrdinaryOutcome::Failed(_) => "failed",
            ForgeQueryOrdinaryOutcome::MissingRequiredAspect(_) => "missing_required_aspect",
            ForgeQueryOrdinaryOutcome::RebindRequired(_) => "rebind_required",
            ForgeQueryOrdinaryOutcome::Refused(_) => "refused",
            ForgeQueryOrdinaryOutcome::Stale(_) => "stale",
            ForgeQueryOrdinaryOutcome::Unavailable(_) => "unavailable",
            ForgeQueryOrdinaryOutcome::Unsupported(_) => "unsupported",
            ForgeQueryOrdinaryOutcome::WrongHandle(_) => "wrong_handle",
            ForgeQueryOrdinaryOutcome::WrongWorld(_) => "wrong_world",
        }
    }

    pub(crate) fn ordinary_posture_kind(&self) -> Option<ForgeQueryOrdinaryPostureKind> {
        match &self.ordinary_outcome {
            ForgeQueryOrdinaryOutcome::Bound(_) => None,
            ForgeQueryOrdinaryOutcome::Ambiguous(value)
            | ForgeQueryOrdinaryOutcome::AspectConflict(value)
            | ForgeQueryOrdinaryOutcome::AuthorityMismatch(value)
            | ForgeQueryOrdinaryOutcome::BasisMismatch(value)
            | ForgeQueryOrdinaryOutcome::Deferred(value)
            | ForgeQueryOrdinaryOutcome::Denied(value)
            | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(value)
            | ForgeQueryOrdinaryOutcome::Failed(value)
            | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(value)
            | ForgeQueryOrdinaryOutcome::RebindRequired(value)
            | ForgeQueryOrdinaryOutcome::Refused(value)
            | ForgeQueryOrdinaryOutcome::Stale(value)
            | ForgeQueryOrdinaryOutcome::Unavailable(value)
            | ForgeQueryOrdinaryOutcome::Unsupported(value)
            | ForgeQueryOrdinaryOutcome::WrongHandle(value)
            | ForgeQueryOrdinaryOutcome::WrongWorld(value) => Some(value.kind()),
        }
    }

    pub(crate) fn inspection(&self) -> &ForgeQueryDeclarationEntryInspection<D, I> {
        &self.inspection
    }
}

#[allow(dead_code)]
pub(crate) enum KernelWorkflowBoundaryError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Progression(ForgeQueryDeclarationEntryProgressionError<D, I>),
    Inspection(ForgeQueryDeclarationEntryInspectionError<D, I>),
}

pub(crate) fn canonical_query_workflow_artifacts<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    entry: I,
) -> Result<KernelCanonicalQueryWorkflowArtifactSet<D, I>, KernelWorkflowBoundaryError<D, I>> {
    let canonical_entries = entry.canonical_declaration_entries();
    let progression = handle
        .declare_review_and_progress(entry.clone())
        .map_err(KernelWorkflowBoundaryError::Progression)?;
    let route_checked = handle.orchestrate_routes_from_progressed_checked(progression.clone());
    let receipt_checked = handle.orchestrate_receipt_from_progressed_checked(progression.clone());
    let envelope_checked = handle.orchestrate_envelope_from_progressed_checked(progression.clone());
    let inspection_envelope_checked =
        handle.orchestrate_envelope_from_progressed_checked(progression.clone());
    let ordinary_outcome = handle.orchestrate_declaration_entry_outcome(entry);
    let inspection = handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            inspection_envelope_checked,
        ))
        .map_err(KernelWorkflowBoundaryError::Inspection)?;

    Ok(KernelCanonicalQueryWorkflowArtifactSet {
        canonical_entries,
        progression,
        route_checked,
        receipt_checked,
        envelope_checked,
        ordinary_outcome,
        inspection,
    })
}
