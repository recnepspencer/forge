use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use crate::bindings::query_native_planar_predicate_consumption::authoring::PredicateCertificateConsumptionEntry;
use crate::bindings::query_native_planar_predicate_consumption::domain::PredicateCertificateConsumptionQueryDomain;
use crate::bindings::query_native_planar_predicate_consumption::inspection::PredicateCertificateConsumptionInspectionRow;
use crate::planar_contracts::predicate_consumption::{
    PredicateCertificateConsumptionCounters, PredicateCertificateConsumptionReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PredicateCertificateConsumptionFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl PredicateCertificateConsumptionFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn predicate_certificate_consumption_facts<WC>(
    entry: &PredicateCertificateConsumptionEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        PredicateCertificateConsumptionQueryDomain,
        WC,
    >,
) -> Result<PredicateCertificateConsumptionReceipt, PredicateCertificateConsumptionFactError>
where
    WC: ForgeQueryDomainOperatingContext<PredicateCertificateConsumptionQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let basis = entry.case().basis().clone();
            let inspection_rows = PredicateCertificateConsumptionInspectionRow::from_basis(&basis);
            let envelope_digest = format!("{:?}", envelope.envelope_digest());
            let declaration_digest = envelope.declaration_digest().to_string();
            let fact_digest = PredicateCertificateConsumptionReceipt::fact_digest_for(
                &basis,
                &declaration_digest,
                &envelope_digest,
            );
            Ok(PredicateCertificateConsumptionReceipt::new(
                basis.clone(),
                declaration_digest,
                envelope_digest,
                fact_digest,
                PredicateCertificateConsumptionCounters::certified(
                    basis.consumption_rows().len(),
                    inspection_rows.len(),
                    basis.consumption_rows().len(),
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
            PredicateCertificateConsumptionFactError::outcome_not_bound(&posture),
        ),
    }
}
