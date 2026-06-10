use forge_foundational::facade::CanonicalDerivedDigest;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_recovery::authoring::PlanarRecoveryPostureEntry;
use crate::bindings::query_native_planar_recovery::domain::PlanarRecoveryPostureQueryDomain;
use crate::bindings::query_native_planar_recovery::inspection::PlanarRecoveryPostureInspectionRow;
use crate::planar_contracts::planar_recovery::{
    PlanarRecoveryPostureCounters, PlanarRecoveryPostureReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PlanarRecoveryPostureFactError {
    TruncatedRecoveryPostureBasis { reason: &'static str },
}

impl PlanarRecoveryPostureFactError {
    pub fn reason(&self) -> &str {
        match self {
            Self::TruncatedRecoveryPostureBasis { reason } => reason,
        }
    }
}

pub fn planar_recovery_posture<WC>(
    entry: &PlanarRecoveryPostureEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PlanarRecoveryPostureQueryDomain, WC>,
) -> Result<PlanarRecoveryPostureReceipt, PlanarRecoveryPostureFactError>
where
    WC: ForgeQueryDomainOperatingContext<PlanarRecoveryPostureQueryDomain>,
{
    let artifacts = planar_recovery_query_artifacts(entry, handle)?;
    Ok(recovery_posture_receipt_from_query_artifacts(artifacts))
}

struct PlanarRecoveryQueryArtifacts {
    basis: crate::planar_contracts::planar_recovery::PlanarRecoveryPostureBasis,
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: String,
    query_receipt_digest: String,
    envelope_digest: String,
    recovery_posture_digest: String,
    inspection_row_count: usize,
}

fn planar_recovery_query_artifacts<WC>(
    entry: &PlanarRecoveryPostureEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PlanarRecoveryPostureQueryDomain, WC>,
) -> Result<PlanarRecoveryQueryArtifacts, PlanarRecoveryPostureFactError>
where
    WC: ForgeQueryDomainOperatingContext<PlanarRecoveryPostureQueryDomain>,
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
                    PlanarRecoveryPostureFactError::TruncatedRecoveryPostureBasis {
                        reason: "planar recovery requires Query progression digest",
                    },
                )?
                .to_string();
            let route_plan_digest = envelope
                .route_plan_digest()
                .ok_or(
                    PlanarRecoveryPostureFactError::TruncatedRecoveryPostureBasis {
                        reason: "planar recovery requires Query route-plan digest",
                    },
                )?
                .to_string();
            let query_receipt_digest = canonical_digest_token(envelope.receipt_digest());
            let envelope_digest = canonical_digest_token(envelope.envelope_digest());
            let recovery_posture_digest = PlanarRecoveryPostureReceipt::recovery_posture_digest_for(
                &basis,
                &declaration_digest,
                &progression_digest,
                &route_plan_digest,
                &query_receipt_digest,
                &envelope_digest,
            );
            let inspection_rows = PlanarRecoveryPostureInspectionRow::from_basis(&basis);
            Ok(PlanarRecoveryQueryArtifacts {
                basis,
                declaration_digest,
                progression_digest,
                route_plan_digest,
                query_receipt_digest,
                envelope_digest,
                recovery_posture_digest,
                inspection_row_count: inspection_rows.len(),
            })
        }
        Ok(
            ForgeQueryDeclarationEnvelopeChecked::Deferred(_)
            | ForgeQueryDeclarationEnvelopeChecked::Denied(_)
            | ForgeQueryDeclarationEnvelopeChecked::Failed(_),
        ) => Err(
            PlanarRecoveryPostureFactError::TruncatedRecoveryPostureBasis {
                reason: "planar recovery requires an enveloped Query declaration entry",
            },
        ),
        Err(_) => Err(
            PlanarRecoveryPostureFactError::TruncatedRecoveryPostureBasis {
                reason: "planar recovery requires a progressed Query declaration entry",
            },
        ),
    }
}

fn recovery_posture_receipt_from_query_artifacts(
    artifacts: PlanarRecoveryQueryArtifacts,
) -> PlanarRecoveryPostureReceipt {
    let counters = PlanarRecoveryPostureCounters::certified(
        1,
        basis_receipts_consumed(&artifacts.basis),
        1,
        artifacts.inspection_row_count,
    );
    PlanarRecoveryPostureReceipt::new(
        artifacts.basis,
        artifacts.declaration_digest,
        artifacts.progression_digest,
        artifacts.route_plan_digest,
        artifacts.query_receipt_digest,
        artifacts.envelope_digest,
        artifacts.recovery_posture_digest,
        counters,
    )
}

fn basis_receipts_consumed(
    basis: &crate::planar_contracts::planar_recovery::PlanarRecoveryPostureBasis,
) -> usize {
    usize::from(basis.retained_planar_facts().is_some())
        + usize::from(basis.projection_consumed_facts().is_some())
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
