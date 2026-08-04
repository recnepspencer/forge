use serde::Serialize;

use super::declaration::ValidatedResourcePolicyDeclaration;
use super::errors::ResourcePolicyResolutionError;
use super::identity::ResourcePolicyDigest;
use super::reference::FrozenResourcePolicyDescriptor;
use super::registry::FrozenResourcePolicyRegistry;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FrozenResourcePolicyDescriptorSet {
    retry: FrozenResourcePolicyDescriptor,
    timeout: FrozenResourcePolicyDescriptor,
    cancellation: FrozenResourcePolicyDescriptor,
    stale_after: FrozenResourcePolicyDescriptor,
    supersession: FrozenResourcePolicyDescriptor,
    revalidation: FrozenResourcePolicyDescriptor,
    observation: FrozenResourcePolicyDescriptor,
    output_continuity: FrozenResourcePolicyDescriptor,
    retention: FrozenResourcePolicyDescriptor,
    diagnostics: FrozenResourcePolicyDescriptor,
    replay: FrozenResourcePolicyDescriptor,
    registry_digest: ResourcePolicyDigest,
}

impl FrozenResourcePolicyDescriptorSet {
    pub(crate) fn from_validated_declaration(
        validated: &ValidatedResourcePolicyDeclaration,
        registry: &FrozenResourcePolicyRegistry,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        if validated.registry_digest() != registry.registry_digest() {
            return Err(ResourcePolicyResolutionError::RegistryDigestDrift {
                expected: validated.registry_digest().clone(),
                actual: registry.registry_digest().clone(),
            });
        }
        Ok(Self {
            retry: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.retry(),
                registry,
            )?,
            timeout: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.timeout(),
                registry,
            )?,
            cancellation: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.cancellation(),
                registry,
            )?,
            stale_after: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.stale_after(),
                registry,
            )?,
            supersession: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.supersession(),
                registry,
            )?,
            revalidation: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.revalidation(),
                registry,
            )?,
            observation: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.observation(),
                registry,
            )?,
            output_continuity: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.output_continuity(),
                registry,
            )?,
            retention: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.retention(),
                registry,
            )?,
            diagnostics: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.diagnostics(),
                registry,
            )?,
            replay: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.replay(),
                registry,
            )?,
            registry_digest: validated.registry_digest().clone(),
        })
    }

    pub fn retry(&self) -> &FrozenResourcePolicyDescriptor {
        &self.retry
    }
    pub fn timeout(&self) -> &FrozenResourcePolicyDescriptor {
        &self.timeout
    }
    pub fn cancellation(&self) -> &FrozenResourcePolicyDescriptor {
        &self.cancellation
    }
    pub fn stale_after(&self) -> &FrozenResourcePolicyDescriptor {
        &self.stale_after
    }
    pub fn supersession(&self) -> &FrozenResourcePolicyDescriptor {
        &self.supersession
    }
    pub fn revalidation(&self) -> &FrozenResourcePolicyDescriptor {
        &self.revalidation
    }
    pub fn observation(&self) -> &FrozenResourcePolicyDescriptor {
        &self.observation
    }
    pub fn output_continuity(&self) -> &FrozenResourcePolicyDescriptor {
        &self.output_continuity
    }
    pub fn retention(&self) -> &FrozenResourcePolicyDescriptor {
        &self.retention
    }
    pub fn diagnostics(&self) -> &FrozenResourcePolicyDescriptor {
        &self.diagnostics
    }
    pub fn replay(&self) -> &FrozenResourcePolicyDescriptor {
        &self.replay
    }
    pub fn registry_digest(&self) -> &ResourcePolicyDigest {
        &self.registry_digest
    }
}
