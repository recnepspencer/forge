use forge_foundational::facade::CanonicalDerivedDigest;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_retained_planar_facts::authoring::RetainedPlanarFactsEntry;
use crate::bindings::query_native_retained_planar_facts::domain::RetainedPlanarFactsQueryDomain;
use crate::bindings::query_native_retained_planar_facts::inspection::RetainedPlanarFactsInspectionRow;
use crate::planar_contracts::retained_planar_facts::{
    RetainedPlanarFactsCounters, RetainedPlanarFactsReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum RetainedPlanarFactsFactError {
    TruncatedRetainedBasis { reason: &'static str },
}

impl RetainedPlanarFactsFactError {
    pub fn reason(&self) -> &str {
        match self {
            Self::TruncatedRetainedBasis { reason } => reason,
        }
    }
}

pub fn retained_planar_facts<WC>(
    entry: &RetainedPlanarFactsEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<RetainedPlanarFactsQueryDomain, WC>,
) -> Result<RetainedPlanarFactsReceipt, RetainedPlanarFactsFactError>
where
    WC: ForgeQueryDomainOperatingContext<RetainedPlanarFactsQueryDomain>,
{
    match handle
        .declare_review_and_progress(entry.clone())
        .map(|progressed| handle.orchestrate_envelope_from_progressed_checked(progressed))
    {
        Ok(ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope)) => {
            let basis = entry.case().basis().clone();
            let inspection_rows = RetainedPlanarFactsInspectionRow::from_basis(&basis);
            let envelope_digest = canonical_digest_token(envelope.envelope_digest());
            let declaration_digest = envelope.declaration_digest().to_string();
            let progression_digest =
                envelope
                    .progression_digest()
                    .ok_or(RetainedPlanarFactsFactError::TruncatedRetainedBasis {
                        reason: "retained planar facts require Query progression digest before fact retention",
                    })?
                    .to_string();
            let route_plan_digest =
                envelope
                    .route_plan_digest()
                    .ok_or(RetainedPlanarFactsFactError::TruncatedRetainedBasis {
                        reason: "retained planar facts require Query route-plan digest before fact retention",
                    })?
                    .to_string();
            let query_receipt_digest = canonical_digest_token(envelope.receipt_digest());
            let retained_fact_digest = RetainedPlanarFactsReceipt::retained_fact_digest_for(
                &basis,
                &declaration_digest,
                &progression_digest,
                &route_plan_digest,
                &query_receipt_digest,
                &envelope_digest,
            );
            let retained_family_rows = basis
                .boolean_readiness_receipt()
                .basis()
                .family_rows()
                .len();
            let retained_fact_rows = basis
                .boolean_readiness_receipt()
                .basis()
                .family_rows()
                .iter()
                .map(|row| row.retained_fact_digests().len())
                .sum();
            Ok(RetainedPlanarFactsReceipt::new(
                basis,
                declaration_digest,
                progression_digest,
                route_plan_digest,
                query_receipt_digest,
                envelope_digest,
                retained_fact_digest,
                RetainedPlanarFactsCounters::retained(
                    retained_family_rows,
                    retained_fact_rows,
                    inspection_rows.len(),
                ),
            ))
        }
        Ok(
            ForgeQueryDeclarationEnvelopeChecked::Deferred(_)
            | ForgeQueryDeclarationEnvelopeChecked::Denied(_)
            | ForgeQueryDeclarationEnvelopeChecked::Failed(_),
        ) => Err(RetainedPlanarFactsFactError::TruncatedRetainedBasis {
            reason: "retained planar facts require an enveloped Query declaration entry before fact retention",
        }),
        Err(_) => Err(RetainedPlanarFactsFactError::TruncatedRetainedBasis {
            reason: "retained planar facts require a progressed Query declaration entry before fact retention",
        }),
    }
}

fn canonical_digest_token(digest: &CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
