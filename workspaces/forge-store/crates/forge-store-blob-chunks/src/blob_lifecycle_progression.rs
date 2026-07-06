use std::convert::Infallible;

use forge_proof::TransitionOutcome;

use crate::{
    blob_lifecycle_authority::{
        execute_lifecycle_proof, prove_lifecycle_lowering, prove_lifecycle_readiness,
        prove_lifecycle_resolution, BlobLifecycleExecutionReadyRecipe, BlobLifecycleLoweredRecipe,
        BlobLifecycleResolvedRecipe,
    },
    AdmittedBlobPlacement, BlobChunkReachabilityProofSet, BlobLifecycleCounterSnapshot,
    BlobLifecycleDeclaration, BlobLifecycleDenial, BlobLifecycleLoweringCapability,
    BlobLifecycleReadinessAuthority, BlobLifecycleStoreAuthority, BlobPlacementProof,
    LifecycleReceipt,
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
        reachability: BlobChunkReachabilityProofSet,
    ) -> BlobLifecycleReachabilityAdmissionOutcome {
        BlobLifecycleReachabilityAdmitted {
            proof_recipe: self.proof_recipe,
            reachability,
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
    reachability: BlobChunkReachabilityProofSet,
    counters: BlobLifecycleCounterSnapshot,
}

impl BlobLifecycleReachabilityAdmitted {
    pub fn admit_placement(
        self,
        placement: AdmittedBlobPlacement,
    ) -> BlobLifecyclePlacementAdmissionOutcome {
        if !placement.matches_reachability(&self.reachability) {
            return TransitionOutcome::denied(
                BlobLifecycleDenial::PlacementReachabilityBasisMismatch {
                    counters: self.counters.record_denial(),
                },
            );
        }
        let placement = BlobPlacementProof::from_admitted_placement(&placement);
        TransitionOutcome::success(BlobLifecyclePlacementAdmitted {
            proof_recipe: self.proof_recipe,
            reachability: self.reachability,
            placement,
            counters: self.counters.record_placement_admission(),
        })
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
            || !self
                .reachability
                .matches_lifecycle_declaration(self.proof_recipe.payload().declaration())
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

pub type BlobLifecyclePlacementAdmissionOutcome = TransitionOutcome<
    BlobLifecyclePlacementAdmitted,
    BlobLifecycleDenial,
    Infallible,
    Infallible,
    Infallible,
    Infallible,
>;

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
        if self
            .proof_recipe
            .payload()
            .declaration()
            .stored_chunk_digest()
            != self.placement.stored_digest()
            || readiness_authority.admitted_placement().stored_digest()
                != self.placement.stored_digest()
            || readiness_authority.admitted_placement().security_metadata()
                != self.placement.security_metadata()
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
    reachability: BlobChunkReachabilityProofSet,
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
