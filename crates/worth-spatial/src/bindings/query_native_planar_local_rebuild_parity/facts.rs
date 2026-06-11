use forge_foundational::facade::CanonicalDerivedDigest;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_local_rebuild_parity::authoring::PlanarLocalRebuildParityEntry;
use crate::bindings::query_native_planar_local_rebuild_parity::domain::PlanarLocalRebuildParityQueryDomain;
use crate::planar_contracts::local_rebuild_parity::{
    PlanarLocalRebuildParityCounters, PlanarLocalRebuildParityReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PlanarLocalRebuildParityFactError {
    TruncatedParityBasis { reason: &'static str },
}

impl PlanarLocalRebuildParityFactError {
    pub fn reason(&self) -> &str {
        match self {
            Self::TruncatedParityBasis { reason } => reason,
        }
    }
}

pub fn planar_local_rebuild_parity<WC>(
    entry: &PlanarLocalRebuildParityEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PlanarLocalRebuildParityQueryDomain, WC>,
) -> Result<PlanarLocalRebuildParityReceipt, PlanarLocalRebuildParityFactError>
where
    WC: ForgeQueryDomainOperatingContext<PlanarLocalRebuildParityQueryDomain>,
{
    let artifacts = planar_local_rebuild_query_artifacts(entry, handle)?;
    Ok(local_rebuild_receipt_from_query_artifacts(artifacts))
}

struct PlanarLocalRebuildQueryArtifacts {
    basis: crate::planar_contracts::local_rebuild_parity::PlanarLocalRebuildParityBasis,
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: String,
    query_receipt_digest: String,
    envelope_digest: String,
    parity_digest: String,
}

fn planar_local_rebuild_query_artifacts<WC>(
    entry: &PlanarLocalRebuildParityEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PlanarLocalRebuildParityQueryDomain, WC>,
) -> Result<PlanarLocalRebuildQueryArtifacts, PlanarLocalRebuildParityFactError>
where
    WC: ForgeQueryDomainOperatingContext<PlanarLocalRebuildParityQueryDomain>,
{
    match handle
        .declare_review_and_progress(entry.clone())
        .map(|progressed| handle.orchestrate_envelope_from_progressed_checked(progressed))
    {
        Ok(ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope)) => {
            let basis = entry.case().basis().clone();
            let declaration_digest = envelope.declaration_digest().to_string();
            let progression_digest = envelope
                .progression_digest()
                .ok_or(PlanarLocalRebuildParityFactError::TruncatedParityBasis {
                    reason: "planar local rebuild parity requires Query progression digest",
                })?
                .to_string();
            let route_plan_digest = envelope
                .route_plan_digest()
                .ok_or(PlanarLocalRebuildParityFactError::TruncatedParityBasis {
                    reason: "planar local rebuild parity requires Query route-plan digest",
                })?
                .to_string();
            let query_receipt_digest = canonical_digest_token(envelope.receipt_digest());
            let envelope_digest = canonical_digest_token(envelope.envelope_digest());
            let parity_digest = PlanarLocalRebuildParityReceipt::parity_digest_for(
                &basis,
                &declaration_digest,
                &progression_digest,
                &route_plan_digest,
                &query_receipt_digest,
                &envelope_digest,
            );
            Ok(PlanarLocalRebuildQueryArtifacts {
                basis,
                declaration_digest,
                progression_digest,
                route_plan_digest,
                query_receipt_digest,
                envelope_digest,
                parity_digest,
            })
        }
        Ok(
            ForgeQueryDeclarationEnvelopeChecked::Deferred(_)
            | ForgeQueryDeclarationEnvelopeChecked::Denied(_)
            | ForgeQueryDeclarationEnvelopeChecked::Failed(_),
        ) => Err(PlanarLocalRebuildParityFactError::TruncatedParityBasis {
            reason: "planar local rebuild parity requires an enveloped Query declaration entry",
        }),
        Err(_) => Err(PlanarLocalRebuildParityFactError::TruncatedParityBasis {
            reason: "planar local rebuild parity requires a progressed Query declaration entry",
        }),
    }
}

fn local_rebuild_receipt_from_query_artifacts(
    artifacts: PlanarLocalRebuildQueryArtifacts,
) -> PlanarLocalRebuildParityReceipt {
    let counters = PlanarLocalRebuildParityCounters::certified(
        1,
        1,
        7,
        PlanarLocalRebuildParityReceipt::SOURCE_RECEIPTS_CONSUMED,
    );
    PlanarLocalRebuildParityReceipt::new(
        artifacts.basis,
        artifacts.declaration_digest,
        artifacts.progression_digest,
        artifacts.route_plan_digest,
        artifacts.query_receipt_digest,
        artifacts.envelope_digest,
        artifacts.parity_digest,
        counters,
    )
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
