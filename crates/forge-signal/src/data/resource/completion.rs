use serde::{Deserialize, Serialize};

use super::denial::{AsyncDenialId, CompletionDenialClass};
use super::descriptor::{ResourceDescriptorId, ResourcePayloadContractDigest};
use super::lifecycle::ResourceLifecycleTransition;
use super::request::{
    ResourceAttemptId, ResourceBranchEpoch, ResourceCompletionOrdinal, ResourceGeneration,
    ResourceNodeId, ResourceRequestHandle, ResourceRequestId,
};

/// Raw host-delivered completion envelope. It is untrusted input only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawCompletionEnvelope {
    request_id: ResourceRequestId,
    generation: ResourceGeneration,
    branch_epoch: ResourceBranchEpoch,
    attempt: ResourceAttemptId,
    payload_contract_digest: ResourcePayloadContractDigest,
    payload_byte_len: u64,
}

impl RawCompletionEnvelope {
    pub fn new(
        request_id: ResourceRequestId,
        generation: ResourceGeneration,
        branch_epoch: ResourceBranchEpoch,
        attempt: ResourceAttemptId,
        payload_contract_digest: ResourcePayloadContractDigest,
        payload_byte_len: u64,
    ) -> Self {
        Self {
            request_id,
            generation,
            branch_epoch,
            attempt,
            payload_contract_digest,
            payload_byte_len,
        }
    }

    pub fn request_id(&self) -> ResourceRequestId {
        self.request_id
    }

    pub fn generation(&self) -> ResourceGeneration {
        self.generation
    }

    pub fn branch_epoch(&self) -> ResourceBranchEpoch {
        self.branch_epoch
    }

    pub fn attempt(&self) -> ResourceAttemptId {
        self.attempt
    }

    pub fn payload_contract_digest(&self) -> &ResourcePayloadContractDigest {
        &self.payload_contract_digest
    }

    pub fn payload_byte_len(&self) -> u64 {
        self.payload_byte_len
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatedCompletionEnvelope {
    handle: ResourceRequestHandle,
    attempt: ResourceAttemptId,
    payload_byte_len: u64,
}

impl ValidatedCompletionEnvelope {
    pub(crate) fn new(
        handle: ResourceRequestHandle,
        attempt: ResourceAttemptId,
        payload_byte_len: u64,
    ) -> Self {
        Self {
            handle,
            attempt,
            payload_byte_len,
        }
    }

    pub fn handle(self) -> ResourceRequestHandle {
        self.handle
    }

    pub fn attempt(self) -> ResourceAttemptId {
        self.attempt
    }

    pub fn payload_byte_len(self) -> u64 {
        self.payload_byte_len
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmittedResourceCompletion {
    handle: ResourceRequestHandle,
    node: ResourceNodeId,
    descriptor_id: ResourceDescriptorId,
    completion_ordinal: ResourceCompletionOrdinal,
    payload_byte_len: u64,
    lifecycle_transition: ResourceLifecycleTransition,
}

impl AdmittedResourceCompletion {
    pub(crate) fn new(
        handle: ResourceRequestHandle,
        node: ResourceNodeId,
        descriptor_id: ResourceDescriptorId,
        completion_ordinal: ResourceCompletionOrdinal,
        payload_byte_len: u64,
        lifecycle_transition: ResourceLifecycleTransition,
    ) -> Self {
        Self {
            handle,
            node,
            descriptor_id,
            completion_ordinal,
            payload_byte_len,
            lifecycle_transition,
        }
    }

    pub fn handle(self) -> ResourceRequestHandle {
        self.handle
    }

    pub fn node(self) -> ResourceNodeId {
        self.node
    }

    pub fn descriptor_id(self) -> ResourceDescriptorId {
        self.descriptor_id
    }

    pub fn completion_ordinal(self) -> ResourceCompletionOrdinal {
        self.completion_ordinal
    }

    pub fn payload_byte_len(self) -> u64 {
        self.payload_byte_len
    }

    pub fn lifecycle_transition(self) -> ResourceLifecycleTransition {
        self.lifecycle_transition
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedResourceCompletion {
    denial_id: AsyncDenialId,
    class: CompletionDenialClass,
    request_id: ResourceRequestId,
    generation: ResourceGeneration,
    branch_epoch: ResourceBranchEpoch,
    attempt: ResourceAttemptId,
    payload_byte_len: u64,
}

impl DeniedResourceCompletion {
    pub(crate) fn new(
        denial_id: AsyncDenialId,
        class: CompletionDenialClass,
        raw: &RawCompletionEnvelope,
    ) -> Self {
        Self {
            denial_id,
            class,
            request_id: raw.request_id(),
            generation: raw.generation(),
            branch_epoch: raw.branch_epoch(),
            attempt: raw.attempt(),
            payload_byte_len: raw.payload_byte_len(),
        }
    }

    pub fn denial_id(self) -> AsyncDenialId {
        self.denial_id
    }

    pub fn class(self) -> CompletionDenialClass {
        self.class
    }

    pub fn request_id(self) -> ResourceRequestId {
        self.request_id
    }

    pub fn generation(self) -> ResourceGeneration {
        self.generation
    }

    pub fn branch_epoch(self) -> ResourceBranchEpoch {
        self.branch_epoch
    }

    pub fn attempt(self) -> ResourceAttemptId {
        self.attempt
    }

    pub fn payload_byte_len(self) -> u64 {
        self.payload_byte_len
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StagedResourceCompletionEffect {
    admitted_completion: AdmittedResourceCompletion,
}

impl StagedResourceCompletionEffect {
    pub(crate) fn new(admitted_completion: AdmittedResourceCompletion) -> Self {
        Self {
            admitted_completion,
        }
    }

    pub fn admitted_completion(&self) -> AdmittedResourceCompletion {
        self.admitted_completion
    }

    pub fn handle(&self) -> ResourceRequestHandle {
        self.admitted_completion.handle()
    }

    pub fn node(&self) -> ResourceNodeId {
        self.admitted_completion.node()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StagedDeniedResourceCompletionEffect {
    denied_completion: DeniedResourceCompletion,
}

impl StagedDeniedResourceCompletionEffect {
    pub(crate) fn new(denied_completion: DeniedResourceCompletion) -> Self {
        Self { denied_completion }
    }

    pub fn denied_completion(&self) -> DeniedResourceCompletion {
        self.denied_completion
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommittedResourceCompletionArtifact {
    staged_effect: StagedResourceCompletionEffect,
    lifecycle_transition: ResourceLifecycleTransition,
}

impl CommittedResourceCompletionArtifact {
    pub(crate) fn new(
        staged_effect: StagedResourceCompletionEffect,
        lifecycle_transition: ResourceLifecycleTransition,
    ) -> Self {
        Self {
            staged_effect,
            lifecycle_transition,
        }
    }

    pub fn staged_effect(&self) -> &StagedResourceCompletionEffect {
        &self.staged_effect
    }

    pub fn lifecycle_transition(&self) -> ResourceLifecycleTransition {
        self.lifecycle_transition
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceCompletionRollbackSubject {
    Admitted {
        handle: ResourceRequestHandle,
        node: ResourceNodeId,
        completion_ordinal: ResourceCompletionOrdinal,
    },
    Denied {
        denial_id: AsyncDenialId,
        class: CompletionDenialClass,
        request_id: ResourceRequestId,
    },
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolledBackResourceCompletionArtifact {
    subject: ResourceCompletionRollbackSubject,
}

impl RolledBackResourceCompletionArtifact {
    pub(crate) fn admitted(staged_effect: StagedResourceCompletionEffect) -> Self {
        let admitted = staged_effect.admitted_completion();
        Self {
            subject: ResourceCompletionRollbackSubject::Admitted {
                handle: admitted.handle(),
                node: admitted.node(),
                completion_ordinal: admitted.completion_ordinal(),
            },
        }
    }

    pub(crate) fn denied(staged_effect: StagedDeniedResourceCompletionEffect) -> Self {
        let denied = staged_effect.denied_completion();
        Self {
            subject: ResourceCompletionRollbackSubject::Denied {
                denial_id: denied.denial_id(),
                class: denied.class(),
                request_id: denied.request_id(),
            },
        }
    }

    pub fn subject(&self) -> ResourceCompletionRollbackSubject {
        self.subject
    }
}
