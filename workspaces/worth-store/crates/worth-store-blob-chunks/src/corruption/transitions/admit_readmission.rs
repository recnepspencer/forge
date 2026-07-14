use worth_store_authority::StoreCurrentAuthorityWitness;

use crate::BlobCorruptionDenial;

pub fn verify_current_store_authority_for_readmission(
    witness: &StoreCurrentAuthorityWitness,
) -> Result<(), BlobCorruptionDenial> {
    if witness.identity().aspect_key().as_str().is_empty() {
        Err(BlobCorruptionDenial::StoreAuthorityReadmissionRejected)
    } else {
        Ok(())
    }
}
