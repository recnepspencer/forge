use serde::Serialize;

use super::super::policy::ResourcePolicyName;
use super::descriptor::ResourcePolicyDescriptor;
use super::digest::frozen_policy_descriptor_digest;
use super::errors::ResourcePolicyResolutionError;
use super::identity::{
    ResourcePolicyDescriptorId, ResourcePolicyDigest, ResourcePolicyKind,
    ResourcePolicySelectionBasis,
};
use super::registry::FrozenResourcePolicyRegistry;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidatedResourcePolicyReference {
    descriptor_id: ResourcePolicyDescriptorId,
    kind: ResourcePolicyKind,
    semantic_name: ResourcePolicyName,
    selection_basis: ResourcePolicySelectionBasis,
    parameter_digest: ResourcePolicyDigest,
}

impl ValidatedResourcePolicyReference {
    pub(super) fn new(
        descriptor: ResourcePolicyDescriptor,
        selection_basis: ResourcePolicySelectionBasis,
        parameter_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            descriptor_id: descriptor.id(),
            kind: descriptor.kind(),
            semantic_name: descriptor.semantic_name().clone(),
            selection_basis,
            parameter_digest,
        }
    }

    pub fn descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.descriptor_id
    }

    pub fn kind(&self) -> ResourcePolicyKind {
        self.kind
    }

    pub fn semantic_name(&self) -> &ResourcePolicyName {
        &self.semantic_name
    }

    pub fn selection_basis(&self) -> ResourcePolicySelectionBasis {
        self.selection_basis
    }

    pub fn parameter_digest(&self) -> &ResourcePolicyDigest {
        &self.parameter_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FrozenResourcePolicyDescriptor {
    descriptor: ResourcePolicyDescriptor,
    selection_basis: ResourcePolicySelectionBasis,
    parameter_digest: ResourcePolicyDigest,
    frozen_digest: ResourcePolicyDigest,
}

impl FrozenResourcePolicyDescriptor {
    pub(super) fn from_validated_reference(
        reference: &ValidatedResourcePolicyReference,
        registry: &FrozenResourcePolicyRegistry,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        let descriptor = registry
            .resolve_by_id(reference.descriptor_id())
            .cloned()
            .ok_or_else(|| ResourcePolicyResolutionError::MissingDescriptor {
                kind: reference.kind(),
                name: reference.semantic_name().clone(),
            })?;
        let frozen_digest = frozen_policy_descriptor_digest(
            &descriptor,
            reference.selection_basis(),
            reference.parameter_digest(),
        );
        Ok(Self {
            descriptor,
            selection_basis: reference.selection_basis(),
            parameter_digest: reference.parameter_digest().clone(),
            frozen_digest,
        })
    }

    pub fn descriptor(&self) -> &ResourcePolicyDescriptor {
        &self.descriptor
    }

    pub fn selection_basis(&self) -> ResourcePolicySelectionBasis {
        self.selection_basis
    }

    pub fn parameter_digest(&self) -> &ResourcePolicyDigest {
        &self.parameter_digest
    }

    pub fn frozen_digest(&self) -> &ResourcePolicyDigest {
        &self.frozen_digest
    }

    pub fn resolved_digest(&self) -> &ResourcePolicyDigest {
        &self.frozen_digest
    }
}

pub type ResourceResolvedPolicy = FrozenResourcePolicyDescriptor;
