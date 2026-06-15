use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use crate::bindings::query_native_planar_topology_contract::authoring::PlanarTopologyContractCompletenessEntry;
use crate::bindings::query_native_planar_topology_contract::domain::PlanarTopologyContractCompletenessQueryDomain;
use crate::bindings::query_native_planar_topology_contract::inspection::{
    PlanarTopologyContractCompletenessInspectionKind,
    PlanarTopologyContractCompletenessInspectionRow,
};
use crate::planar_contracts::topology_contract_completeness::{
    PlanarTopologyContractCompletenessCounters, PlanarTopologyContractCompletenessReceipt,
    REQUIRED_TOPOLOGY_COMPLETENESS_FACT_ROWS,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PlanarTopologyContractCompletenessFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl PlanarTopologyContractCompletenessFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn planar_topology_contract_completeness_facts<WC>(
    entry: &PlanarTopologyContractCompletenessEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        PlanarTopologyContractCompletenessQueryDomain,
        WC,
    >,
) -> Result<PlanarTopologyContractCompletenessReceipt, PlanarTopologyContractCompletenessFactError>
where
    WC: ForgeQueryDomainOperatingContext<PlanarTopologyContractCompletenessQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let basis = entry.case().basis().clone();
            let rows = PlanarTopologyContractCompletenessInspectionRow::from_basis(&basis);
            Ok(PlanarTopologyContractCompletenessReceipt::new(
                basis.clone(),
                envelope.declaration_digest().to_string(),
                format!("{:?}", envelope.envelope_digest()),
                PlanarTopologyContractCompletenessReceipt::fact_digest_for(&basis),
                topology_contract_completeness_counters_from_inspection_rows(&rows),
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
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            Err(PlanarTopologyContractCompletenessFactError::outcome_not_bound(&posture))
        }
    }
}

fn topology_contract_completeness_counters_from_inspection_rows(
    rows: &[PlanarTopologyContractCompletenessInspectionRow],
) -> PlanarTopologyContractCompletenessCounters {
    let topology_fact_rows = rows
        .iter()
        .filter(|row| row.kind() == PlanarTopologyContractCompletenessInspectionKind::TopologyFact)
        .count();
    PlanarTopologyContractCompletenessCounters::certified(
        topology_fact_rows,
        REQUIRED_TOPOLOGY_COMPLETENESS_FACT_ROWS,
    )
}
