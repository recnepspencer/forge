use crate::{
    AuthenticatedFrameDigest, BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectId,
    BlobPlacementClass, ChunkTreeRoot, LogicalContentDigest, StoredChunkDigest,
};
use forge_store_physical_isolation::{
    ChunkMigrationReadInterlockPlan, PhysicalPlacementMovementExecutionReceipt,
};

use super::plan::BlobPlacementMovementBasis;
use super::{
    counter_backed_placement_movement_performance_receipt, AdmittedBlobPlacementMovementPlan,
    BlobMovementReadPhase, BlobPlacementMovementCounterBackedPerformanceReceipt,
    BlobPlacementMovementCounterSnapshot, BlobPlacementMovementDenial,
    BlobPlacementMovementPhysicalExecutionIntent, BlobReadDuringPlacementMove,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreOwnedPlacementMovementExecution {
    _private: (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreOwnedPlacementMovementPublication {
    _private: (),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreOwnedPlacementMovementExecutionReceipt {
    basis: BlobPlacementMovementBasis,
    source_class: BlobPlacementClass,
    target_class: BlobPlacementClass,
    movement_interlock: ChunkMigrationReadInterlockPlan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedBlobPlacementMovementReceipt {
    basis: BlobPlacementMovementBasis,
    source_class: BlobPlacementClass,
    target_class: BlobPlacementClass,
    counters: BlobPlacementMovementCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedBlobPlacementObservation {
    basis: BlobPlacementMovementBasis,
    placement_class: BlobPlacementClass,
    counters: BlobPlacementMovementCounterSnapshot,
}

impl StoreOwnedPlacementMovementExecution {
    pub const fn store_owned() -> Self {
        Self { _private: () }
    }

    pub fn execute_physical_movement(
        self,
        plan: &AdmittedBlobPlacementMovementPlan,
        physical_receipt: PhysicalPlacementMovementExecutionReceipt<
            BlobPlacementMovementPhysicalExecutionIntent,
        >,
    ) -> Result<StoreOwnedPlacementMovementExecutionReceipt, BlobPlacementMovementDenial> {
        if physical_receipt.movement_interlock() != plan.read_hold().movement_interlock()
            || physical_receipt.intent().basis_digest() != &plan.physical_execution_basis_digest()
        {
            return Err(
                BlobPlacementMovementDenial::MovementExecutionReceiptMismatch {
                    counters: plan.counters().record_protected_denial(),
                },
            );
        }
        Ok(StoreOwnedPlacementMovementExecutionReceipt {
            basis: plan.basis().clone(),
            source_class: plan.source_class(),
            target_class: plan.target_class(),
            movement_interlock: physical_receipt.movement_interlock(),
        })
    }
}

impl StoreOwnedPlacementMovementPublication {
    pub const fn store_owned() -> Self {
        Self { _private: () }
    }
}

impl ExecutedBlobPlacementMovementReceipt {
    pub fn publish_observation(
        self,
        _: StoreOwnedPlacementMovementPublication,
    ) -> PublishedBlobPlacementObservation {
        PublishedBlobPlacementObservation {
            basis: self.basis,
            placement_class: self.target_class,
            counters: self.counters.record_published_observation(),
        }
    }

    pub fn read_guard(&self, phase: BlobMovementReadPhase) -> BlobReadDuringPlacementMove {
        BlobReadDuringPlacementMove::from_executed(self, phase)
    }

    pub fn lower_to_foundational_performance(
        &self,
    ) -> BlobPlacementMovementCounterBackedPerformanceReceipt {
        counter_backed_placement_movement_performance_receipt(self.counters)
    }

    pub const fn object_id(&self) -> &BlobObjectId {
        self.basis.object_id()
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.basis.generation()
    }

    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        self.basis.chunk_tree_root()
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        self.basis.logical_content_digest()
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        self.basis.stored_digest()
    }

    pub const fn authenticated_frame_digest(&self) -> &AuthenticatedFrameDigest {
        self.basis.authenticated_frame_digest()
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.basis.security_metadata()
    }

    pub const fn source_class(&self) -> BlobPlacementClass {
        self.source_class
    }

    pub const fn target_class(&self) -> BlobPlacementClass {
        self.target_class
    }

    pub const fn counters(&self) -> BlobPlacementMovementCounterSnapshot {
        self.counters
    }

    pub(crate) const fn basis(&self) -> &BlobPlacementMovementBasis {
        &self.basis
    }
}

impl StoreOwnedPlacementMovementExecutionReceipt {
    pub(crate) fn execute_plan(
        self,
        plan: AdmittedBlobPlacementMovementPlan,
    ) -> Result<ExecutedBlobPlacementMovementReceipt, BlobPlacementMovementDenial> {
        if self.movement_interlock != plan.read_hold().movement_interlock()
            || self.basis != *plan.basis()
            || self.source_class != plan.source_class()
            || self.target_class != plan.target_class()
        {
            return Err(
                BlobPlacementMovementDenial::MovementExecutionReceiptMismatch {
                    counters: plan.counters().record_protected_denial(),
                },
            );
        }
        let counters = plan.counters().record_execution_receipt();
        Ok(ExecutedBlobPlacementMovementReceipt {
            basis: self.basis,
            source_class: self.source_class,
            target_class: self.target_class,
            counters,
        })
    }
}

impl PublishedBlobPlacementObservation {
    pub fn read_guard(&self) -> BlobReadDuringPlacementMove {
        BlobReadDuringPlacementMove::from_published(self)
    }

    pub const fn object_id(&self) -> &BlobObjectId {
        self.basis.object_id()
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.basis.generation()
    }

    pub const fn placement_class(&self) -> BlobPlacementClass {
        self.placement_class
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        self.basis.stored_digest()
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.basis.security_metadata()
    }

    pub const fn counters(&self) -> BlobPlacementMovementCounterSnapshot {
        self.counters
    }

    pub(crate) const fn basis(&self) -> &BlobPlacementMovementBasis {
        &self.basis
    }
}
