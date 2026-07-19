use crate::{
    InMemoryPhysicalFormatModelCounterSnapshot, InMemoryPhysicalFormatModelRequest,
    MinimalManifestVerifierReport, OfflinePhysicalVerifier, PersistedPhysicalLayout,
    PhysicalHeaderAuthority,
};
use worth_store_contracts::AcceptedHandoffReadiness;

use super::denials::{InMemoryPhysicalFormatModelDenial, InMemoryPhysicalFormatModelDenialKind};
use super::storage::InMemoryPhysicalFormatModelStorage;
use super::InMemoryPhysicalFormatModel;

impl InMemoryPhysicalFormatModel {
    /// Starts an empty detached heap model; this does not open physical media.
    pub fn start_empty_model(
        readiness: AcceptedHandoffReadiness,
        request: InMemoryPhysicalFormatModelRequest,
    ) -> Result<Self, InMemoryPhysicalFormatModelDenial> {
        verify_handoff_readiness(&readiness)?;
        Ok(Self::new(
            readiness.scope(),
            request.headers().clone(),
            InMemoryPhysicalFormatModelStorage::empty(),
            InMemoryPhysicalFormatModelCounterSnapshot::empty().with_open(),
            request.store_identity().clone(),
        ))
    }

    /// Restores detached model state from a caller-supplied model artifact.
    pub fn restore(
        readiness: AcceptedHandoffReadiness,
        request: InMemoryPhysicalFormatModelRequest,
        replay_artifact: crate::InMemoryPhysicalFormatReplayArtifact,
    ) -> Result<Self, InMemoryPhysicalFormatModelDenial> {
        replay_artifact.restore_model(readiness, request)
    }
}

pub(crate) fn verify_handoff_readiness(
    readiness: &AcceptedHandoffReadiness,
) -> Result<(), InMemoryPhysicalFormatModelDenial> {
    readiness
        .physical_authority_scope()
        .map(|_| ())
        .map_err(|_| {
            InMemoryPhysicalFormatModelDenial::new(
                InMemoryPhysicalFormatModelDenialKind::HandoffReadinessRejected,
            )
        })
}

pub(crate) fn collect_restore_layout_evidence<'a>(
    request: &'a InMemoryPhysicalFormatModelRequest,
    layout: &'a PersistedPhysicalLayout,
) -> RestoreLayoutEvidence<'a> {
    RestoreLayoutEvidence {
        headers: request.headers().clone(),
        layout,
    }
}

pub(crate) struct RestoreLayoutEvidence<'a> {
    pub headers: PhysicalHeaderAuthority,
    pub layout: &'a PersistedPhysicalLayout,
}

pub(crate) fn verify_persisted_layout_for_restore(
    evidence: &RestoreLayoutEvidence<'_>,
) -> Result<MinimalManifestVerifierReport, InMemoryPhysicalFormatModelDenial> {
    OfflinePhysicalVerifier::for_canonical_physical_format(evidence.headers.clone())
        .verify(evidence.layout)
        .map_err(map_verifier_denial_for_restore)
}

pub(crate) fn construct_storage_from_verified_layout(
    layout: &PersistedPhysicalLayout,
    verifier_report: &MinimalManifestVerifierReport,
) -> InMemoryPhysicalFormatModelStorage {
    InMemoryPhysicalFormatModelStorage::from_persisted_layout(
        layout,
        verifier_report.layout().discovered_references().to_vec(),
    )
}

pub(crate) fn restore_from_verified_layout(
    readiness: AcceptedHandoffReadiness,
    request: InMemoryPhysicalFormatModelRequest,
    headers: PhysicalHeaderAuthority,
    layout: PersistedPhysicalLayout,
    artifact_store_identity: crate::PhysicalStoreIdentity,
) -> Result<InMemoryPhysicalFormatModel, InMemoryPhysicalFormatModelDenial> {
    verify_handoff_readiness(&readiness)?;
    if request.store_identity() != &artifact_store_identity {
        return Err(InMemoryPhysicalFormatModelDenial::new(
            InMemoryPhysicalFormatModelDenialKind::StoreIdentityMismatch,
        ));
    }
    let request = InMemoryPhysicalFormatModelRequest::for_store(headers, artifact_store_identity);
    let evidence = collect_restore_layout_evidence(&request, &layout);
    let verifier_report = verify_persisted_layout_for_restore(&evidence)?;
    let storage = construct_storage_from_verified_layout(&layout, &verifier_report);
    Ok(InMemoryPhysicalFormatModel::new(
        readiness.scope(),
        request.headers().clone(),
        storage,
        InMemoryPhysicalFormatModelCounterSnapshot::empty()
            .with_open()
            .with_restore(),
        request.store_identity().clone(),
    ))
}

pub(crate) fn map_verifier_denial_for_restore(
    denial: crate::OfflineVerifierDenial,
) -> InMemoryPhysicalFormatModelDenial {
    let kind = match denial.kind() {
        crate::OfflineVerifierDenialKind::MissingRootManifest => {
            InMemoryPhysicalFormatModelDenialKind::MissingPhysicalRoot
        }
        crate::OfflineVerifierDenialKind::AmbiguousRootManifest => {
            InMemoryPhysicalFormatModelDenialKind::AmbiguousRootPublication
        }
        _ => InMemoryPhysicalFormatModelDenialKind::OfflineVerifierDenied,
    };
    InMemoryPhysicalFormatModelDenial::new(kind).with_verifier_denial(denial)
}
