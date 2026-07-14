use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{AsyncCompletionEnvelopeIdentityTag, BridgeIdentity};
use worth_signal::facade::{RawCompletionEnvelope, ResourcePayloadContractDigest};

use super::super::request_identity::AdmittedBridgeAsyncRequestIdentity;
use super::super::{BridgeAsyncRequestFamilyAdmission, LoweredBridgeAsyncSourceDeclaration};
use super::counters::BridgeAsyncCompletionCounters;
use super::rejection::{BridgeAsyncCompletionRejection, BridgeAsyncCompletionRejectionKind};

pub type BridgeAsyncCompletionEnvelopeIdentity = BridgeIdentity<AsyncCompletionEnvelopeIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncCompletionEnvelope {
    raw: RawCompletionEnvelope,
    envelope_identity: BridgeAsyncCompletionEnvelopeIdentity,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAsyncCompletionEnvelope {
    pub fn new(raw: RawCompletionEnvelope) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-async-completion-envelope|request={}#{}|branch={}#{}|attempt={}|payload-contract={}|payload-bytes={}",
            raw.request_id().get(),
            raw.generation().get(),
            raw.branch_epoch().branch_id().0,
            raw.branch_epoch().restore_epoch(),
            raw.attempt().get(),
            raw.payload_contract_digest().as_str(),
            raw.payload_byte_len(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            raw,
            envelope_identity: BridgeAsyncCompletionEnvelopeIdentity::admit_bridge_owned(format!(
                "bridge-async-completion-envelope-id:sha256:{digest:x}"
            )),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-async-completion-envelope:sha256:{digest:x}"
            )),
        }
    }

    pub fn raw(&self) -> RawCompletionEnvelope {
        self.raw.clone()
    }

    pub fn envelope_identity(&self) -> &BridgeAsyncCompletionEnvelopeIdentity {
        &self.envelope_identity
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedBridgeAsyncCompletionEnvelope {
    envelope: BridgeAsyncCompletionEnvelope,
    declaration_identity: Arc<str>,
    request_identity: Arc<str>,
    request_handle_digest: Arc<str>,
    payload_contract_digest: ResourcePayloadContractDigest,
    family_admission: BridgeAsyncRequestFamilyAdmission,
    counters: BridgeAsyncCompletionCounters,
}

impl ValidatedBridgeAsyncCompletionEnvelope {
    pub fn validate(
        request_identity: &AdmittedBridgeAsyncRequestIdentity,
        raw: RawCompletionEnvelope,
    ) -> Result<Self, BridgeAsyncCompletionRejection> {
        let envelope = BridgeAsyncCompletionEnvelope::new(raw);
        let handle = request_identity.request_handle();
        if envelope.raw.request_id() != handle.request_id()
            || envelope.raw.generation() != handle.generation()
            || envelope.raw.branch_epoch() != handle.branch_epoch()
        {
            return Err(BridgeAsyncCompletionRejection::new(
                BridgeAsyncCompletionRejectionKind::EnvelopeHandleMismatch,
                format!(
                    "bridge async completion envelope `{}` does not match admitted request handle {}#{} on branch {}#{}",
                    envelope.envelope_identity().as_str(),
                    handle.request_id().get(),
                    handle.generation().get(),
                    handle.branch_epoch().branch_id().0,
                    handle.branch_epoch().restore_epoch(),
                ),
            ));
        }
        if envelope.raw.attempt() != request_identity.attempt() {
            return Err(BridgeAsyncCompletionRejection::new(
                BridgeAsyncCompletionRejectionKind::EnvelopeAttemptMismatch,
                format!(
                    "bridge async completion envelope `{}` carries attempt {} but admitted request `{}` expects attempt {}",
                    envelope.envelope_identity().as_str(),
                    envelope.raw.attempt().get(),
                    request_identity.request_identity().as_str(),
                    request_identity.attempt().get(),
                ),
            ));
        }
        let payload_contract_digest = expected_payload_contract_digest(request_identity.lowered())?;
        if envelope.raw.payload_contract_digest() != &payload_contract_digest {
            return Err(BridgeAsyncCompletionRejection::new(
                BridgeAsyncCompletionRejectionKind::PayloadContractDigestMismatch,
                format!(
                    "bridge async completion envelope `{}` carries payload contract `{}` but admitted request `{}` expects `{}`",
                    envelope.envelope_identity().as_str(),
                    envelope.raw.payload_contract_digest().as_str(),
                    request_identity.request_identity().as_str(),
                    payload_contract_digest.as_str(),
                ),
            ));
        }

        Ok(Self {
            envelope,
            declaration_identity: Arc::from(
                request_identity
                    .lowered()
                    .declaration_identity()
                    .as_str()
                    .to_owned(),
            ),
            request_identity: Arc::from(request_identity.request_identity().as_str().to_owned()),
            request_handle_digest: Arc::from(format!(
                "{}#{}@{}#{}",
                handle.request_id().get(),
                handle.generation().get(),
                handle.branch_epoch().branch_id().0,
                handle.branch_epoch().restore_epoch(),
            )),
            payload_contract_digest,
            family_admission: request_identity.family_admission().clone(),
            counters: BridgeAsyncCompletionCounters::envelope_validated(),
        })
    }

    pub fn envelope(&self) -> &BridgeAsyncCompletionEnvelope {
        &self.envelope
    }

    pub fn raw(&self) -> RawCompletionEnvelope {
        self.envelope.raw()
    }

    pub fn declaration_identity(&self) -> &str {
        self.declaration_identity.as_ref()
    }

    pub fn request_identity(&self) -> &str {
        self.request_identity.as_ref()
    }

    pub fn request_handle_digest(&self) -> &str {
        self.request_handle_digest.as_ref()
    }

    pub fn payload_contract_digest(&self) -> &ResourcePayloadContractDigest {
        &self.payload_contract_digest
    }

    pub fn family_admission(&self) -> &BridgeAsyncRequestFamilyAdmission {
        &self.family_admission
    }

    pub fn counters(&self) -> &BridgeAsyncCompletionCounters {
        &self.counters
    }
}

fn expected_payload_contract_digest(
    lowered: &LoweredBridgeAsyncSourceDeclaration,
) -> Result<ResourcePayloadContractDigest, BridgeAsyncCompletionRejection> {
    if let Some(descriptor) = lowered.resource_descriptor() {
        return Ok(descriptor.payload_contract_digest().clone());
    }
    if let Some(bundle) = lowered.async_node_capability_bundle() {
        return Ok(ResourcePayloadContractDigest::new(
            bundle.payload_contract_digest().as_str(),
        ));
    }
    Err(BridgeAsyncCompletionRejection::new(
        BridgeAsyncCompletionRejectionKind::FamilyKindMismatch,
        format!(
            "bridge async completion validation requires one lowered bridge async source declaration family with a retained payload contract digest for `{}`",
            lowered.declaration_identity().as_str(),
        ),
    ))
}
