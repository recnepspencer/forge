use std::convert::Infallible;

use forge_proof::TransitionOutcome;

use crate::{
    lifecycle::authority::{
        execute_lifecycle_proof, prove_lifecycle_lowering, prove_lifecycle_readiness,
        prove_lifecycle_resolution, BlobLifecycleExecutionReadyRecipe, BlobLifecycleLoweredRecipe,
        BlobLifecycleResolvedRecipe,
    },
    AdmittedBlobPlacement, BlobChunkReachabilityProofSet, BlobLifecycleCounterSnapshot,
    BlobLifecycleDeclaration, BlobLifecycleDenial, BlobLifecycleLoweringCapability,
    BlobLifecycleReadinessAuthority, BlobLifecycleStoreAuthority, BlobPlacementProof,
    LifecycleReceipt,
};

pub(crate) fn resolve_store_authority(
    declaration: BlobLifecycleDeclaration,
    authority: BlobLifecycleStoreAuthority,
) -> (BlobLifecycleResolvedRecipe, BlobLifecycleCounterSnapshot) {
    (
        prove_lifecycle_resolution(authority, declaration),
        BlobLifecycleCounterSnapshot::start().record_authority_resolution(),
    )
}

pub(crate) fn lower_lifecycle_plan(
    proof_recipe: BlobLifecycleResolvedRecipe,
    counters: BlobLifecycleCounterSnapshot,
    capability: BlobLifecycleLoweringCapability,
) -> (BlobLifecycleLoweredRecipe, BlobLifecycleCounterSnapshot) {
    (
        prove_lifecycle_lowering(proof_recipe, capability),
        counters.record_lowered_plan(),
    )
}

pub(crate) fn admit_reachability_stage(
    proof_recipe: BlobLifecycleLoweredRecipe,
    reachability: BlobChunkReachabilityProofSet,
    counters: BlobLifecycleCounterSnapshot,
) -> Result<(BlobLifecycleLoweredRecipe, BlobChunkReachabilityProofSet, BlobLifecycleCounterSnapshot), BlobLifecycleDenial>
{
    let counters = counters
        .record_scoped_chunk()
        .record_reachability_admission();
    if proof_recipe.payload().declaration().stored_chunk_digest() != reachability.stored_digest()
        || !reachability.matches_lifecycle_declaration(proof_recipe.payload().declaration())
    {
        return Err(BlobLifecycleDenial::DeclarationReachabilityDigestMismatch {
            counters: counters.record_denial(),
        });
    }
    Ok((proof_recipe, reachability, counters))
}

pub(crate) fn admit_placement_stage(
    proof_recipe: BlobLifecycleLoweredRecipe,
    reachability: BlobChunkReachabilityProofSet,
    counters: BlobLifecycleCounterSnapshot,
    placement: AdmittedBlobPlacement,
) -> Result<
    (
        BlobLifecycleLoweredRecipe,
        BlobChunkReachabilityProofSet,
        BlobPlacementProof,
        BlobLifecycleCounterSnapshot,
    ),
    BlobLifecycleDenial,
> {
    if !placement.matches_reachability(&reachability) {
        return Err(BlobLifecycleDenial::PlacementReachabilityBasisMismatch {
            counters: counters.record_denial(),
        });
    }
    let placement = BlobPlacementProof::from_admitted_placement(&placement);
    Ok((
        proof_recipe,
        reachability,
        placement,
        counters.record_placement_admission(),
    ))
}

pub(crate) fn ready_for_execution_stage(
    proof_recipe: BlobLifecycleLoweredRecipe,
    reachability: BlobChunkReachabilityProofSet,
    placement: BlobPlacementProof,
    counters: BlobLifecycleCounterSnapshot,
    readiness_authority: BlobLifecycleReadinessAuthority,
) -> Result<
    (
        BlobLifecycleExecutionReadyRecipe,
        BlobChunkReachabilityProofSet,
        BlobPlacementProof,
        BlobLifecycleCounterSnapshot,
    ),
    BlobLifecycleDenial,
> {
    if proof_recipe.payload().declaration().stored_chunk_digest() != placement.stored_digest()
        || readiness_authority.admitted_placement().stored_digest() != placement.stored_digest()
        || readiness_authority.admitted_placement().security_metadata()
            != placement.security_metadata()
    {
        return Err(BlobLifecycleDenial::DeclarationPlacementDigestMismatch {
            counters: counters.record_denial(),
        });
    }
    let (_seed, proof_recipe) = prove_lifecycle_readiness(proof_recipe, readiness_authority);
    Ok((
        proof_recipe,
        reachability,
        placement,
        counters.record_execution_ready(),
    ))
}

pub(crate) fn execute_lifecycle_replay_stage(
    proof_recipe: BlobLifecycleExecutionReadyRecipe,
    reachability: BlobChunkReachabilityProofSet,
    placement: BlobPlacementProof,
    counters: BlobLifecycleCounterSnapshot,
    replay_digest: &crate::StoredChunkDigest,
) -> Result<LifecycleReceipt, BlobLifecycleDenial> {
    if replay_digest != proof_recipe.payload().declaration().stored_chunk_digest()
        || replay_digest != reachability.stored_digest()
    {
        return Err(BlobLifecycleDenial::ReplayStoredChunkDigestMismatch {
            counters: counters.record_denial(),
        });
    }
    let executed_proof = execute_lifecycle_proof(proof_recipe);
    Ok(LifecycleReceipt::new(
        reachability,
        placement,
        counters.record_executed_receipt(),
        executed_proof,
    ))
}

pub type BlobLifecycleReachabilityAdmissionOutcome = TransitionOutcome<
    super::progression::BlobLifecycleReachabilityAdmitted,
    BlobLifecycleDenial,
    Infallible,
    Infallible,
    Infallible,
    Infallible,
>;

pub type BlobLifecyclePlacementAdmissionOutcome = TransitionOutcome<
    super::progression::BlobLifecyclePlacementAdmitted,
    BlobLifecycleDenial,
    Infallible,
    Infallible,
    Infallible,
    Infallible,
>;

pub type BlobLifecycleExecutionReadyOutcome = TransitionOutcome<
    super::progression::BlobLifecycleExecutionReady,
    BlobLifecycleDenial,
    Infallible,
    Infallible,
    Infallible,
    Infallible,
>;

pub type BlobLifecycleExecutionOutcome = TransitionOutcome<
    super::progression::BlobLifecycleExecuted,
    BlobLifecycleDenial,
    Infallible,
    Infallible,
    Infallible,
    Infallible,
>;