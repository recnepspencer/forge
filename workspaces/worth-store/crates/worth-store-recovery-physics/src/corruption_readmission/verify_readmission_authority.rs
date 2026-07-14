use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_physical_integrity::QuarantineRecord;

use crate::{
    BlobReplayAdmissionDenial, BlobReplayAdmissionDenialKind, IntegrityHandoffDenial,
    RecoveryIntegrityHandoffReceipt,
};

/// Readmission rebuilds current Store authority ÃƒÂ¢Ã¢â€šÂ¬Ã¢â‚¬Â deserialized quarantine records are not proof.
pub fn verify_store_authority_for_readmission(
    current_store_authority: &StoreCurrentAuthorityWitness,
) -> Result<(), BlobReplayAdmissionDenial> {
    if current_store_authority
        .identity()
        .aspect_key()
        .as_str()
        .is_empty()
    {
        Err(BlobReplayAdmissionDenial::new(
            BlobReplayAdmissionDenialKind::MissingStoreAuthorityReadmission,
            None,
        ))
    } else {
        Ok(())
    }
}

/// Verifies quarantine handoff receipt basis before readmission capability is considered.
pub fn verify_quarantine_handoff_for_readmission(
    record: &QuarantineRecord,
    receipt: &RecoveryIntegrityHandoffReceipt,
) -> Result<(), IntegrityHandoffDenial> {
    receipt.require_quarantine_record_basis(record)
}
