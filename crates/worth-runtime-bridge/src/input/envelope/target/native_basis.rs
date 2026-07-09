use worth_foundational::facade::{
    canonicalization, AspectFieldLocator, AspectLocator, AspectMask, CanonicalBasisReadyArtifact,
    CanonicalizationRuleVersion, DigestPreparationMaskMode, MutationMask, ProjectionMask,
};
use worth_proof::TransitionOutcome;

use super::committed_patch_surface_kind_label;
use crate::canonical_basis::canonical_basis_ready_text;
use crate::mapping::TruthDeltaSurfaceKind;

const COMMITTED_PATCH_TARGET_CANONICAL_VERSION: &str = "bridge.committed-patch-target.v1";

pub(super) fn committed_patch_target_canonical_basis(
    aspect_locator: &AspectLocator,
    field_locator: Option<&AspectFieldLocator>,
    mutation_mask: &AspectMask<MutationMask>,
    projection_mask: &AspectMask<ProjectionMask>,
    surface_kind: TruthDeltaSurfaceKind,
) -> String {
    let locator_basis =
        committed_patch_target_locator_canonical_basis(aspect_locator, field_locator.cloned());
    let mutation_mask_basis =
        committed_patch_target_mask_canonical_basis(aspect_locator, mutation_mask);
    let projection_mask_basis =
        committed_patch_target_mask_canonical_basis(aspect_locator, projection_mask);
    format!(
        "committed-patch-target|locator={}|mutation-mask={}|projection-mask={}|kind={}",
        canonical_basis_ready_text(&locator_basis)
            .expect("committed patch target locator basis is renderable"),
        canonical_basis_ready_text(&mutation_mask_basis)
            .expect("committed patch target mutation mask basis is renderable"),
        canonical_basis_ready_text(&projection_mask_basis)
            .expect("committed patch target projection mask basis is renderable"),
        committed_patch_surface_kind_label(surface_kind),
    )
}

fn committed_patch_target_locator_canonical_basis(
    aspect_locator: &AspectLocator,
    field_locator: Option<AspectFieldLocator>,
) -> CanonicalBasisReadyArtifact {
    let outcome = match field_locator {
        Some(field_locator) => canonicalization()
            .basis()
            .at(committed_patch_target_canonical_version())
            .from_aspect_field_locator(field_locator),
        None => canonicalization()
            .basis()
            .at(committed_patch_target_canonical_version())
            .from_aspect_locator(aspect_locator.clone()),
    };
    expect_committed_patch_target_basis(outcome)
}

fn committed_patch_target_mask_canonical_basis<M>(
    aspect_locator: &AspectLocator,
    mask: &AspectMask<M>,
) -> CanonicalBasisReadyArtifact
where
    M: Clone + DigestPreparationMaskMode,
    AspectMask<M>: Clone,
{
    expect_committed_patch_target_basis(
        canonicalization()
            .basis()
            .at(committed_patch_target_canonical_version())
            .from_mask(aspect_locator.aspect_key().clone(), mask.clone()),
    )
}

fn committed_patch_target_canonical_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new(COMMITTED_PATCH_TARGET_CANONICAL_VERSION)
        .expect("committed patch target canonical version is a valid foundational rule version")
}

fn expect_committed_patch_target_basis(
    outcome: TransitionOutcome<
        CanonicalBasisReadyArtifact,
        worth_foundational::facade::CanonicalBasisConstructionDenial,
    >,
) -> CanonicalBasisReadyArtifact {
    match outcome {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            panic!("committed patch target canonical basis denied: {denial:?}")
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("foundational canonical basis preparation succeeds or denies")
        }
    }
}
