use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityKind, CapabilityEvidenceClass,
};
use worth_store_tiering::ColdPlacementState;

use crate::placement::admission::{
    basis::BlobPlacementReachabilityBasis, BlobPlacementAdmissionDenial, BlobPlacementClass,
    BlobPlacementCounterSnapshot, BlobPlacementIntent,
};

pub(crate) fn verify_class_backend_capability(
    backend: &AdmittedBackendCapabilityWitness,
    intent: &BlobPlacementIntent<'_>,
    basis: &BlobPlacementReachabilityBasis,
) -> Result<BlobPlacementCounterSnapshot, BlobPlacementAdmissionDenial> {
    match intent.class() {
        BlobPlacementClass::Inline => verify_inline_capability(backend),
        BlobPlacementClass::External => verify_external_capability(backend, intent, basis),
        BlobPlacementClass::Cold => verify_cold_capability(backend, intent),
    }
}

fn verify_inline_capability(
    backend: &AdmittedBackendCapabilityWitness,
) -> Result<BlobPlacementCounterSnapshot, BlobPlacementAdmissionDenial> {
    require_backend_capability(
        backend,
        BackendCapabilityKind::BufferedFile,
        BlobPlacementClass::Inline,
    )?;
    Ok(BlobPlacementCounterSnapshot::for_class(BlobPlacementClass::Inline).record_inline_read())
}

fn verify_external_capability(
    backend: &AdmittedBackendCapabilityWitness,
    intent: &BlobPlacementIntent<'_>,
    basis: &BlobPlacementReachabilityBasis,
) -> Result<BlobPlacementCounterSnapshot, BlobPlacementAdmissionDenial> {
    require_backend_capability(
        backend,
        BackendCapabilityKind::DirectIo,
        BlobPlacementClass::External,
    )?;
    if let Some(observation) = intent.external_sidecar_denial() {
        return Err(
            BlobPlacementAdmissionDenial::ExternalSidecarWithoutStoreAuthority {
                observation: observation.clone(),
                counters: BlobPlacementCounterSnapshot::for_class(BlobPlacementClass::External)
                    .record_external_read(),
            },
        );
    }
    let recoverability = intent
        .external_recoverability()
        .expect("external intent variants carry recoverability or explicit denial");
    if !basis.admits_external_recoverability(recoverability) {
        return Err(
            BlobPlacementAdmissionDenial::ExternalPlacementRecoverabilityBasisMismatch {
                counters: BlobPlacementCounterSnapshot::for_class(BlobPlacementClass::External)
                    .record_external_read(),
            },
        );
    }
    Ok(
        BlobPlacementCounterSnapshot::for_class(BlobPlacementClass::External)
            .record_external_read(),
    )
}

fn verify_cold_capability(
    backend: &AdmittedBackendCapabilityWitness,
    intent: &BlobPlacementIntent<'_>,
) -> Result<BlobPlacementCounterSnapshot, BlobPlacementAdmissionDenial> {
    require_backend_capability(
        backend,
        BackendCapabilityKind::AsyncIo,
        BlobPlacementClass::Cold,
    )?;
    let state = intent
        .cold_state()
        .unwrap_or(ColdPlacementState::ColdUnavailable);
    if !state.permits_immediate_publication() {
        return Err(BlobPlacementAdmissionDenial::ColdChunkUnavailable {
            state,
            counters: BlobPlacementCounterSnapshot::for_class(BlobPlacementClass::Cold)
                .record_unavailable_cold_chunk()
                .record_tier_move_protected_denial(),
        });
    }
    Ok(BlobPlacementCounterSnapshot::for_class(BlobPlacementClass::Cold).record_cold_fetch())
}

fn require_backend_capability(
    backend: &AdmittedBackendCapabilityWitness,
    capability: BackendCapabilityKind,
    class: BlobPlacementClass,
) -> Result<(), BlobPlacementAdmissionDenial> {
    backend
        .require(capability, CapabilityEvidenceClass::CertifiedBackendProfile)
        .map(|_| ())
        .map_err(|source| BlobPlacementAdmissionDenial::BackendCapability {
            source,
            counters: BlobPlacementCounterSnapshot::for_class(class),
        })
}
