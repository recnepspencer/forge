mod authority_identity;
mod category_ids;

pub use authority_identity::{
    admit_foundational_authority_identity, admit_foundational_external_identity_token,
    admitted_foundational_identity_value, derive_foundational_digest_identity_evidence,
    project_foundational_identity, readmit_foundational_authority_identity,
    readmit_revalidated_foundational_authority_identity, revalidated_foundational_identity_value,
    FoundationalAdmittedIdentityValue, FoundationalAuthorityIdentity,
    FoundationalBoundaryBridgedIdentity, FoundationalDigestIdentityEvidence,
    FoundationalExternalIdentityToken, FoundationalIdentityBasis,
    FoundationalIdentityDigestDerivationEvidence, FoundationalIdentityKind,
    FoundationalIdentityProjectionEvidence, FoundationalProjectionIdentity,
    FoundationalRevalidatedIdentityValue,
};
pub use category_ids::{
    BoundaryArtifactId, BoundaryEpoch, BoundaryHandle, CanonicalDigestId, EquivalenceBasisId,
};

use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "identity_categories",
        "typed identity, handle, key, and basis-id boundary categories",
        "producer-private identity allocation or storage indexes",
    )
}
