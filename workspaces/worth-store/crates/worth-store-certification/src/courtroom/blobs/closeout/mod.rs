mod certificate;
mod classifier;
mod denial;
mod input;
mod materialization;
mod proof;
mod source_denial;
mod sources;
#[cfg(test)]
mod tests;
mod verifier;

pub use certificate::BlobStoreCloseoutCertificate;
pub use denial::{
    BlobCloseoutDenial, BlobCloseoutShortcutAttempt, BlobCloseoutShortcutRejectionReport,
};
pub use input::{
    BlobCloseoutCertificationInput, BlobCloseoutEvidencePolicy, BlobCloseoutRequest,
    BlobCloseoutShortcutInput,
};
pub use materialization::{materialize_blob_closeout_evidence, BlobCloseoutEvidenceBundle};
pub use proof::{BlobCloseoutProofSummary, BlobCloseoutProofTopology};
pub use source_denial::BlobCloseoutSourceDenial;
#[cfg(any(test, feature = "certification-test-support"))]
pub use sources::blob_harness_closeout_sources_for_certification;
pub use sources::BlobCloseoutSources;

use certificate::build_closeout_certificate;
use classifier::classify_blob_closeout_request;
use verifier::verify_blob_closeout_request;

pub fn evaluate_blob_closeout_request(
    request: BlobCloseoutRequest,
) -> Result<BlobStoreCloseoutCertificate, BlobCloseoutDenial> {
    let request = classify_blob_closeout_request(request)?;
    let verified = verify_blob_closeout_request(request)?;
    Ok(build_closeout_certificate(verified))
}

pub fn certify_native_blob_store_closeout(
    input: BlobCloseoutCertificationInput,
) -> Result<BlobStoreCloseoutCertificate, BlobCloseoutDenial> {
    evaluate_blob_closeout_request(BlobCloseoutRequest::Canonical(Box::new(input)))
}
