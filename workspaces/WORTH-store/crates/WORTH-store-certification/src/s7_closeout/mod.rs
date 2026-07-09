//! S.7 closeout cannot be minted from weaker public artifacts:
//!
//! ```compile_fail
//! use worth_store_certification::certify_s7_native_blob_store_closeout;
//! use worth_store_readiness::S6ClosedS7PlacementAdmissionSeed;
//!
//! let copied_receipt = String::from("receipt");
//! let copied_chunk_rows = vec![String::from("chunk-row")];
//! let copied_proof_id = String::from("proof-id");
//! let s6_seed: S6ClosedS7PlacementAdmissionSeed = todo!();
//! let raw_counters = 7usize;
//!
//! let _ = certify_s7_native_blob_store_closeout(copied_receipt);
//! let _ = certify_s7_native_blob_store_closeout(copied_chunk_rows);
//! let _ = certify_s7_native_blob_store_closeout(copied_proof_id);
//! let _ = certify_s7_native_blob_store_closeout(s6_seed);
//! let _ = certify_s7_native_blob_store_closeout("future-placeholder");
//! let _ = certify_s7_native_blob_store_closeout("terminal-projection");
//! let _ = certify_s7_native_blob_store_closeout(raw_counters);
//! ```

mod certificate;
mod classifier;
mod denial;
mod handoffs;
mod input;
mod verifier;

pub use certificate::S7NativeBlobStoreCloseout;
pub use denial::{S7CloseoutDenial, S7CloseoutShortcutAttempt, S7CloseoutShortcutRejectionReport};
pub use handoffs::{
    admit_s7_backup_non_claim_handoff, admit_s7_full_certification_non_claim_handoff,
    admit_s7_key_lifecycle_non_claim_handoff, admit_s7_layout_readiness_handoff,
    S10BlobBackupRepairNonClaimHandoff, S11KeyLifecycleNonClaimHandoff,
    S12FullCertificationNonClaimHandoff, S7S8LayoutReadinessHandoff,
};
pub use input::{
    S7CloseoutCertificationInput, S7CloseoutEvidencePolicy, S7CloseoutRequest,
    S7CloseoutShortcutInput,
};

use crate::s7_closeout::certificate::build_closeout_certificate;
use crate::s7_closeout::classifier::classify_s7_closeout_request;
use crate::s7_closeout::verifier::verify_s7_closeout_request;

pub fn evaluate_s7_closeout_request(
    request: S7CloseoutRequest,
) -> Result<S7NativeBlobStoreCloseout, S7CloseoutDenial> {
    let request = classify_s7_closeout_request(request)?;
    let verified = verify_s7_closeout_request(request)?;
    Ok(build_closeout_certificate(verified))
}

pub fn certify_s7_native_blob_store_closeout(
    input: S7CloseoutCertificationInput,
) -> Result<S7NativeBlobStoreCloseout, S7CloseoutDenial> {
    evaluate_s7_closeout_request(S7CloseoutRequest::Canonical(input))
}
