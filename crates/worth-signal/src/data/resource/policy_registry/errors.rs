use super::super::policy::ResourcePolicyName;
use super::identity::{
    ResourcePolicyCompatibilityPosture, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicyVersion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourcePolicyRegistryError {
    DuplicateId(ResourcePolicyDescriptorId),
    DuplicateName {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
    },
    MalformedDescriptor {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
        reason: &'static str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourcePolicyResolutionError {
    UnknownPolicy {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
    },
    MissingDescriptor {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
    },
    RegistryDigestDrift {
        expected: ResourcePolicyDigest,
        actual: ResourcePolicyDigest,
    },
    IncompatibleDescriptor {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
        version: ResourcePolicyVersion,
        compatibility_posture: ResourcePolicyCompatibilityPosture,
    },
    MalformedDescriptor {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
        reason: &'static str,
    },
    UnsupportedExecutablePolicy {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
        reason: &'static str,
    },
}
