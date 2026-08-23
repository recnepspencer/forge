use crate::QuarantineRecord;
use worth_store_authority::StoreCurrentAuthorityWitness;

use crate::{IntegrityHandoffDenial, RecoveryIntegrityHandoffReceipt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreAuthorityReadmissionDenial {
    MissingStoreAuthorityReadmission,
}

/// Readmission rebuilds current Store authority ÃƒÂ¢Ã¢â€šÂ¬Ã¢â‚¬Â deserialized quarantine records are not proof.
pub fn verify_store_authority_for_readmission(
    current_store_authority: &StoreCurrentAuthorityWitness,
) -> Result<(), StoreAuthorityReadmissionDenial> {
    if current_store_authority
        .identity()
        .aspect_key()
        .as_str()
        .is_empty()
    {
        Err(StoreAuthorityReadmissionDenial::MissingStoreAuthorityReadmission)
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
