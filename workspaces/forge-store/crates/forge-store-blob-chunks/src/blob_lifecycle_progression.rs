use std::convert::Infallible;

use forge_proof::TransitionOutcome;

use crate::{
    blob_lifecycle_authority::{
        execute_lifecycle_proof, prove_lifecycle_lowering, prove_lifecycle_readiness,
        prove_lifecycle_resolution, BlobLifecycleExecutionReadyRecipe, BlobLifecycleLoweredRecipe,
        BlobLifecycleResolvedRecipe,
    },
    BlobLifecycleCounterSnapshot, BlobLifecycleDeclaration, BlobLifecycleDenial,
    BlobLifecycleLoweringCapability, BlobLifecycleReadinessAuthority, BlobLifecycleStoreAuthority,
    BlobPlacementProof, BlobReachabilityProof, LifecycleReceipt, ScopedBlobChunk,
};

pub type BlobLifecycleExecutionOutcome = TransitionOutcome<
    BlobLifecycleExecuted,
    BlobLifecycleDenial,
    Infallible,
    Infallible,
    Infallible,
    Infallible,
>;

#[derive(Debug, PartialEq, Eq)]
pub struct BlobLifecycleAdmission {
    declaration: BlobLifecycleDeclaration,
}

impl BlobLifecycleAdmission {
    pub const fn start(declaration: BlobLifecycleDeclaration) -> Self {
        Self { declaration }
    }

    pub fn resolve_store_authority(
        self,
        authority: BlobLifecycleStoreAuthority,
    ) -> BlobLifecycleResolved {
        BlobLifecycleResolved {
            proof_recipe: prove_lifecycle_resolution(authority, self.declaration),
            counters: BlobLifecycleCounterSnapshot::start().record_authority_resolution(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobLifecycleResolved {
    proof_recipe: BlobLifecycleResolvedRecipe,
    counters: BlobLifecycleCounterSnapshot,
}

impl BlobLifecycleResolved {
    pub fn lower_lifecycle_plan(
        self,
        capability: BlobLifecycleLoweringCapability,
    ) -> BlobLifecycleLowered {
        BlobLifecycleLowered {
            proof_recipe: prove_lifecycle_lowering(self.proof_recipe, capability),
            counters: self.counters.record_lowered_plan(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobLifecycleLowered {
    proof_recipe: BlobLifecycleLoweredRecipe,
    counters: BlobLifecycleCounterSnapshot,
}

impl BlobLifecycleLowered {
    pub fn admit_reachability(
        self,
        scoped_chunk: ScopedBlobChunk,
    ) -> BlobLifecycleReachabilityAdmissionOutcome {
        BlobLifecycleReachabilityAdmitted {
            proof_recipe: self.proof_recipe,
            reachability: BlobReachabilityProof::from_scoped_chunk(scoped_chunk),
            counters: self
                .counters
                .record_scoped_chunk()
                .record_reachability_admission(),
        }
        .reject_if_declaration_reachability_digest_mismatch()
    }
}

pub type BlobLifecycleReachabilityAdmissionOutcome = TransitionOutcome<
    BlobLifecycleReachabilityAdmitted,
    BlobLifecycleDenial,
    Infallible,
    Infallible,
    Infallible,
    Infallible,
>;

#[derive(Debug, PartialEq, Eq)]
pub struct BlobLifecycleReachabilityAdmitted {
    proof_recipe: BlobLifecycleLoweredRecipe,
    reachability: BlobReachabilityProof,
    counters: BlobLifecycleCounterSnapshot,
}

impl BlobLifecycleReachabilityAdmitted {
    pub fn admit_placement(
        self,
        readiness_authority: &BlobLifecycleReadinessAuthority,
    ) -> BlobLifecyclePlacementAdmitted {
        let placement = BlobPlacementProof::from_reachability_and_placement_readiness(
            &self.reachability,
            readiness_authority.placement_readiness(),
        );
        BlobLifecyclePlacementAdmitted {
            proof_recipe: self.proof_recipe,
            reachability: self.reachability,
            placement,
            counters: self.counters.record_placement_admission(),
        }
    }

    fn reject_if_declaration_reachability_digest_mismatch(
        self,
    ) -> BlobLifecycleReachabilityAdmissionOutcome {
        if self
            .proof_recipe
            .payload()
            .declaration()
            .stored_chunk_digest()
            != self.reachability.stored_digest()
        {
            return TransitionOutcome::denied(
                BlobLifecycleDenial::DeclarationReachabilityDigestMismatch {
                    counters: self.counters.record_denial(),
                },
            );
        }
        TransitionOutcome::success(self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobLifecyclePlacementAdmitted {
    proof_recipe: BlobLifecycleLoweredRecipe,
    reachability: BlobReachabilityProof,
    placement: BlobPlacementProof,
    counters: BlobLifecycleCounterSnapshot,
}

impl BlobLifecyclePlacementAdmitted {
    pub fn ready_for_execution(
        self,
        readiness_authority: BlobLifecycleReadinessAuthority,
    ) -> BlobLifecycleExecutionReadyOutcome {
        if self
            .proof_recipe
            .payload()
            .declaration()
            .stored_chunk_digest()
            != self.placement.stored_digest()
        {
            return TransitionOutcome::denied(
                BlobLifecycleDenial::DeclarationPlacementDigestMismatch {
                    counters: self.counters.record_denial(),
                },
            );
        }
        let (_seed, proof_recipe) =
            prove_lifecycle_readiness(self.proof_recipe, readiness_authority);
        BlobLifecycleExecutionReady {
            proof_recipe,
            reachability: self.reachability,
            placement: self.placement,
            counters: self.counters.record_execution_ready(),
        }
        .into_success()
    }
}

pub type BlobLifecycleExecutionReadyOutcome = TransitionOutcome<
    BlobLifecycleExecutionReady,
    BlobLifecycleDenial,
    Infallible,
    Infallible,
    Infallible,
    Infallible,
>;

#[derive(Debug, PartialEq, Eq)]
pub struct BlobLifecycleExecutionReady {
    proof_recipe: BlobLifecycleExecutionReadyRecipe,
    reachability: BlobReachabilityProof,
    placement: BlobPlacementProof,
    counters: BlobLifecycleCounterSnapshot,
}

impl BlobLifecycleExecutionReady {
    fn into_success(self) -> BlobLifecycleExecutionReadyOutcome {
        TransitionOutcome::success(self)
    }

    pub fn admitted_replay_input(&self) -> BlobLifecycleReplayInput {
        BlobLifecycleReplayInput::from_stored_chunk_digest(
            self.proof_recipe
                .payload()
                .declaration()
                .stored_chunk_digest()
                .clone(),
        )
    }

    pub fn execute_lifecycle_replay(
        self,
        replay: BlobLifecycleReplayInput,
    ) -> BlobLifecycleExecutionOutcome {
        if replay.stored_chunk_digest()
            != self
                .proof_recipe
                .payload()
                .declaration()
                .stored_chunk_digest()
            || replay.stored_chunk_digest() != self.reachability.stored_digest()
        {
            return TransitionOutcome::denied(
                BlobLifecycleDenial::ReplayStoredChunkDigestMismatch {
                    counters: self.counters.record_denial(),
                },
            );
        }

        let executed_proof = execute_lifecycle_proof(self.proof_recipe);
        let receipt = LifecycleReceipt::new(
            self.reachability,
            self.placement,
            self.counters.record_executed_receipt(),
            executed_proof,
        );
        TransitionOutcome::success(BlobLifecycleExecuted { receipt })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobLifecycleExecuted {
    receipt: LifecycleReceipt,
}

impl BlobLifecycleExecuted {
    pub const fn lifecycle_receipt(&self) -> &LifecycleReceipt {
        &self.receipt
    }

    pub fn into_lifecycle_receipt(self) -> LifecycleReceipt {
        self.receipt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobLifecycleReplayInput {
    stored_chunk_digest: crate::StoredChunkDigest,
}

impl BlobLifecycleReplayInput {
    pub(crate) const fn from_stored_chunk_digest(
        stored_chunk_digest: crate::StoredChunkDigest,
    ) -> Self {
        Self {
            stored_chunk_digest,
        }
    }

    pub const fn stored_chunk_digest(&self) -> &crate::StoredChunkDigest {
        &self.stored_chunk_digest
    }
}
