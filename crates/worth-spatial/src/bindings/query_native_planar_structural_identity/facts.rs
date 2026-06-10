use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use crate::bindings::query_native_planar_structural_identity::authoring::PlanarStructuralIdentityEntry;
use crate::bindings::query_native_planar_structural_identity::domain::PlanarStructuralIdentityQueryDomain;
use crate::bindings::query_native_planar_structural_identity::inspection::{
    PlanarStructuralIdentityInspectionKind, PlanarStructuralIdentityInspectionRow,
};
use crate::planar_contracts::structural_identity::{
    PlanarStructuralIdentityCounters, PlanarStructuralIdentityReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PlanarStructuralIdentityFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl PlanarStructuralIdentityFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn planar_structural_identity_facts<WC>(
    entry: &PlanarStructuralIdentityEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PlanarStructuralIdentityQueryDomain, WC>,
) -> Result<PlanarStructuralIdentityReceipt, PlanarStructuralIdentityFactError>
where
    WC: ForgeQueryDomainOperatingContext<PlanarStructuralIdentityQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let basis = entry.case().basis().clone();
            let rows = PlanarStructuralIdentityInspectionRow::from_basis(&basis);
            let envelope_digest = format!("{:?}", envelope.envelope_digest());
            let declaration_digest = envelope.declaration_digest().to_string();
            let structural_digest = PlanarStructuralIdentityReceipt::structural_digest_for(&basis);
            let transform_digest = PlanarStructuralIdentityReceipt::transform_digest_for(&basis);
            Ok(PlanarStructuralIdentityReceipt::new(
                basis,
                declaration_digest,
                envelope_digest,
                structural_digest,
                transform_digest,
                counters(&rows),
            ))
        }
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
            PlanarStructuralIdentityFactError::outcome_not_bound(&posture),
        ),
    }
}

fn counters(rows: &[PlanarStructuralIdentityInspectionRow]) -> PlanarStructuralIdentityCounters {
    let structural = rows
        .iter()
        .filter(|row| row.kind() == PlanarStructuralIdentityInspectionKind::StructuralAuthority)
        .count();
    let transform = rows
        .iter()
        .filter(|row| row.kind() == PlanarStructuralIdentityInspectionKind::CanonicalTransformBasis)
        .count();
    let contrast = rows
        .iter()
        .filter(|row| row.kind() == PlanarStructuralIdentityInspectionKind::ContrastOnly)
        .count();
    PlanarStructuralIdentityCounters::certified(structural, contrast, transform)
}
