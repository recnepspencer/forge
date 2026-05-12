mod contract_preparation;
mod digest_entry;
mod mask_preparation;
mod patch_preparation;
mod readiness;
mod state_preparation;

pub use contract_preparation::{
    aspect_contract_digest_preparation_basis, prepare_aspect_contract_for_digest,
};
pub use digest_entry::{
    CanonicalDigestAspectShapeKind, CanonicalDigestMaskMode, CanonicalDigestPreparationEntry,
};
pub use mask_preparation::{
    aspect_mask_digest_preparation_basis, prepare_aspect_mask_for_digest, DigestPreparationMaskMode,
};
pub use patch_preparation::{
    aspect_patch_digest_preparation_basis, prepare_aspect_patch_for_digest,
};
pub use readiness::{
    DigestPreparationReady, DigestPreparationReadyAspectContract,
    DigestPreparationReadyAspectContractArtifact, DigestPreparationReadyAspectMask,
    DigestPreparationReadyAspectMaskArtifact, DigestPreparationReadyAspectPatch,
    DigestPreparationReadyAspectPatchArtifact, DigestPreparationReadyAspectState,
    DigestPreparationReadyAspectStateArtifact, Milestone2DigestReadinessNote,
};
pub use state_preparation::{
    aspect_state_digest_preparation_basis, prepare_aspect_state_for_digest,
};

use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "canonical_ordering_and_equality",
        "stable ordering, equality, and digest-preparation basis vocabulary",
        "final digest algorithms or cryptographic receipt construction",
    )
}
