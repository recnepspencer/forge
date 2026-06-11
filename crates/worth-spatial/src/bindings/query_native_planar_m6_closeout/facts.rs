use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use crate::bindings::query_native_planar_m6_closeout::authoring::M6PlanarCloseoutEntry;
use crate::bindings::query_native_planar_m6_closeout::domain::M6PlanarCloseoutQueryDomain;
use crate::planar_contracts::m6_closeout::M6PlanarCloseoutReceipt;

#[derive(Clone, Debug, PartialEq)]
pub enum M6PlanarCloseoutFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl M6PlanarCloseoutFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn m6_planar_closeout_facts<WC>(
    entry: &M6PlanarCloseoutEntry,
    closeout_handle: &ForgeQueryAdmittedConfiguredDomainHandle<M6PlanarCloseoutQueryDomain, WC>,
) -> Result<M6PlanarCloseoutReceipt, M6PlanarCloseoutFactError>
where
    WC: ForgeQueryDomainOperatingContext<M6PlanarCloseoutQueryDomain>,
{
    match closeout_handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => Ok(M6PlanarCloseoutReceipt::new(
            entry.case().basis().clone(),
            envelope.declaration_digest().to_string(),
            format!("{:?}", envelope.envelope_digest()),
        )),
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
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            Err(M6PlanarCloseoutFactError::outcome_not_bound(&posture))
        }
    }
}
