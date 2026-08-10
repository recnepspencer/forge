use crate::subscription_support::trust::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

pub(super) fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<String, SupportTrustFailure> {
    let bytes = serde_json::to_vec(value).map_err(|_| {
        SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "domain support certification evidence must serialize deterministically",
        )
    })?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(format!("{:x}", hasher.finalize()))
}
