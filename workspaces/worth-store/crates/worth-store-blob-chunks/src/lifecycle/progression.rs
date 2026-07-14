use worth_proof::TransitionOutcome;

use super::progression_steps::{
    admit_placement_stage, admit_reachability_stage, execute_lifecycle_replay_stage,
    lower_lifecycle_plan, ready_for_execution_stage, resolve_store_authority,
};
use crate::{
    lifecycle::authority::{
        BlobLifecycleExecutionReadyRecipe, BlobLifecycleLoweredRecipe, BlobLifecycleResolvedRecipe,
    },
    AdmittedBlobPlacement, BlobChunkReachabilityProofSet, BlobLifecycleCounterSnapshot,
    BlobLifecycleDeclaration, BlobLifecycleLoweringCapability, BlobLifecycleReadinessAuthority,
    BlobLifecycleStoreAuthority, BlobPlacementProof, LifecycleReceipt, StoredChunkDigest,
};

pub use super::progression_steps::{
    BlobLifecycleExecutionOutcome, BlobLifecycleExecutionReadyOutcome,
    BlobLifecyclePlacementAdmissionOutcome, BlobLifecycleReachabilityAdmissionOutcome,
};

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
        let (proof_recipe, counters) = resolve_store_authority(self.declaration, authority);
        BlobLifecycleResolved {
            proof_recipe,
            counters,
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
        let (proof_recipe, counters) =
            lower_lifecycle_plan(self.proof_recipe, self.counters, capability);
        BlobLifecycleLowered {
            proof_recipe,
            counters,
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
        reachability: BlobChunkReachabilityProofSet,
    ) -> BlobLifecycleReachabilityAdmissionOutcome {
        match admit_reachability_stage(self.proof_recipe, reachability, self.counters) {
            Ok((proof_recipe, reachability, counters)) => {
                TransitionOutcome::success(BlobLifecycleReachabilityAdmitted {
                    proof_recipe,
                    reachability,
                    counters,
                })
            }
            Err(denial) => TransitionOutcome::denied(denial),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobLifecycleReachabilityAdmitted {
    proof_recipe: BlobLifecycleLoweredRecipe,
    reachability: BlobChunkReachabilityProofSet,
    counters: BlobLifecycleCounterSnapshot,
}

impl BlobLifecycleReachabilityAdmitted {
    pub fn admit_placement(
        self,
        placement: AdmittedBlobPlacement,
    ) -> BlobLifecyclePlacementAdmissionOutcome {
        match admit_placement_stage(
            self.proof_recipe,
            self.reachability,
            self.counters,
            placement,
        ) {
            Ok((proof_recipe, reachability, placement, counters)) => {
                TransitionOutcome::success(BlobLifecyclePlacementAdmitted {
                    proof_recipe,
                    reachability,
                    placement,
                    counters,
                })
            }
            Err(denial) => TransitionOutcome::denied(denial),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobLifecyclePlacementAdmitted {
    proof_recipe: BlobLifecycleLoweredRecipe,
    reachability: BlobChunkReachabilityProofSet,
    placement: BlobPlacementProof,
    counters: BlobLifecycleCounterSnapshot,
}

impl BlobLifecyclePlacementAdmitted {
    pub fn ready_for_execution(
        self,
        readiness_authority: BlobLifecycleReadinessAuthority,
    ) -> BlobLifecycleExecutionReadyOutcome {
        match ready_for_execution_stage(
            self.proof_recipe,
            self.reachability,
            self.placement,
            self.counters,
            readiness_authority,
        ) {
            Ok((proof_recipe, reachability, placement, counters)) => {
                TransitionOutcome::success(BlobLifecycleExecutionReady {
                    proof_recipe,
                    reachability,
                    placement,
                    counters,
                })
            }
            Err(denial) => TransitionOutcome::denied(denial),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobLifecycleExecutionReady {
    proof_recipe: BlobLifecycleExecutionReadyRecipe,
    reachability: BlobChunkReachabilityProofSet,
    placement: BlobPlacementProof,
    counters: BlobLifecycleCounterSnapshot,
}

impl BlobLifecycleExecutionReady {
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
        match execute_lifecycle_replay_stage(
            self.proof_recipe,
            self.reachability,
            self.placement,
            self.counters,
            replay.stored_chunk_digest(),
        ) {
            Ok(receipt) => TransitionOutcome::success(BlobLifecycleExecuted { receipt }),
            Err(denial) => TransitionOutcome::denied(denial),
        }
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
    stored_chunk_digest: StoredChunkDigest,
}

impl BlobLifecycleReplayInput {
    pub(crate) const fn from_stored_chunk_digest(stored_chunk_digest: StoredChunkDigest) -> Self {
        Self {
            stored_chunk_digest,
        }
    }

    pub const fn stored_chunk_digest(&self) -> &StoredChunkDigest {
        &self.stored_chunk_digest
    }
}
