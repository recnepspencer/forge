use super::{
    ForgeServerBinaryPolicyDecision, ForgeServerFileMetadataReceipt,
    ForgeServerFileTransferProvenance,
};
use crate::{
    ForgeServerCacheabilityPolicy, ForgeServerCanonicalFilename,
    ForgeServerMetadataNormalizationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCompatibilityFileEnvelope {
    metadata_receipt: ForgeServerFileMetadataReceipt,
    canonical_filename: ForgeServerCanonicalFilename,
    metadata_normalization_receipt: ForgeServerMetadataNormalizationReceipt,
    cacheability_policy: ForgeServerCacheabilityPolicy,
    policy_decision: ForgeServerBinaryPolicyDecision,
    transfer_provenance: ForgeServerFileTransferProvenance,
    canonical_file_identity: String,
    canonical_digest: String,
}

impl ForgeServerCompatibilityFileEnvelope {
    pub(crate) fn new(
        metadata_receipt: ForgeServerFileMetadataReceipt,
        canonical_filename: ForgeServerCanonicalFilename,
        metadata_normalization_receipt: ForgeServerMetadataNormalizationReceipt,
        cacheability_policy: ForgeServerCacheabilityPolicy,
        policy_decision: ForgeServerBinaryPolicyDecision,
        transfer_provenance: ForgeServerFileTransferProvenance,
    ) -> Self {
        let canonical_file_identity = format!(
            "forge-server-compat-file-identity-v1|metadata={}|filename={}",
            metadata_receipt.metadata_identity(),
            canonical_filename.canonical(),
        );
        let canonical_digest = format!(
            "forge-server-compat-file-envelope-v2|identity={canonical_file_identity}|metadata={}|filename={}|normalization={}|cacheability={}|policy={}|transfer={}",
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

    pub fn metadata_receipt(&self) -> &ForgeServerFileMetadataReceipt {
        &self.metadata_receipt
    }

    pub fn canonical_filename(&self) -> &ForgeServerCanonicalFilename {
        &self.canonical_filename
    }

    pub fn metadata_normalization_receipt(&self) -> &ForgeServerMetadataNormalizationReceipt {
        &self.metadata_normalization_receipt
    }

    pub fn cacheability_policy(&self) -> &ForgeServerCacheabilityPolicy {
        &self.cacheability_policy
    }

    pub fn policy_decision(&self) -> &ForgeServerBinaryPolicyDecision {
        &self.policy_decision
    }

    pub fn transfer_provenance(&self) -> &ForgeServerFileTransferProvenance {
        &self.transfer_provenance
    }

    pub fn canonical_file_identity(&self) -> &str {
        &self.canonical_file_identity
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
