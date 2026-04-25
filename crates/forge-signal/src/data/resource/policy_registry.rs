use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::declaration::ResourceNodeDeclaration;
use super::policy::{
    ResourceCancellationPolicyDeclaration, ResourceObservationPolicyDeclaration,
    ResourceOutputContinuityPolicyDeclaration, ResourcePolicyName,
    ResourceRetentionPolicyDeclaration, ResourceRetryPolicyDeclaration,
    ResourceRevalidationPolicyDeclaration, ResourceStaleAfterPolicyDeclaration,
    ResourceSupersessionPolicyDeclaration, ResourceTimeoutPolicyDeclaration,
};
use super::summary::ResourceCostContractId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ResourcePolicyKind {
    Retry,
    Timeout,
    Cancellation,
    StaleAfter,
    Supersession,
    Revalidation,
    Observation,
    OutputContinuity,
    Retention,
}

impl ResourcePolicyKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Retry => "retry",
            Self::Timeout => "timeout",
            Self::Cancellation => "cancellation",
            Self::StaleAfter => "stale-after",
            Self::Supersession => "supersession",
            Self::Revalidation => "revalidation",
            Self::Observation => "observation",
            Self::OutputContinuity => "output-continuity",
            Self::Retention => "retention",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourcePolicyDescriptorId(u64);

impl ResourcePolicyDescriptorId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourcePolicyVersion {
    major: u16,
    minor: u16,
}

impl ResourcePolicyVersion {
    pub const INITIAL: Self = Self { major: 1, minor: 0 };

    pub fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    pub fn major(self) -> u16 {
        self.major
    }

    pub fn minor(self) -> u16 {
        self.minor
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourcePolicyDigest(String);

impl ResourcePolicyDigest {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourcePolicyCompatibilityPosture {
    ExactDescriptorMatch,
    CompatibleVersion,
    IncompatibleVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourcePolicySelectionBasis {
    BuiltInDefault,
    DeclaredBuiltIn,
    DeclaredName,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourcePolicyDescriptor {
    id: ResourcePolicyDescriptorId,
    kind: ResourcePolicyKind,
    semantic_name: ResourcePolicyName,
    version: ResourcePolicyVersion,
    descriptor_digest: ResourcePolicyDigest,
    cost_contract: ResourceCostContractId,
    compatibility_posture: ResourcePolicyCompatibilityPosture,
}

impl ResourcePolicyDescriptor {
    pub(crate) fn new(
        id: ResourcePolicyDescriptorId,
        kind: ResourcePolicyKind,
        semantic_name: ResourcePolicyName,
        version: ResourcePolicyVersion,
        cost_contract: ResourceCostContractId,
        compatibility_posture: ResourcePolicyCompatibilityPosture,
    ) -> Self {
        let descriptor_digest = descriptor_digest(id, kind, &semantic_name, version, cost_contract);
        Self {
            id,
            kind,
            semantic_name,
            version,
            descriptor_digest,
            cost_contract,
            compatibility_posture,
        }
    }

    pub fn id(&self) -> ResourcePolicyDescriptorId {
        self.id
    }

    pub fn kind(&self) -> ResourcePolicyKind {
        self.kind
    }

    pub fn semantic_name(&self) -> &ResourcePolicyName {
        &self.semantic_name
    }

    pub fn version(&self) -> ResourcePolicyVersion {
        self.version
    }

    pub fn descriptor_digest(&self) -> &ResourcePolicyDigest {
        &self.descriptor_digest
    }

    pub fn cost_contract(&self) -> ResourceCostContractId {
        self.cost_contract
    }

    pub fn compatibility_posture(&self) -> ResourcePolicyCompatibilityPosture {
        self.compatibility_posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceResolvedPolicy {
    descriptor: ResourcePolicyDescriptor,
    selection_basis: ResourcePolicySelectionBasis,
    parameter_digest: ResourcePolicyDigest,
    resolved_digest: ResourcePolicyDigest,
}

impl ResourceResolvedPolicy {
    fn new(
        descriptor: ResourcePolicyDescriptor,
        selection_basis: ResourcePolicySelectionBasis,
        parameter_digest: ResourcePolicyDigest,
    ) -> Self {
        let resolved_digest = ResourcePolicyDigest::new(format!(
            "resolved-policy:{}:{}:{}:{}",
            descriptor.kind().as_str(),
            descriptor.descriptor_digest().as_str(),
            selection_basis.as_str(),
            parameter_digest.as_str()
        ));
        Self {
            descriptor,
            selection_basis,
            parameter_digest,
            resolved_digest,
        }
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

    pub fn resolved_digest(&self) -> &ResourcePolicyDigest {
        &self.resolved_digest
    }
}

impl ResourcePolicySelectionBasis {
    fn as_str(self) -> &'static str {
        match self {
            Self::BuiltInDefault => "built-in-default",
            Self::DeclaredBuiltIn => "declared-built-in",
            Self::DeclaredName => "declared-name",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceResolvedPolicyBundle {
    retry: ResourceResolvedPolicy,
    timeout: ResourceResolvedPolicy,
    cancellation: ResourceResolvedPolicy,
    stale_after: ResourceResolvedPolicy,
    supersession: ResourceResolvedPolicy,
    revalidation: ResourceResolvedPolicy,
    observation: ResourceResolvedPolicy,
    output_continuity: ResourceResolvedPolicy,
    retention: ResourceResolvedPolicy,
    bundle_digest: ResourcePolicyDigest,
}

impl ResourceResolvedPolicyBundle {
    pub(crate) fn from_declaration(
        declaration: &ResourceNodeDeclaration,
        registry: &FrozenResourcePolicyRegistry,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        let retry = registry.resolve_retry(declaration.retry_policy())?;
        let timeout = registry.resolve_timeout(declaration.timeout_policy())?;
        let cancellation = registry.resolve_cancellation(declaration.cancellation_policy())?;
        let stale_after = registry.resolve_stale_after(declaration.stale_after_policy())?;
        let supersession = registry.resolve_supersession(declaration.supersession_policy())?;
        let revalidation = registry.resolve_revalidation(declaration.revalidation_policy())?;
        let observation = registry.resolve_observation(declaration.observation_policy())?;
        let output_continuity =
            registry.resolve_output_continuity(declaration.output_continuity_policy())?;
        let retention = registry.resolve_retention(declaration.retention_policy())?;
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
        ]);
        Ok(Self {
            retry,
            timeout,
            cancellation,
            stale_after,
            supersession,
            revalidation,
            observation,
            output_continuity,
            retention,
            bundle_digest,
        })
    }

    pub fn retry(&self) -> &ResourceResolvedPolicy {
        &self.retry
    }

    pub fn timeout(&self) -> &ResourceResolvedPolicy {
        &self.timeout
    }

    pub fn bundle_digest(&self) -> &ResourcePolicyDigest {
        &self.bundle_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourcePolicyRegistration {
    descriptor: ResourcePolicyDescriptor,
}

impl ResourcePolicyRegistration {
    pub fn new(descriptor: ResourcePolicyDescriptor) -> Self {
        Self { descriptor }
    }

    pub fn descriptor(&self) -> &ResourcePolicyDescriptor {
        &self.descriptor
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourcePolicyRegistryError {
    DuplicateId(ResourcePolicyDescriptorId),
    DuplicateName {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourcePolicyResolutionError {
    UnknownPolicy {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenResourcePolicyRegistry {
    descriptors: Vec<ResourcePolicyDescriptor>,
    index_by_id: BTreeMap<ResourcePolicyDescriptorId, usize>,
    index_by_kind_name: BTreeMap<(ResourcePolicyKind, ResourcePolicyName), usize>,
}

impl FrozenResourcePolicyRegistry {
    pub fn new(
        registrations: Vec<ResourcePolicyRegistration>,
    ) -> Result<Self, ResourcePolicyRegistryError> {
        let mut descriptors = Vec::with_capacity(registrations.len());
        let mut index_by_id = BTreeMap::new();
        let mut index_by_kind_name = BTreeMap::new();

        for registration in registrations {
            let descriptor = registration.descriptor;
            let index = descriptors.len();
            if index_by_id.insert(descriptor.id(), index).is_some() {
                return Err(ResourcePolicyRegistryError::DuplicateId(descriptor.id()));
            }
            let kind_name = (descriptor.kind(), descriptor.semantic_name().clone());
            if index_by_kind_name
                .insert(kind_name.clone(), index)
                .is_some()
            {
                let (kind, name) = kind_name;
                return Err(ResourcePolicyRegistryError::DuplicateName { kind, name });
            }
            descriptors.push(descriptor);
        }

        Ok(Self {
            descriptors,
            index_by_id,
            index_by_kind_name,
        })
    }

    pub fn built_in() -> Self {
        Self::new(built_in_policy_registrations()).expect("built-in resource policy registry")
    }

    pub fn resolve_by_name(
        &self,
        kind: ResourcePolicyKind,
        name: &ResourcePolicyName,
    ) -> Option<&ResourcePolicyDescriptor> {
        self.index_by_kind_name
            .get(&(kind, name.clone()))
            .and_then(|index| self.descriptors.get(*index))
    }

    fn resolve_named(
        &self,
        kind: ResourcePolicyKind,
        name: &ResourcePolicyName,
    ) -> Result<ResourceResolvedPolicy, ResourcePolicyResolutionError> {
        let descriptor = self.resolve_by_name(kind, name).cloned().ok_or_else(|| {
            ResourcePolicyResolutionError::UnknownPolicy {
                kind,
                name: name.clone(),
            }
        })?;
        Ok(ResourceResolvedPolicy::new(
            descriptor,
            ResourcePolicySelectionBasis::DeclaredName,
            ResourcePolicyDigest::new(format!("named:{}", name.as_str())),
        ))
    }

    fn built_in_policy(
        &self,
        kind: ResourcePolicyKind,
        name: &str,
        selection_basis: ResourcePolicySelectionBasis,
        parameter_digest: ResourcePolicyDigest,
    ) -> ResourceResolvedPolicy {
        let descriptor = self
            .resolve_by_name(kind, &ResourcePolicyName::new(name))
            .expect("built-in resource policy descriptor exists")
            .clone();
        ResourceResolvedPolicy::new(descriptor, selection_basis, parameter_digest)
    }

    fn resolve_retry(
        &self,
        policy: &ResourceRetryPolicyDeclaration,
    ) -> Result<ResourceResolvedPolicy, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceRetryPolicyDeclaration::Disabled => self.built_in_policy(
                ResourcePolicyKind::Retry,
                "signal.resource.retry.disabled",
                ResourcePolicySelectionBasis::BuiltInDefault,
                ResourcePolicyDigest::new("retry:disabled"),
            ),
            ResourceRetryPolicyDeclaration::RuntimeBackoff { delay } => self.built_in_policy(
                ResourcePolicyKind::Retry,
                "signal.resource.retry.runtime-backoff",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new(format!("retry:runtime-backoff:{}", delay.get())),
            ),
            ResourceRetryPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Retry, name)?
            }
        })
    }

    fn resolve_timeout(
        &self,
        policy: &ResourceTimeoutPolicyDeclaration,
    ) -> Result<ResourceResolvedPolicy, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceTimeoutPolicyDeclaration::Disabled => self.built_in_policy(
                ResourcePolicyKind::Timeout,
                "signal.resource.timeout.disabled",
                ResourcePolicySelectionBasis::BuiltInDefault,
                ResourcePolicyDigest::new("timeout:disabled"),
            ),
            ResourceTimeoutPolicyDeclaration::RuntimeTimeout { timeout } => self.built_in_policy(
                ResourcePolicyKind::Timeout,
                "signal.resource.timeout.runtime-timeout",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new(format!("timeout:runtime-timeout:{}", timeout.get())),
            ),
            ResourceTimeoutPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Timeout, name)?
            }
        })
    }

    fn resolve_cancellation(
        &self,
        policy: &ResourceCancellationPolicyDeclaration,
    ) -> Result<ResourceResolvedPolicy, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceCancellationPolicyDeclaration::RuntimeDenialOnly => self.built_in_policy(
                ResourcePolicyKind::Cancellation,
                "signal.resource.cancellation.runtime-denial-only",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("cancellation:runtime-denial-only"),
            ),
            ResourceCancellationPolicyDeclaration::BestEffortHostSignalAndRuntimeDenial => self
                .built_in_policy(
                    ResourcePolicyKind::Cancellation,
                    "signal.resource.cancellation.best-effort-host-signal-and-runtime-denial",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new(
                        "cancellation:best-effort-host-signal-and-runtime-denial",
                    ),
                ),
            ResourceCancellationPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Cancellation, name)?
            }
        })
    }

    fn resolve_stale_after(
        &self,
        policy: &ResourceStaleAfterPolicyDeclaration,
    ) -> Result<ResourceResolvedPolicy, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceStaleAfterPolicyDeclaration::Disabled => self.built_in_policy(
                ResourcePolicyKind::StaleAfter,
                "signal.resource.stale-after.disabled",
                ResourcePolicySelectionBasis::BuiltInDefault,
                ResourcePolicyDigest::new("stale-after:disabled"),
            ),
            ResourceStaleAfterPolicyDeclaration::RuntimeStaleAfter { stale_after } => self
                .built_in_policy(
                    ResourcePolicyKind::StaleAfter,
                    "signal.resource.stale-after.runtime-stale-after",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(format!(
                        "stale-after:runtime-stale-after:{}",
                        stale_after.get()
                    )),
                ),
            ResourceStaleAfterPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::StaleAfter, name)?
            }
        })
    }

    fn resolve_supersession(
        &self,
        policy: &ResourceSupersessionPolicyDeclaration,
    ) -> Result<ResourceResolvedPolicy, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceSupersessionPolicyDeclaration::NewGenerationSupersedesPrior => self
                .built_in_policy(
                    ResourcePolicyKind::Supersession,
                    "signal.resource.supersession.new-generation-supersedes-prior",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new("supersession:new-generation-supersedes-prior"),
                ),
            ResourceSupersessionPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Supersession, name)?
            }
        })
    }

    fn resolve_revalidation(
        &self,
        policy: &ResourceRevalidationPolicyDeclaration,
    ) -> Result<ResourceResolvedPolicy, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceRevalidationPolicyDeclaration::ExplicitIntentOnly => self.built_in_policy(
                ResourcePolicyKind::Revalidation,
                "signal.resource.revalidation.explicit-intent-only",
                ResourcePolicySelectionBasis::BuiltInDefault,
                ResourcePolicyDigest::new("revalidation:explicit-intent-only"),
            ),
            ResourceRevalidationPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Revalidation, name)?
            }
        })
    }

    fn resolve_observation(
        &self,
        policy: &ResourceObservationPolicyDeclaration,
    ) -> Result<ResourceResolvedPolicy, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceObservationPolicyDeclaration::LifecycleOnly => self.built_in_policy(
                ResourcePolicyKind::Observation,
                "signal.resource.observation.lifecycle-only",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("observation:lifecycle-only"),
            ),
            ResourceObservationPolicyDeclaration::LifecycleAndOutput => self.built_in_policy(
                ResourcePolicyKind::Observation,
                "signal.resource.observation.lifecycle-and-output",
                ResourcePolicySelectionBasis::BuiltInDefault,
                ResourcePolicyDigest::new("observation:lifecycle-and-output"),
            ),
            ResourceObservationPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Observation, name)?
            }
        })
    }

    fn resolve_output_continuity(
        &self,
        policy: &ResourceOutputContinuityPolicyDeclaration,
    ) -> Result<ResourceResolvedPolicy, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceOutputContinuityPolicyDeclaration::PreserveLifecycleOutputSeparation => self
                .built_in_policy(
                    ResourcePolicyKind::OutputContinuity,
                    "signal.resource.output-continuity.preserve-lifecycle-output-separation",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new(
                        "output-continuity:preserve-lifecycle-output-separation",
                    ),
                ),
            ResourceOutputContinuityPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::OutputContinuity, name)?
            }
        })
    }

    fn resolve_retention(
        &self,
        policy: &ResourceRetentionPolicyDeclaration,
    ) -> Result<ResourceResolvedPolicy, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceRetentionPolicyDeclaration::RetainOperationalLifecycleSummary => self
                .built_in_policy(
                    ResourcePolicyKind::Retention,
                    "signal.resource.retention.retain-operational-lifecycle-summary",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new("retention:retain-operational-lifecycle-summary"),
                ),
            ResourceRetentionPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Retention, name)?
            }
        })
    }
}

fn built_in_policy_registrations() -> Vec<ResourcePolicyRegistration> {
    let rows = [
        (
            0,
            ResourcePolicyKind::Retry,
            "signal.resource.retry.disabled",
            5,
        ),
        (
            1,
            ResourcePolicyKind::Retry,
            "signal.resource.retry.runtime-backoff",
            5,
        ),
        (
            2,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.disabled",
            4,
        ),
        (
            3,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.runtime-timeout",
            4,
        ),
        (
            4,
            ResourcePolicyKind::Cancellation,
            "signal.resource.cancellation.runtime-denial-only",
            3,
        ),
        (
            5,
            ResourcePolicyKind::Cancellation,
            "signal.resource.cancellation.best-effort-host-signal-and-runtime-denial",
            3,
        ),
        (
            6,
            ResourcePolicyKind::StaleAfter,
            "signal.resource.stale-after.disabled",
            7,
        ),
        (
            7,
            ResourcePolicyKind::StaleAfter,
            "signal.resource.stale-after.runtime-stale-after",
            7,
        ),
        (
            8,
            ResourcePolicyKind::Supersession,
            "signal.resource.supersession.new-generation-supersedes-prior",
            1,
        ),
        (
            9,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-intent-only",
            7,
        ),
        (
            10,
            ResourcePolicyKind::Observation,
            "signal.resource.observation.lifecycle-only",
            7,
        ),
        (
            11,
            ResourcePolicyKind::Observation,
            "signal.resource.observation.lifecycle-and-output",
            7,
        ),
        (
            12,
            ResourcePolicyKind::OutputContinuity,
            "signal.resource.output-continuity.preserve-lifecycle-output-separation",
            7,
        ),
        (
            13,
            ResourcePolicyKind::Retention,
            "signal.resource.retention.retain-operational-lifecycle-summary",
            7,
        ),
    ];

    rows.into_iter()
        .map(|(id, kind, name, contract)| {
            ResourcePolicyRegistration::new(ResourcePolicyDescriptor::new(
                ResourcePolicyDescriptorId::new(id),
                kind,
                ResourcePolicyName::new(name),
                ResourcePolicyVersion::INITIAL,
                ResourceCostContractId::new(contract),
                ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
            ))
        })
        .collect()
}

fn descriptor_digest(
    id: ResourcePolicyDescriptorId,
    kind: ResourcePolicyKind,
    semantic_name: &ResourcePolicyName,
    version: ResourcePolicyVersion,
    cost_contract: ResourceCostContractId,
) -> ResourcePolicyDigest {
    ResourcePolicyDigest::new(format!(
        "resource-policy-descriptor:{}:{}:{}:{}.{}:{}",
        id.get(),
        kind.as_str(),
        semantic_name.as_str(),
        version.major(),
        version.minor(),
        cost_contract.get()
    ))
}

fn bundle_digest(policies: &[&ResourceResolvedPolicy]) -> ResourcePolicyDigest {
    let joined = policies
        .iter()
        .map(|policy| policy.resolved_digest().as_str())
        .collect::<Vec<_>>()
        .join("|");
    ResourcePolicyDigest::new(format!("resource-policy-bundle:{joined}"))
}
