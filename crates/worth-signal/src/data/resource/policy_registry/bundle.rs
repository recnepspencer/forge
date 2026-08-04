use serde::Serialize;

use super::descriptor_set::FrozenResourcePolicyDescriptorSet;
use super::digest::bundle_digest;
use super::identity::ResourcePolicyDigest;
use super::reference::FrozenResourcePolicyDescriptor;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoweredResourcePolicyBundle {
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
    bundle_digest: ResourcePolicyDigest,
}

impl LoweredResourcePolicyBundle {
    pub(crate) fn from_frozen_descriptors(frozen: &FrozenResourcePolicyDescriptorSet) -> Self {
        let retry = frozen.retry().clone();
        let timeout = frozen.timeout().clone();
        let cancellation = frozen.cancellation().clone();
        let stale_after = frozen.stale_after().clone();
        let supersession = frozen.supersession().clone();
        let revalidation = frozen.revalidation().clone();
        let observation = frozen.observation().clone();
        let output_continuity = frozen.output_continuity().clone();
        let retention = frozen.retention().clone();
        let diagnostics = frozen.diagnostics().clone();
        let replay = frozen.replay().clone();
        let bundle_digest = bundle_digest(&[
            &retry,
            &timeout,
            &cancellation,
            &stale_after,
            &supersession,
            &revalidation,
            &observation,
            &output_continuity,
            &retention,
            &diagnostics,
            &replay,
        ]);
        Self {
            retry,
            timeout,
            cancellation,
            stale_after,
            supersession,
            revalidation,
            observation,
            output_continuity,
            retention,
            diagnostics,
            replay,
            registry_digest: frozen.registry_digest().clone(),
            bundle_digest,
        }
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

    pub fn bundle_digest(&self) -> &ResourcePolicyDigest {
        &self.bundle_digest
    }
}

pub type ResourceResolvedPolicyBundle = LoweredResourcePolicyBundle;
