use super::{
    WorthServerBinaryPolicyDecision, WorthServerFileMetadataReceipt,
    WorthServerFileTransferProvenance,
};
use crate::{
    WorthServerCacheabilityPolicy, WorthServerCanonicalFilename,
    WorthServerMetadataNormalizationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerCompatibilityFileEnvelope {
    metadata_receipt: WorthServerFileMetadataReceipt,
    canonical_filename: WorthServerCanonicalFilename,
    metadata_normalization_receipt: WorthServerMetadataNormalizationReceipt,
    cacheability_policy: WorthServerCacheabilityPolicy,
    policy_decision: WorthServerBinaryPolicyDecision,
    transfer_provenance: WorthServerFileTransferProvenance,
    canonical_file_identity: String,
    canonical_digest: String,
}

impl WorthServerCompatibilityFileEnvelope {
    pub(crate) fn new(
        metadata_receipt: WorthServerFileMetadataReceipt,
        canonical_filename: WorthServerCanonicalFilename,
        metadata_normalization_receipt: WorthServerMetadataNormalizationReceipt,
        cacheability_policy: WorthServerCacheabilityPolicy,
        policy_decision: WorthServerBinaryPolicyDecision,
        transfer_provenance: WorthServerFileTransferProvenance,
    ) -> Self {
        let canonical_file_identity = format!(
            "worth-server-compat-file-identity-v1|metadata={}|filename={}",
            metadata_receipt.metadata_identity(),
            canonical_filename.canonical(),
        );
        let canonical_digest = format!(
            "worth-server-compat-file-envelope-v2|identity={canonical_file_identity}|metadata={}|filename={}|normalization={}|cacheability={}|policy={}|transfer={}",
            metadata_receipt.canonical_digest(),
            canonical_filename.canonical_digest(),
            metadata_normalization_receipt.canonical_digest(),
            cacheability_policy.canonical_digest(),
            policy_decision.canonical_digest(),
            transfer_provenance.canonical_digest(),
        );
        Self {
            metadata_receipt,
            canonical_filename,
            metadata_normalization_receipt,
            cacheability_policy,
            policy_decision,
            transfer_provenance,
            canonical_file_identity,
            canonical_digest,
        }
    }

    pub fn metadata_receipt(&self) -> &WorthServerFileMetadataReceipt {
        &self.metadata_receipt
    }

    pub fn canonical_filename(&self) -> &WorthServerCanonicalFilename {
        &self.canonical_filename
    }

    pub fn metadata_normalization_receipt(&self) -> &WorthServerMetadataNormalizationReceipt {
        &self.metadata_normalization_receipt
    }

    pub fn cacheability_policy(&self) -> &WorthServerCacheabilityPolicy {
        &self.cacheability_policy
    }

    pub fn policy_decision(&self) -> &WorthServerBinaryPolicyDecision {
        &self.policy_decision
    }

    pub fn transfer_provenance(&self) -> &WorthServerFileTransferProvenance {
        &self.transfer_provenance
    }

    pub fn canonical_file_identity(&self) -> &str {
        &self.canonical_file_identity
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
