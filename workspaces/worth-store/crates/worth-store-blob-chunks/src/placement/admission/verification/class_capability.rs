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
    intent: &BlobPlacementIntent,
    basis: &BlobPlacementReachabilityBasis,
) -> Result<BlobPlacementCounterSnapshot, BlobPlacementAdmissionDenial> {
    match intent.class() {
        BlobPlacementClass::Inline => {
            backend
                .require(
                    BackendCapabilityKind::BufferedFile,
                    CapabilityEvidenceClass::CertifiedBackendProfile,
                )
                .map_err(|source| BlobPlacementAdmissionDenial::BackendCapability {
                    source,
                    counters: BlobPlacementCounterSnapshot::for_class(intent.class()),
                })?;
            Ok(BlobPlacementCounterSnapshot::for_class(intent.class()).record_inline_read())
        }
        BlobPlacementClass::External => {
            backend
                .require(
                    BackendCapabilityKind::DirectIo,
                    CapabilityEvidenceClass::CertifiedBackendProfile,
                )
                .map_err(|source| BlobPlacementAdmissionDenial::BackendCapability {
                    source,
                    counters: BlobPlacementCounterSnapshot::for_class(intent.class()),
                })?;
            if let Some(denial) = intent.external_sidecar_denial() {
                return Err(
                    BlobPlacementAdmissionDenial::ExternalSidecarWithoutStoreAuthority {
                        observation: denial.clone(),
                        counters: BlobPlacementCounterSnapshot::for_class(intent.class())
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
                        counters: BlobPlacementCounterSnapshot::for_class(intent.class())
                            .record_external_read(),
                    },
                );
            }
            Ok(BlobPlacementCounterSnapshot::for_class(intent.class()).record_external_read())
        }
        BlobPlacementClass::Cold => {
            backend
                .require(
                    BackendCapabilityKind::AsyncIo,
                    CapabilityEvidenceClass::CertifiedBackendProfile,
                )
                .map_err(|source| BlobPlacementAdmissionDenial::BackendCapability {
                    source,
                    counters: BlobPlacementCounterSnapshot::for_class(intent.class()),
                })?;
            let state = intent
                .cold_state()
                .unwrap_or(ColdPlacementState::ColdUnavailable);
            if !state.permits_immediate_publication() {
                return Err(BlobPlacementAdmissionDenial::ColdChunkUnavailable {
                    state,
                    counters: BlobPlacementCounterSnapshot::for_class(intent.class())
                        .record_unavailable_cold_chunk()
                        .record_tier_move_protected_denial(),
                });
            }
            Ok(BlobPlacementCounterSnapshot::for_class(intent.class()).record_cold_fetch())
        }
    }
}
