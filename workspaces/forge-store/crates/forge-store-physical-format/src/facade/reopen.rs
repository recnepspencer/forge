use crate::{
    MinimalManifestVerifierReport, OfflinePhysicalVerifier, PersistedPhysicalLayout,
    PhysicalHeaderAuthority, PlatformPhysicalFacadeCounterSnapshot, PlatformPhysicalOpenRequest,
};
use forge_store_contracts::AcceptedHandoffReadiness;

use super::denials::{PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind};
use super::storage::PlatformPhysicalFacadeStorage;
use super::PlatformPhysicalFacade;

impl PlatformPhysicalFacade {
    pub fn open_physical_format(
        readiness: AcceptedHandoffReadiness,
        request: PlatformPhysicalOpenRequest,
    ) -> Result<Self, PlatformPhysicalFacadeDenial> {
        verify_handoff_readiness(&readiness)?;
        Ok(Self::new(
            readiness.scope(),
            request.headers().clone(),
            PlatformPhysicalFacadeStorage::empty(),
            PlatformPhysicalFacadeCounterSnapshot::empty().with_open(),
            request.store_identity().clone(),
        ))
    }

    pub fn reopen(
        readiness: AcceptedHandoffReadiness,
        request: PlatformPhysicalOpenRequest,
        replay_artifact: crate::PlatformPhysicalReplayArtifact,
    ) -> Result<Self, PlatformPhysicalFacadeDenial> {
        replay_artifact.reopen_physical_format(readiness, request)
    }
}

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
    OfflinePhysicalVerifier::for_canonical_physical_format(evidence.headers.clone())
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

pub(crate) fn reopen_from_verified_layout(
    readiness: AcceptedHandoffReadiness,
    request: PlatformPhysicalOpenRequest,
    headers: PhysicalHeaderAuthority,
    layout: PersistedPhysicalLayout,
    artifact_store_identity: crate::PhysicalStoreIdentity,
) -> Result<PlatformPhysicalFacade, PlatformPhysicalFacadeDenial> {
    verify_handoff_readiness(&readiness)?;
    if request.store_identity() != &artifact_store_identity {
        return Err(PlatformPhysicalFacadeDenial::new(
            PlatformPhysicalFacadeDenialKind::StoreIdentityMismatch,
        ));
    }
    let request = PlatformPhysicalOpenRequest::for_store(headers, artifact_store_identity);
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
        request.store_identity().clone(),
    ))
}

pub(crate) fn map_verifier_denial_for_reopen(
    denial: crate::OfflineVerifierDenial,
) -> PlatformPhysicalFacadeDenial {
    let kind = match denial.kind() {
        crate::OfflineVerifierDenialKind::MissingRootManifest => {
            PlatformPhysicalFacadeDenialKind::MissingPhysicalRoot
        }
        crate::OfflineVerifierDenialKind::AmbiguousRootManifest => {
            PlatformPhysicalFacadeDenialKind::AmbiguousRootPublication
        }
        _ => PlatformPhysicalFacadeDenialKind::OfflineVerifierDenied,
    };
    PlatformPhysicalFacadeDenial::new(kind).with_verifier_denial(denial)
}
