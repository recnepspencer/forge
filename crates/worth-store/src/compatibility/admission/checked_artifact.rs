use super::*;

pub(crate) fn check_artifact_with_read_receipt(
    artifact: QuarantinedDecodedArtifact,
    receipt: &ReadCompatibilityReceipt,
) -> Result<CompatibilityCheckedArtifact, CompatibilityRejection> {
    if artifact.family_id() != receipt.receipt().family_id()
        || artifact.manifest_digest() != receipt.receipt().manifest_digest()
        || artifact.semantic_version() != receipt.receipt().observed_semantic_version()
    {
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::ReceiptArtifactMismatch,
            artifact.family_id().clone(),
            "read receipt does not match quarantined artifact",
        ));
    }
    Ok(CompatibilityCheckedArtifact::new(
        artifact,
        CompatibilityDecision::Admit(receipt.receipt().relation()),
    ))
}
