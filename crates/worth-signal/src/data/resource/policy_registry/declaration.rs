use serde::Serialize;

use super::super::declaration::ResourceNodeDeclaration;
use super::errors::ResourcePolicyResolutionError;
use super::identity::ResourcePolicyDigest;
use super::reference::ValidatedResourcePolicyReference;
use super::registry::FrozenResourcePolicyRegistry;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidatedResourcePolicyDeclaration {
    declaration: ResourceNodeDeclaration,
    retry: ValidatedResourcePolicyReference,
    timeout: ValidatedResourcePolicyReference,
    cancellation: ValidatedResourcePolicyReference,
    stale_after: ValidatedResourcePolicyReference,
    supersession: ValidatedResourcePolicyReference,
    revalidation: ValidatedResourcePolicyReference,
    observation: ValidatedResourcePolicyReference,
    output_continuity: ValidatedResourcePolicyReference,
    retention: ValidatedResourcePolicyReference,
    diagnostics: ValidatedResourcePolicyReference,
    replay: ValidatedResourcePolicyReference,
    registry_digest: ResourcePolicyDigest,
}

impl ValidatedResourcePolicyDeclaration {
    pub(crate) fn from_declaration(
        declaration: &ResourceNodeDeclaration,
        registry: &FrozenResourcePolicyRegistry,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        Ok(Self {
            declaration: declaration.clone(),
            retry: registry.resolve_retry(declaration)?,
            timeout: registry.resolve_timeout(declaration.timeout_policy())?,
            cancellation: registry.resolve_cancellation(declaration.cancellation_policy())?,
            stale_after: registry.resolve_stale_after(declaration.stale_after_policy())?,
            supersession: registry.resolve_supersession(declaration.supersession_policy())?,
            revalidation: registry.resolve_revalidation(declaration.revalidation_policy())?,
            observation: registry.resolve_observation(declaration.observation_policy())?,
            output_continuity: registry
                .resolve_output_continuity(declaration.output_continuity_policy())?,
            retention: registry.resolve_retention(declaration.retention_policy())?,
            diagnostics: registry.resolve_diagnostics(declaration.diagnostics_policy())?,
            replay: registry.resolve_replay(declaration.replay_policy())?,
            registry_digest: registry.registry_digest().clone(),
        })
    }

    pub fn declaration(&self) -> &ResourceNodeDeclaration {
        &self.declaration
    }

    pub fn retry(&self) -> &ValidatedResourcePolicyReference {
        &self.retry
    }

    pub fn timeout(&self) -> &ValidatedResourcePolicyReference {
        &self.timeout
    }

    pub fn cancellation(&self) -> &ValidatedResourcePolicyReference {
        &self.cancellation
    }

    pub fn stale_after(&self) -> &ValidatedResourcePolicyReference {
        &self.stale_after
    }

    pub fn supersession(&self) -> &ValidatedResourcePolicyReference {
        &self.supersession
    }

    pub fn revalidation(&self) -> &ValidatedResourcePolicyReference {
        &self.revalidation
    }

    pub fn observation(&self) -> &ValidatedResourcePolicyReference {
        &self.observation
    }

    pub fn output_continuity(&self) -> &ValidatedResourcePolicyReference {
        &self.output_continuity
    }

    pub fn retention(&self) -> &ValidatedResourcePolicyReference {
        &self.retention
    }

    pub fn diagnostics(&self) -> &ValidatedResourcePolicyReference {
        &self.diagnostics
    }

    pub fn replay(&self) -> &ValidatedResourcePolicyReference {
        &self.replay
    }

    pub fn registry_digest(&self) -> &ResourcePolicyDigest {
        &self.registry_digest
    }
}
