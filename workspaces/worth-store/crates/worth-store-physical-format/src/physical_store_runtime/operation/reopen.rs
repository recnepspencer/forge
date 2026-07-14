use crate::{
    MinimalManifestVerifierReport, OfflinePhysicalVerifier, PersistedPhysicalLayout,
    PhysicalHeaderAuthority, PhysicalStoreRuntimeCounterSnapshot, PlatformPhysicalOpenRequest,
};
use worth_store_contracts::AcceptedHandoffReadiness;

use super::denials::{PhysicalStoreRuntimeDenial, PhysicalStoreRuntimeDenialKind};
use super::storage::PhysicalStoreRuntimeStorage;
use super::PhysicalStoreRuntime;

impl PhysicalStoreRuntime {
    pub fn open_physical_format(
        readiness: AcceptedHandoffReadiness,
        request: PlatformPhysicalOpenRequest,
    ) -> Result<Self, PhysicalStoreRuntimeDenial> {
        verify_handoff_readiness(&readiness)?;
        Ok(Self::new(
            readiness.scope(),
            request.headers().clone(),
            PhysicalStoreRuntimeStorage::empty(),
            PhysicalStoreRuntimeCounterSnapshot::empty().with_open(),
            request.store_identity().clone(),
        ))
    }

    pub fn reopen(
        readiness: AcceptedHandoffReadiness,
        request: PlatformPhysicalOpenRequest,
        replay_artifact: crate::PlatformPhysicalReplayArtifact,
    ) -> Result<Self, PhysicalStoreRuntimeDenial> {
        replay_artifact.reopen_physical_format(readiness, request)
    }
}

pub(crate) fn verify_handoff_readiness(
    readiness: &AcceptedHandoffReadiness,
) -> Result<(), PhysicalStoreRuntimeDenial> {
    readiness
        .physical_authority_scope()
        .map(|_| ())
        .map_err(|_| {
            PhysicalStoreRuntimeDenial::new(
                PhysicalStoreRuntimeDenialKind::HandoffReadinessRejected,
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
) -> Result<MinimalManifestVerifierReport, PhysicalStoreRuntimeDenial> {
    OfflinePhysicalVerifier::for_canonical_physical_format(evidence.headers.clone())
        .verify(evidence.layout)
        .map_err(map_verifier_denial_for_reopen)
}

pub(crate) fn construct_storage_from_verified_layout(
    layout: &PersistedPhysicalLayout,
    verifier_report: &MinimalManifestVerifierReport,
) -> PhysicalStoreRuntimeStorage {
    PhysicalStoreRuntimeStorage::from_persisted_layout(
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
) -> Result<PhysicalStoreRuntime, PhysicalStoreRuntimeDenial> {
    verify_handoff_readiness(&readiness)?;
    if request.store_identity() != &artifact_store_identity {
        return Err(PhysicalStoreRuntimeDenial::new(
            PhysicalStoreRuntimeDenialKind::StoreIdentityMismatch,
        ));
    }
    let request = PlatformPhysicalOpenRequest::for_store(headers, artifact_store_identity);
    let evidence = collect_reopen_layout_evidence(&request, &layout);
    let verifier_report = verify_persisted_layout_for_reopen(&evidence)?;
    let storage = construct_storage_from_verified_layout(&layout, &verifier_report);
    Ok(PhysicalStoreRuntime::new(
        readiness.scope(),
        request.headers().clone(),
        storage,
        PhysicalStoreRuntimeCounterSnapshot::empty()
            .with_open()
            .with_reopen(),
        request.store_identity().clone(),
    ))
}

pub(crate) fn map_verifier_denial_for_reopen(
    denial: crate::OfflineVerifierDenial,
) -> PhysicalStoreRuntimeDenial {
    let kind = match denial.kind() {
        crate::OfflineVerifierDenialKind::MissingRootManifest => {
            PhysicalStoreRuntimeDenialKind::MissingPhysicalRoot
        }
        crate::OfflineVerifierDenialKind::AmbiguousRootManifest => {
            PhysicalStoreRuntimeDenialKind::AmbiguousRootPublication
        }
        _ => PhysicalStoreRuntimeDenialKind::OfflineVerifierDenied,
    };
    PhysicalStoreRuntimeDenial::new(kind).with_verifier_denial(denial)
}
