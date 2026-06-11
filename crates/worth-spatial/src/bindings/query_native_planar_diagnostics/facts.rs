use forge_foundational::facade::CanonicalDerivedDigest;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_diagnostics::authoring::PlanarDiagnosticBundleEntry;
use crate::bindings::query_native_planar_diagnostics::domain::PlanarDiagnosticBundleQueryDomain;
use crate::bindings::query_native_planar_diagnostics::inspection::PlanarDiagnosticInspectionRow;
use crate::planar_contracts::planar_diagnostics::{
    PlanarDiagnosticBundleReceipt, PlanarDiagnosticCounters,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PlanarDiagnosticBundleFactError {
    TruncatedDiagnosticBundleBasis { reason: &'static str },
}

impl PlanarDiagnosticBundleFactError {
    pub fn reason(&self) -> &str {
        match self {
            Self::TruncatedDiagnosticBundleBasis { reason } => reason,
        }
    }
}

pub fn planar_diagnostic_bundle<WC>(
    entry: &PlanarDiagnosticBundleEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PlanarDiagnosticBundleQueryDomain, WC>,
) -> Result<PlanarDiagnosticBundleReceipt, PlanarDiagnosticBundleFactError>
where
    WC: ForgeQueryDomainOperatingContext<PlanarDiagnosticBundleQueryDomain>,
{
    let artifacts = planar_diagnostic_query_artifacts(entry, handle)?;
    Ok(diagnostic_bundle_receipt_from_query_artifacts(artifacts))
}

struct PlanarDiagnosticQueryArtifacts {
    basis: crate::planar_contracts::planar_diagnostics::PlanarDiagnosticBundleBasis,
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: String,
    query_receipt_digest: String,
    envelope_digest: String,
    diagnostic_bundle_digest: String,
    inspection_row_count: usize,
}

fn planar_diagnostic_query_artifacts<WC>(
    entry: &PlanarDiagnosticBundleEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PlanarDiagnosticBundleQueryDomain, WC>,
) -> Result<PlanarDiagnosticQueryArtifacts, PlanarDiagnosticBundleFactError>
where
    WC: ForgeQueryDomainOperatingContext<PlanarDiagnosticBundleQueryDomain>,
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
                    PlanarDiagnosticBundleFactError::TruncatedDiagnosticBundleBasis {
                        reason: "planar diagnostics require Query progression digest",
                    },
                )?
                .to_string();
            let route_plan_digest = envelope
                .route_plan_digest()
                .ok_or(
                    PlanarDiagnosticBundleFactError::TruncatedDiagnosticBundleBasis {
                        reason: "planar diagnostics require Query route-plan digest",
                    },
                )?
                .to_string();
            let query_receipt_digest = canonical_digest_token(envelope.receipt_digest());
            let envelope_digest = canonical_digest_token(envelope.envelope_digest());
            let diagnostic_bundle_digest =
                PlanarDiagnosticBundleReceipt::diagnostic_bundle_digest_for(
                    &basis,
                    &declaration_digest,
                    &progression_digest,
                    &route_plan_digest,
                    &query_receipt_digest,
                    &envelope_digest,
                );
            let inspection_rows = PlanarDiagnosticInspectionRow::from_basis(&basis);
            Ok(PlanarDiagnosticQueryArtifacts {
                basis,
                declaration_digest,
                progression_digest,
                route_plan_digest,
                query_receipt_digest,
                envelope_digest,
                diagnostic_bundle_digest,
                inspection_row_count: inspection_rows.len(),
            })
        }
        Ok(
            ForgeQueryDeclarationEnvelopeChecked::Deferred(_)
            | ForgeQueryDeclarationEnvelopeChecked::Denied(_)
            | ForgeQueryDeclarationEnvelopeChecked::Failed(_),
        ) => Err(
            PlanarDiagnosticBundleFactError::TruncatedDiagnosticBundleBasis {
                reason: "planar diagnostics require an enveloped Query declaration entry",
            },
        ),
        Err(_) => Err(
            PlanarDiagnosticBundleFactError::TruncatedDiagnosticBundleBasis {
                reason: "planar diagnostics require a progressed Query declaration entry",
            },
        ),
    }
}

fn diagnostic_bundle_receipt_from_query_artifacts(
    artifacts: PlanarDiagnosticQueryArtifacts,
) -> PlanarDiagnosticBundleReceipt {
    let inspected_source_receipts = artifacts.basis.subject().evidence().len();
    let counters = PlanarDiagnosticCounters::certified(
        inspected_source_receipts,
        usize::from(artifacts.basis.topology_evidence().is_some()),
        usize::from(artifacts.basis.causal_evidence().is_some()),
        artifacts.inspection_row_count,
        0,
    );
    PlanarDiagnosticBundleReceipt::new(
        artifacts.basis,
        artifacts.declaration_digest,
        artifacts.progression_digest,
        artifacts.route_plan_digest,
        artifacts.query_receipt_digest,
        artifacts.envelope_digest,
        artifacts.diagnostic_bundle_digest,
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
