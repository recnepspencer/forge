use crate::{
    MinimalManifestVerifierReport, OfflinePhysicalVerifier, PersistedPhysicalLayout,
    PhysicalHeaderAuthority, PlatformPhysicalFacadeCounterSnapshot, PlatformPhysicalOpenRequest,
};
use forge_store_contracts::AcceptedHandoffReadiness;

use super::denials::{PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind};
use super::storage::PlatformPhysicalFacadeStorage;
use super::{map_verifier_denial_for_reopen, PlatformPhysicalFacade};

pub(crate) fn verify_handoff_readiness(
    readiness: &AcceptedHandoffReadiness,
) -> Result<(), PlatformPhysicalFacadeDenial> {
    readiness
        .physical_authority_scope()
        .map(|_| ())
        .map_err(|_| {
            PlatformPhysicalFacadeDenial::new(
                PlatformPhysicalFacadeDenialKind::HandoffReadinessRejected,
            )
        })
}

pub(crate) fn collect_reopen_layout_evidence<'a>(
    request: &'a PlatformPhysicalOpenRequest,
    layout: &'a PersistedPhysicalLayout,
) -> ReopenLayoutEvidence<'a> {
    ReopenLayoutEvidence {
        headers: request.headers().clone(),
        layout,
    }
}

pub(crate) struct ReopenLayoutEvidence<'a> {
    pub headers: PhysicalHeaderAuthority,
    pub layout: &'a PersistedPhysicalLayout,
}

pub(crate) fn verify_persisted_layout_for_reopen(
    evidence: &ReopenLayoutEvidence<'_>,
) -> Result<MinimalManifestVerifierReport, PlatformPhysicalFacadeDenial> {
    OfflinePhysicalVerifier::s1(evidence.headers.clone())
        .verify(evidence.layout)
        .map_err(map_verifier_denial_for_reopen)
}

pub(crate) fn construct_storage_from_verified_layout(
    layout: &PersistedPhysicalLayout,
    verifier_report: &MinimalManifestVerifierReport,
) -> PlatformPhysicalFacadeStorage {
    PlatformPhysicalFacadeStorage::from_persisted_layout(
        layout,
        verifier_report.layout().discovered_references().to_vec(),
    )
}

pub(crate) fn reopen_s1(
    readiness: AcceptedHandoffReadiness,
    request: PlatformPhysicalOpenRequest,
    layout: PersistedPhysicalLayout,
) -> Result<PlatformPhysicalFacade, PlatformPhysicalFacadeDenial> {
    verify_handoff_readiness(&readiness)?;
    let evidence = collect_reopen_layout_evidence(&request, &layout);
    let verifier_report = verify_persisted_layout_for_reopen(&evidence)?;
    let storage = construct_storage_from_verified_layout(&layout, &verifier_report);
    Ok(PlatformPhysicalFacade::new(
        readiness.scope(),
        request.headers().clone(),
        storage,
        PlatformPhysicalFacadeCounterSnapshot::empty()
            .with_open()
            .with_reopen(),
    ))
}