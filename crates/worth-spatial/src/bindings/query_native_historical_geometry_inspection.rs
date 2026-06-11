use crate::bindings::query_native_rebinding::PrimitiveRebindingQueryDomain;
use crate::bindings::query_native_rebinding_authoring::PrimitiveRebindingDeclarationEntry;
use crate::bindings::query_native_rebinding_projection::PrimitiveRebindingProjectionFactError;
use crate::bindings::query_native_retained_geometry::{
    checked_declaration_digest, checked_envelope_digest, checked_progression_digest,
    checked_receipt_digest, checked_route_plan_digest, retained_source_digest, subject_kind_label,
    HistoricalGeometryInspectionDeclarationFamily, PrimitiveRebindingRetainedSubject,
};
use crate::bindings::query_native_retained_view_payload::PrimitiveRebindingRetainedViewPayload;
use crate::bindings::rebinding::PrimitiveRebindingRetainedFactSource;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationEntryInspection, ForgeQueryDeclarationEntryInspectionError,
    ForgeQueryDeclarationEntryInspectionInput, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDeclarationInput, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryNextStep,
    ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPostureKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRebindingHistoricalInspectionFactReceipt {
    source: PrimitiveRebindingRetainedFactSource,
    payload: PrimitiveRebindingRetainedViewPayload,
}

impl PrimitiveRebindingHistoricalInspectionFactReceipt {
    pub fn source(&self) -> &PrimitiveRebindingRetainedFactSource {
        &self.source
    }

    pub(crate) fn payload(&self) -> &PrimitiveRebindingRetainedViewPayload {
        &self.payload
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HistoricalGeometryInspectionEntry {
    source: PrimitiveRebindingRetainedFactSource,
    subject: PrimitiveRebindingRetainedSubject,
    payload: PrimitiveRebindingRetainedViewPayload,
}

impl HistoricalGeometryInspectionEntry {
    pub fn inspect_checked<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
        subject: ForgeQueryDeclarationEnvelopeChecked<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    ) -> Result<PrimitiveRebindingHistoricalInspection, PrimitiveRebindingHistoricalInspectionError>
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
    {
        let _retained_entry = entry_envelope(self, handle)?;
        assert_retained_subject_matches_checked(&self.subject, &subject)?;
        let inspection = handle
            .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                subject,
            ))
            .map_err(PrimitiveRebindingHistoricalInspectionError::Inspection)?;
        if inspection.progression_digest().is_none()
            || inspection.route_plan_digest().is_none()
            || inspection.receipt_digest().is_none()
        {
            return Err(
                PrimitiveRebindingHistoricalInspectionError::TruncatedRetainedBasis {
                    reason: "historical rebinding inspection requires retained progression, route-plan, and receipt truth before interpretation",
                },
            );
        }
        if self.subject.declaration_digest() != inspection.declaration_digest() {
            return Err(
                PrimitiveRebindingHistoricalInspectionError::RetainedBasisMismatch {
                    reason: "historical rebinding inspection requires retained declaration truth that matches the provided retained rebinding subject",
                },
            );
        }

        let historical_digest = self.payload.historical_digest(&inspection);
        Ok(PrimitiveRebindingHistoricalInspection {
            inspection,
            source: self.source.clone(),
            payload: self.payload.clone(),
            historical_digest,
        })
    }
}

impl ForgeQueryDeclarationInput<PrimitiveRebindingQueryDomain>
    for HistoricalGeometryInspectionEntry
{
    type Family = HistoricalGeometryInspectionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "retained_view.kind",
                "historical_geometry_inspection",
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "source.retained_receipt_digest",
                retained_source_digest(&self.source),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "subject.binding_kind",
                subject_kind_label(&self.subject),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "subject.retained_declaration_digest",
                self.subject.declaration_digest(),
            ),
        ]
    }
}

pub struct PrimitiveRebindingHistoricalInspection {
    inspection: ForgeQueryDeclarationEntryInspection<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
    source: PrimitiveRebindingRetainedFactSource,
    payload: PrimitiveRebindingRetainedViewPayload,
    historical_digest: String,
}

impl PrimitiveRebindingHistoricalInspection {
    pub fn inspection(
        &self,
    ) -> &ForgeQueryDeclarationEntryInspection<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    > {
        &self.inspection
    }

    pub fn historical_digest(&self) -> &str {
        &self.historical_digest
    }

    pub fn source(&self) -> &PrimitiveRebindingRetainedFactSource {
        &self.source
    }

    pub fn receipt(&self) -> &PrimitiveRebindingRetainedViewPayload {
        &self.payload
    }

    pub fn retained_fact_receipt(&self) -> PrimitiveRebindingHistoricalInspectionFactReceipt {
        PrimitiveRebindingHistoricalInspectionFactReceipt {
            source: self.source.clone(),
            payload: self.payload.clone(),
        }
    }
}

pub enum PrimitiveRebindingHistoricalInspectionError {
    RetainedFactSource(PrimitiveRebindingProjectionFactError),
    EntryOutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
    Inspection(
        ForgeQueryDeclarationEntryInspectionError<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    ),
    RetainedBasisMismatch {
        reason: &'static str,
    },
    TruncatedRetainedBasis {
        reason: &'static str,
    },
}

impl PrimitiveRebindingHistoricalInspectionError {
    pub fn reason(&self) -> &'static str {
        match self {
            Self::RetainedFactSource(_) => {
                "historical rebinding inspection requires retained Query-backed rebinding facts"
            }
            Self::EntryOutcomeNotBound { .. } => {
                "historical geometry inspection requires an admitted retained-view declaration envelope"
            }
            Self::Inspection(error) => error.reason(),
            Self::RetainedBasisMismatch { reason } | Self::TruncatedRetainedBasis { reason } => {
                reason
            }
        }
    }
}

impl std::fmt::Debug for PrimitiveRebindingHistoricalInspectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("PrimitiveRebindingHistoricalInspectionError");
        debug.field("reason", &self.reason());
        match self {
            Self::RetainedFactSource(error) => debug.field("retained_fact_source", error),
            Self::EntryOutcomeNotBound {
                kind,
                reason,
                next_step,
            } => debug
                .field("outcome_kind", kind)
                .field("outcome_reason", reason)
                .field("next_step", next_step),
            Self::Inspection(error) => debug.field("inspection_reason", &error.reason()),
            Self::RetainedBasisMismatch { .. } | Self::TruncatedRetainedBasis { .. } => &mut debug,
        };
        debug.finish()
    }
}

pub fn historical_geometry_inspection_entry(
    source: PrimitiveRebindingRetainedFactSource,
    subject: PrimitiveRebindingRetainedSubject,
) -> HistoricalGeometryInspectionEntry {
    let payload = PrimitiveRebindingRetainedViewPayload::from_retained_source(&source);
    HistoricalGeometryInspectionEntry {
        source,
        subject,
        payload,
    }
}

fn entry_envelope<C>(
    entry: &HistoricalGeometryInspectionEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<
    forge_query::facade::ForgeQueryDeclarationEnvelope<
        PrimitiveRebindingQueryDomain,
        HistoricalGeometryInspectionEntry,
    >,
    PrimitiveRebindingHistoricalInspectionError,
>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => Ok(envelope),
        ForgeQueryOrdinaryOutcome::Ambiguous(posture)
        | ForgeQueryOrdinaryOutcome::AspectConflict(posture)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(posture)
        | ForgeQueryOrdinaryOutcome::Deferred(posture)
        | ForgeQueryOrdinaryOutcome::Denied(posture)
        | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture)
        | ForgeQueryOrdinaryOutcome::Failed(posture)
        | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(posture)
        | ForgeQueryOrdinaryOutcome::RebindRequired(posture)
        | ForgeQueryOrdinaryOutcome::Refused(posture)
        | ForgeQueryOrdinaryOutcome::Stale(posture)
        | ForgeQueryOrdinaryOutcome::Unavailable(posture)
        | ForgeQueryOrdinaryOutcome::Unsupported(posture)
        | ForgeQueryOrdinaryOutcome::WrongHandle(posture)
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => Err(
            PrimitiveRebindingHistoricalInspectionError::EntryOutcomeNotBound {
                kind: posture.kind(),
                reason: posture.reason().to_string(),
                next_step: posture.next_step(),
            },
        ),
    }
}

fn assert_retained_subject_matches_checked(
    retained_subject: &PrimitiveRebindingRetainedSubject,
    checked: &ForgeQueryDeclarationEnvelopeChecked<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> Result<(), PrimitiveRebindingHistoricalInspectionError> {
    if retained_subject.declaration_digest() != checked_declaration_digest(checked)
        || retained_subject.progression_digest() != checked_progression_digest(checked)
        || retained_subject.route_plan_digest() != checked_route_plan_digest(checked)
        || retained_subject.receipt_digest() != checked_receipt_digest(checked).as_deref()
        || retained_subject.envelope_digest() != checked_envelope_digest(checked)
    {
        return Err(
            PrimitiveRebindingHistoricalInspectionError::RetainedBasisMismatch {
                reason: "historical rebinding inspection requires the provided retained subject to match the retained-view declaration subject artifact",
            },
        );
    }
    Ok(())
}
