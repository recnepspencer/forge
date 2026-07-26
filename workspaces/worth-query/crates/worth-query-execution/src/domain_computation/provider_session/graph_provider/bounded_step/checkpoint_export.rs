use std::sync::Arc;

use sha2::{Digest, Sha256};
use worth_foundational::facade::RetentionDeliveryProfile;
use worth_query_installation::facade::WorthQueryArtifactGovernanceContract;
use worth_query_installation::facade::{
    WorthQueryArtifactClassification, WorthQueryArtifactDeletionPosture,
    WorthQueryArtifactLegalHoldPosture, WorthQueryArtifactRedactionPosture,
};

use crate::domain_computation::WorthQueryGraphProviderFailure;
use crate::execution_digest::hash_protocol_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProviderCheckpointFormat {
    identity: Arc<str>,
    version: u64,
    compatibility_identity: Arc<str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProviderCheckpointExport {
    format: WorthQueryProviderCheckpointFormat,
    governance: WorthQueryArtifactGovernanceContract,
    payload: Arc<[u8]>,
    payload_digest: Arc<str>,
    payload_bytes: usize,
    contract_digest: Arc<str>,
}

pub(crate) enum WorthQueryProviderCheckpointExportInvocation {
    Returned(Result<WorthQueryProviderCheckpointExport, WorthQueryGraphProviderFailure>),
    Panicked,
}

impl WorthQueryProviderCheckpointFormat {
    pub fn new(
        identity: impl Into<Arc<str>>,
        version: u64,
        compatibility_identity: impl Into<Arc<str>>,
    ) -> Result<Self, WorthQueryGraphProviderFailure> {
        let identity = identity.into();
        let compatibility_identity = compatibility_identity.into();
        if identity.trim().is_empty() || identity.trim() != identity.as_ref() {
            return Err(WorthQueryGraphProviderFailure::new(
                "checkpoint export format identity must be nonempty and canonical",
            ));
        }
        if version == 0 {
            return Err(WorthQueryGraphProviderFailure::new(
                "checkpoint export format version must be nonzero",
            ));
        }
        if compatibility_identity.trim().is_empty()
            || compatibility_identity.trim() != compatibility_identity.as_ref()
        {
            return Err(WorthQueryGraphProviderFailure::new(
                "checkpoint export compatibility identity must be nonempty and canonical",
            ));
        }
        Ok(Self {
            identity,
            version,
            compatibility_identity,
        })
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub const fn version(&self) -> u64 {
        self.version
    }

    pub fn compatibility_identity(&self) -> &str {
        &self.compatibility_identity
    }
}

impl WorthQueryProviderCheckpointExport {
    pub fn new(
        format: WorthQueryProviderCheckpointFormat,
        governance: WorthQueryArtifactGovernanceContract,
        payload: impl Into<Arc<[u8]>>,
    ) -> Result<Self, WorthQueryGraphProviderFailure> {
        let payload = payload.into();
        let payload_digest = Arc::from(payload_digest(&payload));
        let payload_bytes = payload.len();
        let contract_digest = Arc::from(hash_protocol_parts(&contract_material(
            &format,
            &governance,
            &payload_digest,
            payload_bytes,
        )));
        Ok(Self {
            format,
            governance,
            payload,
            payload_digest,
            payload_bytes,
            contract_digest,
        })
    }

    pub fn format(&self) -> &WorthQueryProviderCheckpointFormat {
        &self.format
    }

    pub fn format_identity(&self) -> &str {
        self.format.identity()
    }

    pub const fn format_version(&self) -> u64 {
        self.format.version()
    }

    pub fn compatibility_identity(&self) -> &str {
        self.format.compatibility_identity()
    }

    pub fn governance(&self) -> &WorthQueryArtifactGovernanceContract {
        &self.governance
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn payload_digest(&self) -> &str {
        &self.payload_digest
    }

    pub const fn payload_bytes(&self) -> usize {
        self.payload_bytes
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }
}

fn payload_digest(payload: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"worth_query_provider_checkpoint_payload_v1");
    hasher.update(
        u64::try_from(payload.len())
            .unwrap_or(u64::MAX)
            .to_le_bytes(),
    );
    hasher.update(payload);
    format!("{:x}", hasher.finalize())
}

fn contract_material(
    format: &WorthQueryProviderCheckpointFormat,
    governance: &WorthQueryArtifactGovernanceContract,
    payload_digest: &str,
    payload_bytes: usize,
) -> Vec<String> {
    let mut material = vec![
        "worth_query_provider_checkpoint_export_contract_v1".to_owned(),
        format!("format:{}", format.identity()),
        format!("format-version:{}", format.version()),
        format!("compatibility:{}", format.compatibility_identity()),
        format!("payload:{payload_digest}"),
        format!("payload-bytes:{payload_bytes}"),
    ];
    material.extend(
        governance
            .audiences()
            .iter()
            .map(|audience| format!("governance-audience:{audience}")),
    );
    material.extend([
        format!(
            "governance-classification:{}",
            classification_label(governance.classification())
        ),
        format!(
            "governance-redaction:{}",
            redaction_label(governance.redaction())
        ),
        format!(
            "governance-retention:{}",
            retention_label(governance.retention())
        ),
        format!(
            "governance-deletion:{}",
            deletion_label(governance.deletion())
        ),
        format!(
            "governance-legal-hold:{}",
            legal_hold_label(governance.legal_hold())
        ),
    ]);
    material
}

const fn classification_label(value: WorthQueryArtifactClassification) -> &'static str {
    match value {
        WorthQueryArtifactClassification::Public => "public",
        WorthQueryArtifactClassification::Internal => "internal",
        WorthQueryArtifactClassification::Confidential => "confidential",
        WorthQueryArtifactClassification::Restricted => "restricted",
    }
}

const fn redaction_label(value: WorthQueryArtifactRedactionPosture) -> &'static str {
    match value {
        WorthQueryArtifactRedactionPosture::NotRequired => "not-required",
        WorthQueryArtifactRedactionPosture::CanonicalProjectionOnly => "canonical-projection-only",
        WorthQueryArtifactRedactionPosture::DomainRedactorRequired => "domain-redactor-required",
        WorthQueryArtifactRedactionPosture::NeverDisclose => "never-disclose",
    }
}

const fn retention_label(value: RetentionDeliveryProfile) -> &'static str {
    match value {
        RetentionDeliveryProfile::Ephemeral => "ephemeral",
        RetentionDeliveryProfile::Retained => "retained",
        RetentionDeliveryProfile::Durable => "durable",
    }
}

const fn deletion_label(value: WorthQueryArtifactDeletionPosture) -> &'static str {
    match value {
        WorthQueryArtifactDeletionPosture::DeleteWithRun => "delete-with-run",
        WorthQueryArtifactDeletionPosture::DeleteAfterRetention => "delete-after-retention",
        WorthQueryArtifactDeletionPosture::DomainControlled => "domain-controlled",
        WorthQueryArtifactDeletionPosture::ExternallyControlled => "externally-controlled",
    }
}

const fn legal_hold_label(value: WorthQueryArtifactLegalHoldPosture) -> &'static str {
    match value {
        WorthQueryArtifactLegalHoldPosture::NotEligible => "not-eligible",
        WorthQueryArtifactLegalHoldPosture::DomainControlled => "domain-controlled",
        WorthQueryArtifactLegalHoldPosture::RequiredWhenDirected => "required-when-directed",
    }
}

#[cfg(test)]
#[path = "checkpoint_export_tests.rs"]
mod tests;
