use sha2::{Digest, Sha256};
use worth_foundational::facade::{
    canonicalization, AspectFieldLocator, AspectLocator, AspectMask, CanonicalBasisReadyArtifact,
    CanonicalizationRuleVersion, ProjectionMask,
};
use worth_proof::TransitionOutcome;

use crate::canonical_basis::canonical_basis_ready_text;

use super::BridgeMaterializedFieldIdentity;

const ROW_FIELD_PROJECTION_CANONICAL_VERSION: &str = "bridge.row-field-projection.v1";

pub(super) fn materialized_field_projection_canonical_basis(
    locator_basis: &CanonicalBasisReadyArtifact,
    mask_basis: &CanonicalBasisReadyArtifact,
    field_identity: &BridgeMaterializedFieldIdentity,
) -> String {
    format!(
        "row-field-projection|locator={}|projection-mask={}|identity={}",
        canonical_basis_ready_text(locator_basis)
            .expect("row field projection locator basis is renderable"),
        canonical_basis_ready_text(mask_basis)
            .expect("row field projection mask basis is renderable"),
        field_identity.as_str(),
    )
}

pub(super) fn row_field_projection_identity_from_basis(
    locator_basis: &CanonicalBasisReadyArtifact,
    mask_basis: &CanonicalBasisReadyArtifact,
) -> BridgeMaterializedFieldIdentity {
    let basis = format!(
        "row-field-identity|locator={}|projection-mask={}",
        canonical_basis_ready_text(locator_basis)
            .expect("row field projection locator basis is renderable"),
        canonical_basis_ready_text(mask_basis)
            .expect("row field projection mask basis is renderable"),
    );
    let digest = Sha256::digest(basis.as_bytes());
    BridgeMaterializedFieldIdentity::new(format!("bridge-row-field:sha256:{digest:x}"))
}

pub(super) fn row_field_projection_locator_canonical_basis(
    aspect_locator: &AspectLocator,
    field_locator: Option<AspectFieldLocator>,
) -> CanonicalBasisReadyArtifact {
    let version = row_field_projection_canonical_version();
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
    expect_row_field_projection_basis(outcome)
}

pub(super) fn row_field_projection_mask_canonical_basis(
    aspect_locator: &AspectLocator,
    projection_mask: &AspectMask<ProjectionMask>,
) -> CanonicalBasisReadyArtifact {
    expect_row_field_projection_basis(
        canonicalization()
            .basis()
            .at(row_field_projection_canonical_version())
            .from_mask(aspect_locator.aspect_key().clone(), projection_mask.clone()),
    )
}

fn row_field_projection_canonical_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new(ROW_FIELD_PROJECTION_CANONICAL_VERSION)
        .expect("row field projection canonical version is a valid foundational rule version")
}

fn expect_row_field_projection_basis(
    outcome: TransitionOutcome<
        CanonicalBasisReadyArtifact,
        worth_foundational::facade::CanonicalBasisConstructionDenial,
    >,
) -> CanonicalBasisReadyArtifact {
    match outcome {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            panic!("row field projection canonical basis denied: {denial:?}")
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("foundational canonical basis preparation succeeds or denies")
        }
    }
}
