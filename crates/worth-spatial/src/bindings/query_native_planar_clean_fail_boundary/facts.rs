use forge_foundational::facade::CanonicalDerivedDigest;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_clean_fail_boundary::authoring::PlanarCleanFailBoundaryEntry;
use crate::bindings::query_native_planar_clean_fail_boundary::domain::PlanarCleanFailBoundaryQueryDomain;
use crate::planar_contracts::clean_fail_boundary::{
    PlanarCleanFailBoundaryCounters, PlanarCleanFailBoundaryReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PlanarCleanFailBoundaryFactError {
    TruncatedCleanFailBoundaryBasis { reason: &'static str },
}

impl PlanarCleanFailBoundaryFactError {
    pub fn reason(&self) -> &str {
        match self {
            Self::TruncatedCleanFailBoundaryBasis { reason } => reason,
        }
    }
}

pub fn planar_clean_fail_boundary<WC>(
    entry: &PlanarCleanFailBoundaryEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PlanarCleanFailBoundaryQueryDomain, WC>,
) -> Result<PlanarCleanFailBoundaryReceipt, PlanarCleanFailBoundaryFactError>
where
    WC: ForgeQueryDomainOperatingContext<PlanarCleanFailBoundaryQueryDomain>,
{
    let artifacts = planar_clean_fail_query_artifacts(entry, handle)?;
    Ok(clean_fail_receipt_from_query_artifacts(artifacts))
}

struct PlanarCleanFailQueryArtifacts {
    basis: crate::planar_contracts::clean_fail_boundary::PlanarCleanFailBoundaryBasis,
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: String,
    query_receipt_digest: String,
    envelope_digest: String,
    clean_fail_boundary_digest: String,
}

fn planar_clean_fail_query_artifacts<WC>(
    entry: &PlanarCleanFailBoundaryEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PlanarCleanFailBoundaryQueryDomain, WC>,
) -> Result<PlanarCleanFailQueryArtifacts, PlanarCleanFailBoundaryFactError>
where
    WC: ForgeQueryDomainOperatingContext<PlanarCleanFailBoundaryQueryDomain>,
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
                .ok_or(
                    PlanarCleanFailBoundaryFactError::TruncatedCleanFailBoundaryBasis {
                        reason: "planar clean-fail boundary requires Query progression digest",
                    },
                )?
                .to_string();
            let route_plan_digest = envelope
                .route_plan_digest()
                .ok_or(
                    PlanarCleanFailBoundaryFactError::TruncatedCleanFailBoundaryBasis {
                        reason: "planar clean-fail boundary requires Query route-plan digest",
                    },
                )?
                .to_string();
            let query_receipt_digest = canonical_digest_token(envelope.receipt_digest());
            let envelope_digest = canonical_digest_token(envelope.envelope_digest());
            let clean_fail_boundary_digest =
                PlanarCleanFailBoundaryReceipt::clean_fail_boundary_digest_for(
                    &basis,
                    &declaration_digest,
                    &progression_digest,
                    &route_plan_digest,
                    &query_receipt_digest,
                    &envelope_digest,
                );
            Ok(PlanarCleanFailQueryArtifacts {
                basis,
                declaration_digest,
                progression_digest,
                route_plan_digest,
                query_receipt_digest,
                envelope_digest,
                clean_fail_boundary_digest,
            })
        }
        Ok(
            ForgeQueryDeclarationEnvelopeChecked::Deferred(_)
            | ForgeQueryDeclarationEnvelopeChecked::Denied(_)
            | ForgeQueryDeclarationEnvelopeChecked::Failed(_),
        ) => Err(
            PlanarCleanFailBoundaryFactError::TruncatedCleanFailBoundaryBasis {
                reason: "planar clean-fail boundary requires an enveloped Query declaration entry",
            },
        ),
        Err(_) => Err(
            PlanarCleanFailBoundaryFactError::TruncatedCleanFailBoundaryBasis {
                reason: "planar clean-fail boundary requires a progressed Query declaration entry",
            },
        ),
    }
}

fn clean_fail_receipt_from_query_artifacts(
    artifacts: PlanarCleanFailQueryArtifacts,
) -> PlanarCleanFailBoundaryReceipt {
    let counters = PlanarCleanFailBoundaryCounters::certified(1, 1, 1, 1);
    PlanarCleanFailBoundaryReceipt::new(
        artifacts.basis,
        artifacts.declaration_digest,
        artifacts.progression_digest,
        artifacts.route_plan_digest,
        artifacts.query_receipt_digest,
        artifacts.envelope_digest,
        artifacts.clean_fail_boundary_digest,
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
