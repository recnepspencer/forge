use forge_store_io_scheduler::S6LaterReadinessReadmissionState;
use forge_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityKind, CapabilityEvidenceClass,
};
use forge_store_tiering::S7ColdPlacementState;

use crate::{BlobChunkReachabilityProofSet, BlobChunkSecurityMetadataWitness, StoredChunkDigest};

use super::{
    basis::BlobPlacementReachabilityBasis, BlobPlacementAdmissionDenial, BlobPlacementClass,
    BlobPlacementCounterSnapshot, BlobPlacementIntent, BlobPlacementNonClaim,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobPlacementAdmissionAuthority {
    backend: AdmittedBackendCapabilityWitness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedBlobPlacement {
    basis: BlobPlacementReachabilityBasis,
    stored_digest: StoredChunkDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    class: BlobPlacementClass,
    cold_state: Option<S7ColdPlacementState>,
    counters: BlobPlacementCounterSnapshot,
    non_claims: [BlobPlacementNonClaim; 3],
}

impl BlobPlacementAdmissionAuthority {
    pub const fn from_admitted_backend(backend: AdmittedBackendCapabilityWitness) -> Self {
        Self { backend }
    }

    pub fn admit(
        &self,
        reachability: &BlobChunkReachabilityProofSet,
        intent: BlobPlacementIntent,
    ) -> Result<AdmittedBlobPlacement, BlobPlacementAdmissionDenial> {
        require_readmitted_s6_readiness(&intent)?;
        let basis = BlobPlacementReachabilityBasis::from_reachability(reachability);
        require_matching_readiness_basis(&basis, &intent)?;
        let counters = self.admit_class_capability(&intent, &basis)?;
        Ok(AdmittedBlobPlacement {
            basis,
            stored_digest: reachability.stored_digest().clone(),
            security_metadata: reachability.security_metadata(),
            class: intent.class(),
            cold_state: intent.cold_state(),
            counters,
            non_claims: BlobPlacementNonClaim::required(),
        })
    }

    fn admit_class_capability(
        &self,
        intent: &BlobPlacementIntent,
        basis: &BlobPlacementReachabilityBasis,
    ) -> Result<BlobPlacementCounterSnapshot, BlobPlacementAdmissionDenial> {
        match intent.class() {
            BlobPlacementClass::Inline => {
                self.backend
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
                self.backend
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
                let Some(recoverability) = intent.external_recoverability() else {
                    return Err(
                        BlobPlacementAdmissionDenial::ExternalPlacementMissingRecoverability {
                            counters: BlobPlacementCounterSnapshot::for_class(intent.class())
                                .record_external_read(),
                        },
                    );
                };
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
                self.backend
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
                    .unwrap_or(S7ColdPlacementState::ColdUnavailable);
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
}

impl AdmittedBlobPlacement {
    pub(crate) fn matches_reachability(
        &self,
        reachability: &BlobChunkReachabilityProofSet,
    ) -> bool {
        self.basis.matches_reachability(reachability)
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn class(&self) -> BlobPlacementClass {
        self.class
    }

    pub const fn cold_state(&self) -> Option<S7ColdPlacementState> {
        self.cold_state
    }

    pub const fn counters(&self) -> BlobPlacementCounterSnapshot {
        self.counters
    }

    pub const fn non_claims(&self) -> &[BlobPlacementNonClaim; 3] {
        &self.non_claims
    }
}

fn require_readmitted_s6_readiness(
    intent: &BlobPlacementIntent,
) -> Result<(), BlobPlacementAdmissionDenial> {
    let readmission = intent.readiness().handoff().readmission_state();
    if readmission != S6LaterReadinessReadmissionState::ReadmittedAfterPublication {
        return Err(BlobPlacementAdmissionDenial::StaleS6Readiness {
            readmission,
            counters: BlobPlacementCounterSnapshot::for_class(intent.class()),
        });
    }
    Ok(())
}

fn require_matching_readiness_basis(
    basis: &BlobPlacementReachabilityBasis,
    intent: &BlobPlacementIntent,
) -> Result<(), BlobPlacementAdmissionDenial> {
    if !basis.admits_readiness(intent.readiness()) {
        return Err(
            BlobPlacementAdmissionDenial::PlacementReadinessBasisMismatch {
                counters: BlobPlacementCounterSnapshot::for_class(intent.class()),
            },
        );
    }
    Ok(())
}
