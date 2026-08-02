use sha2::{Digest, Sha256};
use worth_store::physical_runtime::PhysicalMutationIdempotencyMaterial;

pub(crate) fn certification_material(
    scenario: &'static str,
    publication_ordinal: u64,
) -> PhysicalMutationIdempotencyMaterial {
    let mut digest = Sha256::new();
    digest.update(b"worth-store.integration-certification-publication.v1");
    digest.update(scenario.as_bytes());
    digest.update(publication_ordinal.to_le_bytes());
    PhysicalMutationIdempotencyMaterial::new(digest.finalize().into())
}
