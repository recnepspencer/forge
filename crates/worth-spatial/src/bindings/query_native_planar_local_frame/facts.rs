use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use crate::bindings::query_native_planar_local_frame::authoring::PlanarLocalFrameCertificateEntry;
use crate::bindings::query_native_planar_local_frame::domain::PlanarLocalFrameCertificateQueryDomain;
use crate::planar_contracts::local_frame::{
    PlanarLocalFrameCertificateReceipt, PlanarLocalFramePerformanceCounters,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PlanarLocalFrameCertificateFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl PlanarLocalFrameCertificateFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn planar_local_frame_certificate_facts<C>(
    entry: &PlanarLocalFrameCertificateEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PlanarLocalFrameCertificateQueryDomain, C>,
) -> Result<PlanarLocalFrameCertificateReceipt, PlanarLocalFrameCertificateFactError>
where
    C: ForgeQueryDomainOperatingContext<PlanarLocalFrameCertificateQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let basis = entry.case().basis().clone();
            let envelope_digest = format!("{:?}", envelope.envelope_digest());
            let declaration_digest = envelope.declaration_digest().to_string();
            let basis_part_count = PlanarLocalFrameCertificateReceipt::digest_parts(
                &basis,
                &declaration_digest,
                &envelope_digest,
            )
            .len();
            let fact_digest = PlanarLocalFrameCertificateReceipt::fact_digest_for(
                &basis,
                &declaration_digest,
                &envelope_digest,
            );
            Ok(PlanarLocalFrameCertificateReceipt::new(
                basis,
                declaration_digest,
                envelope_digest,
                fact_digest,
                PlanarLocalFramePerformanceCounters::certified(basis_part_count),
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
            PlanarLocalFrameCertificateFactError::outcome_not_bound(&posture),
        ),
    }
}
