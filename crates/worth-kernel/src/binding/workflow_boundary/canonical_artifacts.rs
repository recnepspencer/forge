#![cfg_attr(not(test), allow(dead_code))]

use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationEntryInspection,
    ForgeQueryDeclarationEntryInspectionError, ForgeQueryDeclarationEntryInspectionInput,
    ForgeQueryDeclarationEntryProgressionError, ForgeQueryDeclarationEntryReadinessReport,
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
    readiness: ForgeQueryDeclarationEntryReadinessReport<D, I>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    route_checked: ForgeQueryDeclarationRoutePlanChecked<D, I>,
    receipt_checked: ForgeQueryDeclarationReceiptChecked<D, I>,
    envelope_checked: ForgeQueryDeclarationEnvelopeChecked<D, I>,
    ordinary_outcome_label: &'static str,
    ordinary_posture_kind: Option<ForgeQueryOrdinaryPostureKind>,
    inspection: ForgeQueryDeclarationEntryInspection<D, I>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    KernelCanonicalQueryWorkflowArtifactSet<D, I>
{
    pub(crate) fn canonical_entries(&self) -> &[ForgeQueryDeclarationCanonicalEntry] {
        &self.canonical_entries
    }

    pub(crate) fn readiness(&self) -> &ForgeQueryDeclarationEntryReadinessReport<D, I> {
        &self.readiness
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
        self.ordinary_outcome_label
    }

    pub(crate) fn ordinary_posture_kind(&self) -> Option<ForgeQueryOrdinaryPostureKind> {
        self.ordinary_posture_kind
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
    let ordinary_outcome = handle.orchestrate_declaration_entry_outcome(entry.clone());
    let (ordinary_outcome_label, ordinary_posture_kind) = ordinary_outcome_shape(&ordinary_outcome);
    canonical_query_workflow_artifacts_with_ordinary_shape(
        handle,
        entry,
        ordinary_outcome_label,
        ordinary_posture_kind,
    )
}

pub(crate) fn canonical_query_workflow_artifacts_with_ordinary_shape<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    entry: I,
    ordinary_outcome_label: &'static str,
    ordinary_posture_kind: Option<ForgeQueryOrdinaryPostureKind>,
) -> Result<KernelCanonicalQueryWorkflowArtifactSet<D, I>, KernelWorkflowBoundaryError<D, I>> {
    let canonical_entries = entry.canonical_declaration_entries();
    let readiness = handle.declaration_entry_readiness::<I>();
    let progression = handle
        .declare_review_and_progress(entry.clone())
        .map_err(KernelWorkflowBoundaryError::Progression)?;
    let route_checked = handle.orchestrate_routes_from_progressed_checked(progression.clone());
    let receipt_checked = handle.orchestrate_receipt_from_progressed_checked(progression.clone());
    let envelope_checked = handle.orchestrate_envelope_from_progressed_checked(progression.clone());
    let inspection_envelope_checked =
        handle.orchestrate_envelope_from_progressed_checked(progression.clone());
    let inspection = handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            inspection_envelope_checked,
        ))
        .map_err(KernelWorkflowBoundaryError::Inspection)?;

    Ok(KernelCanonicalQueryWorkflowArtifactSet {
        canonical_entries,
        readiness,
        progression,
        route_checked,
        receipt_checked,
        envelope_checked,
        ordinary_outcome_label,
        ordinary_posture_kind,
        inspection,
    })
}

pub(crate) fn ordinary_outcome_shape<T>(
    outcome: &ForgeQueryOrdinaryOutcome<T>,
) -> (&'static str, Option<ForgeQueryOrdinaryPostureKind>) {
    match outcome {
        ForgeQueryOrdinaryOutcome::Bound(_) => ("bound", None),
        ForgeQueryOrdinaryOutcome::Ambiguous(value) => ("ambiguous", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::AspectConflict(value) => ("aspect_conflict", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::AuthorityMismatch(value) => {
            ("authority_mismatch", Some(value.kind()))
        }
        ForgeQueryOrdinaryOutcome::BasisMismatch(value) => ("basis_mismatch", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::Deferred(value) => ("deferred", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::Denied(value) => ("denied", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(value) => {
            ("explicit_narrowing_required", Some(value.kind()))
        }
        ForgeQueryOrdinaryOutcome::Failed(value) => ("failed", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::MissingRequiredAspect(value) => {
            ("missing_required_aspect", Some(value.kind()))
        }
        ForgeQueryOrdinaryOutcome::RebindRequired(value) => ("rebind_required", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::Refused(value) => ("refused", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::Stale(value) => ("stale", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::Unavailable(value) => ("unavailable", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::Unsupported(value) => ("unsupported", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::WrongHandle(value) => ("wrong_handle", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::WrongWorld(value) => ("wrong_world", Some(value.kind())),
    }
}
