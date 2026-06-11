use crate::bindings::query_native_rebinding::PrimitiveRebindingQueryDomain;
use crate::bindings::query_native_rebinding_authoring::PrimitiveRebindingDeclarationEntry;
use crate::bindings::query_native_rebinding_projection::PrimitiveRebindingProjectionFactError;
use crate::bindings::query_native_retained_geometry::{
    checked_declaration_digest, checked_envelope_digest, checked_progression_digest,
    checked_receipt_digest, checked_route_plan_digest, retained_source_digest, subject_kind_label,
    BranchLocalGeometryInspectionDeclarationFamily, PrimitiveRebindingRetainedSubject,
};
use crate::bindings::query_native_retained_view_payload::PrimitiveRebindingRetainedViewPayload;
use crate::bindings::rebinding::PrimitiveRebindingRetainedFactSource;
use forge_query::facade::{
    readmit_lower_runtime_evidence, BasisFamily, DeniedBasisCapability,
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationEntryInspection, ForgeQueryDeclarationEntryInspectionError,
    ForgeQueryDeclarationEntryInspectionInput, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDeclarationInput, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryNextStep,
    ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPostureKind, LowerRuntimeBasisEvidence,
    ScopedInspectionBasis,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRebindingBranchLocalInspectionFactReceipt {
    source: PrimitiveRebindingRetainedFactSource,
    payload: PrimitiveRebindingRetainedViewPayload,
}

impl PrimitiveRebindingBranchLocalInspectionFactReceipt {
    pub fn source(&self) -> &PrimitiveRebindingRetainedFactSource {
        &self.source
    }

    pub(crate) fn payload(&self) -> &PrimitiveRebindingRetainedViewPayload {
        &self.payload
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BranchLocalGeometryInspectionEntry {
    source: PrimitiveRebindingRetainedFactSource,
    scoped_basis: ScopedInspectionBasis,
    branch_basis_evidence: LowerRuntimeBasisEvidence,
    subject: PrimitiveRebindingRetainedSubject,
    payload: PrimitiveRebindingRetainedViewPayload,
}

impl BranchLocalGeometryInspectionEntry {
    pub fn inspect_checked<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
        subject: ForgeQueryDeclarationEnvelopeChecked<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    ) -> Result<PrimitiveRebindingBranchLocalInspection, PrimitiveRebindingBranchLocalInspectionError>
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
    {
        let bound_basis = readmit_lower_runtime_evidence(
            self.scoped_basis.clone(),
            self.branch_basis_evidence.clone(),
        )
        .map_err(PrimitiveRebindingBranchLocalInspectionError::LowerRuntimeBasis)?;
        if !matches!(
            bound_basis.scoped_basis().family(),
            BasisFamily::BranchHead | BasisFamily::BranchSnapshot
        ) {
            return Err(
                PrimitiveRebindingBranchLocalInspectionError::UnsupportedBranchBasisFamily {
                    family: bound_basis.scoped_basis().family(),
                },
            );
        }
        let _retained_entry = entry_envelope(self, handle)?;
        let branch_binding_digest = bound_basis.lower_runtime_binding_digest();
        assert_retained_subject_matches_checked(&self.subject, &subject)?;
        let inspection = handle
            .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                subject,
            ))
            .map_err(PrimitiveRebindingBranchLocalInspectionError::Inspection)?;
        if inspection.progression_digest().is_none()
            || inspection.route_plan_digest().is_none()
            || inspection.receipt_digest().is_none()
        {
            return Err(
                PrimitiveRebindingBranchLocalInspectionError::TruncatedRetainedBasis {
                    reason: "branch-local rebinding inspection requires retained progression, route-plan, and receipt truth before interpretation",
                },
            );
        }
        if self.subject.declaration_digest() != inspection.declaration_digest() {
            return Err(
                PrimitiveRebindingBranchLocalInspectionError::RetainedBasisMismatch {
                    reason: "branch-local rebinding inspection requires retained declaration truth that matches the provided retained rebinding subject",
                },
            );
        }

        let branch_local_digest = self
            .payload
            .branch_local_digest(&inspection, bound_basis.scoped_basis());
        Ok(PrimitiveRebindingBranchLocalInspection {
            inspection,
            source: self.source.clone(),
            payload: self.payload.clone(),
            branch_basis_digest: bound_basis.scoped_basis().scoped_basis_digest().to_string(),
            branch_binding_digest: branch_binding_digest.to_string(),
            branch_local_digest,
        })
    }
}

impl ForgeQueryDeclarationInput<PrimitiveRebindingQueryDomain>
    for BranchLocalGeometryInspectionEntry
{
    type Family = BranchLocalGeometryInspectionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "retained_view.kind",
                "branch_local_geometry_inspection",
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
                "branch_basis.family",
                self.scoped_basis.family().as_str(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "branch_basis.digest",
                self.scoped_basis.scoped_basis_digest(),
            ),
        ]
    }
}

pub struct PrimitiveRebindingBranchLocalInspection {
    inspection: ForgeQueryDeclarationEntryInspection<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
    source: PrimitiveRebindingRetainedFactSource,
    payload: PrimitiveRebindingRetainedViewPayload,
    branch_basis_digest: String,
    branch_binding_digest: String,
    branch_local_digest: String,
}

impl PrimitiveRebindingBranchLocalInspection {
    pub fn inspection(
        &self,
    ) -> &ForgeQueryDeclarationEntryInspection<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    > {
        &self.inspection
    }

    pub fn branch_basis_digest(&self) -> &str {
        &self.branch_basis_digest
    }

    pub fn branch_binding_digest(&self) -> &str {
        &self.branch_binding_digest
    }

    pub fn branch_local_digest(&self) -> &str {
        &self.branch_local_digest
    }

    pub fn source(&self) -> &PrimitiveRebindingRetainedFactSource {
        &self.source
    }

    pub fn receipt(&self) -> &PrimitiveRebindingRetainedViewPayload {
        &self.payload
    }

    pub fn retained_fact_receipt(&self) -> PrimitiveRebindingBranchLocalInspectionFactReceipt {
        PrimitiveRebindingBranchLocalInspectionFactReceipt {
            source: self.source.clone(),
            payload: self.payload.clone(),
        }
    }
}

pub enum PrimitiveRebindingBranchLocalInspectionError {
    RetainedFactSource(PrimitiveRebindingProjectionFactError),
    EntryOutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
    UnsupportedBranchBasisFamily {
        family: BasisFamily,
    },
    LowerRuntimeBasis(DeniedBasisCapability),
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

impl PrimitiveRebindingBranchLocalInspectionError {
    pub fn reason(&self) -> &'static str {
        match self {
            Self::RetainedFactSource(_) => {
                "branch-local rebinding inspection requires retained Query-backed rebinding facts"
            }
            Self::EntryOutcomeNotBound { .. } => {
                "branch-local geometry inspection requires an admitted retained-view declaration envelope"
            }
            Self::UnsupportedBranchBasisFamily { .. } => {
                "branch-local rebinding inspection requires a branch-head or branch-snapshot inspection basis"
            }
            Self::LowerRuntimeBasis(_) => {
                "branch-local rebinding inspection requires readmitted branch-scoped lower-runtime evidence from the same admitted branch basis"
            }
            Self::Inspection(error) => error.reason(),
            Self::RetainedBasisMismatch { reason } | Self::TruncatedRetainedBasis { reason } => {
                reason
            }
        }
    }
}

impl std::fmt::Debug for PrimitiveRebindingBranchLocalInspectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("PrimitiveRebindingBranchLocalInspectionError");
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
            Self::UnsupportedBranchBasisFamily { family } => debug.field("family", family),
            Self::LowerRuntimeBasis(denial) => debug.field("basis_denial", denial),
            Self::Inspection(error) => debug.field("inspection_reason", &error.reason()),
            Self::RetainedBasisMismatch { .. } | Self::TruncatedRetainedBasis { .. } => &mut debug,
        };
        debug.finish()
    }
}

pub fn branch_local_geometry_inspection_entry(
    source: PrimitiveRebindingRetainedFactSource,
    scoped_basis: ScopedInspectionBasis,
    branch_basis_evidence: LowerRuntimeBasisEvidence,
    subject: PrimitiveRebindingRetainedSubject,
) -> BranchLocalGeometryInspectionEntry {
    let payload = PrimitiveRebindingRetainedViewPayload::from_retained_source(&source);
    BranchLocalGeometryInspectionEntry {
        source,
        scoped_basis,
        branch_basis_evidence,
        subject,
        payload,
    }
}

fn entry_envelope<C>(
    entry: &BranchLocalGeometryInspectionEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<
    forge_query::facade::ForgeQueryDeclarationEnvelope<
        PrimitiveRebindingQueryDomain,
        BranchLocalGeometryInspectionEntry,
    >,
    PrimitiveRebindingBranchLocalInspectionError,
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
            PrimitiveRebindingBranchLocalInspectionError::EntryOutcomeNotBound {
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
) -> Result<(), PrimitiveRebindingBranchLocalInspectionError> {
    if retained_subject.declaration_digest() != checked_declaration_digest(checked)
        || retained_subject.progression_digest() != checked_progression_digest(checked)
        || retained_subject.route_plan_digest() != checked_route_plan_digest(checked)
        || retained_subject.receipt_digest() != checked_receipt_digest(checked).as_deref()
        || retained_subject.envelope_digest() != checked_envelope_digest(checked)
    {
        return Err(
            PrimitiveRebindingBranchLocalInspectionError::RetainedBasisMismatch {
                reason: "branch-local rebinding inspection requires the provided retained subject to match the retained-view declaration subject artifact",
            },
        );
    }
    Ok(())
}
