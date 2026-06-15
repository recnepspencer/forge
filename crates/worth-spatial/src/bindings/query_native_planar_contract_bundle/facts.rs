use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use crate::bindings::query_native_planar_contract_bundle::authoring::PlanarContractBundleValidationEntry;
use crate::bindings::query_native_planar_contract_bundle::domain::PlanarContractBundleValidationQueryDomain;
use crate::bindings::query_native_planar_contract_bundle::inspection::PlanarContractBundleInspectionRow;
use crate::planar_contracts::contract_bundle::{
    PlanarContractBundleDenial, PlanarContractBundleValidationCounters,
    PlanarContractBundleValidationReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PlanarContractBundleValidationFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
    BundleBasis {
        denial: PlanarContractBundleDenial,
    },
}

impl PlanarContractBundleValidationFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn planar_contract_bundle_validation_facts<WC>(
    entry: &PlanarContractBundleValidationEntry,
    bundle_handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        PlanarContractBundleValidationQueryDomain,
        WC,
    >,
) -> Result<PlanarContractBundleValidationReceipt, PlanarContractBundleValidationFactError>
where
    WC: ForgeQueryDomainOperatingContext<PlanarContractBundleValidationQueryDomain>,
{
    match bundle_handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let basis = entry.case().basis().clone();
            let inspection_rows = PlanarContractBundleInspectionRow::from_basis(&basis);
            let envelope_digest = format!("{:?}", envelope.envelope_digest());
            let declaration_digest = envelope.declaration_digest().to_string();
            let fact_digest = PlanarContractBundleValidationReceipt::fact_digest_for(
                &basis,
                &declaration_digest,
                &envelope_digest,
            );
            Ok(PlanarContractBundleValidationReceipt::new(
                basis.clone(),
                declaration_digest,
                envelope_digest,
                fact_digest,
                PlanarContractBundleValidationCounters::certified(
                    inspection_rows.len(),
                    basis.family_rows().len(),
                    basis.projection_receipts().len(),
                    retained_fact_rows(&basis),
                    1,
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
            PlanarContractBundleValidationFactError::outcome_not_bound(&posture),
        ),
    }
}

fn retained_fact_rows(
    basis: &crate::planar_contracts::contract_bundle::PlanarContractBundleValidationBasis,
) -> usize {
    basis
        .family_rows()
        .iter()
        .map(|row| row.retained_fact_digests().len())
        .sum()
}
