use crate::{
    AdmittedBlobPlacement, AuthenticatedFrameDigest, BlobChunkReachabilityProofSet,
    BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectId, BlobPlacementClass,
    ChunkTreeRoot, LifecycleReceipt, LogicalContentDigest, StoredChunkDigest,
};
use forge_store_contracts::StableDigest;

use super::{
    BlobMovementReadPhase, BlobPlacementMovementColdOutcome, BlobPlacementMovementCounterSnapshot,
    BlobPlacementMovementDenial, BlobPlacementMovementForegroundReservation,
    BlobPlacementMovementReadHold, BlobPlacementMovementRequest, BlobReadDuringPlacementMove,
    StoreOwnedPlacementMovementExecutionReceipt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPlacementMovementFreshness {
    Current,
    Stale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobPlacementMovementAuthority {
    _private: (),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BlobPlacementMovementBasis {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    stored_digest: StoredChunkDigest,
    authenticated_frame_digest: AuthenticatedFrameDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedBlobPlacementMovementPlan {
    basis: BlobPlacementMovementBasis,
    source_class: BlobPlacementClass,
    target_class: BlobPlacementClass,
    read_hold: BlobPlacementMovementReadHold,
    cold_outcome: BlobPlacementMovementColdOutcome,
    counters: BlobPlacementMovementCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPlacementMovementPhysicalExecutionIntent {
    basis_digest: StableDigest,
}

impl BlobPlacementMovementAuthority {
    pub const fn store_owned() -> Self {
        Self { _private: () }
    }

    pub fn plan_movement(
        &self,
        request: BlobPlacementMovementRequest,
    ) -> Result<AdmittedBlobPlacementMovementPlan, BlobPlacementMovementDenial> {
        let counters = BlobPlacementMovementCounterSnapshot::start(
            request.source().class(),
            request.target().class(),
        );
        if request.freshness() != BlobPlacementMovementFreshness::Current {
            return Err(BlobPlacementMovementDenial::StaleMovementPlan { counters });
        }
        let Some(read_hold) = request.read_hold() else {
            return Err(BlobPlacementMovementDenial::MissingMovementReadHold {
                counters: counters.record_protected_denial(),
            });
        };
        match request.foreground_reservation() {
            BlobPlacementMovementForegroundReservation::Violated(violation) => {
                return Err(BlobPlacementMovementDenial::ForegroundReservationViolated {
                    violation,
                    counters: counters.record_tier_move_retry().record_protected_denial(),
                });
            }
            BlobPlacementMovementForegroundReservation::Admitted(reservation)
                if reservation.security_scope_identity()
                    != request
                        .lifecycle()
                        .declaration()
                        .security_metadata()
                        .identity() =>
            {
                return Err(
                    BlobPlacementMovementDenial::ForegroundReservationScopeMismatch {
                        counters: counters.record_protected_denial(),
                    },
                );
            }
            BlobPlacementMovementForegroundReservation::Admitted(_) => {}
        }
        if !request.cold_outcome().permits_movement() {
            return Err(BlobPlacementMovementDenial::ColdPlacementUnavailable {
                state: request.cold_outcome().state(),
                counters: counters
                    .record_unavailable_cold_chunk()
                    .record_tier_move_retry()
                    .record_protected_denial(),
            });
        }
        if !placement_matches_lifecycle(request.source(), request.lifecycle()) {
            return Err(
                BlobPlacementMovementDenial::LifecycleSourcePlacementBasisMismatch {
                    counters: counters.record_protected_denial(),
                },
            );
        }
        if !placement_matches_lifecycle(request.target(), request.lifecycle()) {
            return Err(
                BlobPlacementMovementDenial::LifecycleTargetPlacementBasisMismatch {
                    counters: counters.record_protected_denial(),
                },
            );
        }

        Ok(AdmittedBlobPlacementMovementPlan {
            basis: BlobPlacementMovementBasis::from_lifecycle(request.lifecycle()),
            source_class: request.source().class(),
            target_class: request.target().class(),
            read_hold,
            cold_outcome: request.cold_outcome(),
            counters: counters
                .record_read(request.source().class())
                .record_read(request.target().class())
                .record_move(),
        })
    }
}

impl AdmittedBlobPlacementMovementPlan {
    pub fn execute_with_receipt(
        self,
        receipt: StoreOwnedPlacementMovementExecutionReceipt,
    ) -> Result<super::ExecutedBlobPlacementMovementReceipt, BlobPlacementMovementDenial> {
        receipt.execute_plan(self)
    }

    pub fn read_guard(&self, phase: BlobMovementReadPhase) -> BlobReadDuringPlacementMove {
        BlobReadDuringPlacementMove::from_plan(self, phase)
    }

    pub const fn counters(&self) -> BlobPlacementMovementCounterSnapshot {
        self.counters
    }

    pub const fn source_class(&self) -> BlobPlacementClass {
        self.source_class
    }

    pub const fn target_class(&self) -> BlobPlacementClass {
        self.target_class
    }

    pub const fn read_hold(&self) -> BlobPlacementMovementReadHold {
        self.read_hold
    }

    pub const fn cold_outcome(&self) -> BlobPlacementMovementColdOutcome {
        self.cold_outcome
    }

    pub(crate) const fn basis(&self) -> &BlobPlacementMovementBasis {
        &self.basis
    }

    pub fn physical_execution_intent(&self) -> BlobPlacementMovementPhysicalExecutionIntent {
        BlobPlacementMovementPhysicalExecutionIntent {
            basis_digest: self.physical_execution_basis_digest(),
        }
    }

    pub(crate) fn physical_execution_basis_digest(&self) -> StableDigest {
        self.basis
            .physical_execution_basis_digest(self.source_class, self.target_class)
    }
}

impl BlobPlacementMovementPhysicalExecutionIntent {
    pub(crate) const fn basis_digest(&self) -> &StableDigest {
        &self.basis_digest
    }
}

impl BlobPlacementMovementBasis {
    fn from_lifecycle(receipt: &LifecycleReceipt) -> Self {
        Self {
            object_id: receipt.declaration().object_id().clone(),
            generation: receipt.declaration().generation(),
            chunk_tree_root: receipt.declaration().chunk_tree_root().clone(),
            logical_content_digest: receipt.declaration().logical_content_digest().clone(),
            stored_digest: receipt.declaration().stored_chunk_digest().clone(),
            authenticated_frame_digest: receipt.declaration().authenticated_frame_digest().clone(),
            security_metadata: receipt.declaration().security_metadata(),
        }
    }

    pub(crate) const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub(crate) const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub(crate) const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub(crate) const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub(crate) const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub(crate) const fn authenticated_frame_digest(&self) -> &AuthenticatedFrameDigest {
        &self.authenticated_frame_digest
    }

    pub(crate) const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub(crate) fn matches_verified_basis(
        &self,
        read: &super::BlobMovementVerifiedReadEvidence,
    ) -> bool {
        self.object_id == *read.object_id()
            && self.generation == read.generation()
            && self.chunk_tree_root == *read.chunk_tree_root()
            && self.logical_content_digest == *read.logical_content_digest()
            && self.stored_digest == *read.stored_digest()
            && self.security_metadata == read.security_metadata()
    }

    fn physical_execution_basis_digest(
        &self,
        source_class: BlobPlacementClass,
        target_class: BlobPlacementClass,
    ) -> StableDigest {
        StableDigest::new(format!(
            "s7:placement-movement:{}:{}:{}:{}:{}:{}:{:?}:{:?}:{:?}",
            self.object_id.digest().as_str(),
            self.generation.sequence(),
            self.chunk_tree_root.digest().as_str(),
            self.logical_content_digest.digest().as_str(),
            self.stored_digest.digest().as_str(),
            self.authenticated_frame_digest.digest().as_str(),
            self.security_metadata.identity(),
            source_class,
            target_class,
        ))
        .expect("placement movement execution basis digest is nonempty")
    }
}

fn placement_matches_lifecycle(
    placement: &AdmittedBlobPlacement,
    receipt: &LifecycleReceipt,
) -> bool {
    placement.stored_digest() == receipt.reachability().stored_digest()
        && placement.security_metadata() == receipt.reachability().security_metadata()
        && placement.stored_digest() == receipt.placement().stored_digest()
        && placement.security_metadata() == receipt.placement().security_metadata()
}

#[allow(dead_code)]
fn _reachability_is_the_authority(_: &BlobChunkReachabilityProofSet) {}
