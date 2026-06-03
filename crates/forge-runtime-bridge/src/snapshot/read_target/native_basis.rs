use forge_foundational::facade::{
    canonicalization, AspectFieldLocator, AspectLocator, AspectMask, CanonicalBasisReadyArtifact,
    CanonicalizationRuleVersion, ProjectionMask,
};
use forge_proof::TransitionOutcome;

use crate::canonical_basis::canonical_basis_ready_text;

const SNAPSHOT_READ_TARGET_CANONICAL_VERSION: &str = "bridge.snapshot-read-target.v1";

pub(super) fn snapshot_read_target_canonical_basis(
    aspect_locator: &AspectLocator,
    field_locator: Option<&AspectFieldLocator>,
    projection_mask: &AspectMask<ProjectionMask>,
) -> String {
    let locator_basis =
        snapshot_read_target_locator_canonical_basis(aspect_locator, field_locator.cloned());
    let mask_basis =
        snapshot_read_target_projection_mask_canonical_basis(aspect_locator, projection_mask);
    format!(
        "snapshot-read-target|locator={}|projection-mask={}",
        canonical_basis_ready_text(&locator_basis)
            .expect("snapshot read target locator basis is renderable"),
        canonical_basis_ready_text(&mask_basis)
            .expect("snapshot read target projection mask basis is renderable"),
    )
}

fn snapshot_read_target_locator_canonical_basis(
    aspect_locator: &AspectLocator,
    field_locator: Option<AspectFieldLocator>,
) -> CanonicalBasisReadyArtifact {
    let version = snapshot_read_target_canonical_version();
    let outcome = match field_locator {
        Some(field_locator) => canonicalization()
            .basis()
            .at(version)
            .from_aspect_field_locator(field_locator),
        None => canonicalization()
            .basis()
            .at(version)
            .from_aspect_locator(aspect_locator.clone()),
    };
    expect_snapshot_read_target_basis(outcome)
}

fn snapshot_read_target_projection_mask_canonical_basis(
    aspect_locator: &AspectLocator,
    projection_mask: &AspectMask<ProjectionMask>,
) -> CanonicalBasisReadyArtifact {
    expect_snapshot_read_target_basis(
        canonicalization()
            .basis()
            .at(snapshot_read_target_canonical_version())
            .from_mask(aspect_locator.aspect_key().clone(), projection_mask.clone()),
    )
}

fn snapshot_read_target_canonical_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new(SNAPSHOT_READ_TARGET_CANONICAL_VERSION)
        .expect("snapshot read target canonical version is a valid foundational rule version")
}

fn expect_snapshot_read_target_basis(
    outcome: TransitionOutcome<
        CanonicalBasisReadyArtifact,
        forge_foundational::facade::CanonicalBasisConstructionDenial,
    >,
) -> CanonicalBasisReadyArtifact {
    match outcome {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            panic!("snapshot read target canonical basis denied: {denial:?}")
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("foundational canonical basis preparation succeeds or denies")
        }
    }
}
