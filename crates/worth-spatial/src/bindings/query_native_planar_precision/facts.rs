use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use crate::bindings::query_native_planar_precision::authoring::PlanarPrecisionCertificationEntry;
use crate::bindings::query_native_planar_precision::domain::PlanarPrecisionCertificationQueryDomain;
use crate::planar_contracts::precision_basis::{
    PlanarPrecisionCertificateReceipt, PlanarPrecisionPerformanceCounters,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PlanarPrecisionCertificationFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl PlanarPrecisionCertificationFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn planar_precision_certification_facts<C>(
    entry: &PlanarPrecisionCertificationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PlanarPrecisionCertificationQueryDomain, C>,
) -> Result<PlanarPrecisionCertificateReceipt, PlanarPrecisionCertificationFactError>
where
    C: ForgeQueryDomainOperatingContext<PlanarPrecisionCertificationQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let basis = entry.case().basis().clone();
            let precision = entry
                .case()
                .predicate_receipt()
                .precision_escalation()
                .clone();
            let envelope_digest = format!("{:?}", envelope.envelope_digest());
            let declaration_digest = envelope.declaration_digest().to_string();
            let basis_part_count = PlanarPrecisionCertificateReceipt::digest_parts(
                &basis,
                &precision,
                &declaration_digest,
                &envelope_digest,
            )
            .len();
            let fact_digest = PlanarPrecisionCertificateReceipt::fact_digest_for(
                &basis,
                &precision,
                &declaration_digest,
                &envelope_digest,
            );
            Ok(PlanarPrecisionCertificateReceipt::new(
                basis,
                precision.clone(),
                declaration_digest,
                envelope_digest,
                fact_digest,
                PlanarPrecisionPerformanceCounters::certified(
                    basis_part_count,
                    precision.get_expansion_length().unwrap_or(0),
                ),
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
            PlanarPrecisionCertificationFactError::outcome_not_bound(&posture),
        ),
    }
}
